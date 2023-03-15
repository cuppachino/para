use clap::Parser;
use exclusions::*;
use lazy_static::lazy_static;
use log::{debug, info, verbose, warn, Logger};
#[allow(unused_imports)]
use rayon::prelude::*;
use regex::Regex;

use crate::cache::{load_cache, save_cache};

mod args;
mod cache;
mod cli;
mod exclusions;
mod log;
mod parser;
mod stack;
mod utils;

fn main() -> std::io::Result<()> {
    let cli = cli::Cli::parse();
    let logger = Logger(cli.log_level);
    let cache_dir = cache::generate_cache_dir(&logger)?;

    // Parse - cache command
    // if let Some(command) = cli.cache {
    // match command {
    //     Commands::Cache { clear, dump, get } => {}
    // }
    // }

    // Exit - if the cache command was used.
    if let Some(_) = args::handle_cli_cache_command(cli.cache_action, &cache_dir, &logger) {
        return Ok(());
    }

    // Parse - tsconfig paths
    let paths = args::parse_cli_paths(cli.paths, cli.paths_arg, &logger);

    // Parse - exclude paths
    let exclude_globset =
        args::parse_cli_exclude_paths(cli.exclude, cli.merge_with_default_exclude)
            .into_globset()
            .unwrap();

    // Parse - tsconfig contents
    let (configs, skipped) = parser::load_configs(&paths);
    verbose::tsconfigs(&configs, &logger);
    warn::paths_skipped(skipped, &logger);
    info::configs_loaded(paths.len(), configs.len(), &logger);

    // Exit - if no configs were found
    if configs.is_empty() {
        return Ok(());
    }

    // Init - stack
    use stack::*;
    let mut stack: Vec<Action> = vec![];
    configs
        .iter()
        .for_each(|config| stack.push(Action::ReadDir((&config.resolved_out_dir).into())));

    // Init - cache
    let mut cache = load_cache(&cache_dir, &logger).unwrap();
    verbose::dump_cache(&cache, &logger);

    // MAIN LOOP
    while let Some(node) = stack.pop() {
        if let Some(path) = node.is_match(&exclude_globset) {
            debug::excluded_path(path, &logger);
            continue;
        }
        match node {
            Action::FinishJob(_, _) => {}
            Action::ReadFile(path) => {
                debug::is_file(&path, &logger);
                let (hash, contents) = utils::hash_file(&path);
                stack.push(Action::CompareHash(path, hash, contents));
            }
            Action::ReadDir(path) => {
                debug::is_dir(&path, &logger);
                stack.extend(
                    path.read_dir()
                        .expect(r#"failed to "read_dir""#)
                        .filter_map(|entry| {
                            let path = entry.expect("entry is invalid").path();
                            if path.is_dir() {
                                Some(Action::ReadDir(path))
                            } else if path.is_file() {
                                Some(Action::ReadFile(path))
                            } else {
                                None
                            }
                        }),
                );
            }
            Action::CompareHash(path, hash, contents) => {
                if cache.contains_key(&hash) {
                    log::hit(&path, &logger);
                    stack.push(Action::FinishJob(path, hash));
                } else {
                    log::miss(&path, &logger);
                    cache.insert(hash.clone(), path.clone());
                    stack.push(Action::FindCaptures(hash, contents));
                }
            }
            Action::FindCaptures(hash, contents) => {
                // todo: replace alias with resolved path
                lazy_static! {
                    static ref RE: Regex =
                        Regex::new(r#"(?:require|import)\(['"](@/)([^'"]+)['"]\)"#).unwrap();
                }
                for _cap in RE.captures_iter(&contents) {
                    // println!("capture: {:#?}", cap);
                }
                stack.push(Action::CacheFile(hash));
            }
            Action::CacheFile(hash) => {
                let path = cache.get(&hash).unwrap();
                stack.push(Action::FinishJob(path.clone(), hash.clone()));
            }
        }
    }

    assert!(stack.is_empty());
    verbose::dump_cache(&cache, &logger);
    save_cache(&cache_dir, cache.into()).unwrap();

    Ok(())
}
