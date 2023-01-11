use clap::ArgMatches;

use crate::Output;

#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug)]
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
        let mut ages_in_days: Vec<u64> = args
            .get_many::<u64>("age")
            .expect("age is required")
            .copied()
            .collect();
        ages_in_days.sort_unstable();
        ages_in_days.dedup();

        let output = args
            .get_one::<Output>("format")
            .copied()
            .expect("format is required or has default");

        let debug = args.get_flag("debug");
        let progress = args.get_flag("progress") || debug;

        Self {
            debug,
            progress,
            ages_in_days,
            output,

            #[cfg(target_family = "unix")]
            one_file_system: args.get_flag("one-file-system"),

            #[cfg(feature = "spectrum-scale")]
            spectrum_scale: args.get_flag("spectrum-scale")
                || args.contains_id("spectrum-scale-N")
                || args.contains_id("spectrum-scale-g")
                || args.contains_id("spectrum-scale-s"),

            #[cfg(feature = "spectrum-scale")]
            spectrum_scale_nodes: args
                .get_one::<String>("spectrum-scale-N")
                .cloned(),

            #[cfg(feature = "spectrum-scale")]
            spectrum_scale_global_work_dir: args
                .get_one::<String>("spectrum-scale-g")
                .cloned(),

            #[cfg(feature = "spectrum-scale")]
            spectrum_scale_local_work_dir: args
                .get_one::<String>("spectrum-scale-s")
                .cloned(),
        }
    }
}
