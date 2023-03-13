use anyhow::{anyhow, Error, Result};
use globset::{Glob, GlobSet, GlobSetBuilder};

/// This transforms a list of exclusions from the CLI into a list of globs.
///
/// This trait is implemented in `IntoGlobSet` and `IntoGlobs`.
trait IntoGlobs {
    fn into_globs(&self) -> Vec<Glob>;
}

impl IntoGlobs for Vec<String> {
    fn into_globs(&self) -> Vec<Glob> {
        self.into_iter()
            .map(|glob| {
                Glob::new(&{
                    if !glob.starts_with("*/") {
                        format!("*/{}", glob)
                    } else {
                        glob.to_string()
                    }
                    .replace(r#"\\"#, r#"/"#)
                })
                .unwrap()
            })
            .collect::<Vec<_>>()
    }
}

pub trait IntoGlobSet {
    fn into_globset(&self) -> Result<GlobSet, Error>;
}

/// Transforms the exclude list into a globset.
impl IntoGlobSet for Vec<String> {
    fn into_globset(&self) -> Result<GlobSet, Error> {
        let mut builder = GlobSetBuilder::new();
        for glob in self.into_globs() {
            builder.add(glob);
        }
        builder
            .build()
            .map_err(|e| anyhow!("Failed to build globset from exclusions: {}", e.to_string()))
    }
}
