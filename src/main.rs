extern crate atty;
extern crate bytesize;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;
extern crate mktemp;
extern crate regex;

use atty::Stream;
use bytesize::ByteSize;
use std::path::Path;

mod acc;
mod analysis;
mod cli;
mod config;

use acc::Acc;
use analysis::*;
use config::Config;

fn main() {
    let color = atty::is(Stream::Stdout);
    let matches = cli::build(color).get_matches();

    let dir = Path::new(matches.value_of("dir").unwrap());
    let age_days: u64 = matches.value_of("age").unwrap().parse().unwrap();
    let spectrum_scale = matches.is_present("spectrum-scale");

    let config = Config {
        debug: matches.is_present("debug"),
        verbose: matches.is_present("verbose"),
        age_days,
    };

    let Acc { total, access, modify } = if spectrum_scale {
        analyze_spectrum_scale(dir, &config).unwrap()
    } else {
        analyze(dir, &config).unwrap()
    };

    println!("total: {}", ByteSize(total).to_string_as(true));

    println!(
        "unaccessed for {} days: {}% ({})",
        config.age_days,
        ((access as f64) / (total as f64) * 100.0).round(),
        ByteSize(access).to_string_as(true),
    );

    println!(
        "unmodified for {} days: {}% ({})",
        config.age_days,
        ((modify as f64) / (total as f64) * 100.0).round(),
        ByteSize(modify).to_string_as(true),
    );
}
