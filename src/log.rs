pub fn error(message: String) {
    eprintln!("{}: {}", crate_name!(), message);
}
