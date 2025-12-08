use std::sync::Arc;

use axum::{Router, routing::get};
use tokio::sync::RwLock;

use crate::{AppState, serve};

pub fn get_router(app_state: Arc<RwLock<AppState>>) -> Router {
    Router::new()
        .route("/", get(serve::handle_root))
        .route("/{*file}", get(serve::serve_path))
        .with_state(app_state)
}
