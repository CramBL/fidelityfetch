use std::path::{Path, PathBuf};

use clap::{
    builder::{
        styling::{AnsiColor, Effects},
        Styles,
    },
    Parser, ValueEnum,
};
use tracing::Level;

#[derive(Parser, Debug)]
#[command(name = "Fidelity Fetch", version, styles = cli_styles())]
pub struct Config {
    /// Directory from which files can be browsed and served
    #[arg(short, long, default_value = ".")]
    root: PathBuf,

    /// Set the port to serve on. If not set, an available port will be assigned.
    #[arg(short, long, default_value_t = 0)]
    port: u16,

    #[arg(short, long, default_value = "info")]
    verbosity: LogLevel,

    /// Optional service to register which can be used as the hostname to access served content.
    #[arg(short, long)]
    mdns: Option<String>,
}

impl Config {
    pub fn verbosity(&self) -> Level {
        match self.verbosity {
            LogLevel::Trace => Level::TRACE,
            LogLevel::Debug => Level::DEBUG,
            LogLevel::Info => Level::INFO,
            LogLevel::Warn => Level::WARN,
            LogLevel::Error => Level::ERROR,
        }
    }

    /// Directory from which files can be browsed and served
    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn mdns(&self) -> Option<&str> {
        self.mdns.as_deref()
    }
}

#[derive(Debug, ValueEnum, Clone, Copy)]
enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

pub fn cli_styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Yellow.on_default() | Effects::BOLD)
        .usage(AnsiColor::Yellow.on_default() | Effects::BOLD)
        .literal(AnsiColor::Blue.on_default())
        .placeholder(AnsiColor::Green.on_default())
}
