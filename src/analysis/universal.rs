use std::collections::HashMap;
use std::fs::{self, ReadDir};
use std::io::ErrorKind;
use std::path::Path;
use std::time::{Duration, SystemTime};

#[cfg(target_family = "unix")]
use std::os::unix::fs::MetadataExt;

use anyhow::Result;

use crate::Data;

pub fn run(
    dir: &str,
    ages_in_days: &[u64],
    one_file_system: bool,
) -> Result<Data> {
    let thresholds = thresholds(ages_in_days);

    #[cfg(target_family = "unix")]
    let dev = if one_file_system {
        Some(fs::metadata(dir)?.dev())
    } else {
        None
    };

    #[cfg(not(target_family = "unix"))]
    let dev = None;

    walk(Path::new(dir), &thresholds, ages_in_days, dev)
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
    ages_in_days: &[u64],
    dev: Option<u64>,
) -> Result<Data> {
    let data = Data::default().with_ages(ages_in_days);

    match fs::read_dir(dir) {
        Ok(entries) => iterate(entries, data, thresholds, ages_in_days, dev),

        Err(error) if error.kind() == ErrorKind::PermissionDenied => {
            log::info!("skipping permission denied: {dir:?}");
            Ok(data)
        }

        Err(error) => Err(error.into()),
    }
}

fn iterate(
    entries: ReadDir,
    mut data: Data,
    thresholds: &HashMap<u64, SystemTime>,
    ages_in_days: &[u64],
    dev: Option<u64>,
) -> Result<Data> {
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let meta = entry.metadata()?;
        let file_type = meta.file_type();

        if dev_check(dev, &meta) {
            log::debug!("skipping different file system: {path:?}");
        } else if file_type.is_file() {
            log::debug!("visiting: {path:?}");

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
            log::debug!("descending: {path:?}");

            data += walk(&path, thresholds, ages_in_days, dev)?;
        } else {
            log::debug!(
                "skipping neither regular file nor directory: {path:?}"
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
const fn dev_check(_dev: Option<u64>, _meta: &fs::Metadata) -> bool {
    false
}
