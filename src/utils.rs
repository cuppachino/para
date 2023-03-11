use camino::{Utf8Path, Utf8PathBuf};
use std::{
    ops::{Deref, DerefMut},
    path::Path,
};

pub struct Cwd(Utf8PathBuf);

impl Cwd {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self(Utf8PathBuf::from_path_buf(std::env::current_dir().unwrap().to_path_buf()).unwrap())
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
