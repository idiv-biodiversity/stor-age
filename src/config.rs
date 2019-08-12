use clap::{value_t, ArgMatches};

use crate::output::Output;

#[derive(Clone)]
pub struct Config {
    pub debug: bool,
    pub age_days: u64,
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
        let age_days = args.value_of("age").unwrap();
        let age_days = age_days.parse().unwrap();

        let output = value_t!(args, "format", Output).unwrap();

        Config {
            debug: args.is_present("debug"),
            age_days,
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
