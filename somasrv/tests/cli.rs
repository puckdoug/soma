use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;

/// Test that the somasrv binary can be executed without arguments
#[test]
fn test_somasrv_runs_without_args() {
    let mut cmd = Command::cargo_bin("somasrv").unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Cli {"))
        .stdout(predicate::str::contains("config: None"))
        .stdout(predicate::str::contains("validate: false"))
        .stdout(predicate::str::contains("loglevel: \"info\""));
}

/// Test the --help flag
#[test]
fn test_somasrv_help_flag() {
    let mut cmd = Command::cargo_bin("somasrv").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("USAGE:"))
        .stdout(predicate::str::contains("FLAGS:"))
        .stdout(predicate::str::contains("OPTIONS:"))
        .stdout(predicate::str::contains("--config"))
        .stdout(predicate::str::contains("--validate"))
        .stdout(predicate::str::contains("--loglevel"))
        .stdout(predicate::str::contains("Specify a configuration file"))
        .stdout(predicate::str::contains(
            "Describe what would be done without doing it",
        ))
        .stdout(predicate::str::contains("Set the log level"));
}

/// Test the -h flag (short help)
#[test]
fn test_somasrv_help_short_flag() {
    let mut cmd = Command::cargo_bin("somasrv").unwrap();
    cmd.arg("-h")
        .assert()
        .success()
        .stdout(predicate::str::contains("USAGE:"))
        .stdout(predicate::str::contains("FLAGS:"))
        .stdout(predicate::str::contains("OPTIONS:"));
}

/// Test the --version flag
#[test]
fn test_somasrv_version_flag() {
    let mut cmd = Command::cargo_bin("somasrv").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("somasrv"))
        .stdout(predicate::str::contains("0.1.0"));
}

/// Test the -V flag (short version)
#[test]
fn test_somasrv_version_short_flag() {
    let mut cmd = Command::cargo_bin("somasrv").unwrap();
    cmd.arg("-V")
        .assert()
        .success()
        .stdout(predicate::str::contains("somasrv"))
        .stdout(predicate::str::contains("0.1.0"));
}

/// Test the --config flag with a valid config file
#[test]
fn test_somasrv_config_flag() {
    let temp = assert_fs::TempDir::new().unwrap();
    let config_file = temp.child("config.toml");
    config_file.write_str("[server]\nport = 8080\n").unwrap();

    let mut cmd = Command::cargo_bin("somasrv").unwrap();
    cmd.arg("--config")
        .arg(config_file.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("config: Some("));
}

/// Test the -c flag (short config)
#[test]
fn test_somasrv_config_short_flag() {
    let temp = assert_fs::TempDir::new().unwrap();
    let config_file = temp.child("config.toml");
    config_file.write_str("[server]\nport = 8080\n").unwrap();

    let mut cmd = Command::cargo_bin("somasrv").unwrap();
    cmd.arg("-c")
        .arg(config_file.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("config: Some("));
}

/// Test the --validate flag
#[test]
fn test_somasrv_validate_flag() {
    let mut cmd = Command::cargo_bin("somasrv").unwrap();
    cmd.arg("--validate")
        .assert()
        .success()
        .stdout(predicate::str::contains("validate: true"))
        .stdout(predicate::str::contains("loglevel: \"info\""));
}

/// Test the --loglevel flag with different log levels
#[test]
fn test_somasrv_loglevel_error() {
    let mut cmd = Command::cargo_bin("somasrv").unwrap();
    cmd.arg("--loglevel")
        .arg("error")
        .assert()
        .success()
        .stdout(predicate::str::contains("loglevel: \"error\""));
}

#[test]
fn test_somasrv_loglevel_warn() {
    let mut cmd = Command::cargo_bin("somasrv").unwrap();
    cmd.arg("--loglevel")
        .arg("warn")
        .assert()
        .success()
        .stdout(predicate::str::contains("loglevel: \"warn\""));
}

#[test]
fn test_somasrv_loglevel_info() {
    let mut cmd = Command::cargo_bin("somasrv").unwrap();
    cmd.arg("--loglevel")
        .arg("info")
        .assert()
        .success()
        .stdout(predicate::str::contains("loglevel: \"info\""));
}

#[test]
fn test_somasrv_loglevel_debug() {
    let mut cmd = Command::cargo_bin("somasrv").unwrap();
    cmd.arg("--loglevel")
        .arg("debug")
        .assert()
        .success()
        .stdout(predicate::str::contains("loglevel: \"debug\""));
}

#[test]
fn test_somasrv_loglevel_trace() {
    let mut cmd = Command::cargo_bin("somasrv").unwrap();
    cmd.arg("--loglevel")
        .arg("trace")
        .assert()
        .success()
        .stdout(predicate::str::contains("loglevel: \"trace\""));
}

/// Test the default log level is info
#[test]
fn test_somasrv_default_loglevel() {
    let mut cmd = Command::cargo_bin("somasrv").unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("loglevel: \"info\""));
}

