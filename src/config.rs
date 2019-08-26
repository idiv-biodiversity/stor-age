use clap::{value_t, ArgMatches};

use crate::Output;

#[derive(Clone)]
pub struct Config {
    pub debug: bool,
    pub ages_in_days: Vec<u64>,
    pub output: Output,
    pub one_file_system: bool,

    #[cfg(feature = "spectrum-scale")]
    pub spectrum_scale: bool,

    #[cfg(feature = "spectrum-scale")]
    pub spectrum_scale_nodes: Option<String>,

    #[cfg(feature = "spectrum-scale")]
    pub spectrum_scale_global_work_dir: Option<String>,

    #[cfg(feature = "spectrum-scale")]
    pub spectrum_scale_local_work_dir: Option<String>,
}

impl Config {
    pub fn from_args(args: &ArgMatches) -> Config {
        let ages_in_days = args.values_of("age").unwrap();
        let ages_in_days = ages_in_days.map(|age| age.parse().unwrap());
        let mut ages_in_days: Vec<u64> = ages_in_days.collect();
        ages_in_days.sort();
        ages_in_days.dedup();

        let output = value_t!(args, "format", Output).unwrap();

        Config {
            debug: args.is_present("debug"),
            ages_in_days,
            output,
            one_file_system: args.is_present("one-file-system"),

            #[cfg(feature = "spectrum-scale")]
            spectrum_scale: args.is_present("spectrum-scale")
                || args.is_present("spectrum-scale-N")
                || args.is_present("spectrum-scale-g")
                || args.is_present("spectrum-scale-s"),

            #[cfg(feature = "spectrum-scale")]
            spectrum_scale_nodes: args
                .value_of("spectrum-scale-N")
                .map(String::from),

            #[cfg(feature = "spectrum-scale")]
            spectrum_scale_global_work_dir: args
                .value_of("spectrum-scale-g")
                .map(String::from),

            #[cfg(feature = "spectrum-scale")]
            spectrum_scale_local_work_dir: args
                .value_of("spectrum-scale-s")
                .map(String::from),
        }
    }
}
