extern crate atty;
extern crate bytesize;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;
extern crate regex;

use atty::Stream;
use bytesize::ByteSize;
use clap::{App, AppSettings, Arg};
use regex::Regex;
use std::fs::{self, DirEntry};
use std::io;
use std::ops::AddAssign;
use std::path::Path;
use std::time::{Duration, SystemTime};

struct Config {
    debug: bool,
    verbose: bool,
    age_days: u64,
}

struct Acc {
    total: u64,
    access: u64,
    modify: u64,
}

impl Acc {
    fn new(total: u64, access: u64, modify: u64) -> Acc {
        Acc { total, access, modify }
    }

    fn empty() -> Acc {
        Acc::new(0, 0, 0)
    }
}

impl AddAssign for Acc {
    fn add_assign(&mut self, other: Acc) {
        *self = Acc {
            total: self.total + other.total,
            access: self.access + other.access,
            modify: self.modify + other.modify,
        }
    }
}

fn main() {
    let color = if atty::is(Stream::Stdout) {
        AppSettings::ColoredHelp
    } else {
        AppSettings::ColorNever
    };

    let matches = App::new("stor-age")
        .version(crate_version!())
        .global_setting(color)
        .about("analyze storage ageing")
        .arg(Arg::with_name("age")
             .help("threshold in days")
             .required(true)
             .validator(is_number))
        .arg(Arg::with_name("dir")
             .help("the directory for which to gather information")
             .required(true)
             .validator(is_dir))
        .arg(Arg::with_name("debug")
             .long("debug")
             .help("debug output"))
        .arg(Arg::with_name("verbose")
             .short("v")
             .long("verbose")
             .help("verbose output"))
        .get_matches();

    let dir = Path::new(matches.value_of("dir").unwrap());
    let age_days: u64 = matches.value_of("age").unwrap().parse().unwrap();

    let config = Config {
        debug: matches.is_present("debug"),
        verbose: matches.is_present("verbose"),
        age_days,
    };

    let Acc { total, access, modify } = analyze(dir, &config).unwrap();

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

fn analyze(dir: &Path, config: &Config) -> io::Result<Acc> {
    let sys_time = SystemTime::now();
    let age = Duration::from_secs(config.age_days * 3600 * 24);
    let threshold = sys_time - age;

    let fun = |entry: &DirEntry| -> io::Result<Acc> {
        if config.debug {
            println!("visiting entry: {:?}: ", entry);
        }

        let meta = entry.metadata()?;

        let len = meta.len();

        let access = if meta.accessed()? < threshold {
            len
        } else {
            0
        };

        let modify = if meta.modified()? < threshold {
            len
        } else {
            0
        };

        Ok(Acc::new(len, access, modify))
    };

    visit_dirs(dir, &fun, config)
}

fn visit_dirs(
    dir: &Path,
    cb: &Fn(&DirEntry) -> io::Result<Acc>,
    config: &Config,
) -> io::Result<Acc> {
    let mut sum = Acc::empty();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            sum += cb(&entry)?;
        } else if path.is_dir() {
            if config.verbose {
                eprintln!("decending into: {:?}", path);
            }

            sum += visit_dirs(&path, cb, config)?;
        } else {
            if config.debug {
                eprintln!(
                    "neither directory nor regular file, skipping: {:?}",
                    path,
                );
            }
        }
    }

    Ok(sum)
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
        static ref NUMBER_RE: Regex = Regex::new(
            r#"^\d+$"#
        ).unwrap();
    }

    if NUMBER_RE.is_match(&s) {
        Ok(())
    } else {
        Err(format!("not a number: {}", s))
    }
}
