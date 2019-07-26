use assert_cmd::crate_name;
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::error::Error;
use std::process::Command;

#[test]
fn arg_age_invalid() -> Result<(), Box<Error>> {
    let mut cmd = Command::cargo_bin(crate_name!()).unwrap();
    cmd.arg("not-an-age");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("not a positive number"));

    Ok(())
}

#[test]
fn arg_dir_does_not_exist() -> Result<(), Box<Error>> {
    let mut cmd = Command::cargo_bin(crate_name!()).unwrap();
    cmd.arg("90").arg("test/file/doesnt/exist");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("does not exist"));

    Ok(())
}

#[test]
fn arg_dir_not_a_dir() -> Result<(), Box<Error>> {
    let mut cmd = Command::cargo_bin(crate_name!()).unwrap();
    cmd.arg("90").arg("Cargo.toml");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("is not a directory"));

    Ok(())
}

#[cfg(target_family = "unix")]
#[test]
fn arg_dir_permission_denied() -> Result<(), Box<Error>> {
    let mut cmd = Command::cargo_bin(crate_name!()).unwrap();
    cmd.arg("90").arg("/root");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("permission denied"));

    Ok(())
}
