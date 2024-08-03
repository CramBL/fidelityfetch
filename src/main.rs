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
use mdns_sd::{ServiceDaemon, ServiceInfo};
use std::{collections::HashMap, net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

pub mod async_util;
pub mod config;
pub mod dir_entry;
pub mod icon;
pub mod serve;
pub mod util;

#[derive(Debug, Clone)]
pub struct AppState {
    root_dir: PathBuf,
}

#[tokio::main]
async fn main() {
    let cfg = Config::parse();

    tracing_subscriber::fmt()
        .with_max_level(cfg.verbosity())
        .init();

    let app_state = Arc::new(RwLock::new(AppState {
        root_dir: cfg.root().to_path_buf(),
    }));

    let local_ip = local_ip_address::local_ip().unwrap_or_else(|_| "127.0.0.1".parse().unwrap());

    let app = Router::new()
        .route("/", get(serve::handle_root))
        .route("/*file", get(serve::serve_path))
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], cfg.port()));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    let local_port = listener.local_addr().unwrap().port();
    tracing::info!("Listening on http://{local_ip}:{local_port}");

    if let Some(mdns_hostname) = cfg.mdns() {
        let service_type = "_http._tcp.local.";
        let mut service_props = HashMap::from([(
            "root".to_string(),
            cfg.root().to_str().unwrap_or(".").to_owned(),
        )]);
        service_props.insert("port".to_string(), local_port.to_string());

        let mdns = ServiceDaemon::new().expect("Failed to create mDNS daemon");
        let service_info = ServiceInfo::new(
            service_type,
            "fidelityfetch",
            &format!("{mdns_hostname}.local."),
            local_ip.to_string(),
            local_port,
            service_props,
        )
        .unwrap();

        mdns.register(service_info)
            .expect("Failed to register mDNS service");
    }

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
