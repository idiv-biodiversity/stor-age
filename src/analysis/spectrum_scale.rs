use bstr::io::BufReadExt;
use bstr::ByteSlice;
use std::fs::File;
use std::io::{self, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use tempfile::{tempdir, tempdir_in};

use crate::acc::Acc;
use crate::config::Config;
use crate::log;

pub fn run(dir: &str, config: &Config) -> io::Result<Acc> {
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
        let access_f = tmp.path().join("stor-age.list.access");
        let modify_f = tmp.path().join("stor-age.list.modify");

        let tot_size = sum_bytes(&total_f)?;
        let acc_size = sum_bytes(&access_f)?;
        let mod_size = sum_bytes(&modify_f)?;

        Ok(Acc::new(tot_size, acc_size, mod_size))
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "mmapplypolicy was no success",
        ))
    }
}

fn write_policy_file(file: &PathBuf, config: &Config) -> io::Result<()> {
    let mut file = File::create(file)?;

    let content = format!(
        "
define(access_age, (DAYS(CURRENT_TIMESTAMP) - DAYS(ACCESS_TIME)))
define(modify_age, (DAYS(CURRENT_TIMESTAMP) - DAYS(MODIFICATION_TIME)))

RULE EXTERNAL LIST 'total' EXEC ''
RULE EXTERNAL LIST 'access' EXEC ''
RULE EXTERNAL LIST 'modify' EXEC ''

RULE 'TOTAL' LIST 'total' SHOW(VARCHAR(FILE_SIZE))
RULE 'ACCESS' LIST 'access' SHOW(VARCHAR(FILE_SIZE)) WHERE (access_age > {})
RULE 'MODIFY' LIST 'modify' SHOW(VARCHAR(FILE_SIZE)) WHERE (modify_age > {})
",
        config.age_days, config.age_days
    );

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
