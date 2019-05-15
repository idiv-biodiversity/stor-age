use std::error::Error;
use std::fs::{self, ReadDir};
use std::io::{self, ErrorKind};
use std::path::Path;
use std::time::{Duration, SystemTime};

use crate::acc::Acc;
use crate::config::Config;
use crate::log;

pub fn run(dir: &str, config: Config) -> io::Result<Acc> {
    let sys_time = SystemTime::now();
    let age = Duration::from_secs(config.age_days * 3600 * 24);
    let threshold = sys_time - age;

    walk(Path::new(dir), threshold, config)
}

fn walk(dir: &Path, threshold: SystemTime, config: Config) -> io::Result<Acc> {
    let sum = Acc::empty();

    match fs::read_dir(dir) {
        Ok(entries) => iterate(entries, sum, threshold, config),

        Err(ref error) if error.kind() == ErrorKind::PermissionDenied => {
            log::info(format!("skipping: {:?}: {}", dir, error.description()));
            Ok(sum)
        }

        Err(error) => Err(error),
    }
}

fn iterate(
    entries: ReadDir,
    mut sum: Acc,
    threshold: SystemTime,
    config: Config,
) -> io::Result<Acc> {
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let meta = entry.metadata()?;
        let file_type = meta.file_type();

        if file_type.is_file() {
            log::debug(format!("visiting: {:?}", path), config);

            let len = meta.len();

            let access = if meta.accessed()? < threshold { len } else { 0 };
            let modify = if meta.modified()? < threshold { len } else { 0 };

            sum += Acc::new(len, access, modify);
        } else if file_type.is_dir() {
            log::debug(format!("descending: {:?}", path), config);

            sum += walk(&path, threshold, config)?;
        } else {
            log::debug(
                format!(
                    "skipping: {:?}: neither regular file nor directory",
                    path
                ),
                config,
            );
        }
    }

    Ok(sum)
}
