use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};

use anyhow::{anyhow, Result};
use bstr::io::BufReadExt;
use bstr::ByteSlice;
use tempfile::{tempdir, tempdir_in};

use crate::log;
use crate::Config;
use crate::Data;

pub fn run(dir: &str, config: &Config) -> Result<Data> {
    let tmp =
        if let Some(local_work_dir) = &config.spectrum_scale_local_work_dir {
            tempdir_in(local_work_dir)?
        } else {
            tempdir()?
        };

    let policy = tmp.path().join(".policy");
    let prefix = tmp.path().join("stor-age");

    let mut file = File::create(&policy)?;
    write_policy(&mut file, &config.ages_in_days)?;
    file.sync_all()?;

    let mut command = Command::new("mmapplypolicy");
    command
        .arg(dir)
        .args(&["-P", policy.to_str().unwrap()])
        .args(&["-f", prefix.to_str().unwrap()])
        .args(&["--choice-algorithm", "fast"])
        .args(&["-I", "defer"])
        .args(&["-L", "0"]);

    if let Some(nodes) = &config.spectrum_scale_nodes {
        command.args(&["-N", nodes]);
    };

    if let Some(local_work_dir) = &config.spectrum_scale_local_work_dir {
        command.args(&["-s", local_work_dir]);
    };

    if let Some(global_work_dir) = &config.spectrum_scale_global_work_dir {
        command.args(&["-g", global_work_dir]);
    };

    log::debug(format!("command: {:?}", command), config);

    let mut child = command
        .stdout(Stdio::null())
        .spawn()
        .expect("mmapplypolicy failed to start, make sure it's on your PATH");

    log::debug("waiting for mmapplypolicy to finish", config);

    let ecode = child.wait().expect("failed waiting on mmapplypolicy");

    if ecode.success() {
        let total_file = tmp.path().join("stor-age.list.total");
        let (tot_bytes, tot_files) = sum(&total_file)?;

        let mut data = Data::default()
            .with_ages(&config.ages_in_days)
            .with_total_bytes(tot_bytes)
            .with_total_files(tot_files);

        for age in &config.ages_in_days {
            let access_file =
                tmp.path().join(format!("stor-age.list.access_{}", age));

            let modify_file =
                tmp.path().join(format!("stor-age.list.modify_{}", age));

            let (a_b, a_f) = sum(&access_file)?;
            let (m_b, m_f) = sum(&modify_file)?;

            data.insert(*age, a_b, m_b, a_f, m_f);
        }

        Ok(data)
    } else {
        Err(anyhow!("mmapplypolicy was no success"))
    }
}

fn write_policy(mut w: impl io::Write, ages: &[u64]) -> io::Result<()> {
    write!(
        w,
        "
define(access_age, (DAYS(CURRENT_TIMESTAMP) - DAYS(ACCESS_TIME)))
define(modify_age, (DAYS(CURRENT_TIMESTAMP) - DAYS(MODIFICATION_TIME)))

RULE EXTERNAL LIST 'total' EXEC ''
",
    )?;

    for age in ages {
        write!(
            w,
            "
RULE EXTERNAL LIST 'access_{}' EXEC ''
RULE EXTERNAL LIST 'modify_{}' EXEC ''
",
            age, age
        )?;
    }

    write!(
        w,
        "
RULE
  LIST 'total'
  SHOW(VARCHAR(FILE_SIZE))
",
    )?;

    for age in ages {
        write!(
            w,
            "
RULE
  LIST 'access_{}'
    SHOW(VARCHAR(FILE_SIZE))
    WHERE (access_age < {})

RULE
  LIST 'modify_{}'
    SHOW(VARCHAR(FILE_SIZE))
    WHERE (modify_age < {})
",
            age, age, age, age
        )?;
    }

    Ok(())
}

fn sum(file: &Path) -> Result<(u64, u64)> {
    let mut sum_bytes = 0;
    let mut sum_files = 0;

    if file.exists() {
        let file = File::open(file)?;
        let file = BufReader::new(file);

        for line in file.byte_lines() {
            let line = line?;

            let size = line.splitn_str(6, " ").nth(4).unwrap();
            let size = size.to_str().unwrap();
            let size: u64 = size.parse().unwrap();

            sum_bytes += size;
            sum_files += 1;
        }
    }

    Ok((sum_bytes, sum_files))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn policy() {
        let ages = vec![90, 365];

        let mut result = vec![];
        write_policy(&mut result, &ages).unwrap();

        let result = std::str::from_utf8(&result).unwrap();

        let expected = "
define(access_age, (DAYS(CURRENT_TIMESTAMP) - DAYS(ACCESS_TIME)))
define(modify_age, (DAYS(CURRENT_TIMESTAMP) - DAYS(MODIFICATION_TIME)))

RULE EXTERNAL LIST 'total' EXEC ''

RULE EXTERNAL LIST 'access_90' EXEC ''
RULE EXTERNAL LIST 'modify_90' EXEC ''

RULE EXTERNAL LIST 'access_365' EXEC ''
RULE EXTERNAL LIST 'modify_365' EXEC ''

RULE
  LIST 'total'
  SHOW(VARCHAR(FILE_SIZE))

RULE
  LIST 'access_90'
    SHOW(VARCHAR(FILE_SIZE))
    WHERE (access_age < 90)

RULE
  LIST 'modify_90'
    SHOW(VARCHAR(FILE_SIZE))
    WHERE (modify_age < 90)

RULE
  LIST 'access_365'
    SHOW(VARCHAR(FILE_SIZE))
    WHERE (access_age < 365)

RULE
  LIST 'modify_365'
    SHOW(VARCHAR(FILE_SIZE))
    WHERE (modify_age < 365)
";

        assert_eq!(result, expected);
    }
}
