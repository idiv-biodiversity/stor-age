use std::str::FromStr;

use clap::{value_t, ArgMatches};

#[derive(Clone, Copy)]
pub enum Output {
    Oneline,
    Prometheus,
    #[cfg(feature = "table")]
    Table,
}

impl Output {
    #[must_use]
    pub fn variants<'a>() -> Vec<&'a str> {
        vec![
            "oneline",
            "prometheus",
            #[cfg(feature = "table")]
            "table",
        ]
    }
}

impl FromStr for Output {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        let s = s.as_str();

        match s {
            "oneline" => Ok(Self::Oneline),
            "prometheus" => Ok(Self::Prometheus),
            #[cfg(feature = "table")]
            "table" => Ok(Self::Table),
            _ => Err(String::from("invalid output")),
        }
    }
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Clone)]
pub struct Config {
    pub debug: bool,
    pub progress: bool,
    pub ages_in_days: Vec<u64>,
    pub output: Output,

    #[cfg(target_family = "unix")]
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
    /// Returns configuration from `clap` arguments.
    ///
    /// # Panics
    ///
    /// Panics if required arguments are not present.
    #[must_use]
    pub fn from_args(args: &ArgMatches) -> Self {
        let ages_in_days = args.values_of("age").unwrap();
        let ages_in_days = ages_in_days.map(|age| age.parse().unwrap());
        let mut ages_in_days: Vec<u64> = ages_in_days.collect();
        ages_in_days.sort_unstable();
        ages_in_days.dedup();

        let output = value_t!(args, "format", Output).unwrap();

        let debug = args.is_present("debug");
        let progress = args.is_present("progress") || debug;

        Self {
            debug,
            progress,
            ages_in_days,
            output,

            #[cfg(target_family = "unix")]
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
