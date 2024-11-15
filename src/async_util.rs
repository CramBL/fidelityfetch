use std::{
    io,
    path::{Path, PathBuf},
    str,
};

use crate::{
    dir_entry::FifeDirEntry,
    icon::FileTypeCategory,
    util::{self, format_data_size, format_system_time},
};
use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use futures_util::stream::StreamExt;
use percent_encoding::percent_decode_str;
use tokio_stream::wrappers::ReadDirStream;

#[derive(thiserror::Error, Debug)]
pub enum PathError {
    #[error("Failed to decode path: {0}")]
    DecodingError(#[from] str::Utf8Error),

    #[error("Failed to canonicalize path: {0}")]
    CanonicalizationError(#[from] io::Error),

    #[error("Path not found: {0}")]
    NotFound(PathBuf),
}

impl IntoResponse for PathError {
    fn into_response(self) -> Response<Body> {
        let (status, error_message) = match self {
            Self::DecodingError(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            Self::CanonicalizationError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            Self::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
        };

        (status, error_message).into_response()
    }
}

/// Get file info for a directory entry
async fn get_file_info(entry: &tokio::fs::DirEntry) -> io::Result<(String, String, String)> {
    let metadata = entry.metadata().await?;
    let file_type = entry.file_type().await?;

    let file_size = if file_type.is_file() {
        format_data_size(metadata.len())
    } else if file_type.is_dir() {
        let count = count_directory_entries(entry.path()).await?;
        format!("{} item{}", count, if count == 1 { "" } else { "s" })
    } else if file_type.is_symlink() {
        "Symbolic Link".to_owned()
    } else {
        String::new()
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
                "Unknown size".to_owned(),
                "Unknown date".to_owned(),
                String::new(),
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
                tracing::error!("Error checking if directory '{file_name}' is empty: {e}");
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
) -> Result<PathBuf, PathError> {
    let mut path = base_path.to_owned();
    if !requested_path.is_empty() && requested_path != "/" {
        let decoded_path = percent_decode_str(requested_path).decode_utf8()?;
        path.push(decoded_path.trim_start_matches('/'));
    }

    match tokio::fs::canonicalize(&path).await {
        Ok(canonicalized) => Ok(canonicalized),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Err(PathError::NotFound(path)),
        Err(e) => Err(PathError::CanonicalizationError(e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_prelude::*;

    #[tokio::test]
    async fn test_get_canonicalized_path() -> TestResult {
        let temp_dir = TempDir::new()?;
        let base_path = temp_dir.path();

        // Create test directories and files
        tokio::fs::create_dir_all(base_path.join("folder with spaces")).await?;
        tokio::fs::create_dir_all(base_path.join("æøå")).await?;
        tokio::fs::create_dir_all(base_path.join("한글")).await?;
        tokio::fs::create_dir_all(base_path.join("日本語")).await?;
        let _ = File::create(base_path.join("file with spaces.txt")).await?;

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
        let non_existent_file_name = "non existent file name";

        let result = get_canonicalized_path(base_path, &non_existent_file_name).await;
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_matches!(error, PathError::NotFound(_));
        let expected_path = base_path.join(non_existent_file_name);
        expect_eq!(
            error.to_string(),
            format!("Path not found: {}", expected_path.display())
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_invalid_encoding() -> TestResult {
        let temp_dir = TempDir::new()?;
        let base_path = temp_dir.path();

        // Test cases with invalid percent-encoding
        let invalid_encodings = vec![
            "%C3%28",    // Invalid UTF-8 sequence
            "%E0%A4%B",  // Incomplete multi-byte sequence
            "%80",       // Invalid UTF-8 (continuation byte)
            "%ED%A0%80", // Invalid UTF-8 (surrogate pair)
        ];

        for invalid_encoding in invalid_encodings {
            let result = get_canonicalized_path(base_path, invalid_encoding).await;
            assert!(
                result.is_err(),
                "Expected error for input: {}",
                invalid_encoding
            );
            let error = result.unwrap_err();
            assert_matches!(
                error,
                PathError::DecodingError(_),
                "Expected DecodingError for input: {}",
                invalid_encoding
            );
        }

        Ok(())
    }
}
