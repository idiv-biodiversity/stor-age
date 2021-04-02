use atty::Stream;
use clap::{crate_description, crate_name, crate_version};
use clap::{App, AppSettings, Arg};
use std::fs;
use std::path::Path;

use stor_age::Output;

pub fn build() -> App<'static, 'static> {
    let color = if atty::is(Stream::Stdout) {
        AppSettings::ColoredHelp
    } else {
        AppSettings::ColorNever
    };

    let age = Arg::with_name("age")
        .help("threshold in days")
        .multiple(true)
        .required(true)
        .validator(is_number);

    let debug = Arg::with_name("debug")
        .long("debug")
        .long_help(
"Adds very verbose output useful for debugging. Implies `--progress`."
        )
        .hidden_short_help(true);

    let progress = Arg::with_name("progress")
        .long("progress")
        .help("show progress messages")
        .long_help("Show progress message for each directory.")
        .display_order(3);

    let dir = Arg::with_name("dir")
        .help("input directories")
        .long_help(
"The input directories for which to gather information. If none are given, \
 directories are read from standard input. This way, this tool can be used in \
 pipes that get their input from e.g. `find`.",
        )
        .multiple(true)
        .required(atty::is(Stream::Stdin))
        .last(true)
        .validator(is_dir);

    let format = Arg::with_name("format")
        .long("format")
        .help("output format")
        .long_help(
"Specify output format of the report. `prometheus` uses the Prometheus \
 metric exposition format. `oneline` is intended as machine-readable output \
 that shows a colon (\":\") separated list of age, total, accessed, and \
 modified size in bytes, total, accessed, and modified number of files, \
 followed by the directory. `table` (cargo feature, enabled by default) shows \
 a pretty-printed table."
        )
        .takes_value(true)
        .case_insensitive(true)
        .possible_values(&Output::variants())
        .display_order(1);

    let format = if cfg!(feature = "table") {
        format.default_value("table")
    } else {
        format.required(true)
    };

    let conditional_compilation_args: Vec<Arg> = vec![
        #[cfg(target_family = "unix")]
        Arg::with_name("one-file-system")
            .short("x")
            .long("one-file-system")
            .help("do not cross file system boundaries")
            .long_help(
"Do not cross file system boundaries, i.e. skip files and directories on \
 different file systems than the directory being scanned."
            )
            .display_order(1),

        #[cfg(feature = "spectrum-scale")]
        Arg::with_name("spectrum-scale")
            .long("spectrum-scale")
            .help("use mmapplypolicy instead of universal directory traversal")
            .long_help(
"On IBM Spectrum Scale file systems exists a dedicated command that allows \
 more efficient file system traversal, called `mmapplypolicy`. Using this \
 flag forces the usage of this command over the universal directory \
 traversal. At the time of this writing, according to Spectrum Scale \
 documentation, only the super-user `root` may use the `mmapplypolicy` \
 command.",
            )
            .display_order(2),

        #[cfg(feature = "spectrum-scale")]
        Arg::with_name("spectrum-scale-N")
            .long("spectrum-scale-N")
            .help("use for mmapplypolicy -N argument")
            .long_help(
"Specify list of nodes to use with `mmapplypolicy -N`. For detailed \
 information, see `man mmapplypolicy`. Implies `--spectrum-scale`.",
            )
            .takes_value(true)
            .value_name("all|mount|Node,...|NodeFile|NodeClass"),

        #[cfg(feature = "spectrum-scale")]
        Arg::with_name("spectrum-scale-g")
            .long("spectrum-scale-g")
            .help("use for mmapplypolicy -g argument")
            .long_help(
"Specify global work directory to use with `mmapplypolicy -g`. For detailed \
 information, see `man mmapplypolicy`. Implies `--spectrum-scale`.",
            )
            .takes_value(true)
            .value_name("dir")
            .validator(is_dir),

        #[cfg(feature = "spectrum-scale")]
        Arg::with_name("spectrum-scale-s")
            .long("spectrum-scale-s")
            .help("use for mmapplypolicy -s argument and policy output")
            .long_help(
"Specify local work directory to use with `mmapplypolicy -s`. Also, the \
 output of the LIST policies will be written to this directory temporarily \
 before being processed by this tool. Defaults to the system temporary \
 directory. This might be too small for large directories, e.g. more than 30 \
 GiB are needed for a directory with 180 million files. For detailed \
 information about the `-s` argument, see `man mmapplypolicy`. Implies \
 `--spectrum-scale`.",
            )
            .takes_value(true)
            .value_name("dir")
            .validator(is_dir),
    ];

    App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .global_setting(color)
        .help_short("?")
        .arg(age)
        .arg(dir)
        .arg(debug)
        .arg(format)
        .arg(progress)
        .args(&conditional_compilation_args)
}

fn is_dir(s: String) -> Result<(), String> {
    let path = Path::new(&s);

    if !path.exists() {
        Err(format!("does not exist: {:?}", path))
    } else if !path.is_dir() {
        Err(format!("is not a directory: {:?}", path))
    } else if let Err(error) = fs::read_dir(path) {
        Err(error.to_string())
    } else {
        Ok(())
    }
}

fn is_number(s: String) -> Result<(), String> {
    if s.parse::<u64>().is_ok() {
        Ok(())
    } else {
        Err(format!("not a positive number: {}", s))
    }
}
