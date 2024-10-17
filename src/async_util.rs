use std::{
    io,
    path::{Path, PathBuf},
};

use crate::{
    dir_entry::FifeDirEntry,
    icon::FileTypeCategory,
    util::{self, format_data_size, format_system_time},
};
use axum::http::StatusCode;
use futures_util::stream::StreamExt;
use percent_encoding::percent_decode_str;
use tokio_stream::wrappers::ReadDirStream;

/// Get file info for a directory entry
async fn get_file_info(entry: &tokio::fs::DirEntry) -> std::io::Result<(String, String, String)> {
    let metadata = entry.metadata().await?;
    let file_type = entry.file_type().await?;

    let file_size = if file_type.is_file() {
        format_data_size(metadata.len())
    } else if file_type.is_dir() {
        let count = count_directory_entries(entry.path()).await?;
        format!("{} item{}", count, if count == 1 { "" } else { "s" })
    } else if file_type.is_symlink() {
        "Symbolic Link".to_string()
    } else {
        "".to_string()
    };

    let modified_date = format_system_time(metadata.modified()?);

    Ok((file_size, modified_date, format_data_size(metadata.len())))
}

/// Count the number of directory entries
async fn count_directory_entries(path: impl AsRef<Path>) -> io::Result<usize> {
    let read_dir = tokio::fs::read_dir(path).await?;
    let stream = ReadDirStream::new(read_dir);
    Ok(stream.count().await)
}

/// Extract file details from a directory entry
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
        tracing::debug!("Determining type of directory: {file_name}");

        match util::is_directory_empty(&path) {
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
        let ext_lower = ext.to_lowercase();
        FileTypeCategory::from_extension_lower(&ext_lower)
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
        let decoded_path = percent_decode_str(requested_path)
            .decode_utf8()
            .map_err(|_| StatusCode::BAD_REQUEST)?;
        path.push(decoded_path.trim_start_matches('/'));
    }
    tokio::fs::canonicalize(&path).await.map_err(|e| {
        tracing::error!("Error canonicalizing path: {}", e);
        StatusCode::NOT_FOUND
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use temp_dir::TempDir;
    use testresult::TestResult;
    use tokio::fs::File;

    #[tokio::test]
    async fn test_get_canonicalized_path() -> TestResult {
        let temp_dir = TempDir::new()?;
        let base_path = temp_dir.path();

        // Create test directories and files
        tokio::fs::create_dir_all(base_path.join("folder with spaces")).await?;
        tokio::fs::create_dir_all(base_path.join("æøå")).await?;
        tokio::fs::create_dir_all(base_path.join("한글")).await?;
        tokio::fs::create_dir_all(base_path.join("日本語")).await?;
        File::create(base_path.join("file with spaces.txt")).await?;

        // Test cases
        let test_cases = vec![
            ("folder%20with%20spaces", "folder with spaces"),
            ("%C3%A6%C3%B8%C3%A5", "æøå"),
            ("%ED%95%9C%EA%B8%80", "한글"),
            ("%E6%97%A5%E6%9C%AC%E8%AA%9E", "日本語"),
            ("file%20with%20spaces.txt", "file with spaces.txt"),
        ];

        for (input, expected) in test_cases {
            let result = get_canonicalized_path(base_path, input).await;
            assert!(result.is_ok(), "Failed for input: {}", input);
            let canonical_path = result?;
            assert!(
                canonical_path.ends_with(expected),
                "Expected path to end with '{}', but got '{:?}'",
                expected,
                canonical_path
            );
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_invalid_path() -> TestResult {
        let temp_dir = TempDir::new()?;
        let base_path = temp_dir.path();

        let result = get_canonicalized_path(base_path, "non_existent_file").await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StatusCode::NOT_FOUND);
        Ok(())
    }
}
