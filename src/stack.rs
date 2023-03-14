use std::path::PathBuf;

use globset::GlobSet;

#[derive(Debug)]
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

pub trait CanExcludeSelf {
    fn is_blacklisted(&self, globset: &GlobSet) -> bool;
}

impl CanExcludeSelf for Action {
    fn is_blacklisted(&self, globset: &GlobSet) -> bool {
        let path = match self {
            Action::ReadFile(path) => Some(path),
            Action::ReadDir(path) => Some(path),
            Action::CacheFile(_) => None,
            Action::CompareHash(path, _, _) => Some(path),
            Action::FindCaptures(_, _) => None,
            Action::FinishJob(path, _) => Some(path),
        };
        if path.is_none() {
            return false;
        }
        globset.is_match(path.unwrap())
    }
}
