use crate::util;
use axum::{
    body::Body,
    http::{StatusCode, header},
    response::IntoResponse,
};
use std::io::SeekFrom;
use tokio::io::AsyncSeekExt;
use tokio::{fs::File, io::AsyncReadExt};
use tokio_util::io::ReaderStream;

pub async fn handle_range_request(
    range_header: &header::HeaderValue,
    mut file: File,
    file_size: u64,
    content_type: String,
) -> impl IntoResponse {
    let range = util::parse_range_header(range_header, file_size);
    match range {
        Ok((start, end)) => {
            let length = end - start + 1;
            if let Err(e) = file.seek(SeekFrom::Start(start)).await {
                tracing::error!("Error seeking file: {e}");
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }

            let stream = ReaderStream::new(file.take(length));
            let body = Body::from_stream(stream);

            (
                StatusCode::PARTIAL_CONTENT,
                [
                    (header::CONTENT_TYPE, content_type),
                    (header::ACCEPT_RANGES, "bytes".to_owned()),
                    (
                        header::CONTENT_RANGE,
                        format!("bytes {start}-{end}/{file_size}"),
                    ),
                    (header::CONTENT_LENGTH, length.to_string()),
                ],
                body,
            )
                .into_response()
        }
        Err(status) => status.into_response(),
    }
}
