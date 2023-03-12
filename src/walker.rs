use camino::Utf8PathBuf;
use clean_path::clean;
use rayon::prelude::*;
use std::path::PathBuf;

use std::fs::{metadata, read_dir};

fn walk_dir(paths: &Vec<PathBuf>, blacklist: &[&str]) -> Vec<PathBuf> {
    paths
        .par_iter()
        .filter_map(|entry| {
            if blacklist
                .iter()
                .any(|&s| clean(entry.to_string_lossy().to_lowercase()).ends_with(s.to_lowercase()))
            {
                return None;
            }
            let entry = clean(entry);

            let meta = metadata(&entry).ok();
            meta.as_ref()?;
            let meta = meta.unwrap();

            if meta.is_file() {
                return Some(vec![entry]);
            }
            if meta.is_dir() {
                let dir = read_dir(&entry);
                if dir.is_err() {
                    return None;
                }
                let dir = dir.unwrap();
                let paths = dir.map(|entry| entry.unwrap().path()).collect::<Vec<_>>();
                return Some(walk_dir(&paths, blacklist));
            }
            None
        })
        .flatten()
        .collect::<Vec<_>>()
}

pub fn demo(dist_folder: PathBuf, exclude: Vec<Utf8PathBuf>) -> Vec<PathBuf> {
    let paths = vec![dist_folder];

    walk_dir(
        &paths,
        &exclude.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
    )
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn should_walk_to_single_file() {
        let paths = vec![PathBuf::from_str("./myapp/tsconfig.json").unwrap()];
        let result = walk_dir(&paths, &["node_modules", "dist"]);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], clean("./myapp/tsconfig.json"));
    }

    #[test]
    fn should_walk_multiple_directories() {
        let paths = vec![clean("./myapp/dist"), clean("./myapp/pkg")];
        let result = walk_dir(&paths, &["node_modules"]);

        assert_eq!(result.len(), 4);
        assert!(result.contains(&clean("./myapp/dist/index.js")));
        assert!(result.contains(&clean("./myapp/dist/lib.js")));
        assert!(result.contains(&clean("./myapp/pkg/index.ts")));
        assert!(result.contains(&clean("./myapp/pkg/lib.ts")));
    }
}
