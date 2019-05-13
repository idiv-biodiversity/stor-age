#[cfg(feature = "spectrum-scale")]
mod spectrum_scale;
mod universal;

use std::io::Result;

use crate::acc::Acc;
use crate::config::Config;
use crate::log;
use crate::output::{self, Output};

pub fn run(dir: &str, config: Config) {
    let result = run_conditional(dir, config);

    match result {
        Ok(acc) => match config.output {
            Output::Pretty => output::pretty(dir, acc, config.age_days),
            Output::Oneline => output::oneline(dir, acc),
        },

        Err(error) => {
            log::error(&format!("skipping directory {}: {}", dir, error));
        }
    }
}

#[cfg(not(feature = "spectrum-scale"))]
fn run_conditional(dir: &str, config: Config) -> Result<Acc> {
    crate::analysis::universal::run(dir, config)
}

#[cfg(feature = "spectrum-scale")]
fn run_conditional(dir: &str, config: Config) -> Result<Acc> {
    if config.spectrum_scale {
        crate::analysis::spectrum_scale::run(dir, config)
    } else {
        crate::analysis::universal::run(dir, config)
    }
}
