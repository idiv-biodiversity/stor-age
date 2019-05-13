use std::fs::{self, read_link};
use std::io;
use std::path::Path;
use std::time::{Duration, SystemTime};

use crate::acc::Acc;
use crate::config::Config;

pub fn run(dir: &str, config: Config) -> io::Result<Acc> {
    let sys_time = SystemTime::now();
    let age = Duration::from_secs(config.age_days * 3600 * 24);
    let threshold = sys_time - age;

    visit_dirs(Path::new(dir), threshold, config)
}

fn visit_dirs(
    dir: &Path,
    threshold: SystemTime,
    config: Config,
) -> io::Result<Acc> {
    let mut sum = Acc::empty();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let link = read_link(path.clone());

        if link.is_ok() {
            if config.debug {
                println!("skipping link: {:?}", path);
            }

            continue;
        } else if path.is_file() {
            if config.debug {
                println!("visiting entry: {:?}", entry);
            }

            let meta = entry.metadata()?;

            let len = meta.len();

            let access = if meta.accessed()? < threshold { len } else { 0 };
            let modify = if meta.modified()? < threshold { len } else { 0 };

            sum += Acc::new(len, access, modify);
        } else if path.is_dir() {
            if config.verbose {
                eprintln!("descending into: {:?}", path);
            }

            sum += visit_dirs(&path, threshold, config)?;
        } else if config.debug {
            eprintln!(
                "neither directory nor regular file, skipping: {:?}",
                path,
            );
        }
    }

    Ok(sum)
}
