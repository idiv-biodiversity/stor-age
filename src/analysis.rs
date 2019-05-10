use acc::Acc;
use config::Config;
use log;
use mktemp::Temp;
use output::{self, Output};
use regex::Regex;
use std::fs::{self, read_link, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, SystemTime};

pub fn analyze(dir: &str, config: &Config) {
    let result = if config.spectrum_scale {
        analyze_spectrum_scale(dir, config)
    } else {
        analyze_universal(dir, config)
    };

    match result {
        Ok(acc) => {
            match config.output {
                Output::Pretty =>
                    output::pretty(dir, acc, config.age_days),

                Output::Oneline =>
                    output::oneline(dir, acc),
            }
        },

        Err(error) => {
            log::error(
                &format!("skipping directory {}: {}", dir, error)
            );
        }
    }
}

// ----------------------------------------------------------------------------
// normal directory traversal
// ----------------------------------------------------------------------------

fn analyze_universal(dir: &str, config: &Config) -> io::Result<Acc> {
    let sys_time = SystemTime::now();
    let age = Duration::from_secs(config.age_days * 3600 * 24);
    let threshold = sys_time - age;

    visit_dirs(Path::new(dir), threshold, config)
}

fn visit_dirs(
    dir: &Path,
    threshold: SystemTime,
    config: &Config,
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

            let access = if meta.accessed()? < threshold {
                len
            } else {
                0
            };

            let modify = if meta.modified()? < threshold {
                len
            } else {
                0
            };

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

// ----------------------------------------------------------------------------
// with spectrum scale we can use mmapplypolicy for faster execution
// ----------------------------------------------------------------------------

fn analyze_spectrum_scale(dir: &str, config: &Config) -> io::Result<Acc> {
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
            "mmapplypolicy was no success"
        ))
    }
}

fn write_policy_file(file: &PathBuf, config: &Config) -> io::Result<()> {
    let mut file = File::create(file)?;

    let content = format!("
define(access_age, (DAYS(CURRENT_TIMESTAMP) - DAYS(ACCESS_TIME)))
define(modify_age, (DAYS(CURRENT_TIMESTAMP) - DAYS(MODIFICATION_TIME)))

RULE EXTERNAL LIST 'total' EXEC ''
RULE EXTERNAL LIST 'access' EXEC ''
RULE EXTERNAL LIST 'modify' EXEC ''

RULE 'TOTAL' LIST 'total' SHOW(VARCHAR(FILE_SIZE))
RULE 'ACCESS' LIST 'access' SHOW(VARCHAR(FILE_SIZE)) WHERE (access_age > {})
RULE 'MODIFY' LIST 'modify' SHOW(VARCHAR(FILE_SIZE)) WHERE (modify_age > {})
", config.age_days, config.age_days);

    file.write_all(content.as_bytes())?;

    Ok(())
}

fn sum_bytes(file: &Path) -> io::Result<u64> {
    let mut sum = 0;

    if file.exists() {
        lazy_static! {
            static ref RE_LIST: Regex = Regex::new(
                r#"^\d+\s+\d+\s+\d+\s+(\d+)\s+--"#
            ).unwrap();
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
