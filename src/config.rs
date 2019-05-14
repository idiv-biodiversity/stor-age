use clap::{value_t, ArgMatches};

use crate::output::Output;

#[derive(Clone, Copy)]
pub struct Config {
    pub debug: bool,
    pub age_days: u64,
    pub output: Output,

    #[cfg(feature = "spectrum-scale")]
    pub spectrum_scale: bool,
}

impl Config {
    pub fn from_args(args: &ArgMatches) -> Config {
        let age_days = args.value_of("age").unwrap();
        let age_days = age_days.parse().unwrap();

        let output = value_t!(args, "format", Output).unwrap();

        Config {
            debug: args.is_present("debug"),
            age_days,
            output,

            #[cfg(feature = "spectrum-scale")]
            spectrum_scale: args.is_present("spectrum-scale"),
        }
    }
}
