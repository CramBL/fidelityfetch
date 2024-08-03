use std::{os::unix::fs::MetadataExt, path::Path};

use crate::{
    dir_entry::FifeDirEntry,
    icon::FileTypeCategory,
    util::{format_data_size, format_system_time},
};

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

    let file_size = if file_type.is_file() {
        match entry.metadata().await {
            Ok(metadata) => format_data_size(metadata.len()),
            Err(_) => "Unknown size".to_string(),
        }
    } else {
        "".to_string()
    };

    let modified_date = if file_type.is_file() {
        match entry.metadata().await {
            Ok(metadata) => match metadata.modified() {
                Ok(time) => format_system_time(time),
                Err(_) => "Unknown date".to_string(),
            },
            Err(_) => "Unknown date".to_string(),
        }
    } else {
        "".to_string()
    };

    let path = entry.path();
    let ext = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");

    let file_type_category = if file_type.is_dir() {
        tracing::debug!("Determing type of directory: {file_name}");
        if file_name == ".git" || file_name == ".gitignore" {
            FileTypeCategory::Git
        } else {
            match is_directory_empty(&path) {
                Ok(true) => FileTypeCategory::DirectoryEmpty,
                Ok(false) => FileTypeCategory::Directory,
                Err(e) => {
                    tracing::error!("Error checking if directory '{file_name}' is empty: {}", e);
                    FileTypeCategory::Directory // Default to non-empty if there's an error
                }
            }
        }
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

fn is_directory_empty(path: &Path) -> std::io::Result<bool> {
    let entries = std::fs::read_dir(path)?;
    Ok(entries.filter_map(Result::ok).next().is_none())
}
