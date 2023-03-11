use clap::Parser;
use rayon::prelude::*;

mod cli;
mod format;
mod log;
mod parser;
mod utils;

fn main() -> std::io::Result<()> {
    let pod = cli::Pod::parse();
    let logger = log::Logger::init(pod.log_level);

    // # 1. Extract paths from CLI
    let paths = pod.paths.as_parallel_slice();
    log::debug::tsconfig_paths(&paths, &logger);

    // # 2. Filter & Parse tsconfig.json(s) Vec<Tsconfig>
    let (configs, skipped) = parser::load_configs(paths);
    log::verbose::tsconfigs(&configs, &logger);
    log::warn::paths_skipped(skipped, &logger);
    log::info::configs_loaded(paths.len(), configs.len(), &logger);

    Ok(())
}
