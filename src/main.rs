mod acc;
mod analysis;
mod cli;
mod config;
mod log;
mod output;

use atty::Stream;
use std::io::{self, BufRead};

use config::Config;

fn main() {
    let color = atty::is(Stream::Stdout);
    let args = cli::build(color).get_matches();
    let config = Config::from_args(&args);

    match args.values_of("dir") {
        Some(dirs) => {
            for dir in dirs {
                analysis::run(dir, &config);
            }
        }

        None => {
            let interactive = atty::is(Stream::Stdin);

            if interactive {
                log::warn("input is read from terminal");
                log::warn("only experts do this on purpose");
                log::warn("you may have forgotten to either");
                log::warn("- specify directories on the command line or");
                log::warn("- pipe data into this tool");
                log::warn("press CTRL-D or CTRL-C to exit");
            }

            let stdin = io::stdin();

            for line in stdin.lock().lines() {
                analysis::run(&line.unwrap().trim(), &config)
            }
        }
    }
}
