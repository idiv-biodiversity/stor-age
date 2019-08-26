use clap::crate_name;

use crate::Config;

pub fn debug<S: AsRef<str>>(message: S, config: &Config) {
    if config.debug {
        eprintln!("{}: {}", crate_name!(), message.as_ref())
    }
}

pub fn error<S: AsRef<str>>(message: S) {
    eprintln!("{}: {}", crate_name!(), message.as_ref());
}

pub fn info<S: AsRef<str>>(message: S) {
    eprintln!("{}: {}", crate_name!(), message.as_ref());
}

pub fn warn<S: AsRef<str>>(message: S) {
    eprintln!("{}: warning: {}", crate_name!(), message.as_ref());
}
