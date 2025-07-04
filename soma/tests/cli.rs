use assert_cmd::Command;
use predicates::prelude::*;

/// Test that the soma binary can be executed without arguments
#[test]
fn test_soma_runs_without_args() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Cli {"))
        .stdout(predicate::str::contains("noaction: false"))
        .stdout(predicate::str::contains("verbose: false"));
}

/// Test the --help flag
#[test]
fn test_soma_help_flag() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("USAGE:"))
        .stdout(predicate::str::contains("FLAGS:"))
        .stdout(predicate::str::contains("--noaction"))
        .stdout(predicate::str::contains("--verbose"))
        .stdout(predicate::str::contains(
            "Describe what would be done without doing it",
        ))
        .stdout(predicate::str::contains(
            "Produce verbose output as the process runs",
        ));
}

/// Test the -h flag (short help)
#[test]
fn test_soma_help_short_flag() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.arg("-h")
        .assert()
        .success()
        .stdout(predicate::str::contains("USAGE:"))
        .stdout(predicate::str::contains("FLAGS:"));
}

/// Test the --version flag
#[test]
fn test_soma_version_flag() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("soma"))
        .stdout(predicate::str::contains("0.1.0"));
}

/// Test the -V flag (short version)
#[test]
fn test_soma_version_short_flag() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.arg("-V")
        .assert()
        .success()
        .stdout(predicate::str::contains("soma"))
        .stdout(predicate::str::contains("0.1.0"));
}

/// Test the --noaction flag
#[test]
fn test_soma_noaction_flag() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.arg("--noaction")
        .assert()
        .success()
        .stdout(predicate::str::contains("noaction: true"))
        .stdout(predicate::str::contains("verbose: false"));
}

/// Test the -n flag (short noaction)
#[test]
fn test_soma_noaction_short_flag() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.arg("-n")
        .assert()
        .success()
        .stdout(predicate::str::contains("noaction: true"))
        .stdout(predicate::str::contains("verbose: false"));
}

/// Test the --verbose flag
#[test]
fn test_soma_verbose_flag() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.arg("--verbose")
        .assert()
        .success()
        .stdout(predicate::str::contains("verbose: true"))
        .stdout(predicate::str::contains("noaction: false"));
}

/// Test the -v flag (short verbose)
#[test]
fn test_soma_verbose_short_flag() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.arg("-v")
        .assert()
        .success()
        .stdout(predicate::str::contains("verbose: true"))
        .stdout(predicate::str::contains("noaction: false"));
}

/// Test combining --noaction and --verbose flags
#[test]
fn test_soma_combined_flags() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.args(&["--noaction", "--verbose"])
        .assert()
        .success()
        .stdout(predicate::str::contains("noaction: true"))
        .stdout(predicate::str::contains("verbose: true"));
}

/// Test combining short flags
#[test]
fn test_soma_combined_short_flags() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.args(&["-n", "-v"])
        .assert()
        .success()
        .stdout(predicate::str::contains("noaction: true"))
        .stdout(predicate::str::contains("verbose: true"));
}

/// Test combining flags in different order
#[test]
fn test_soma_flags_different_order() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.args(&["--verbose", "--noaction"])
        .assert()
        .success()
        .stdout(predicate::str::contains("noaction: true"))
        .stdout(predicate::str::contains("verbose: true"));
}

/// Test invalid flag handling
#[test]
fn test_soma_invalid_flag() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.arg("--invalid-flag")
        .assert()
        .failure()
        .stderr(predicate::str::contains("error:"))
        .stderr(predicate::str::contains("Found argument '--invalid-flag'"));
}

/// Test invalid short flag handling
#[test]
fn test_soma_invalid_short_flag() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.arg("-x")
        .assert()
        .failure()
        .stderr(predicate::str::contains("error:"))
        .stderr(predicate::str::contains("Found argument '-x'"));
}

/// Test that unknown positional arguments are rejected
#[test]
fn test_soma_unknown_positional_args() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.arg("unexpected-arg")
        .assert()
        .failure()
        .stderr(predicate::str::contains("error:"))
        .stderr(predicate::str::contains("Found argument 'unexpected-arg'"));
}

/// Test that the binary name appears in help output
#[test]
fn test_soma_binary_name_in_help() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("soma"));
}

/// Test that debug output format is consistent
#[test]
fn test_soma_debug_output_format() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::starts_with("Cli {"))
        .stdout(predicate::str::ends_with("}\n"));
}
