use axum::body::Body;
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use std::io::{Cursor, Write};
use std::path::Path;
use walkdir::WalkDir;
use zip::write::ZipWriter;
use zip::write::{ExtendedFileOptions, FileOptions};
use zip::CompressionMethod;

fn create_zip_archive(base_dir: &Path) -> std::io::Result<Vec<u8>> {
    let mut zip = ZipWriter::new(Cursor::new(Vec::new()));

    let options: FileOptions<'_, ExtendedFileOptions> = FileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .unix_permissions(0o755);

    for entry in WalkDir::new(base_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let file_path = entry.path();
        let Ok(relative_path) = file_path.strip_prefix(base_dir) else {
            tracing::warn!(?file_path, "Skipping path with invalid prefix");
            continue;
        };

        zip.start_file(relative_path.to_string_lossy(), options.clone())?;

        let file_buffer = std::fs::read(file_path)?;
        tracing::debug!(
            ?file_path,
            read_bytes = file_buffer.len(),
            "Read file, writing to zip"
        );
        zip.write_all(&file_buffer)?;
    }

    let final_cursor = zip.finish()?;
    Ok(final_cursor.into_inner())
}

pub async fn zip_directory(path: &Path) -> Response {
    let base_dir = path.to_path_buf();

    match tokio::task::spawn_blocking(move || create_zip_archive(&base_dir)).await {
        Ok(Ok(zip_data)) => {
            let file_name = path
                .file_name()
                .unwrap_or(path.as_os_str())
                .to_string_lossy();

            let content_disposition = format!(r#"attachment; filename="{file_name}.zip""#);

            (
                StatusCode::OK,
                [
                    (header::CONTENT_TYPE, "application/zip"),
                    (header::CONTENT_DISPOSITION, &content_disposition),
                ],
                Body::from(zip_data),
            )
                .into_response()
        }
        Ok(Err(e)) => {
            tracing::error!("Failed to create zip archive: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create zip archive",
            )
                .into_response()
        }
        Err(e) => {
            tracing::error!("Zipping task panicked: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create zip archive",
            )
                .into_response()
        }
    }
}
