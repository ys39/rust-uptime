use anyhow::Result;
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
                            // use std::env;
use regex::Regex;
use std::process::Command; // Run programs
use std::str;

#[test]
fn invalid_argument() -> Result<()> {
    let mut cmd = Command::cargo_bin("rust-uptime")?;

    cmd.arg("-a");
    cmd.assert().failure().stderr(predicate::str::contains(
        "error: unexpected argument '-a' found",
    ));
    Ok(())
}

#[test]
fn long_help_argument() -> Result<()> {
    let mut cmd = Command::cargo_bin("rust-uptime")?;

    cmd.arg("--help");
    let contains_predicate =
        (predicate::str::contains("  -p, --pretty   show uptime in pretty format"))
            .and(predicate::str::contains("  -s, --since    system up since"))
            .and(predicate::str::contains("  -h, --help     Print help"))
            .and(predicate::str::contains("  -V, --version  Print version"));
    cmd.assert().success().stdout(contains_predicate);
    Ok(())
}

#[test]
fn short_help_argument() -> Result<()> {
    let mut cmd = Command::cargo_bin("rust-uptime")?;

    cmd.arg("-h");
    let contains_predicate =
        (predicate::str::contains("  -p, --pretty   show uptime in pretty format"))
            .and(predicate::str::contains("  -s, --since    system up since"))
            .and(predicate::str::contains("  -h, --help     Print help"))
            .and(predicate::str::contains("  -V, --version  Print version"));
    cmd.assert().success().stdout(contains_predicate);
    Ok(())
}

#[test]
fn long_version_argument() -> Result<()> {
    let mut cmd = Command::cargo_bin("rust-uptime")?;

    cmd.arg("--version");
    let contains_predicate = predicate::str::contains("rust-uptime 0.1.0");
    cmd.assert().success().stdout(contains_predicate);
    Ok(())
}

#[test]
fn short_version_argument() -> Result<()> {
    let mut cmd = Command::cargo_bin("rust-uptime")?;

    cmd.arg("-V");
    let contains_predicate = predicate::str::contains("rust-uptime 0.1.0");
    cmd.assert().success().stdout(contains_predicate);
    Ok(())
}

#[test]
fn long_pretty_argument() -> Result<()> {
    let mut cmd = Command::cargo_bin("rust-uptime")?;

    cmd.arg("--pretty");
    let re = Regex::new(r"up \d+ hours?, \d+ minutes")?;
    let output = cmd.output().unwrap();
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(re.is_match(stdout));
    Ok(())
}

#[test]
fn long_since_argument() -> Result<()> {
    let mut cmd = Command::cargo_bin("rust-uptime")?;

    cmd.arg("--since");
    let re = Regex::new(r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}")?;
    let output = cmd.output().unwrap();
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(re.is_match(stdout));
    Ok(())
}
