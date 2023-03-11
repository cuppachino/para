use anyhow::Result;
use camino::{Utf8Path, Utf8PathBuf};
use json_comments::StripComments;
use rayon::{iter::Either, prelude::*};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, path::Path};

#[derive(Default, Debug)]
pub struct ParaConfig {
    pub tsconfig: Tsconfig,
    pub path: Utf8PathBuf,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Tsconfig {
    #[serde(rename = "compilerOptions")]
    pub compiler_options: CompilerOptions,
    #[serde(default)]
    pub include: Vec<String>,
    #[serde(default)]
    pub exclude: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CompilerOptions {
    #[serde(rename = "baseUrl")]
    pub base_url: String,
    #[serde(rename = "outDir")]
    pub out_dir: String,
    pub paths: HashMap<String, Vec<String>>,
}

pub fn parse_tsconfig(path: impl AsRef<Path>) -> Result<ParaConfig> {
    let file = File::open(&path)?;
    let reader = std::io::BufReader::new(file);

    let mut para_config = ParaConfig::default();
    para_config.path = Utf8PathBuf::from_path_buf(path.as_ref().to_path_buf())
        .ok()
        .unwrap();
    para_config.tsconfig =
        serde_json::from_reader(StripComments::new(reader)).map_err(|e: serde_json::Error| {
            crate::log::error::missing_fields(&path, &e);
            e
        })?;

    Ok(para_config)
}

pub fn load_configs(paths: &[Utf8PathBuf]) -> (Vec<ParaConfig>, Vec<&Utf8PathBuf>) {
    let default_tsconfig_name: &str = "tsconfig.json";
    paths.par_iter().partition_map(|path| {
        match parse_tsconfig(normalize_dir_paths(path, default_tsconfig_name)) {
            Ok(config) => Either::Left(config),
            _ => Either::Right(path),
        }
    })
}

/// This will append a file name to a path if the path is a directory.
/// Otherwise, it will return the path as is.
fn normalize_dir_paths(path: &Utf8PathBuf, file: impl AsRef<Utf8Path>) -> Utf8PathBuf {
    let mut path = path.clone();
    if path.is_dir() {
        path.push(file);
    }
    path
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::Cwd;

    #[test]
    fn test_normalize_dir_paths() {
        let cwd = Cwd::new();
        let a = cwd.clone().join("tsconfig.json");

        assert_eq!(a, normalize_dir_paths(&cwd, "tsconfig.json"));
        assert_eq!(a, normalize_dir_paths(&a, cwd.join("tsconfig.json")));
    }

    #[test]
    fn test_load_configs() {
        let cwd = Cwd::new();
        let a = cwd.clone().join("a_tsconfig.jsonc");
        let b = cwd.clone().join("b_tsconfig.jsonc"); // is missing "outDir"
        let c = cwd.clone().join("myapp/tsconfig.json");

        let binding = [a, b, c];
        let (configs, errors) = load_configs(&binding);
        assert_eq!(configs.len(), 2);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0], &binding[1])
    }

    #[test]
    fn test_parse_tsconfig() {
        let cwd = Cwd::new();
        let a = cwd.clone().join("a_tsconfig.jsonc");
        let b = cwd.clone().join("b_tsconfig.jsonc"); // is missing "outDir"
        let c = cwd.clone().join("myapp/tsconfig.json");

        let a_config = parse_tsconfig(&a).unwrap();
        let b_config = parse_tsconfig(&b).ok();
        let c_config = parse_tsconfig(&c).unwrap();

        assert_eq!(a_config.path, a);
        assert!(b_config.is_none());
        assert_eq!(c_config.path, c);

        assert_eq!(a_config.tsconfig.compiler_options.out_dir, "dist");
        assert_eq!(a_config.tsconfig.compiler_options.base_url, ".");
        assert_eq!(a_config.tsconfig.compiler_options.paths.len(), 2);
        assert_eq!(
            a_config.tsconfig.compiler_options.paths["@/*"],
            vec!["pkg/*"]
        );
        assert_eq!(
            a_config.tsconfig.compiler_options.paths["$/*"],
            vec!["node_modules/*"]
        );
    }
}
