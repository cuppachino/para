use camino::Utf8PathBuf;
use clap::Parser;
use owo_colors::OwoColorize;
use rayon::prelude::*;

mod cli;
mod exclusions;
mod format;
mod log;
mod parser;
mod utils;
mod walker;

fn main() -> std::io::Result<()> {
    // Parse CLI arguments
    let cli = cli::Cli::parse();
    let paths = cli
        .paths_arg
        .unwrap_or_default()
        .into_par_iter()
        .chain(
            cli.paths
                .unwrap_or(vec![Utf8PathBuf::from("./tsconfig.json")]),
        )
        .collect::<Vec<_>>();

    // Setup logger
    use log::{debug, info, verbose, warn, Logger};
    let logger = Logger(cli.log_level);
    debug::tsconfig_paths(&paths, &logger);

    // Skip bad paths and parse valid configs.
    let (configs, skipped) = parser::load_configs(&paths);
    verbose::tsconfigs(&configs, &logger);
    warn::paths_skipped(skipped, &logger);
    info::configs_loaded(paths.len(), configs.len(), &logger);

    // Exit if no configs were found
    if configs.is_empty() {
        return Ok(());
    }

    // Convert the CLI exclude args into a GlobSet
    use exclusions::IntoGlobSet;
    let exclude_globset = cli.exclude.into_globset().unwrap();

    Ok(())
}
