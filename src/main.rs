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
    if let Some(shell) = cfg.completions {
        config::Config::generate_completion_script(shell);
        tracing::info!("Completions generated for {shell:?}. Exiting...");
        return;
    }

    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_writer(std::io::stderr)
        .with_max_level(cfg.verbosity())
        .with_file(false)
        .compact()
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

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
        .expect("Failed creating service");

        mdns.register(service_info)
            .expect("Failed to register mDNS service");
    }

    eprintln!("Listening on http://{local_ip}:{local_port}");
    axum::serve(listener, app.into_make_service())
        .await
        .expect("Failed to start server");
}
