use std::error::Error;
use std::process::Command;

use assert_cmd::crate_name;
use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn arg_age_invalid() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin(crate_name!())?;
    cmd.arg("not-an-age");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("invalid digit found"));

    Ok(())
}

#[test]
fn arg_dir_does_not_exist() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin(crate_name!())?;
    cmd.arg("90").arg("--").arg("test/file/doesnt/exist");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("does not exist"));

    Ok(())
}

#[test]
fn arg_dir_not_a_dir() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin(crate_name!())?;
    cmd.arg("90").arg("--").arg("Cargo.toml");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("is not a directory"));

    Ok(())
}

#[cfg(target_os = "linux")]
#[test]
fn arg_dir_permission_denied() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin(crate_name!())?;
    cmd.arg("90").arg("--").arg("/root");

    cmd.assert()
        .failure()
        .stderr(predicate::str::is_match("[pP]ermission denied")?);

    Ok(())
}
