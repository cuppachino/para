use camino::{Utf8Path, Utf8PathBuf};
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
    ops::{Deref, DerefMut},
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
pub fn create_all_dir_until_file(dir: &Path, path: &PathBuf) -> std::io::Result<File> {
    std::fs::create_dir_all(dir)?;

    if path.metadata().is_ok() {
        File::open(path)
    } else {
        File::create(path)
    }
}

pub struct Cwd(Utf8PathBuf);

impl Cwd {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self(Utf8PathBuf::from_path_buf(std::env::current_dir().unwrap()).unwrap())
    }
}

impl Default for Cwd {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for Cwd {
    type Target = Utf8PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Cwd {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<Utf8Path> for Cwd {
    fn as_ref(&self) -> &Utf8Path {
        self.0.as_ref()
    }
}

impl AsRef<Utf8PathBuf> for Cwd {
    fn as_ref(&self) -> &Utf8PathBuf {
        &self.0
    }
}

impl AsRef<Path> for Cwd {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}
