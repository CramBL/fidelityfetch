use std::time::SystemTime;

use axum::http::{header, StatusCode};
use chrono::TimeZone;

use crate::icon::FileTypeCategory;

#[must_use]
pub fn format_data_size(size_bytes: u64) -> String {
    const KI_B_VAL: u64 = 1024;
    const KI_B_DIVIDER: f64 = 1024_f64;
    const MI_B_VAL: u64 = 1024 * KI_B_VAL;
    const MI_B_DIVIDER: f64 = MI_B_VAL as f64;
    const GI_B_VAL: u64 = 1024 * MI_B_VAL;
    const GI_B_DIVIDER: f64 = GI_B_VAL as f64;
    match size_bytes {
        0..=KI_B_VAL => {
            format!("{size_bytes:.2} B")
        }
        1025..=MI_B_VAL => {
            let kib_bytes = size_bytes as f64 / KI_B_DIVIDER;
            format!("{kib_bytes:.2} KiB")
        }
        1_048_577..=GI_B_VAL => {
            let mib_bytes = size_bytes as f64 / MI_B_DIVIDER;
            format!("{mib_bytes:.2} MiB")
        }
        _ => {
            let gib_bytes = size_bytes as f64 / GI_B_DIVIDER;
            format!("{gib_bytes:.2} GiB")
        }
    }
}

#[must_use]
pub fn format_system_time(time: SystemTime) -> String {
    match time.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => {
            let datetime = chrono::Local
                .timestamp_opt(duration.as_secs() as i64, duration.subsec_nanos())
                .unwrap();
            datetime.format("%Y-%m-%d %H:%M:%S").to_string()
        }
        Err(_) => "Unknown date".to_string(),
    }
}

pub fn generate_list_item(
    file_name: &str,
    file_type: std::fs::FileType,
    size: &str,
    modified_date: &str,
    file_category: FileTypeCategory,
) -> String {
    let icon = file_category.icon();

    let (href, display_name) = if file_type.is_dir() {
        (format!("{}/", file_name), format!("{}/", file_name))
    } else {
        (file_name.to_owned(), file_name.to_owned())
    };

    format!(
        r#"
        <li class="file-item {}">
            <div class="file-icon">{}</div>
            <div class="file-details">
                <a href="{}" class="file-name">{}</a>
                <div class="file-info">
                    <span class="file-size">{}</span>
                    <span class="file-date">{}</span>
                </div>
            </div>
        </li>
        "#,
        if file_type.is_dir() {
            "directory"
        } else {
            "file"
        },
        icon,
        href,
        display_name,
        size,
        modified_date
    )
}

pub fn parse_range_header(
    range_header: &header::HeaderValue,
    file_size: u64,
) -> Result<(u64, u64), StatusCode> {
    let range_str = range_header.to_str().map_err(|_| StatusCode::BAD_REQUEST)?;
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
