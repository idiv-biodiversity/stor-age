mod acc;
mod analysis;
mod cli;
mod config;
mod log;
mod output;

use atty::Stream;
use clap::value_t;
use std::io::{self, BufRead};

use config::Config;
use output::Output;

fn main() {
    let color = atty::is(Stream::Stdout);
    let args = cli::build(color).get_matches();

    let age_days: u64 = args.value_of("age").unwrap().parse().unwrap();

    let output = value_t!(args, "format", Output).unwrap_or_else(|e| e.exit());

    let config = Config {
        debug: args.is_present("debug"),
        verbose: args.is_present("verbose"),
        age_days,
        output,

        #[cfg(feature = "spectrum-scale")]
        spectrum_scale: args.is_present("spectrum-scale"),
    };

    match args.values_of("dir") {
        Some(dirs) => {
            for dir in dirs {
                analysis::run(dir, config);
            }
        }

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
                analysis::run(&line.unwrap(), config)
            }
        }
    }
}
