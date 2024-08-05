// Performance lints
#![warn(variant_size_differences)]
#![warn(
    clippy::needless_pass_by_value,
    clippy::unnecessary_wraps,
    clippy::mutex_integer,
    clippy::mem_forget,
    clippy::maybe_infinite_iter
)]

use axum::{routing::get, Router};
use clap::Parser;
use config::Config;
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

pub mod async_util;
pub mod config;
pub mod dir_entry;
pub mod icon;
mod mdns;
pub mod serve;
pub mod util;

#[derive(Debug, Clone)]
pub struct AppState {
    root_dir: PathBuf,
}

impl AppState {
    pub fn new(root_dir: PathBuf) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self { root_dir }))
    }
}

#[tokio::main]
async fn main() {
    let cfg = Config::parse();
    if let Some(shell) = cfg.completions {
        config::Config::generate_completion_script(shell);
        tracing::info!("Completions generated for {shell:?}. Exiting...");
        return;
    }
    cfg.setup_logging();

    let app_state = AppState::new(cfg.root().to_owned());

    let local_ip = local_ip_address::local_ip().unwrap_or_else(|_| "127.0.0.1".parse().unwrap());

    let app = Router::new()
        .route("/", get(serve::handle_root))
        .route("/*file", get(serve::serve_path))
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], cfg.port()));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    let local_port = listener.local_addr().unwrap().port();

    if let Some(mdns_hostname) = cfg.mdns() {
        mdns::register_mdns(
            mdns_hostname,
            local_port,
            local_ip,
            cfg.root().to_str().unwrap().to_owned(),
        )
    }

    eprintln!("Listening on http://{local_ip}:{local_port}");
    axum::serve(listener, app.into_make_service())
        .await
        .expect("Failed to start server");
}
