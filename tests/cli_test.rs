use assert_cmd::Command;
use predicates::prelude::*;
use std::error::Error;

#[test]
fn test_union_command() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("zetci")?;
    cmd.arg("union")
        .arg("--files")
        .arg("./testdata/fee.csv")
        .arg("./testdata/foo.csv");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Union result"));

    Ok(())
}

#[test]
fn test_intersect_command() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("zetci")?;
    cmd.arg("intersect")
        .arg("--files")
        .arg("./testdata/fee.csv")
        .arg("./testdata/foo.csv");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Intersection result"));

    Ok(())
}

#[test]
fn test_xor_command_with_strategy() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("zetci")?;
    cmd.arg("xor")
        .arg("--value-strategy")
        .arg("first")
        .arg("--files")
        .arg("./testdata/fee.csv")
        .arg("./testdata/foo.csv");

    cmd.assert().success();

    Ok(())
}

#[test]
fn test_about_command() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("zetci")?;
    cmd.arg("about");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("zetci")); // Assuming your logo contains "zetci"

    Ok(())
}

#[test]
fn test_invalid_file() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("zetci")?;
    cmd.arg("union")
        .arg("--files")
        .arg("./testdata/nonexistent.csv")
        .arg("./testdata/foo.csv");

    cmd.assert()
        .failure();

    Ok(())
}

#[test]
fn test_no_command() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("zetci")?;

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No valid subcommand was used"));

    Ok(())
}