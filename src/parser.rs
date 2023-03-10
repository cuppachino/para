use anyhow::Result;
use camino::Utf8PathBuf;
use json_comments::StripComments;
use rayon::{iter::Either, prelude::*};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::BufReader};

#[derive(Serialize, Deserialize, Debug)]
pub struct Tsconfig {
    #[serde(rename = "compilerOptions")]
    compiler_options: CompilerOptions,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct CompilerOptions {
    #[serde(rename = "baseUrl")]
    base_url: String,
    paths: HashMap<String, Vec<String>>,
    #[serde(default)]
    include: Vec<String>,
    #[serde(default)]
    exclude: Vec<String>,
}

pub(crate) fn parse_tsconfig<P: AsRef<std::path::Path>>(path: P) -> Result<Tsconfig> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let stripped = StripComments::new(reader);

    let tsconfig: Tsconfig = serde_json::from_reader(stripped)?;

    Ok(tsconfig)
}

pub(crate) fn load_configs(paths: &[Utf8PathBuf]) -> (Vec<Tsconfig>, Vec<&Utf8PathBuf>) {
    paths
        .par_iter()
        .partition_map(|path| match parse_tsconfig(path) {
            Ok(config) => Either::Left(config),
            _ => Either::Right(path),
        })
}
