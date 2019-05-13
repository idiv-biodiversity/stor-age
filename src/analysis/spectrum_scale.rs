use lazy_static::lazy_static;
use mktemp::Temp;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::acc::Acc;
use crate::config::Config;

pub fn run(dir: &str, config: Config) -> io::Result<Acc> {
    let tmp = Temp::new_dir()?;
    let mut policy = tmp.to_path_buf();
    policy.push(".policy");
    let mut prefix = tmp.to_path_buf();
    prefix.push("stor-age");

    write_policy_file(&policy, config)?;

    let mut child = Command::new("mmapplypolicy")
        .arg(dir)
        .args(&["-P", policy.to_str().unwrap()])
        .args(&["-f", prefix.to_str().unwrap()])
        .args(&["-I", "defer"])
        .args(&["-L", "0"])
        .stdout(Stdio::null())
        .spawn()
        .expect("failed to execute child");

    let ecode = child.wait().expect("failed to wait on child");

    if ecode.success() {
        let mut total_f = tmp.to_path_buf();
        total_f.push("stor-age.list.total");
        let mut access_f = tmp.to_path_buf();
        access_f.push("stor-age.list.access");
        let mut modify_f = tmp.to_path_buf();
        modify_f.push("stor-age.list.modify");

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

fn write_policy_file(file: &PathBuf, config: Config) -> io::Result<()> {
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
        lazy_static! {
            static ref RE_LIST: Regex =
                Regex::new(r#"^\d+\s+\d+\s+\d+\s+(\d+)\s+--"#).unwrap();
        }

        let file = File::open(file)?;

        for line in BufReader::new(file).lines() {
            let line = line?;

            for cap in RE_LIST.captures_iter(&line) {
                let size: u64 = cap[1].parse().unwrap();
                sum += size;
            }
        }
    }

    Ok(sum)
}
