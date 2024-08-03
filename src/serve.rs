use axum::{
    body::Body,
    extract::State,
    http::{header, Request, StatusCode},
    response::IntoResponse,
};
use std::{
    io::SeekFrom,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::io::AsyncReadExt;
use tokio::sync::RwLock;
use tokio::{fs::*, io::AsyncSeekExt};

use crate::{async_util::extract_file_details, AppState};

pub async fn serve_path(
    State(state): State<Arc<RwLock<AppState>>>,
    req: Request<Body>,
) -> impl IntoResponse {
    let requested_path = req.uri().path();
    tracing::info!("Requested: {requested_path}");
    if requested_path == "/favicon.ico" {
        return (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "image/x-icon")],
            vec![],
        )
            .into_response();
    }

    let mut path = PathBuf::from(&state.read().await.root_dir);
    path.push(requested_path.trim_start_matches('/'));
    tracing::info!("Requested trimmed: {path:?}");

    // Resolve the absolute, normalized path
    let path = match tokio::fs::canonicalize(&path).await {
        Ok(normalized_path) => normalized_path,
        Err(e) => {
            tracing::error!("Error canonicalizing path: {}", e);
            return (StatusCode::NOT_FOUND, "File not found").into_response();
        }
    };

    tracing::info!("Requested path: {}", path.display());

    if path.is_dir() {
        return serve_directory(&path).await.into_response();
    }

    let mut file = match File::open(&path).await {
        Ok(file) => file,
        Err(e) => {
            tracing::error!("Error opening file: {}", e);
            return (StatusCode::NOT_FOUND, "File not found").into_response();
        }
    };

    let metadata = match file.metadata().await {
        Ok(metadata) => metadata,
        Err(e) => {
            tracing::error!("Error reading metadata: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Error reading metadata").into_response();
        }
    };

    let file_size = metadata.len();
    let content_type = mime_guess::from_path(path)
        .first_or_octet_stream()
        .to_string();

    if let Some(range) = req.headers().get(header::RANGE) {
        return handle_range_request(range.clone(), &mut file, file_size, content_type.clone())
            .await
            .into_response();
    }

    let stream = tokio_util::io::ReaderStream::new(file);
    let body = Body::from_stream(stream);

    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, content_type),
            (header::ACCEPT_RANGES, "bytes".to_string()),
            (header::CONTENT_LENGTH, file_size.to_string()),
        ],
        body,
    )
        .into_response()
}

pub async fn handle_root(State(state): State<Arc<RwLock<AppState>>>) -> impl IntoResponse {
    let path = PathBuf::from(&state.read().await.root_dir);
    let path = match tokio::fs::canonicalize(&path).await {
        Ok(normalized_path) => normalized_path,
        Err(e) => {
            tracing::error!("Error canonicalizing path: {}", e);
            return (StatusCode::NOT_FOUND, "File not found").into_response();
        }
    };
    serve_directory(&path).await.into_response()
}

async fn serve_directory(path: &Path) -> impl IntoResponse {
    tracing::info!("Serving directory: {:?}", path);
    let mut entries = match tokio::fs::read_dir(path).await {
        Ok(entries) => entries,
        Err(e) => {
            tracing::error!("Error reading directory: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read directory",
            )
                .into_response();
        }
    };

    let mut response = String::new();
    response.push_str(
        r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Directory Listing</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; background-color: #f4f4f4; }
        ul { list-style-type: none; padding: 0; }
        li { display: flex; align-items: center; padding: 10px; background-color: #fff; border-radius: 5px; margin-bottom: 0px; box-shadow: 0 2px 5px rgba(0, 0, 0, 0.1); }
        .file-icon { font-size: 24px; margin-right: 10px; }
        .file-details { flex: 1; }
        .file-name { font-size: 16px; text-decoration: none; color: #333; font-weight: bold; }
        .file-name:hover { text-decoration: underline; }
        .file-info { color: #888; font-size: 14px; }
        .file-size, .file-date { display: inline-block; margin-right: 15px; }
        .directory { color: #0056b3; }
        .file { color: #333; }
    </style>
</head>
<body>
"#,
    );

    response.push_str(&format!("<h1>{}</h1>", path.display()));

    response.push_str("<ul>");

    let mut dir_entries = vec![];
    while let Some(entry) = entries.next_entry().await.transpose() {
        match entry {
            Ok(entry) => {
                if let Ok(fife_dir_entry) = extract_file_details(&entry).await {
                    dir_entries.push(fife_dir_entry)
                }
            }
            Err(e) => {
                tracing::error!("Error reading directory entry: {}", e);
                break;
            }
        }
    }

    let is_dir_empty = dir_entries.is_empty();

    dir_entries.sort_unstable_by(|a, b| {
        let a_is_dir = a.ftype.is_dir();
        let b_is_dir = b.ftype.is_dir();
        if a_is_dir && !b_is_dir {
            std::cmp::Ordering::Less
        } else if !a_is_dir && b_is_dir {
            std::cmp::Ordering::Greater
        } else {
            a.name.cmp(&b.name)
        }
    });

    let entry_html = dir_entries.into_iter().fold(String::new(), |mut acc, e| {
        acc.push_str(&e.to_html());
        acc
    });

    response.push_str(&entry_html);
    response.push_str("</ul>");
    response.push_str("</body></html>");

    if is_dir_empty {
        response = "<html><body><h1>Empty directory</h1></body></html>".to_string();
    }

    tracing::info!("Returning directory listing");
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/html")],
        response,
    )
        .into_response()
}

async fn handle_range_request(
    range_header: header::HeaderValue,
    file: &mut File,
    file_size: u64,
    header_content_type: String,
) -> impl IntoResponse {
    if let Ok(range_str) = range_header.to_str() {
        if let Some(range) = range_str.strip_prefix("bytes=") {
            let (start_str, end_str) = range.split_once('-').unwrap_or((range, ""));
            let start = start_str.parse::<u64>().unwrap_or(0);
            let end = match end_str.parse::<u64>() {
                Ok(val) => val,
                Err(_) => file_size.saturating_sub(1),
            };

            if start >= file_size || end >= file_size || start > end {
                return (
                    StatusCode::RANGE_NOT_SATISFIABLE,
                    [(header::CONTENT_RANGE, format!("bytes */{}", file_size))],
                    "Invalid range",
                )
                    .into_response();
            }

            let length = end - start + 1;
            let mut buffer = vec![0; length as usize];
            file.seek(SeekFrom::Start(start)).await.unwrap();
            file.read_exact(&mut buffer).await.unwrap();

            let header_content_length = (header::CONTENT_LENGTH, buffer.len().to_string());
            let header_content_range = (
                header::CONTENT_RANGE,
                format!("bytes {}-{}/{}", start, end, file_size),
            );

            let resp = (
                StatusCode::PARTIAL_CONTENT,
                [
                    (header::CONTENT_TYPE, header_content_type),
                    (header::ACCEPT_RANGES, "bytes".to_owned()),
                    header_content_range,
                    header_content_length,
                ],
                buffer.into_response(),
            );

            let resp_finalized = resp.into_response();
            tracing::info!("Sending response: {:?}", resp_finalized);

            return resp_finalized;
        }
    }

    StatusCode::RANGE_NOT_SATISFIABLE.into_response()
}
