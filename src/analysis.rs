#[cfg(feature = "spectrum-scale")]
mod spectrum_scale;
mod universal;

use std::collections::HashMap;
use std::error::Error;

use crate::log;
use crate::output;
use crate::Config;
use crate::Output;
use crate::Result;

pub fn run(dirs: Vec<&str>, config: &Config) {
    let mut results = HashMap::new();

    for dir in dirs {
        log::progress(format!("analyzing {}", dir), config);

        let result = run_conditional(dir, config);

        match result {
            Ok(acc) => {
                results.insert(dir, acc);
            }

            Err(error) => {
                log::error(format!(
                    "skipping {}: {}",
                    dir,
                    error.description()
                ));
            }
        }
    }

    match config.output {
        Output::Prometheus => output::prometheus(results),
        Output::Oneline => output::oneline(results),
        #[cfg(feature = "table")]
        Output::Table => output::table(results),
    }
}

#[cfg(not(feature = "spectrum-scale"))]
fn run_conditional(dir: &str, config: &Config) -> Result {
    crate::analysis::universal::run(dir, config)
}

#[cfg(feature = "spectrum-scale")]
fn run_conditional(dir: &str, config: &Config) -> Result {
    if config.spectrum_scale {
        crate::analysis::spectrum_scale::run(dir, config)
    } else {
        crate::analysis::universal::run(dir, config)
    }
}
