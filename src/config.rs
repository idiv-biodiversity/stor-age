use clap::ArgMatches;

use crate::Output;

#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug)]
pub struct Config {
    pub debug: bool,
    pub progress: bool,
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

        let one_file_system =
            args.try_contains_id("one-file-system").unwrap_or_default()
                && args.get_flag("one-file-system");

        Self {
            debug,
            progress,
            ages_in_days,
            output,

            one_file_system,

            #[cfg(feature = "spectrum-scale")]
            spectrum_scale: args.get_flag("spectrum-scale")
                || args.contains_id(mmpolicy::clap::ARG_NODES)
                || args.contains_id(mmpolicy::clap::ARG_GLOBAL_WORK_DIR)
                || args.contains_id(mmpolicy::clap::ARG_LOCAL_WORK_DIR),

            #[cfg(feature = "spectrum-scale")]
            spectrum_scale_nodes: args
                .get_one::<String>(mmpolicy::clap::ARG_NODES)
                .cloned(),

            #[cfg(feature = "spectrum-scale")]
            spectrum_scale_global_work_dir: args
                .get_one::<String>(mmpolicy::clap::ARG_GLOBAL_WORK_DIR)
                .cloned(),

            #[cfg(feature = "spectrum-scale")]
            spectrum_scale_local_work_dir: args
                .get_one::<String>(mmpolicy::clap::ARG_LOCAL_WORK_DIR)
                .cloned(),
        }
    }
}
