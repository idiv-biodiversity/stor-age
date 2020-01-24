use std::collections::HashMap;
use std::fs::{self, ReadDir};
use std::io::ErrorKind;
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::time::{Duration, SystemTime};

use crate::log;
use crate::Acc;
use crate::Config;
use crate::Result;

pub fn run(dir: &str, config: &Config) -> Result {
    let sys_time = SystemTime::now();

    let mut thresholds = HashMap::with_capacity(config.ages_in_days.len());

    for age in &config.ages_in_days {
        let duration = Duration::from_secs(60 * 60 * 24 * age);
        let threshold = sys_time - duration;

        thresholds.insert(*age, threshold);
    }

    let dev = if config.one_file_system {
        Some(fs::metadata(dir)?.dev())
    } else {
        None
    };

    walk(Path::new(dir), &thresholds, dev, config)
}

fn walk(
    dir: &Path,
    thresholds: &HashMap<u64, SystemTime>,
    dev: Option<u64>,
    config: &Config,
) -> Result {
    let acc = Acc::new().with_ages(&config.ages_in_days);

    match fs::read_dir(dir) {
        Ok(entries) => iterate(entries, acc, thresholds, dev, config),

        Err(error) if error.kind() == ErrorKind::PermissionDenied => {
            log::info(format!("skipping permission denied: {:?}", dir));
            Ok(acc)
        }

        Err(error) => Err(error.into()),
    }
}

fn iterate(
    entries: ReadDir,
    mut acc: Acc,
    thresholds: &HashMap<u64, SystemTime>,
    dev: Option<u64>,
    config: &Config,
) -> Result {
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let meta = entry.metadata()?;
        let file_type = meta.file_type();

        if config.one_file_system && dev_check(dev, &meta) {
            log::debug(
                format!("skipping different file system: {:?}", path),
                config,
            );
        } else if file_type.is_file() {
            log::debug(format!("visiting: {:?}", path), config);

            let bytes = meta.len();

            let mut current = Acc::new().with_total(bytes);

            for (age, threshold) in thresholds {
                let accessed = if meta.accessed()? > *threshold {
                    bytes
                } else {
                    0
                };

                let modified = if meta.modified()? > *threshold {
                    bytes
                } else {
                    0
                };

                current.insert(*age, accessed, modified);
            }

            acc += current;
        } else if file_type.is_dir() {
            log::debug(format!("descending: {:?}", path), config);

            acc += walk(&path, thresholds, dev, config)?;
        } else {
            log::debug(
                format!(
                    "skipping neither regular file nor directory: {:?}",
                    path
                ),
                config,
            );
        }
    }

    Ok(acc)
}

fn dev_check(dev: Option<u64>, meta: &fs::Metadata) -> bool {
    dev.map_or(false, |dev| dev != meta.dev())
}
