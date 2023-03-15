use camino::Utf8PathBuf;
use owo_colors::OwoColorize;
use std::path::PathBuf;

pub fn parse_cli_paths(
    p1: Option<Vec<Utf8PathBuf>>,
    p2: Option<Vec<Utf8PathBuf>>,
    logger: &crate::log::Logger,
) -> Vec<Utf8PathBuf> {
    let mut paths = p1.unwrap_or_default();
    paths.extend(p2.unwrap_or_default());
    if paths.is_empty() {
        paths.push(Utf8PathBuf::from("./tsconfig.json"));
    }
    crate::log::debug::tsconfig_paths(&paths, &logger);
    paths
}

pub fn parse_cli_exclude_paths(exclude: Vec<String>, merge_with_default: bool) -> Vec<String> {
    match merge_with_default {
        false => exclude,
        true => {
            let mut exclude = exclude;
            exclude.extend(vec![
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
            ]);
            exclude
        }
    }
}

pub fn handle_cli_cache_command(
    cache_action: Option<crate::cli::CacheCommand>,
    cache_dir: &PathBuf,
    logger: &crate::log::Logger,
) -> Option<()> {
    use crate::cli::CacheCommand::*;
    match cache_action {
        None => return None,
        Some(Clear) => {
            let cache = super::cache::parse_cache(cache_dir)?;
            if cache.is_empty() {
                logger.info("Cache is already empty.");
            } else {
                std::fs::remove_file(cache_dir).ok()?;
                logger.info("Cache cleared.");
            }
        }
        Some(Dump) => {
            let cache = super::cache::parse_cache(cache_dir)?;
            if cache.is_empty() {
                logger.info("Cache empty.");
            } else {
                logger.info(cache);
            }
        }
        Some(Path) => {
            logger.info(&format!(
                "Cache directory: {}",
                cache_dir.display().underline()
            ));
        }
    };
    Some(())
}
