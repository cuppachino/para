use crate::log::{debug, verbose, Logger};
use clean_path::Clean;
use sha2::{
    digest::{
        generic_array::GenericArray,
        typenum::{UInt, UTerm, B0, B1},
    },
    Digest, Sha256,
};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

/// This represents a 256-bit hash of a file.
pub type FileHash =
    GenericArray<u8, UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>>;

/// Hashes the contents of a file and returns a tuple of the hash and string contents of the file.
pub fn hash_file(node: &Path) -> (String, String) {
    let file = File::open(node).unwrap();
    let buff = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut contents = String::new();
    for line in buff.lines() {
        let line = line.unwrap();
        contents.push_str(&line);
        hasher.update(line);
    }
    let hash: FileHash = hasher.finalize();
    (format!("{:x}", hash), contents)
}

// Create all directories up to the file, then create the file if it doesn't exist.
// This works just like `std::fs::create_dir_all`, but the last path is a file.
fn create_all_dir_until_file(dir: &Path, path: &PathBuf) -> std::io::Result<File> {
    std::fs::create_dir_all(dir)?;

    if path.metadata().is_ok() {
        File::open(path)
    } else {
        File::create(path)
    }
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
    let cwd_hash = sha2::Sha256::digest(
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
    debug::cache_dir(&dirs.cache_dir().to_path_buf(), logger);
    debug::cache_dir(&project_cache_path, logger);

    create_all_dir_until_file(&dirs.cache_dir(), &project_cache_path)
        .expect("Failed to create/open cache file. Check user permissions.");

    Ok(project_cache_path)
}
