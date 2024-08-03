use std::{
    io,
    path::{Path, PathBuf},
};

use crate::{
    dir_entry::FifeDirEntry,
    icon::FileTypeCategory,
    util::{format_data_size, format_system_time},
};
use axum::http::StatusCode;
use futures_util::stream::StreamExt;
use tokio_stream::wrappers::ReadDirStream;

async fn get_file_info(entry: &tokio::fs::DirEntry) -> std::io::Result<(String, String, String)> {
    let metadata = entry.metadata().await?;
    let file_type = entry.file_type().await?;

    let file_size = if file_type.is_file() {
        format_data_size(metadata.len())
    } else if file_type.is_dir() {
        let count = count_directory_entries(entry.path()).await?;
        format!("{} items", count)
    } else if file_type.is_symlink() {
        "Symbolic Link".to_string()
    } else {
        "".to_string()
    };

    let modified_date = format_system_time(metadata.modified()?);

    Ok((file_size, modified_date, format_data_size(metadata.len())))
}

async fn count_directory_entries(path: impl AsRef<Path>) -> io::Result<usize> {
    let read_dir = tokio::fs::read_dir(path).await?;
    let stream = ReadDirStream::new(read_dir);
    Ok(stream.count().await)
}

pub async fn extract_file_details(entry: &tokio::fs::DirEntry) -> Result<FifeDirEntry, ()> {
    let file_name = match entry.file_name().into_string() {
        Ok(name) => name,
        Err(e) => {
            tracing::error!("Invalid filename encountered: {e:?}");
            return Err(());
        }
    };

    let file_type = match entry.file_type().await {
        Ok(file_type) => file_type,
        Err(e) => {
            tracing::error!("Error getting file type: {}", e);
            return Err(());
        }
    };

    let (file_size, modified_date, _metadata_len) = match get_file_info(entry).await {
        Ok(info) => info,
        Err(e) => {
            tracing::error!("Error getting file info: {}", e);
            (
                "Unknown size".to_string(),
                "Unknown date".to_string(),
                "".to_string(),
            )
        }
    };

    let path = entry.path();
    let ext = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");

    let file_type_category = if file_name.starts_with(".git") {
        FileTypeCategory::Git
    } else if file_name == "Justfile" || file_name == "justfile" {
        FileTypeCategory::ShellScript
    } else if file_type.is_dir() {
        tracing::debug!("Determing type of directory: {file_name}");

        match crate::util::is_directory_empty(&path) {
            Ok(true) => FileTypeCategory::DirectoryEmpty,
            Ok(false) => FileTypeCategory::Directory,
            Err(e) => {
                tracing::error!("Error checking if directory '{file_name}' is empty: {}", e);
                FileTypeCategory::Directory // Default to non-empty if there's an error
            }
        }
    } else if file_type.is_symlink() {
        FileTypeCategory::SymbolicLink
    } else {
        FileTypeCategory::from_extension(ext)
    };
    tracing::debug!(
        "'{file_name}' file categori is: {}",
        file_type_category.description()
    );

    Ok(FifeDirEntry::new(
        file_name,
        file_type,
        file_size,
        file_type_category,
        modified_date,
    ))
}

pub async fn get_canonicalized_path(
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
