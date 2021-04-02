use std::collections::HashMap;
use std::fs::{self, ReadDir};
use std::io::ErrorKind;
use std::path::Path;
use std::time::{Duration, SystemTime};

#[cfg(target_family = "unix")]
use std::os::unix::fs::MetadataExt;

use anyhow::Result;

use crate::log;
use crate::Config;
use crate::Data;

pub fn run(dir: &str, config: &Config) -> Result<Data> {
    let thresholds = thresholds(&config.ages_in_days);

    #[cfg(target_family = "unix")]
    let dev = if config.one_file_system {
        Some(fs::metadata(dir)?.dev())
    } else {
        None
    };

    #[cfg(not(target_family = "unix"))]
    let dev = None;

    walk(Path::new(dir), &thresholds, dev, config)
}

fn thresholds(ages_in_days: &[u64]) -> HashMap<u64, SystemTime> {
    let now = SystemTime::now();

    let mut thresholds = HashMap::with_capacity(ages_in_days.len());

    for age in ages_in_days {
        let duration = Duration::from_secs(60 * 60 * 24 * age);
        let threshold = now - duration;

        thresholds.insert(*age, threshold);
    }

    thresholds
}

fn walk(
    dir: &Path,
    thresholds: &HashMap<u64, SystemTime>,
    dev: Option<u64>,
    config: &Config,
) -> Result<Data> {
    let data = Data::default().with_ages(&config.ages_in_days);

    match fs::read_dir(dir) {
        Ok(entries) => iterate(entries, data, thresholds, dev, config),

        Err(error) if error.kind() == ErrorKind::PermissionDenied => {
            log::info(format!("skipping permission denied: {:?}", dir));
            Ok(data)
        }

        Err(error) => Err(error.into()),
    }
}

fn iterate(
    entries: ReadDir,
    mut data: Data,
    thresholds: &HashMap<u64, SystemTime>,
    dev: Option<u64>,
    config: &Config,
) -> Result<Data> {
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let meta = entry.metadata()?;
        let file_type = meta.file_type();

        if dev_check(dev, &meta) {
            log::debug(
                format!("skipping different file system: {:?}", path),
                config,
            );
        } else if file_type.is_file() {
            log::debug(format!("visiting: {:?}", path), config);

            let bytes = meta.len();

            let mut current =
                Data::default().with_total_bytes(bytes).with_total_files(1);

            for (age, threshold) in thresholds {
                let (a_b, a_f) = if meta.accessed()? > *threshold {
                    (bytes, 1)
                } else {
                    (0, 0)
                };

                let (m_b, m_f) = if meta.modified()? > *threshold {
                    (bytes, 1)
                } else {
                    (0, 0)
                };

                current.insert(*age, a_b, m_b, a_f, m_f);
            }

            data += current;
        } else if file_type.is_dir() {
            log::debug(format!("descending: {:?}", path), config);

            data += walk(&path, thresholds, dev, config)?;
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

    Ok(data)
}

#[cfg(target_family = "unix")]
fn dev_check(dev: Option<u64>, meta: &fs::Metadata) -> bool {
    dev.map_or(false, |dev| dev != meta.dev())
}

#[cfg(not(target_family = "unix"))]
fn dev_check(_dev: Option<u64>, _meta: &fs::Metadata) -> bool {
    false
}
