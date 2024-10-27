use axum::http::{header, StatusCode};
use chrono::TimeZone;
use std::time::SystemTime;
use std::{fs, io, path};

/// Format a `u64` size in bytes to a human-readable string
///
/// # Arguments
///
/// * `size_bytes` - The size in bytes to format
///
/// # Returns
///
/// A human-readable string representing the size
///
/// # Examples
///
/// ```
/// use fidelityfetch::util::format_data_size;
///
/// assert_eq!(format_data_size(1024), "1.00 KiB");
/// ```
#[must_use]
pub fn format_data_size(size_bytes: u64) -> String {
    const KI_B: f64 = 1024.0;
    const MI_B: f64 = KI_B * 1024.0;
    const GI_B: f64 = MI_B * 1024.0;

    match size_bytes {
        0..=1024 => format!("{:.2} B", size_bytes),
        1025..=1_048_576 => format!("{:.2} KiB", size_bytes as f64 / KI_B),
        1_048_577..=1_073_741_824 => format!("{:.2} MiB", size_bytes as f64 / MI_B),
        _ => format!("{:.2} GiB", size_bytes as f64 / GI_B),
    }
}


/// Format a `SystemTime` to a human-readable string
#[must_use]
pub fn format_system_time(time: SystemTime) -> String {
    match time.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => {
            let datetime = chrono::Local
                .timestamp_opt(duration.as_secs() as i64, duration.subsec_nanos())
                .unwrap();
            datetime.format("%Y-%m-%d %H:%M:%S").to_string()
        }
        Err(_) => "Unknown date".to_owned(),
    }
}

/// Parse a range header and return the start and end bytes
///
/// This function takes a range header value and a file size, and returns a tuple
/// containing the start and end byte positions for the range request.
///
/// # Arguments
///
/// * `range_header` - A reference to a `HeaderValue` representing the range header.
/// * `file_size` - The size of the file in bytes.
///
/// # Returns
///
/// * `Ok((start, end))` - A tuple containing the start and end byte positions for the range request.
/// * `Err(StatusCode::BAD_REQUEST)` - If the range header is not valid.
/// * `Err(StatusCode::RANGE_NOT_SATISFIABLE)` - If the range is not satisfiable.
pub fn parse_range_header(
    range_header: &header::HeaderValue,
    file_size: u64,
) -> Result<(u64, u64), StatusCode> {
    let range_str = range_header.to_str().map_err(|err| {
        tracing::warn!("Failed to convert header to string: {err}");
        StatusCode::BAD_REQUEST
    })?;
    let range = range_str
        .strip_prefix("bytes=")
        .ok_or(StatusCode::BAD_REQUEST)?;
    let (start_str, end_str) = range.split_once('-').ok_or(StatusCode::BAD_REQUEST)?;

    let start: u64 = start_str.parse().unwrap_or(0);
    let end: u64 = end_str.parse().unwrap_or(file_size - 1).min(file_size - 1);

    if start >= file_size || start > end {
        return Err(StatusCode::RANGE_NOT_SATISFIABLE);
    }

    Ok((start, end))
}

pub fn is_directory_empty(path: &path::Path) -> io::Result<bool> {
    let mut entries = fs::read_dir(path)?;
    Ok(entries.find_map(Result::ok).is_none())
}

#[cfg(test)]
mod tests {
    use super::*;

    use axum::http::header::HeaderValue;

    #[test]
    fn test_parse_range_header() {
        let range_header = HeaderValue::from_static("bytes=0-100");
        let file_size = 1024;
        let result = parse_range_header(&range_header, file_size);
        assert_eq!(result, Ok((0, 100)));
    }

    #[test]
    fn test_parse_range_header_is_invalid() {
        let range_header = HeaderValue::from_static("bytes=");
        let file_size = 1024;
        let result = parse_range_header(&range_header, file_size);
        assert_eq!(result, Err(StatusCode::BAD_REQUEST));
    }

    #[test]
    fn test_parse_range_header_is_not_satisfiable() {
        let range_header = HeaderValue::from_static("bytes=1024-2048");
        let file_size = 1024;
        let result = parse_range_header(&range_header, file_size);
        assert_eq!(result, Err(StatusCode::RANGE_NOT_SATISFIABLE));
    }

    #[test]
    fn test_parse_range_header_open_ended_from_start() {
        let range_header = HeaderValue::from_static("bytes=0-");
        let file_size = 1024;
        let result = parse_range_header(&range_header, file_size);
        assert_eq!(result, Ok((0, file_size - 1)));
    }

    #[test]
    fn test_parse_range_header_open_ended_from_middle() {
        let range_header = HeaderValue::from_static("bytes=512-");
        let file_size = 1024;
        let result = parse_range_header(&range_header, file_size);
        assert_eq!(result, Ok((512, file_size - 1)));
    }
}
