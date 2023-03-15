use camino::Utf8PathBuf;
use clap::{command, Parser, ValueEnum};

#[derive(Parser)]
#[command(
    author,
    version,
    about = "Another TypeScript alias resolver",
    long_about = r#"
█▀█ ▄▀█ █▀█ ▄▀█
█▀▀ █▀█ █▀▄ █▀█
Another TypeScript alias resolver"#
)]
pub struct Cli {
    /// A list of tsconfig.jsons with paths to resolve
    #[clap(
        display_order = 0,
        use_value_delimiter = true,
        value_name = "\x1b[96mPATHS\x1b[0m",
        value_hint = clap::ValueHint::FilePath
    )]
    pub paths: Option<Vec<Utf8PathBuf>>,

    /// [default: "tsconfig.json"]
    #[arg(
        display_order = 1,
        help_heading = "Config",
        short = 'p',
        long = "path",
        value_name = "\x1b[96mPATHS\x1b[0m",
        value_hint = clap::ValueHint::FilePath,
        use_value_delimiter = true
    )]
    pub paths_arg: Option<Vec<Utf8PathBuf>>,

    /// Exclude directories from the search
    #[arg(
        display_order = 2,
        help_heading = "Config",
        short = 'e',
        long = "exclude",
        value_name = "\x1b[91mDIRS\x1b[0m",
        value_hint = clap::ValueHint::DirPath,
        use_value_delimiter = true,
        default_values_t = [
            "node_modules".to_string(),
            ".git".to_string(),
            ".gitignore".to_string(),
            "*.ts".to_string(),
            "*.tsx".to_string(),
            "*.cts".to_string(),
            "*.mts".to_string(),
            "*.json".to_string(),
            "*.lock".to_string(),
            "*.toml".to_string(),
            "*.yaml".to_string(),
            "*.vscode".to_string(),
            "target".to_string(),
        ]
    )]
    pub exclude: Vec<String>,

    // todo!
    /// [SWITCH] Extend the default exclude list instead of replacing it.
    ///
    /// [default: None]
    #[arg(
        display_order = 3,
        help_heading = "Config",
        short = 'E',
        long = "extend",
        action
    )]
    pub exclude_default: bool,

    // ------------------------------
    /// Set the logging level
    #[arg(
        display_order = 5,
        help_heading = "Verbosity",
        short,
        long = "log",
        value_name = "\x1b[90mLEVEL\x1b[0m",
        value_enum,
        default_value = "info"
    )]
    pub log_level: Level,

    // ------------------------------
    /// Interact with the cache directory
    // #[command(subcommand)]
    #[clap(
        display_order = 5,
        help_heading = "Cache",
        short = 'c',
        long = "cache",
        value_name = "\x1b[33mACTION\x1b[0m",
        value_enum,
        action
    )]
    pub cache: Option<CacheCommand>,
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

#[derive(ValueEnum, Clone)]
pub enum CacheCommand {
    /// Clear the cache
    Clear,

    /// Get para's cache directory for the current user.
    Para,

    /// Get the cache directory for the current project
    Cwd,
}
