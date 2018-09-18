extern crate atty;
extern crate bytesize;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;
extern crate mktemp;
extern crate regex;

mod acc;
mod analysis;
mod cli;
mod config;
mod log;

use analysis::analyze;
use atty::Stream;
use config::Config;

fn main() {
    let color = atty::is(Stream::Stdout);
    let matches = cli::build(color).get_matches();

    let age_days: u64 = matches.value_of("age").unwrap().parse().unwrap();

    let config = Config {
        debug: matches.is_present("debug"),
        verbose: matches.is_present("verbose"),
        age_days,
        spectrum_scale: matches.is_present("spectrum-scale"),
    };

    for dir in matches.values_of("dir").unwrap() {
        let result = analyze(dir, &config);

        for error in result.err() {
            log::error(format!("skipping directory {:?}: {}", dir, error));
        }
    }
}
