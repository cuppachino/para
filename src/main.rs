use camino::Utf8PathBuf;
use clap::Parser;
use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::Regex;
use std::{collections::HashMap, path::PathBuf};

mod cli;
mod exclusions;
mod hash;
mod log;
mod parser;
mod stack;
mod utils;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Cache {
    cache: Vec<Bucket>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Bucket {
    hash: String,
    path: String,
}

impl From<Vec<Bucket>> for Cache {
    fn from(cache: Vec<Bucket>) -> Self {
        Self { cache }
    }
}

fn main() -> std::io::Result<()> {
    // todo: read caches from cache_dir
    // todo: write caches to cache_dir
    /* let original: Cache = Cache {
        cache: vec![
            Bucket {
                hash: String::new(),
                path: "a".to_string(),
            },
            Bucket {
                hash: String::new(),
                path: "b".to_string(),
            },
            Bucket {
                hash: String::new(),
                path: "c".to_string(),
            },
        ],
    };

    let toml_string = toml::to_string(&original).unwrap();

    println!("{}", toml_string);

    let config: Cache = toml::from_str(&toml_string).unwrap();
    println!("{:#?}\u{00A}", config);

    // let config: toml::value::Table = toml::from_str(&toml_string).unwrap();
    let config: toml::Value = toml::from_str(&toml_string).unwrap();
    println!("{:#?}\u{00A}", config) */

    // Parse CLI arguments
    let cli = cli::Cli::parse();
    let paths = cli
        .paths_arg
        .unwrap_or_default()
        .into_par_iter()
        .chain(
            cli.paths
                .unwrap_or(vec![Utf8PathBuf::from("./tsconfig.json")]),
        )
        .collect::<Vec<_>>();

    // Setup logger
    use log::{debug, info, verbose, warn, Logger};
    let logger = Logger(cli.log_level);
    debug::tsconfig_paths(&paths, &logger);

    #[allow(unused)]
    // Generate the cache directory for the [current] project
    let cache_dir = hash::generate_cache_dir(&logger)?;

    // Skip bad paths and parse valid configs.
    let (configs, skipped) = parser::load_configs(&paths);
    verbose::tsconfigs(&configs, &logger);
    warn::paths_skipped(skipped, &logger);
    info::configs_loaded(paths.len(), configs.len(), &logger);

    // Exit if no configs were found
    if configs.is_empty() {
        return Ok(());
    }

    // Convert the CLI exclude args into a GlobSet
    use exclusions::IntoGlobSet;
    let exclude_globset = cli.exclude.into_globset().unwrap();

    // ? Initialize the stack
    use stack::*;
    let mut stack: Vec<Action> = vec![];
    stack.push(Action::ReadDir((&configs[0].resolved_base_url).into()));
    // ? Initialize the cache
    let mut cache: HashMap<String, _> = HashMap::new();

    while !stack.is_empty() {
        let node = stack.pop().unwrap();
        if node.is_blacklisted(&exclude_globset) {
            if let Some(path) = node.reveal_path() {
                debug::excluded_path(path, &logger);
            }
            continue;
        }

        match node {
            Action::FinishJob(_, _) => {}
            Action::ReadFile(path) => {
                debug::is_file(&path, &logger);
                // todo
                let (hash, contents) = hash::hash_file(&path);
                stack.push(Action::CompareHash(path, hash, contents));
            }
            Action::CompareHash(path, hash, contents) => {
                if cache.contains_key(&hash) {
                    log::internal::info(format!("cache hit {:#?}", path));
                    stack.push(Action::FinishJob(path, hash));
                } else {
                    log::miss(&path, &logger);
                    cache.insert(hash.clone(), path.clone());
                    stack.push(Action::FindCaptures(hash, contents));
                }
            }
            Action::FindCaptures(hash, contents) => {
                lazy_static! {
                    static ref RE: Regex =
                        Regex::new(r#"(?:require|import)\(['"](@/)([^'"]+)['"]\)"#).unwrap();
                }
                // todo: replace captures
                for _cap in RE.captures_iter(&contents) {
                    // println!("capture: {:#?}", cap);
                }
                stack.push(Action::CacheFile(hash));
            }
            Action::CacheFile(hash) => {
                let path = cache.get(&hash).unwrap();
                stack.push(Action::FinishJob(path.clone(), hash.clone()));
            }
            Action::ReadDir(path) => {
                debug::is_dir(&path, &logger);
                stack.extend(
                    path.read_dir()
                        .expect("read_dir call failed")
                        .filter_map(|entry| {
                            let entry = entry.expect("entry is not valid");
                            let path = entry.path();
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
        }
    }

    assert!(stack.is_empty());
    println!("{:#?}\u{000A}", cache);

    Ok(())
}
