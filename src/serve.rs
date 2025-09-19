use axum::{
    body::Body,
    extract::State,
    http::{header, Request, StatusCode},
    response::IntoResponse,
};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

use crate::async_util;
use crate::AppState;

static FAVICON: &[u8] = include_bytes!("../assets/favicon.ico");

pub mod dir;
mod html;
pub mod range_req;

pub async fn handle_root(
    State(state): State<Arc<RwLock<AppState>>>,
    req: Request<Body>,
) -> impl IntoResponse {
    let base_path = PathBuf::from(&state.read().await.root_dir);
    let path = match crate::async_util::get_canonicalized_path(&base_path, "").await {
        Ok(path) => path,
        Err(e) => return e.into_response(),
    };
    dir::serve_directory(&path, req.uri(), true)
        .await
        .into_response()
}

pub async fn serve_path(
    State(state): State<Arc<RwLock<AppState>>>,
    req: Request<Body>,
) -> impl IntoResponse {
    let requested_path = req.uri().path();
    tracing::info!("Requested: {requested_path}");
    if requested_path == "/favicon.ico" {
        return (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "image/x-icon")],
            FAVICON.to_vec(),
        )
            .into_response();
    }

    let base_path = PathBuf::from(&state.read().await.root_dir);
    let path = match async_util::get_canonicalized_path(&base_path, requested_path).await {
        Ok(path) => path,
        Err(e) => return e.into_response(),
    };

    tracing::trace!("Requested absolute path: {}", path.display());

    if path.is_dir() {
        return dir::serve_directory(&path, req.uri(), false)
            .await
            .into_response();
    }

    let file = match tokio::fs::File::open(&path).await {
        Ok(file) => file,
        Err(e) => {
            tracing::error!("Error opening file: {e}");
            return (StatusCode::NOT_FOUND, "File not found").into_response();
        }
    };

    let metadata = match file.metadata().await {
        Ok(metadata) => metadata,
        Err(e) => {
            tracing::error!("Error reading metadata: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Error reading metadata").into_response();
        }
    };

    let file_size = metadata.len();
    let content_type = mime_guess::from_path(path)
        .first_or_octet_stream()
        .to_string();

    if let Some(range) = req.headers().get(header::RANGE) {
        return range_req::handle_range_request(range, file, file_size, content_type)
            .await
            .into_response();
    }

    let stream = tokio_util::io::ReaderStream::new(file);
    let body = Body::from_stream(stream);

    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, content_type),
            (header::ACCEPT_RANGES, "bytes".to_owned()),
            (header::CONTENT_LENGTH, file_size.to_string()),
        ],
        body,
    )
        .into_response()
}

#[cfg(test)]
mod tests {
    use crate::test_prelude::*;
    use axum_test::TestServer;
    use std::fs;

    use crate::router::get_router;

    use super::*;

    #[tokio::test]
    async fn test_back_button_on_server() -> TestResult {
        // Arrange
        let temp_dir = tempfile::tempdir()?;
        let root_path = temp_dir.path().to_path_buf();
        let subdir_path = root_path.join("subdir");
        fs::create_dir(&subdir_path)?;
        fs::write(subdir_path.join("file.txt"), "content")?;

        let app = get_router(AppState::new(root_path));
        let server = TestServer::new(app)?;

        // Test subdirectory - should have back button
        let response = server.get("/subdir").await;
        response.assert_status_ok();
        assert!(
            response.text().contains(r#"<a href="..""#),
            "Back button MUST be present on subdirectory page"
        );

        // Test root directory - should NOT have back button
        let response = server.get("/").await;
        response.assert_status_ok();

        assert!(
            !response.text().contains(r#"<a href="..""#),
            "Back button must NOT be present on root page"
        );

        Ok(())
    }
}
