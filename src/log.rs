use crate::cli::Level;
pub struct Logger {
    pub level: Level,
}

impl Logger {
    pub fn init(level: Level) -> Self {
        Self { level }
    }
    pub fn debug<S: std::fmt::Display>(&self, msg: S) {
        if let Level::Debug = self.level {
            internal::debug(msg)
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

// ? Internal farve macros
mod internal {
    use farve::{efarve, farve};
    use owo_colors::OwoColorize;

    farve!(verbose, "verbose".blue().bold(), 1);
    farve!(debug, "debug".green().italic(), 1);
    farve!(info, "info".bright_cyan());
    efarve!(warn, "warn".yellow().underline(), 0);
    efarve!(error, "error".bright_red().bold().underline(), 0);
}

// * Info messages
pub mod info {
    use owo_colors::OwoColorize;

    use crate::format::usize_success;
    use crate::parser::Tsconfig;

    pub fn configs_loaded(_len: usize, configs: &[Tsconfig], logger: &super::Logger) {
        logger.info(format!(
            "Found {} tsconfigurations...",
            configs.len().color(usize_success(configs.len(), _len))
        ));
    }
}

// * Debug messages
pub mod debug {
    use camino::Utf8PathBuf;
    use owo_colors::OwoColorize;

    pub fn tsconfig_paths(paths: &[Utf8PathBuf], logger: &super::Logger) {
        logger.debug(format!(
            "Reading {} tsconfigurations...",
            paths.len().yellow()
        ));
    }
}

// * Warning messages
pub mod warn {
    use camino::Utf8PathBuf;
    use owo_colors::OwoColorize;

    pub fn skip_config(path: &Utf8PathBuf, logger: &super::Logger) {
        logger.warn(format!("Skipping bad path: {}", path.cyan().underline()));
    }
}
