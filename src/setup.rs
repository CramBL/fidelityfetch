use std::{io, net::SocketAddr};

pub async fn setup_tcp_listener(addr: &SocketAddr) -> Result<tokio::net::TcpListener, ()> {
    // Only use listenfd in debug mode
    let listener = if cfg!(debug_assertions) {
        let mut listenfd = listenfd::ListenFd::from_env();
        if let Ok(Some(listener)) = listenfd.take_tcp_listener(0) {
            tracing::debug!("Using existing listener from listenfd");
            listener
                .set_nonblocking(true)
                .expect("failed setting socket nonblocking mode");
            tokio::net::TcpListener::from_std(listener)
                .expect("failed initializing TCP listener from listenfd listener")
        } else {
            match tokio::net::TcpListener::bind(&addr).await {
                Ok(listener) => {
                    tracing::debug!("Created new TCP listener at {addr} (listenfd fallback)");
                    listener
                }
                Err(e) => {
                    match e.kind() {
                        io::ErrorKind::AddrInUse => eprintln!(
                            "Error: {e}\nHINT: Choose another port or use '0' to use any available port",
                        ),
                        _ => eprintln!("Error: {e}"),
                    }
                    return Err(());
                }
            }
        }
    } else {
        // In release mode, just bind normally
        match tokio::net::TcpListener::bind(&addr).await {
            Ok(listener) => {
                tracing::debug!("Created TCP listener at {addr}");
                listener
            }
            Err(e) => {
                match e.kind() {
                    io::ErrorKind::AddrInUse => eprintln!(
                        "Error: {e}\nHINT: Choose another port or use '0' to use any available port",
                    ),
                    _ => eprintln!("Error: {e}"),
                }
                return Err(());
            }
        }
    };

    Ok(listener)
}
