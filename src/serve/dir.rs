use std::path::Path;

use axum::{
    http::{header, StatusCode},
    response::IntoResponse,
};

use crate::async_util;

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

    let mut response = String::new();
    response.push_str(
        r#"
<!DOCTYPE html><html lang="en"><head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>"#,
    );
    response.push_str(&path.display().to_string());
    response.push_str("</title>");
    response.push_str(
    r#"<style>
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
    </style></head><body>"#,
    );

    response.push_str(&format!("<h1>{}</h1>", path.display()));

    response.push_str("<ul>");

    let mut dir_entries = vec![];
    while let Some(entry) = entries.next_entry().await.transpose() {
        match entry {
            Ok(entry) => {
                if let Ok(fife_dir_entry) = async_util::extract_file_details(&entry).await {
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

    tracing::trace!("Returning directory listing");
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/html")],
        response,
    )
        .into_response()
}
