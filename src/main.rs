use axum::{routing::get, Router};
use clap::Parser;
use config::Config;
use std::{
    io,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
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
async fn main() -> io::Result<()> {
    let cfg = Config::parse();
    if let Some(shell) = cfg.completions {
        config::Config::generate_completion_script(shell);
        tracing::info!("Completions generated for {shell:?}. Exiting...");
        return Ok(());
    }
    cfg.setup_logging();

    let app_state = AppState::new(cfg.root().to_owned());

    let local_ip =
        local_ip_address::local_ip().unwrap_or_else(|_| IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));

    let app = Router::new()
        .route("/", get(serve::handle_root))
        .route("/*file", get(serve::serve_path))
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], cfg.port()));
    let listener = tokio::net::TcpListener::bind(&addr).await.map_err(|e| {
        match e.kind() {
            io::ErrorKind::AddrInUse => eprintln!(
                "Error: {}\nHINT: Choose another port or use '0' to use any available port",
                e
            ),
            _ => eprintln!("Error: {}", e),
        }
        e
    })?;

    let local_port = listener.local_addr()?.port();

    if let Some(mdns_hostname) = cfg.mdns() {
        mdns::register_mdns(
            mdns_hostname,
            local_port,
            local_ip,
            cfg.root().to_str().expect("Invalid root").to_owned(),
        );
    }

    eprintln!("Listening on http://{local_ip}:{local_port}");
    axum::serve(listener, app.into_make_service())
        .await
        .expect("Failed to start server");

    Ok(())
}
