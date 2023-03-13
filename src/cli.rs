use camino::Utf8PathBuf;
use clap::{command, Parser, ValueEnum};

#[derive(Parser)]
#[command(
    author,
    version,
    about = "Another TypeScript alias resolver",
    long_about = r#"
█▀█ ▄▀█ █▀█ ▄▀█
█▀▀ █▀█ █▀▄ █▀█"#
)]
pub struct Cli {
    /// A list of tsconfig.jsons with paths to resolve
    #[clap(use_value_delimiter = true, value_name = "\x1b[96mPATHS\x1b[0m", value_hint = clap::ValueHint::FilePath)]
    pub paths: Option<Vec<Utf8PathBuf>>,
    /// [default: "tsconfig.json"]
    #[arg(
        short = 'p',
        long = "path",
        value_name = "\x1b[96mPATHS\x1b[0m",
        value_hint = clap::ValueHint::FilePath,
        use_value_delimiter = true,
    )]
    pub paths_arg: Option<Vec<Utf8PathBuf>>,

    /// Exclude directories from the search
    #[arg(
        short = 'e',
        long = "exclude",
        value_name = "\x1b[91mDIRS\x1b[0m",
        value_hint = clap::ValueHint::DirPath,
        use_value_delimiter = true,
        default_value = "node_modules,target,.git,.vscode,package.json,pnpm-lock.yaml,cargo.toml,cargo.lock"
    )]
    pub exclude: Vec<Utf8PathBuf>,

    /// Set the logging level
    #[arg(
        short,
        long = "log",
        value_name = "\x1b[90mLEVEL\x1b[0m",
        value_enum,
        default_value = "info"
    )]
    pub log_level: Level,
}

#[derive(ValueEnum, Clone)]
pub enum Level {
    /// Write absolutely everything to stdout
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
