use anyhow::Result;
use json_comments::StripComments;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::BufReader};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Tsconfig {
    #[serde(rename = "compilerOptions")]
    compiler_options: CompilerOptions,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct CompilerOptions {
    #[serde(rename = "baseUrl")]
    base_url: String, // Required*
    paths: HashMap<String, Vec<String>>, // Required*
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
