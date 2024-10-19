use clap::{
    builder::{
        styling::{AnsiColor, Effects},
        Styles,
    },
    Parser, ValueEnum,
};
use logging::Logging;
use std::path::{Path, PathBuf};
use tracing::Level;

mod logging;

pub const BIN_NAME: &str = "fife";

#[derive(Parser, Debug)]
#[command(name = "Fidelity Fetch", version, styles = cli_styles())]
#[command(bin_name = BIN_NAME)]
pub struct Config {
    /// Directory from which content is served (recursively)
    #[arg(short, long, default_value = ".")]
    root: PathBuf,

    /// Set the port to serve on. If not set, an available port will be assigned.
    #[arg(short, long, default_value_t = 0)]
    port: u16,

    /// Verbosity of logging output (Not applicable when log=journald)
    #[arg(short, long, default_value_t = LogLevel::Info)]
    verbosity: LogLevel,

    /// Optional service to register which can be used as the hostname to access served content.
    /// e.g. `foo` will be available at `http://foo.local:<port>`
    #[arg(short, long)]
    mdns: Option<String>,

    /// Where should logs be forwarded to
    #[arg(short, long, default_value_t = Logging::Stderr)]
    log: Logging,

    /// Generate completion scripts for the specified shell.
    /// Note: The completion script is printed to stdout
    #[arg(
        long = "completions",
        value_hint = clap::ValueHint::Other,
        value_name = "SHELL"
    )]
    pub completions: Option<clap_complete::Shell>,
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

    /// Generate completion scripts for the specified shell.
    pub fn generate_completion_script(shell: clap_complete::Shell) {
        use clap::CommandFactory;
        clap_complete::generate(
            shell,
            &mut Self::command(),
            BIN_NAME,
            &mut std::io::stdout(),
        );
    }

    pub fn setup_logging(&self) {
        logging::setup_logging(self.log, self.verbosity());
    }
}

#[derive(Debug, ValueEnum, strum::Display, Clone, Copy)]
#[strum(serialize_all = "lowercase")]
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
