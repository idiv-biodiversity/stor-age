use std::fs::{self, read_link};
use std::io;
use std::path::Path;
use std::time::{Duration, SystemTime};

use crate::acc::Acc;
use crate::config::Config;
use crate::log;

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
            log::debug(format!("skipping link: {:?}", path), config);

            continue;
        } else if path.is_file() {
            log::debug(format!("visiting entry: {:?}", entry), config);

            let meta = entry.metadata()?;

            let len = meta.len();

            let access = if meta.accessed()? < threshold { len } else { 0 };
            let modify = if meta.modified()? < threshold { len } else { 0 };

            sum += Acc::new(len, access, modify);
        } else if path.is_dir() {
            log::debug(format!("descending into: {:?}", path), config);

            sum += visit_dirs(&path, threshold, config)?;
        } else {
            let message = format!(
                "neither directory nor regular file, skipping: {:?}",
                path
            );

            log::debug(message, config);
        }
    }

    Ok(sum)
}
