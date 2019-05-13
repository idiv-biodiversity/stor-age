use clap::{crate_description, crate_name, crate_version};
use clap::{App, AppSettings, Arg};
use lazy_static::lazy_static;
use regex::Regex;
use std::path::Path;

use crate::output::Output;

pub fn build(color: bool) -> App<'static, 'static> {
    let color = if color {
        AppSettings::ColoredHelp
    } else {
        AppSettings::ColorNever
    };

    let age = Arg::with_name("age")
        .help("threshold in days")
        .required(true)
        .validator(is_number);

    let debug = Arg::with_name("debug")
        .long("debug")
        .help("debug output")
        .display_order(2);

    let dir = Arg::with_name("dir")
        .help("input directories")
        .long_help(
"The input directories for which to gather information. If none are given, \
 directories are read from standard input. This way, this tool can be used in \
 pipes that get their input from e.g. `find`.",
        )
        .multiple(true)
        .validator(is_dir);

    let format = Arg::with_name("format")
        .long("format")
        .help("output format")
        .long_help(
"Specify output format. Pretty is intended for human-readable interactive \
 use. Oneline is intended as machine-readable output that shows a colon \
 (\":\") separated list of total, unaccessed, and unmodified size in bytes, \
 followed by the directory.",
        )
        .takes_value(true)
        .case_insensitive(true)
        .possible_values(&Output::variants())
        .default_value("Pretty");

    let verbose = Arg::with_name("verbose")
        .short("v")
        .long("verbose")
        .help("verbose output")
        .display_order(2);

    let conditional_compilation_args: Vec<Arg> =
        vec![#[cfg(feature = "spectrum-scale")]
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
            .display_order(1)];

    App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .global_setting(color)
        .help_short("?")
        .arg(age)
        .arg(dir)
        .arg(debug)
        .arg(verbose)
        .arg(format)
        .args(&conditional_compilation_args)
}

fn is_dir(s: String) -> Result<(), String> {
    let path = Path::new(&s);

    if !path.exists() {
        Err(format!("does not exist: {:?}", path))
    } else if !path.is_dir() {
        Err(format!("is not a directory: {:?}", path))
    } else {
        Ok(())
    }
}

fn is_number(s: String) -> Result<(), String> {
    lazy_static! {
        static ref NUMBER_RE: Regex = Regex::new(r#"^\d+$"#).unwrap();
    }

    if NUMBER_RE.is_match(&s) {
        Ok(())
    } else {
        Err(format!("not a number: {}", s))
    }
}
