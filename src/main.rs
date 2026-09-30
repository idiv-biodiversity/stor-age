#![forbid(unsafe_code)]
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]

mod cli;
mod config;

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, IsTerminal, Read, Write};

use anyhow::{Context, Result};
use stor_age::Data;

use crate::cli::{OutputFormat, OutputPath};
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
    let mut results: BTreeMap<&str, Data> = BTreeMap::new();

    for dir in dirs {
        if config.progress {
            log::info!("analyzing {dir}");
        }

        let result = run_conditional(dir, config);

        match result {
            Ok(acc) => {
                if config.outputs.iter().any(|(format, path)| {
                    *format == OutputFormat::Oneline
                        && *path == OutputPath::Stdout
                }) && let Err(err) =
                    stor_age::output::oneline(dir, &acc, &mut io::stdout())
                {
                    log::error!("{err}");
                }

                results.insert(dir, acc);
            }

            Err(error) => {
                log::error!("skipping {dir}: {error}");
            }
        }
    }

    for (format, path) in config.outputs.iter().filter(|(format, path)| {
        *format != OutputFormat::Oneline || *path != OutputPath::Stdout
    }) {
        let mut path: Box<dyn Write> = match path {
            OutputPath::Stdout => Box::new(io::stdout()),

            OutputPath::Path(path) => match File::create(path) {
                Ok(file) => Box::new(file),
                Err(err) => {
                    log::error!("{err}");
                    continue;
                }
            },
        };

        let result = match format {
            #[cfg(feature = "table")]
            OutputFormat::Markdown => {
                stor_age::output::table(&results, true, &mut path)
            }

            OutputFormat::Oneline => {
                stor_age::output::oneline_all(&results, &mut path)
            }

            OutputFormat::Prometheus => {
                stor_age::output::prometheus(&results, &mut path)
            }

            #[cfg(feature = "table")]
            OutputFormat::Table => {
                stor_age::output::table(&results, false, &mut path)
            }
        };

        if let Some(err) = result.err() {
            log::error!("{err}");
        }
    }
}

#[cfg(not(feature = "storage-scale"))]
fn run_conditional(dir: &str, config: &Config) -> Result<Data> {
    stor_age::universal(dir, &config.ages_in_days, config.one_file_system)
}

#[cfg(feature = "storage-scale")]
fn run_conditional(dir: &str, config: &Config) -> Result<Data> {
    if config.storage_scale {
        stor_age::storage_scale(
            dir,
            &config.ages_in_days,
            config.storage_scale_nodes.as_deref(),
            config.storage_scale_local_work_dir.as_deref(),
            config.storage_scale_global_work_dir.as_deref(),
        )
    } else {
        stor_age::universal(dir, &config.ages_in_days, config.one_file_system)
    }
}
