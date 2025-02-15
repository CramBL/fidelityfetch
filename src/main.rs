use axum::{routing::get, Router};
use clap::Parser;
use config::Config;
use std::{
    io,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
    process::ExitCode,
    sync::Arc,
};
use tokio::sync::RwLock;

pub mod async_util;
pub mod config;
pub mod dir_entry;
pub mod icon;
mod mdns;
pub mod serve;
#[cfg(test)]
pub(crate) mod test_prelude;
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
async fn main() -> ExitCode {
    let cfg = Config::parse();
    if let Some(shell) = cfg.completions {
        config::Config::generate_completion_script(shell);
        tracing::info!("Completions generated for {shell:?}. Exiting...");
        return ExitCode::SUCCESS;
    }
    cfg.setup_logging();

    let app_state = AppState::new(cfg.root().to_owned());

    let local_ip =
        local_ip_address::local_ip().unwrap_or_else(|_| IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));

    let app = Router::new()
        .route("/", get(serve::handle_root))
        .route("/{*file}", get(serve::serve_path))
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], cfg.port()));
    let listener = match tokio::net::TcpListener::bind(&addr).await {
        Ok(listener) => listener,
        Err(e) => {
            match e.kind() {
                io::ErrorKind::AddrInUse => eprintln!(
                    "Error: {e}\nHINT: Choose another port or use '0' to use any available port",
                ),
                _ => eprintln!("Error: {e}"),
            }
            return ExitCode::FAILURE;
        }
    };

    let Ok(local_addr) = listener.local_addr() else {
        eprintln!("Failed to get local address for TCP listener");
        return ExitCode::FAILURE;
    };
    let local_port = local_addr.port();

    if let Some(mdns_hostname) = cfg.mdns() {
        mdns::register_mdns(
            mdns_hostname,
            local_port,
            local_ip,
            cfg.root().to_str().expect("Invalid root").to_owned(),
        );
    }

    eprintln!("Listening on http://{local_ip}:{local_port}");
    if let Err(e) = axum::serve(listener, app.into_make_service()).await {
        eprintln!("Server error: {e}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
