use std::path::PathBuf;

use owo_colors::{AnsiColors, OwoColorize};

use crate::cli::Level;

pub struct Logger(pub Level);

#[allow(dead_code)]
pub fn miss(path: &PathBuf, Logger(level): &Logger) {
    match level {
        Level::Error | Level::Warn => (),
        _ => println!(
            "cache {} {}",
            "miss".bright_purple().bold().italic(),
            path.display().underline()
        ),
    }
}

#[allow(dead_code)]
pub fn hit(path: &PathBuf, Logger(level): &Logger) {
    match level {
        Level::Error | Level::Warn => (),
        _ => println!(
            "cache {} {}",
            "hit".bright_yellow().bold().italic(),
            path.display().underline()
        ),
    }
}

pub fn usize_success(hits: usize, outof: usize) -> AnsiColors {
    match hits {
        q if q == outof => AnsiColors::Green,
        0 => AnsiColors::Red,
        _ => AnsiColors::Yellow,
    }
}

impl Logger {
    pub fn verbose<S: std::fmt::Display>(&self, msg: S) {
        if let Level::Verbose = self.0 {
            internal::verbose(msg)
        }
    }
    pub fn debug<S: std::fmt::Display>(&self, msg: S) {
        match self.0 {
            Level::Verbose | Level::Debug => internal::debug(msg),
            _ => (),
        }
    }
    pub fn info<S: std::fmt::Display>(&self, msg: S) {
        match self.0 {
            Level::Error | Level::Warn => (),
            _ => internal::info(msg),
        }
    }
    pub fn warn<S: std::fmt::Display>(&self, msg: S) {
        match self.0 {
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

    farve!(verbose, "verbose".magenta().bold(), 0);
    farve!(debug, "debug".bright_blue().bold(), 1);
    farve!(info, "info".bright_cyan().bold(), 1);
    efarve!(warn, "warn".yellow().bold(), 0);
    efarve!(error, "error".bright_red().bold().underline(), 0);
    efarve!(os, format!("{}:os", "error".red().bold()), 0);
}

/// * Verbose messages
pub mod verbose {
    use crate::hash::FileHash;
    use owo_colors::{colors::Cyan, OwoColorize};

    /// Write a parsed Tsconfig struct to stdout
    pub fn tsconfigs(configs: &[crate::parser::ParaConfig], logger: &super::Logger) {
        configs.iter().for_each(|config| {
            logger.verbose(format!(
                "{}, {:#?}",
                config.tsconfig_path.fg::<Cyan>().underline(),
                config.tsconfig
            ));
        });
    }

    /// Write out the hashed cwd
    pub fn hashed_cwd(hash: &FileHash, logger: &super::Logger) {
        logger.verbose(format!(
            "hashed cwd path {:x}",
            hash.fg::<Cyan>().underline(),
        ));
    }
}

/// * Debug messages
pub mod debug {
    use camino::Utf8PathBuf;
    use owo_colors::{
        colors::{Blue, BrightBlack, Cyan, Yellow},
        OwoColorize,
    };
    use std::path::{Path, PathBuf};

    /// Log the quantity of tsconfig.jsons to be parsed
    pub fn tsconfig_paths(paths: &[Utf8PathBuf], logger: &super::Logger) {
        logger.debug(format!(
            "Reading {} tsconfigs...",
            paths.len().fg::<Yellow>()
        ));
    }

    /// Write out the hashed cwd
    pub fn cache_dir(hash: &PathBuf, logger: &super::Logger) {
        logger.debug(format!(
            "cwd cache path {:?}",
            hash.fg::<Cyan>().underline(),
        ));
    }

    /// Log a path as skipped because it was in the exclude list
    pub fn excluded_path(path: &Path, logger: &super::Logger) {
        logger.debug(format!(
            "{} {:?}",
            "EXCLUDED".fg::<BrightBlack>().bold(),
            path,
        ));
    }

    /// Log a path as a file.
    pub fn is_file(path: &Path, logger: &super::Logger) {
        logger.debug(format!("{} {:?}", "IS FILE".fg::<Cyan>().bold(), path,));
    }

    /// Log a path as a directory.
    pub fn is_dir(path: &Path, logger: &super::Logger) {
        logger.debug(format!("{} {:?}", "IS DIR".fg::<Blue>().bold(), path,));
    }
}

/// * Info messages
pub mod info {
    use super::usize_success;
    use owo_colors::OwoColorize;

    /// Log the quantity of successfully parsed tsconfigs
    pub fn configs_loaded(paths_len: usize, len: usize, logger: &super::Logger) {
        logger.info(format!(
            "Found {} tsconfigs...",
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

/// Error messages - these don't use the logger because we don't provide a way to silence them.
pub mod error {
    use owo_colors::{colors::Cyan, OwoColorize};

    pub fn os_error<P: AsRef<std::path::Path>>(path: P, e: &anyhow::Error) {
        super::internal::os(format!(
            "{} at {}",
            e,
            path.as_ref().to_string_lossy().fg::<Cyan>().underline()
        ));
    }
    pub fn missing_fields<P: AsRef<std::path::Path>>(path: P, e: &serde_json::Error) {
        super::internal::error(format!(
            "Parsing error in {}: {}",
            path.as_ref().display(),
            e
        ));
    }
}
