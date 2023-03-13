use anyhow::Result;
use camino::{Utf8Path, Utf8PathBuf};
use clean_path::clean;
use json_comments::StripComments;
use rayon::{iter::Either, prelude::*};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::File,
    path::{Path, PathBuf},
};

#[derive(Default, Debug)]
pub struct ParaConfig {
    pub tsconfig: Tsconfig,
    pub tsconfig_path: Utf8PathBuf,
    pub tsconfig_parent: Utf8PathBuf,

    pub resolved_out_dir: PathBuf,
    pub resolved_base_url: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Tsconfig {
    #[serde(rename = "compilerOptions")]
    pub compiler_options: CompilerOptions,
    pub include: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CompilerOptions {
    #[serde(rename = "baseUrl")]
    pub base_url: String,
    /// Defines the structure of the dist directory.
    ///
    /// ### Project Structure
    ///
    /// ```
    /// <proj>/src/foo/bar.ts
    /// outDir: dist
    ///
    /// ```
    ///
    /// ### Example Unset
    ///
    /// Defaults to the longest common PathBuf of all input files.
    ///
    ///
    /// ```
    /// dist
    /// └── bar.js
    ///
    /// /src/foo/bar.ts
    ///      ^^^
    /// /dist/bar.js
    ///     ^
    /// ```
    ///
    /// ### Example Set
    ///
    /// Here we set `rootDir` to `src` and `outDir` to `dist`.
    ///
    /// ```
    /// /src/foo/bar.ts -> /dist/foo/bar.js
    /// ```
    ///
    /// ```
    /// dist
    /// └── foo
    ///     └── bar.js
    /// ```
    ///
    #[serde(rename = "rootDir", default)]
    pub root_dir: String,
    #[serde(rename = "rootDirs", default)]
    pub root_dirs: Vec<String>,
    #[serde(rename = "outDir")]
    pub out_dir: String,
    pub paths: HashMap<String, Vec<String>>,
}

pub fn parse_tsconfig<P>(path: P) -> Result<ParaConfig>
where
    P: AsRef<Path>,
{
    let tsconfig_path = Utf8Path::from_path(path.as_ref()).unwrap();
    let file = File::open(tsconfig_path)?;
    let tsconfig: Tsconfig = serde_json::from_reader(StripComments::new(std::io::BufReader::new(
        file,
    )))
    .map_err(|e| {
        crate::log::error::missing_fields(tsconfig_path, &e);
        e
    })?;
    let tsconfig_parent = tsconfig_path.parent().unwrap();
    let (resolved_base_url, resolved_out_dir) = {
        let r = clean(tsconfig_parent.join(&tsconfig.compiler_options.base_url));
        (PathBuf::from(&r), PathBuf::from(&r))
    };

    // ? ---
    let para_config = ParaConfig {
        tsconfig,
        tsconfig_path: tsconfig_path.into(),
        tsconfig_parent: tsconfig_parent.into(),
        resolved_base_url,
        resolved_out_dir,
    };

    Ok(para_config)
}

pub fn load_configs(paths: &[Utf8PathBuf]) -> (Vec<ParaConfig>, Vec<&Utf8PathBuf>) {
    use crate::utils::normalize_dir_paths;
    let default_tsconfig_name: &str = "tsconfig.json";
    paths.par_iter().partition_map(|path| {
        match parse_tsconfig(normalize_dir_paths(path, default_tsconfig_name)) {
            Ok(config) => Either::Left(config),
            Err(e) => {
                crate::log::error::os_error(path, &e);
                Either::Right(path)
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::Cwd;

    #[test]
    fn default_to_tsconfig_dot_json() {
        use crate::utils::normalize_dir_paths;
        let cwd = Cwd::new();
        let a = cwd.clone().join("tsconfig.json");
        assert_eq!(a, normalize_dir_paths(&cwd, "tsconfig.json"));
        assert_eq!(a, normalize_dir_paths(&a, cwd.join("tsconfig.json")));
    }
    #[test]
    fn cannot_parse_without_args_or_default_tsconfig_path() {
        let cwd = Cwd::new();
        let config = parse_tsconfig(&cwd);
        assert!(config.is_err());
    }
    #[test]
    fn cannot_parse_invalid_tsconfig_paths() {
        let cwd = Cwd::new();
        let config = parse_tsconfig(&cwd.join("./tsconfig.json"));
        assert!(config.is_err());
    }
    #[test]
    fn cannot_parse_malformed_tsconfigs() {
        let cwd = Cwd::new();
        let b = cwd.clone().join("b_tsconfig.jsonc");
        let b_config = parse_tsconfig(&b).ok();
        assert!(b_config.is_none());
    }

    #[test]
    fn test_load_configs() {
        let cwd = Cwd::new();
        let tsconfig_paths = &[
            cwd.join("a_tsconfig.jsonc"),
            cwd.join("b_tsconfig.jsonc"),
            cwd.join("myapp/tsconfig.json"),
        ];
        let (configs, errors) = load_configs(tsconfig_paths);
        assert_eq!(configs.len(), 2);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0], &tsconfig_paths[1])
    }

    #[test]
    fn should_parse_found_tsconfigs() {
        let cwd = Cwd::new();
        let config = parse_tsconfig(cwd.join("myapp/tsconfig.json")).unwrap();

        // Tsconfig deserialization
        assert_eq!(config.tsconfig.compiler_options.base_url, "./");
        assert_eq!(config.tsconfig.compiler_options.out_dir, "./dist");
        assert_eq!(config.tsconfig.compiler_options.paths.len(), 2);
        assert_eq!(
            config.tsconfig.compiler_options.paths["@/*"],
            vec!["pkg/*", "pkg/other/*"]
        );
        assert_eq!(
            config.tsconfig.compiler_options.paths["$/*"],
            vec!["node_modules/*"]
        );
        assert_eq!(config.tsconfig_path, cwd.join("myapp/tsconfig.json"));
        assert_eq!(config.tsconfig_parent, cwd.join("myapp"));
        // - resolved_base_url
        assert_eq!(config.resolved_base_url, cwd.join("myapp/"));
        // - resolved_root_dir // todo
        // - resolved_out_dir // todo
        // - thinking on root_dirs... // todo
    }
}
