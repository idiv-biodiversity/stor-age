mod cli;

use std::io::{self, Read};

use stor_age::Config;

fn main() {
    let args = cli::build().get_matches();
    let config = Config::from_args(&args);

    match args.values_of("dir") {
        Some(dirs) => stor_age::run(dirs.collect(), &config),
        None => {
            let mut dirs = String::new();

            io::stdin()
                .read_to_string(&mut dirs)
                .expect("error reading from stdin");

            let dirs = dirs.lines().collect();

            stor_age::run(dirs, &config);
        }
    }
}
