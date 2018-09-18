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
mod output;

use analysis::analyze;
use atty::Stream;
use config::Config;
use output::Output;
use std::io::{self, BufRead};

fn main() {
    let color = atty::is(Stream::Stdout);
    let args = cli::build(color).get_matches();

    let age_days: u64 = args.value_of("age").unwrap().parse().unwrap();

    let output = value_t!(args, "format", Output)
        .unwrap_or_else(|e| e.exit());

    let config = Config {
        debug: args.is_present("debug"),
        verbose: args.is_present("verbose"),
        age_days,
        spectrum_scale: args.is_present("spectrum-scale"),
        output,
    };

    match args.values_of("dir") {
        Some(dirs) => {
            for dir in dirs {
                analyze(dir, &config);
            }
        },

        None => {
            let interactive = atty::is(Stream::Stdin);

            if interactive {
                log::warning("input is read from terminal");
                log::warning("only experts do this on purpose");
                log::warning("you may have forgotten to either");
                log::warning("- specify directories on the command line or");
                log::warning("- pipe data into this tool");
                log::warning("press CTRL-D or CTRL-C to exit");
            }

            let stdin = io::stdin();

            for line in stdin.lock().lines() {
                analyze(&line.unwrap(), &config)
            }
        },
    }
}
