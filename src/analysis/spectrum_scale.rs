use bstr::io::BufReadExt;
use bstr::ByteSlice;
use std::fs::File;
use std::io::{self, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use tempfile::{tempdir, tempdir_in};

use crate::log;
use crate::Acc;
use crate::Config;
use crate::{Error, ErrorKind, Result};

pub fn run(dir: &str, config: &Config) -> Result {
    let tmp = if let Some(ref local_work_dir) =
        config.spectrum_scale_local_work_dir
    {
        tempdir_in(local_work_dir)?
    } else {
        tempdir()?
    };

    let policy = tmp.path().join(".policy");
    let prefix = tmp.path().join("stor-age");

    write_policy_file(&policy, config)?;

    let mut command = Command::new("mmapplypolicy");
    command
        .arg(dir)
        .args(&["-P", policy.to_str().unwrap()])
        .args(&["-f", prefix.to_str().unwrap()])
        .args(&["--choice-algorithm", "fast"])
        .args(&["-I", "defer"])
        .args(&["-L", "0"]);

    if let Some(ref nodes) = config.spectrum_scale_nodes {
        command.args(&["-N", nodes]);
    };

    if let Some(ref local_work_dir) = config.spectrum_scale_local_work_dir {
        command.args(&["-s", local_work_dir]);
    };

    if let Some(ref global_work_dir) = config.spectrum_scale_global_work_dir {
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
        let total_f = tmp.path().join("stor-age.list.total");
        let tot_size = sum_bytes(&total_f)?;

        let mut acc = Acc::new().with_total(tot_size);

        for age in &config.ages_in_days {
            let access_file =
                tmp.path().join(format!("stor-age.list.access_{}", age));

            let modify_file =
                tmp.path().join(format!("stor-age.list.modify_{}", age));

            let acc_size = sum_bytes(&access_file)?;
            let mod_size = sum_bytes(&modify_file)?;

            acc.insert(*age, acc_size, mod_size);
        }

        Ok(acc)
    } else {
        Err(Error::new("mmapplypolicy was no success", ErrorKind::Io))
    }
}

fn write_policy_file(file: &PathBuf, config: &Config) -> io::Result<()> {
    let mut file = File::create(file)?;

    let mut content = String::from(
        "
define(access_age, (DAYS(CURRENT_TIMESTAMP) - DAYS(ACCESS_TIME)))
define(modify_age, (DAYS(CURRENT_TIMESTAMP) - DAYS(MODIFICATION_TIME)))

RULE EXTERNAL LIST 'total' EXEC ''
",
    );

    for age in &config.ages_in_days {
        content.push_str(&format!(
            "
RULE EXTERNAL LIST 'access_{}' EXEC ''
RULE EXTERNAL LIST 'modify_{}' EXEC ''
",
            age, age
        ));
    }

    content.push_str(
        "
RULE
  LIST 'total'
  SHOW(VARCHAR(FILE_SIZE))
",
    );

    for age in &config.ages_in_days {
        content.push_str(&format!(
            "
RULE
  LIST 'access_{}'
    SHOW(VARCHAR(FILE_SIZE))
    WHERE (access_age > {})

RULE
  LIST 'modify_{}'
    SHOW(VARCHAR(FILE_SIZE))
    WHERE (modify_age > {})
",
            age, age, age, age
        ));
    }

    file.write_all(content.as_bytes())?;

    Ok(())
}

fn sum_bytes(file: &Path) -> io::Result<u64> {
    let mut sum = 0;

    if file.exists() {
        let file = File::open(file)?;
        let file = BufReader::new(file);

        for line in file.byte_lines() {
            let line = line?;

            let size = line.splitn_str(6, " ").nth(4).unwrap();
            let size = size.to_str().unwrap();
            let size: u64 = size.parse().unwrap();

            sum += size;
        }
    }

    Ok(sum)
}
