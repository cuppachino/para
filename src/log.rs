use crate::cli::Level;
pub struct Logger {
    pub level: Level,
}

impl Logger {
    pub fn init(level: Level) -> Self {
        Self { level }
    }
    pub fn verbose<S: std::fmt::Display>(&self, msg: S) {
        if let Level::Verbose = self.level {
            internal::verbose(msg)
        }
    }
    pub fn debug<S: std::fmt::Display>(&self, msg: S) {
        match self.level {
            Level::Verbose | Level::Debug => internal::debug(msg),
            _ => (),
        }
    }
    pub fn info<S: std::fmt::Display>(&self, msg: S) {
        match self.level {
            Level::Error | Level::Warn => (),
            _ => internal::info(msg),
        }
    }
    pub fn warn<S: std::fmt::Display>(&self, msg: S) {
        match self.level {
            Level::Error => (),
            _ => internal::warn(msg),
        }
    }
    #[allow(dead_code)]
    pub fn error<S: std::fmt::Display>(&self, msg: S) {
        internal::error(msg);
    }
}

/// Internal farve macros
pub(crate) mod internal {
    use farve::{efarve, farve};
    use owo_colors::OwoColorize;

    farve!(verbose, "verbose".bright_blue().bold(), 0);
    farve!(debug, "debug".green().bold(), 1);
    farve!(info, "info".bright_cyan(), 1);
    efarve!(warn, "warn".yellow().bold(), 0);
    efarve!(error, "error".bright_red().bold().underline(), 0);
}

/// * Verbose messages
pub mod verbose {
    use owo_colors::{colors::Cyan, OwoColorize};

    /// Write a parsed Tsconfig struct to stdout
    pub fn tsconfigs(configs: &Vec<crate::parser::ParaConfig>, logger: &super::Logger) {
        configs.iter().for_each(|config| {
            logger.verbose(format!(
                "{}, {:#?}",
                config.path.fg::<Cyan>().underline(),
                config.tsconfig
            ));
        });
    }
}

/// * Debug messages
pub mod debug {
    use camino::Utf8PathBuf;
    use owo_colors::{colors::Yellow, OwoColorize};

    /// Log the quantity of tsconfig.jsons to be parsed
    pub fn tsconfig_paths(paths: &[Utf8PathBuf], logger: &super::Logger) {
        logger.debug(format!(
            "Reading {} tsconfigurations...",
            paths.len().fg::<Yellow>()
        ));
    }
}

/// * Info messages
pub mod info {
    use owo_colors::OwoColorize;

    use crate::format::usize_success;

    /// Log the quantity of successfully parsed tsconfigs
    pub fn configs_loaded(paths_len: usize, len: usize, logger: &super::Logger) {
        logger.info(format!(
            "Found {} tsconfigurations...",
            len.color(usize_success(len, paths_len))
        ));
    }
}

/// * Warning messages
pub mod warn {
    use camino::Utf8PathBuf;
    use owo_colors::{colors::Cyan, OwoColorize};
    use rayon::prelude::*;

    /// Notify user when a single path is skipped
    fn skip_config(path: &Utf8PathBuf, logger: &super::Logger) {
        logger.warn(format!(
            "Skipped bad path: {}",
            path.fg::<Cyan>().underline()
        ));
    }

    /// Notify user when multiple paths are skipped
    pub fn paths_skipped(paths: Vec<&Utf8PathBuf>, logger: &super::Logger) {
        paths.par_iter().for_each(|path| {
            self::skip_config(path, logger);
        });
    }
}

/// * Error messages
/// do not require a logger instance because errors are always logged.
pub mod error {
    pub fn missing_fields<P: AsRef<std::path::Path>>(path: P, e: &serde_json::Error) {
        super::internal::error(format!(
            "Parsing error in {}: {}",
            path.as_ref().display(),
            e
        ));
    }
}
