#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]

mod cli;

use std::io::{self, Read};

use anyhow::{Context, Result};

use stor_age::Config;

fn main() -> Result<()> {
    let args = cli::build().get_matches();
    let config = Config::from_args(&args);

    stor_age::log::debug(format!("{config:#?}"), &config);

    if let Some(dirs) = args.get_many::<String>("dir") {
        let dirs: Vec<&str> = dirs.map(String::as_str).collect();
        stor_age::run(&dirs, &config);
    } else {
        let mut dirs = String::new();

        io::stdin()
            .read_to_string(&mut dirs)
            .with_context(|| "error reading from stdin")?;

        let dirs: Vec<&str> = dirs.lines().collect();

        stor_age::run(&dirs, &config);
    }

    Ok(())
}
