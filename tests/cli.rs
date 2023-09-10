use anyhow::Result;
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
                            // use std::env;
use std::process::Command; // Run programs

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
