use assert_cmd::crate_name;
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::os;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn skip_link_dir() -> Result<(), Box<dyn Error>> {
    let dir = tempdir()?;

    let subdir = dir.path().join("subdir");
    fs::create_dir(&subdir)?;

    let file = subdir.join("file");
    let mut file = File::create(file)?;
    writeln!(file, "text")?;

    let link = dir.path().join("link");

    #[cfg(target_family = "unix")]
    os::unix::fs::symlink(subdir, &link)?;

    #[cfg(target_family = "windows")]
    os::windows::fs::symlink_dir(subdir, &link)?;

    let mut cmd = Command::cargo_bin(crate_name!()).unwrap();
    cmd.arg("--debug").arg("1").arg("--").arg(dir.path());

    let skip_msg = format!("skipping: {:?}", link);

    cmd.assert()
        .success()
        .stderr(predicate::str::contains(skip_msg));

    drop(file);
    dir.close()?;

    Ok(())
}

#[test]
fn skip_link_file() -> Result<(), Box<dyn Error>> {
    let dir = tempdir()?;

    let path = dir.path().join("foo");
    let mut file = File::create(&path)?;
    writeln!(file, "foo")?;

    let link = dir.path().join("bar");

    #[cfg(target_family = "unix")]
    os::unix::fs::symlink(path, &link)?;

    #[cfg(target_family = "windows")]
    os::windows::fs::symlink_file(path, &link)?;

    let mut cmd = Command::cargo_bin(crate_name!()).unwrap();
    cmd.arg("--debug").arg("1").arg("--").arg(dir.path());

    let skip_msg = format!("skipping: {:?}", link);

    cmd.assert()
        .success()
        .stderr(predicate::str::contains(skip_msg));

    drop(file);
    dir.close()?;

    Ok(())
}