/// Test combining --config and --validate flags
#[test]
fn test_somasrv_config_and_validate() {
    let temp = assert_fs::TempDir::new().unwrap();
    let config_file = temp.child("config.toml");
    config_file.write_str("[server]\nport = 8080\n").unwrap();

    let mut cmd = Command::cargo_bin("somasrv").unwrap();
    cmd.args(&[
        "--config",
        config_file.path().to_str().unwrap(),
        "--validate",
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains("validate: true"))
    .stdout(predicate::str::contains("config: Some("));
}

/// Test combining all flags
#[test]
fn test_somasrv_all_flags_combined() {
    let temp = assert_fs::TempDir::new().unwrap();
    let config_file = temp.child("config.toml");
    config_file.write_str("[server]\nport = 8080\n").unwrap();

    let mut cmd = Command::cargo_bin("somasrv").unwrap();
    cmd.args(&[
        "--config",
        config_file.path().to_str().unwrap(),
        "--validate",
        "--loglevel",
        "debug",
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains("validate: true"))
    .stdout(predicate::str::contains("config: Some("))
    .stdout(predicate::str::contains("loglevel: \"debug\""));
}

/// Test combining short flags
#[test]
fn test_somasrv_short_flags_combined() {
    let temp = assert_fs::TempDir::new().unwrap();
    let config_file = temp.child("config.toml");
    config_file.write_str("[server]\nport = 8080\n").unwrap();

    let mut cmd = Command::cargo_bin("somasrv").unwrap();
    cmd.args(&["-c", config_file.path().to_str().unwrap(), "--validate"])
        .assert()
        .success()
        .stdout(predicate::str::contains("validate: true"))
        .stdout(predicate::str::contains("config: Some("));
}

/// Test config file with non-existent path
#[test]
fn test_somasrv_config_nonexistent_file() {
    let mut cmd = Command::cargo_bin("somasrv").unwrap();
    cmd.args(&["--config", "/nonexistent/path/config.toml"])
        .assert()
        .success() // The binary should still run, config loading happens later
        .stdout(predicate::str::contains("config: Some("));
}

/// Test invalid flag handling
#[test]
fn test_somasrv_invalid_flag() {
    let mut cmd = Command::cargo_bin("somasrv").unwrap();
    cmd.arg("--invalid-flag")
        .assert()
        .failure()
        .stderr(predicate::str::contains("error:"))
        .stderr(predicate::str::contains("Found argument '--invalid-flag'"));
}

/// Test invalid short flag handling
#[test]
fn test_somasrv_invalid_short_flag() {
    let mut cmd = Command::cargo_bin("somasrv").unwrap();
    cmd.arg("-x")
        .assert()
        .failure()
        .stderr(predicate::str::contains("error:"))
        .stderr(predicate::str::contains("Found argument '-x'"));
}

/// Test that unknown positional arguments are rejected
#[test]
fn test_somasrv_unknown_positional_args() {
    let mut cmd = Command::cargo_bin("somasrv").unwrap();
    cmd.arg("unexpected-arg")
        .assert()
        .failure()
        .stderr(predicate::str::contains("error:"))
        .stderr(predicate::str::contains("Found argument 'unexpected-arg'"));
}

/// Test missing argument for --config flag
#[test]
fn test_somasrv_config_missing_argument() {
    let mut cmd = Command::cargo_bin("somasrv").unwrap();
    cmd.arg("--config")
        .assert()
        .failure()
        .stderr(predicate::str::contains("error:"))
        .stderr(predicate::str::contains("requires a value"));
}

/// Test --loglevel flag without argument uses default
#[test]
fn test_somasrv_loglevel_without_argument() {
    let mut cmd = Command::cargo_bin("somasrv").unwrap();
    cmd.arg("--loglevel")
        .assert()
        .success()
        .stdout(predicate::str::contains("loglevel: \"info\""));
}

/// Test that the binary name appears in help output
#[test]
fn test_somasrv_binary_name_in_help() {
    let mut cmd = Command::cargo_bin("somasrv").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("somasrv"));
}

/// Test that debug output format is consistent
#[test]
fn test_somasrv_debug_output_format() {
    let mut cmd = Command::cargo_bin("somasrv").unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::starts_with("Cli {"))
        .stdout(predicate::str::contains("Err(FileNotFound"));
}

/// Test custom log level values (non-standard)
#[test]
fn test_somasrv_custom_loglevel() {
    let mut cmd = Command::cargo_bin("somasrv").unwrap();
    cmd.arg("--loglevel")
        .arg("custom")
        .assert()
        .success()
        .stdout(predicate::str::contains("loglevel: \"custom\""));
}

/// Test config file with relative path
#[test]
fn test_somasrv_config_relative_path() {
    let mut cmd = Command::cargo_bin("somasrv").unwrap();
    cmd.args(&["--config", "./config.toml"])
        .assert()
        .success()
        .stdout(predicate::str::contains("config: Some("));
}

/// Test config file path with spaces
#[test]
fn test_somasrv_config_path_with_spaces() {
    let temp = assert_fs::TempDir::new().unwrap();
    let config_file = temp.child("config file.toml");
    config_file.write_str("[server]\nport = 8080\n").unwrap();

    let mut cmd = Command::cargo_bin("somasrv").unwrap();
    cmd.args(&["--config", config_file.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("config: Some("));
}

/// Test flags in different orders
#[test]
fn test_somasrv_flags_different_order() {
    let temp = assert_fs::TempDir::new().unwrap();
    let config_file = temp.child("config.toml");
    config_file.write_str("[server]\nport = 8080\n").unwrap();

    let mut cmd = Command::cargo_bin("somasrv").unwrap();
    cmd.args(&[
        "--loglevel",
        "warn",
        "--validate",
        "--config",
        config_file.path().to_str().unwrap(),
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains("validate: true"))
    .stdout(predicate::str::contains("config: Some("))
    .stdout(predicate::str::contains("loglevel: \"warn\""));
}
