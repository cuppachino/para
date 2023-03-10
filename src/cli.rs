use camino::Utf8PathBuf;
use clap::{command, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(author,  version, about, long_about = None)]
pub struct Pod {
    /// A list of tsconfig.jsons with paths to resolve
    #[clap(use_value_delimiter = true, default_value = "tsconfig.json")]
    pub paths: Vec<Utf8PathBuf>,

    /// Set the logging level
    #[arg(
        short = 'L',
        long = "log",
        value_name = "LEVEL",
        value_enum,
        default_value = "info"
    )]
    pub log_level: Level,
}

#[derive(Subcommand, ValueEnum, Clone)]
pub enum Level {
    /// Write (potentially sensitive) detailed process events to stdout
    Verbose,
    /// Write unfiltered debug information to stdout
    Debug,
    ///  Write filesystem operations and useful process information to stdout
    Info,
    ///  Write only warning and error messages to stderr
    Warn,
    ///  Disable all output except error messages
    Error,
}
