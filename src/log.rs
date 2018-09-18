pub fn error(message: &str) {
    eprintln!("{}: {}", crate_name!(), message);
}

pub fn warning(message: &str) {
    eprintln!("{}: warning: {}", crate_name!(), message);
}
