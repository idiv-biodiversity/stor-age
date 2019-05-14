use clap::crate_name;

pub fn error(message: &str) {
    eprintln!("{}: {}", crate_name!(), message);
}

pub fn warn(message: &str) {
    eprintln!("{}: warning: {}", crate_name!(), message);
}
