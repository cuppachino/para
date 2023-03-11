use anyhow::Result;
use camino::Utf8PathBuf;
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
    paths
        .par_iter()
        .partition_map(|path| match parse_tsconfig(path) {
            Ok(config) => Either::Left(config),
            _ => Either::Right(path),
        })
}
