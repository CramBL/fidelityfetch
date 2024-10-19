use std::io;

use clap::ValueEnum;
use tracing::Level;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(Debug, strum::Display, Clone, Copy, ValueEnum)]
#[strum(serialize_all = "lowercase")]
pub enum Logging {
    #[cfg(unix)]
    /// Log to journald
    Journald,
    /// Log to stderr
    Stderr,
    /// Log to stdout
    Stdout,
}

// Print extremely verbose output when running in debug mode
fn setup_debug_logging() {
    tracing_subscriber::fmt()
        .with_writer(io::stderr)
        .with_max_level(Level::TRACE)
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .pretty()
        .finish()
        .init();
}

pub(super) fn setup_logging(logging: Logging, loglevel: Level) {
    if cfg!(debug_assertions) {
        setup_debug_logging();
    } else {
        match logging {
            #[cfg(unix)]
            Logging::Journald => {
                init_journald_logger().expect("Failed initializing journald logger");
            }
            Logging::Stderr | Logging::Stdout => {
                let subscriber = tracing_subscriber::fmt()
                    .with_max_level(loglevel)
                    .with_target(false)
                    .with_file(false)
                    .with_line_number(false);
                if matches!(logging, Logging::Stdout) {
                    subscriber.with_writer(io::stderr).finish().init();
                } else {
                    subscriber.with_writer(io::stdout).finish().init();
                };
            }
        }
    }
}

#[cfg(unix)]
fn init_journald_logger() -> io::Result<()> {
    let journald = tracing_journald::layer()?;
    tracing_subscriber::layer::SubscriberExt::with(tracing_subscriber::registry(), journald).init();
    Ok(())
}
