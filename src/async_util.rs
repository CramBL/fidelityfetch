use std::path::Path;

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

    let path = Path::new(&file_name);
    let ext = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");

    let file_type_category = if file_type.is_dir() {
        if file_name == ".git" {
            FileTypeCategory::Git
        } else if path.read_dir().is_ok_and(|e| e.count() == 0) {
            FileTypeCategory::DirectoryEmpty
        } else {
            FileTypeCategory::Directory
        }
    } else {
        FileTypeCategory::from_extension(ext)
    };

    Ok(FifeDirEntry::new(
        file_name,
        file_type,
        file_size,
        file_type_category,
        modified_date,
    ))
}
