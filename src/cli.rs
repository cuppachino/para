use camino::Utf8PathBuf;
use clap::{command, Parser, Subcommand, ValueEnum};

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
#[group(name = "para")]
pub struct Cli {
    /// A list of tsconfig.jsons with paths to resolve
    #[clap(
        use_value_delimiter = true,
        value_name = "\u{1b}[96mPATHS\u{1b}[0m",
        value_hint = clap::ValueHint::FilePath
    )]
    pub paths: Option<Vec<Utf8PathBuf>>,

    /// [default: tsconfig.json]
    #[arg(
        short = 'p',
        long = "project",
        value_name = "\x08\u{1b}[0m<\u{1b}[96mPATHS\u{1b}[0m",
        value_hint = clap::ValueHint::FilePath,
        use_value_delimiter = true
    )]
    pub paths_arg: Option<Vec<Utf8PathBuf>>,

    /// Comma-separated patterns to exclude from resolution
    #[arg(
        short = 'e',
        long = "exclude",
        value_name = "\x08\u{1b}[0m<\u{1b}[91mDIRS\u{1b}[0m", // backspace is \x08
        use_value_delimiter = true,
        default_values_t = vec![
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

    /// [SWITCH] Extend the default exclude list instead of replacing it (requires --exclude)
    #[arg(short = 'E', long = "extend", requires = "exclude", action)]
    pub merge_with_default_exclude: bool,

    /// Interact with the cache directory
    #[clap(
        help_heading = "Cache",
        short = 'c',
        long = "cache",
        value_name = "\x08\u{1b}[0m<\u{1b}[32mACTION\u{1b}[0m",
        value_enum,
        ignore_case = true
    )]
    pub cache_action: Option<CacheCommand>,

    /// Set the logging level
    #[arg(
        help_heading = "Verbosity",
        short,
        long = "log",
        value_name = "\x08\u{1b}[0m<\u{1b}[37mLEVEL\u{1b}[0m",
        value_enum,
        default_value = "info",
        ignore_case = true
    )]
    pub log_level: Level,
}

#[derive(ValueEnum, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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

#[derive(Subcommand, ValueEnum, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CacheCommand {
    /// Clear the cache
    Clear,
    /// Dump the cache to stdout
    Dump,
    /// Print the cache directory
    Path,
}
