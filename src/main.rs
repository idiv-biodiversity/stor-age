#![forbid(unsafe_code)]
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]

mod cli;
mod config;

use std::collections::HashMap;
use std::io::{self, IsTerminal, Read};

use anyhow::{Context, Result};
use stor_age::Data;

use crate::cli::Output;
use crate::config::Config;

fn main() -> Result<()> {
    let stdin_terminal = std::io::stdin().is_terminal();
    let args = cli::build(stdin_terminal).get_matches();
    let config = Config::from_args(&args);

    if config.debug {
        env_logger::Builder::default()
            .filter_level(log::LevelFilter::Debug)
            .init();
    } else if config.progress {
        env_logger::Builder::default()
            .filter_level(log::LevelFilter::Info)
            .init();
    } else {
        env_logger::init();
    }

    log::debug!("{config:#?}");

    if let Some(dirs) = args.get_many::<String>("dir") {
        let dirs: Vec<&str> = dirs.map(String::as_str).collect();
        run(&dirs, &config);
    } else {
        let mut dirs = String::new();

        io::stdin()
            .read_to_string(&mut dirs)
            .with_context(|| "error reading from stdin")?;

        let dirs: Vec<&str> = dirs.lines().collect();

        run(&dirs, &config);
    }

    Ok(())
}

pub fn run(dirs: &[&str], config: &Config) {
    let mut results: HashMap<&str, Data> = HashMap::new();

    for dir in dirs {
        if config.progress {
            log::info!("analyzing {dir}");
        }

        let result = run_conditional(dir, config);

        match result {
            Ok(acc) => {
                results.insert(dir, acc);
            }

            Err(error) => {
                log::error!("skipping {dir}: {error}");
            }
        }
    }

    match config.output {
        Output::Prometheus => stor_age::output::prometheus(&results),
        Output::Oneline => stor_age::output::oneline(&results),
        #[cfg(feature = "table")]
        Output::Table => stor_age::output::table(&results),
    }
}

#[cfg(not(feature = "spectrum-scale"))]
fn run_conditional(dir: &str, config: &Config) -> Result<Data> {
    stor_age::universal(dir, &config.ages_in_days, config.one_file_system)
}

#[cfg(feature = "spectrum-scale")]
fn run_conditional(dir: &str, config: &Config) -> Result<Data> {
    if config.spectrum_scale {
        stor_age::spectrum_scale(
            dir,
            &config.ages_in_days,
            config.spectrum_scale_nodes.as_deref(),
            config.spectrum_scale_local_work_dir.as_deref(),
            config.spectrum_scale_global_work_dir.as_deref(),
        )
    } else {
        stor_age::universal(dir, &config.ages_in_days, config.one_file_system)
    }
}
