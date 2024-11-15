use std::path::Path;

use axum::{
    http::{header, StatusCode},
    response::IntoResponse,
};

use crate::async_util;

const FIFE_DIRECTORY_EMPTY_HTML: &str = "<html><body><h1>Empty directory</h1></body></html>";

pub async fn serve_directory(path: &Path) -> impl IntoResponse {
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

    let mut dir_entries = vec![];
    while let Some(entry) = entries.next_entry().await.transpose() {
        match entry {
            Ok(entry) => {
                if let Ok(file_details) = async_util::extract_file_details(&entry).await {
                    dir_entries.push(file_details);
                }
            }
            Err(e) => {
                tracing::error!("Error reading directory entry: {}", e);
                break;
            }
        }
    }

    if dir_entries.is_empty() {
        tracing::trace!("Empty directory");
        return (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "text/html")],
            FIFE_DIRECTORY_EMPTY_HTML.to_owned(),
        )
            .into_response();
    }

    // Sort entries alphabetically with directories first
    dir_entries.sort_unstable_by(|a, b| match (a.ftype.is_dir(), b.ftype.is_dir()) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.cmp(&b.name),
    });

    let entries_html: String = dir_entries.into_iter().map(|e| e.to_html()).collect();

    let response =
        super::html::build_html_response(path.display().to_string().as_str(), &entries_html);
    tracing::trace!(response_len = response.len(), "Returning directory listing");
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/html")],
        response,
    )
        .into_response()
}
