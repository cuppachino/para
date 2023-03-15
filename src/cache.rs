use crate::{
    log::{verbose, Logger},
    utils::{create_all_dir_until_file, FileHash},
};
use clean_path::Clean;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Cache {
    cache: Vec<Entry>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Entry {
    hash: String,
    path: String,
}

impl Cache {
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }
}

impl std::fmt::Display for Cache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Cache:")?;
        for bucket in &self.cache {
            writeln!(f, "{}\u{9}{}", bucket.hash.to_uppercase(), bucket.path)?;
        }
        Ok(())
    }
}

impl From<Vec<Entry>> for Cache {
    fn from(cache: Vec<Entry>) -> Self {
        Self { cache }
    }
}

impl From<Cache> for HashMap<String, PathBuf> {
    fn from(cache: Cache) -> Self {
        let mut cache_hashmap: HashMap<String, PathBuf> = HashMap::new();
        for bucket in cache.cache {
            cache_hashmap.insert(bucket.hash, PathBuf::from(bucket.path));
        }
        cache_hashmap
    }
}

impl From<HashMap<String, PathBuf>> for Cache {
    fn from(cache: HashMap<String, PathBuf>) -> Self {
        let mut cache_vec: Vec<Entry> = vec![];
        for (hash, path) in cache {
            cache_vec.push(Entry {
                hash,
                path: path.to_str().unwrap().to_string(),
            });
        }
        Self { cache: cache_vec }
    }
}

pub fn parse_cache(cache_dir: &PathBuf) -> Option<Cache> {
    let toml_string = std::fs::read_to_string(cache_dir).ok()?;
    if toml_string.is_empty() {
        return Some(Cache { cache: vec![] });
    }
    toml::from_str(&toml_string).ok()?
}

pub fn load_cache(
    cache_dir: &PathBuf,
    logger: &Logger,
) -> anyhow::Result<HashMap<String, PathBuf>> {
    if let Some(cache) = parse_cache(cache_dir) {
        return Ok(cache.into());
    }
    logger.warn("Cache was corrupt so we're starting fresh.");
    Ok(HashMap::new())
}

pub fn save_cache(cache_dir: &PathBuf, cache: Cache) -> anyhow::Result<()> {
    let toml_string = toml::to_string(&cache)?;
    std::fs::write(cache_dir, toml_string)?;
    Ok(())
}

/// Creates a para directory where your OS likes to store cache files. If the
/// directory already exists, it will return the path to the existing directory.
pub fn generate_cache_dir(logger: &Logger) -> Result<PathBuf, std::io::Error> {
    // locate the root cache directory for para
    let dirs = directories::ProjectDirs::from("com", "cuppachino", "para")
        .ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::NotFound, "Failed to get project dirs")
        })
        .unwrap();

    // hash the current working directory's path to get a unique hash for a blank file in the cache directory.
    let cwd_hash: FileHash = sha2::Sha256::digest(
        std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .as_bytes(),
    );

    // assemble the complete path to the cache file for [this] project
    let project_cache_path = dirs
        .cache_dir()
        .join(format!("{:x}.para", cwd_hash))
        .clean();

    verbose::hashed_cwd(&cwd_hash, logger);
    verbose::cache_dir(&dirs.cache_dir().to_path_buf(), logger);
    verbose::cache_dir(&project_cache_path, logger);

    create_all_dir_until_file(&dirs.cache_dir(), &project_cache_path)
        .expect("Failed to create/open cache file. Check user permissions.");

    Ok(project_cache_path)
}
