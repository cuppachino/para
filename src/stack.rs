use std::path::PathBuf;

use globset::GlobSet;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Action {
    ReadFile(PathBuf),
    ReadDir(PathBuf),
    CompareHash(PathBuf, String, String),
    FindCaptures(String, String),
    CacheFile(String),
    FinishJob(PathBuf, String),
}

pub trait RevealPath {
    fn reveal_path(&self) -> Option<&PathBuf>;
}

impl RevealPath for Action {
    fn reveal_path(&self) -> Option<&PathBuf> {
        match self {
            Action::ReadFile(path) => Some(path),
            Action::ReadDir(path) => Some(path),
            Action::CacheFile(_) => None,
            Action::CompareHash(path, _, _) => Some(path),
            Action::FindCaptures(_, _) => None,
            Action::FinishJob(path, _) => Some(path),
        }
    }
}

pub trait CanMatchGlobset {
    fn is_match(&self, globset: &GlobSet) -> Option<&PathBuf>;
}

impl CanMatchGlobset for Action {
    fn is_match(&self, globset: &GlobSet) -> Option<&PathBuf> {
        if let Some(path) = self.reveal_path() {
            if globset.is_match(path) {
                return Some(path);
            }
        }
        None
    }
}
