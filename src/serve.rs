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
use tokio_util::io::ReaderStream;

use crate::{async_util::extract_file_details, AppState};

pub mod path;
pub mod root;

pub async fn handle_root(State(state): State<Arc<RwLock<AppState>>>) -> impl IntoResponse {
    let base_path = PathBuf::from(&state.read().await.root_dir);
    let path = match get_canonicalized_path(&base_path, "").await {
        Ok(path) => path,
        Err(status) => return (status, "File not found").into_response(),
    };
    serve_directory(&path).await.into_response()
}

async fn get_canonicalized_path(
    base_path: &Path,
    requested_path: &str,
) -> Result<PathBuf, StatusCode> {
    let mut path = base_path.to_owned();
    if !requested_path.is_empty() && requested_path != "/" {
        path.push(requested_path.trim_start_matches('/'));
    }

    tokio::fs::canonicalize(&path).await.map_err(|e| {
        tracing::error!("Error canonicalizing path: {}", e);
        StatusCode::NOT_FOUND
    })
}

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

    let base_path = PathBuf::from(&state.read().await.root_dir);
    let path = match get_canonicalized_path(&base_path, requested_path).await {
        Ok(path) => path,
        Err(status) => return (status, "File not found").into_response(),
    };

    tracing::debug!("Requested path: {}", path.display());

    if path.is_dir() {
        return serve_directory(&path).await.into_response();
    }

    let file = match File::open(&path).await {
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
        return handle_range_request(range.clone(), file, file_size, content_type.clone())
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
    mut file: File,
    file_size: u64,
    content_type: String,
) -> impl IntoResponse {
    let range = crate::util::parse_range_header(&range_header, file_size);
    match range {
        Ok((start, end)) => {
            let length = end - start + 1;
            if let Err(e) = file.seek(SeekFrom::Start(start)).await {
                tracing::error!("Error seeking file: {}", e);
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }

            let stream = ReaderStream::new(file.take(length));
            let body = Body::from_stream(stream);

            (
                StatusCode::PARTIAL_CONTENT,
                [
                    (header::CONTENT_TYPE, content_type),
                    (header::ACCEPT_RANGES, "bytes".to_string()),
                    (
                        header::CONTENT_RANGE,
                        format!("bytes {}-{}/{}", start, end, file_size),
                    ),
                    (header::CONTENT_LENGTH, length.to_string()),
                ],
                body,
            )
                .into_response()
        }
        Err(status) => status.into_response(),
    }
}
