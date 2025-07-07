use assert_cmd::Command;
use predicates::prelude::*;

/// Test that the soma binary can be executed without arguments
#[test]
fn test_soma_runs_without_args() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No subcommand specified"))
        .stdout(predicate::str::contains("Cli {"))
        .stdout(predicate::str::contains("noaction: false"))
        .stdout(predicate::str::contains("verbose: false"))
        .stdout(predicate::str::contains("command: None"));
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
        .stdout(predicate::str::contains("SUBCOMMANDS:"))
        .stdout(predicate::str::contains("--noaction"))
        .stdout(predicate::str::contains("--verbose"))
        .stdout(predicate::str::contains("help"))
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("scan"))
        .stdout(predicate::str::contains("check"));
}

/// Test the -h flag (short help)
#[test]
fn test_soma_help_short_flag() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.arg("-h")
        .assert()
        .success()
        .stdout(predicate::str::contains("USAGE:"))
        .stdout(predicate::str::contains("FLAGS:"))
        .stdout(predicate::str::contains("SUBCOMMANDS:"));
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
        .stdout(predicate::str::contains("verbose: false"))
        .stdout(predicate::str::contains("Running in no-action mode"));
}

/// Test the -n flag (short noaction)
#[test]
fn test_soma_noaction_short_flag() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.arg("-n")
        .assert()
        .success()
        .stdout(predicate::str::contains("noaction: true"))
        .stdout(predicate::str::contains("verbose: false"))
        .stdout(predicate::str::contains("Running in no-action mode"));
}

/// Test the --verbose flag
#[test]
fn test_soma_verbose_flag() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.arg("--verbose")
        .assert()
        .success()
        .stdout(predicate::str::contains("verbose: true"))
        .stdout(predicate::str::contains("noaction: false"))
        .stdout(predicate::str::contains("Running in verbose mode"));
}

/// Test the -v flag (short verbose)
#[test]
fn test_soma_verbose_short_flag() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.arg("-v")
        .assert()
        .success()
        .stdout(predicate::str::contains("verbose: true"))
        .stdout(predicate::str::contains("noaction: false"))
        .stdout(predicate::str::contains("Running in verbose mode"));
}

/// Test combining --noaction and --verbose flags
#[test]
fn test_soma_combined_flags() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.args(&["--noaction", "--verbose"])
        .assert()
        .success()
        .stdout(predicate::str::contains("noaction: true"))
        .stdout(predicate::str::contains("verbose: true"))
        .stdout(predicate::str::contains("Running in verbose mode"))
        .stdout(predicate::str::contains("Running in no-action mode"));
}

/// Test combining short flags
#[test]
fn test_soma_combined_short_flags() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.args(&["-n", "-v"])
        .assert()
        .success()
        .stdout(predicate::str::contains("noaction: true"))
        .stdout(predicate::str::contains("verbose: true"))
        .stdout(predicate::str::contains("Running in verbose mode"))
        .stdout(predicate::str::contains("Running in no-action mode"));
}

/// Test combining flags in different order
#[test]
fn test_soma_flags_different_order() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.args(&["--verbose", "--noaction"])
        .assert()
        .success()
        .stdout(predicate::str::contains("noaction: true"))
        .stdout(predicate::str::contains("verbose: true"))
        .stdout(predicate::str::contains("Running in verbose mode"))
        .stdout(predicate::str::contains("Running in no-action mode"));
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
        .stdout(predicate::str::contains("No subcommand specified"))
        .stdout(predicate::str::contains("Cli {"))
        .stdout(predicate::str::ends_with("}\n"));
}

// SUBCOMMAND TESTS

/// Test help subcommand
#[test]
fn test_soma_help_subcommand() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.arg("help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "SOMA - System Operations Management Agent",
        ))
        .stdout(predicate::str::contains("USAGE:"))
        .stdout(predicate::str::contains("FLAGS:"))
        .stdout(predicate::str::contains("SUBCOMMANDS:"))
        .stdout(predicate::str::contains("help"))
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("scan"))
        .stdout(predicate::str::contains("check"));
}

/// Test help subcommand with verbose flag
#[test]
fn test_soma_help_subcommand_verbose() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.args(&["--verbose", "help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Running in verbose mode"))
        .stdout(predicate::str::contains(
            "SOMA - System Operations Management Agent",
        ));
}

/// Test list subcommand
#[test]
fn test_soma_list_subcommand() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("Managed Hosts:"))
        .stdout(predicate::str::contains("Hostname"))
        .stdout(predicate::str::contains("IP Address"))
        .stdout(predicate::str::contains("Status"))
        .stdout(predicate::str::contains("host1.example.com"))
        .stdout(predicate::str::contains("192.168.1.10"));
}

/// Test list subcommand with JSON output
#[test]
fn test_soma_list_subcommand_json() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.args(&["list", "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"hosts\":"))
        .stdout(predicate::str::contains("\"hostname\":"))
        .stdout(predicate::str::contains("\"ip\":"))
        .stdout(predicate::str::contains("\"status\":"))
        .stdout(predicate::str::contains("host1.example.com"))
        .stdout(predicate::str::contains("192.168.1.10"));
}

/// Test list subcommand with CSV output
#[test]
fn test_soma_list_subcommand_csv() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.args(&["list", "--csv"])
        .assert()
        .success()
        .stdout(predicate::str::contains("hostname,ip,status"))
        .stdout(predicate::str::contains(
            "host1.example.com,192.168.1.10,managed",
        ))
        .stdout(predicate::str::contains(
            "host2.example.com,192.168.1.11,managed",
        ));
}

/// Test list subcommand with noaction flag
#[test]
fn test_soma_list_subcommand_noaction() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.args(&["--noaction", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Running in no-action mode"))
        .stdout(predicate::str::contains("Would list all managed hosts"));
}

/// Test scan subcommand
#[test]
fn test_soma_scan_subcommand() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.arg("scan")
        .assert()
        .success()
        .stdout(predicate::str::contains("Network Scan Results:"))
        .stdout(predicate::str::contains("Hostname"))
        .stdout(predicate::str::contains("IP Address"))
        .stdout(predicate::str::contains("Status"))
        .stdout(predicate::str::contains("managed"))
        .stdout(predicate::str::contains("unmanaged"));
}

/// Test scan subcommand with JSON output
#[test]
fn test_soma_scan_subcommand_json() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.args(&["scan", "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"scan_results\":"))
        .stdout(predicate::str::contains("\"hostname\":"))
        .stdout(predicate::str::contains("\"ip\":"))
        .stdout(predicate::str::contains("\"status\":"))
        .stdout(predicate::str::contains("managed"))
        .stdout(predicate::str::contains("unmanaged"));
}

/// Test scan subcommand with CSV output
#[test]
fn test_soma_scan_subcommand_csv() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.args(&["scan", "--csv"])
        .assert()
        .success()
        .stdout(predicate::str::contains("hostname,ip,status"))
        .stdout(predicate::str::contains("managed"))
        .stdout(predicate::str::contains("unmanaged"));
}

/// Test scan subcommand with noaction flag
#[test]
fn test_soma_scan_subcommand_noaction() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.args(&["--noaction", "scan"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Running in no-action mode"))
        .stdout(predicate::str::contains(
            "Would scan network for managed and unmanaged hosts",
        ));
}

/// Test check subcommand without hosts
#[test]
fn test_soma_check_subcommand_no_hosts() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.arg("check")
        .assert()
        .success()
        .stdout(predicate::str::contains("Host Status Reports:"))
        .stdout(predicate::str::contains("Hostname"))
        .stdout(predicate::str::contains("Status"))
        .stdout(predicate::str::contains("Health"))
        .stdout(predicate::str::contains("Last Seen"))
        .stdout(predicate::str::contains("online"))
        .stdout(predicate::str::contains("healthy"));
}

/// Test check subcommand with specific hosts
#[test]
fn test_soma_check_subcommand_with_hosts() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.args(&["check", "host1.example.com", "host2.example.com"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Host Status Reports:"))
        .stdout(predicate::str::contains("host1.example.com"))
        .stdout(predicate::str::contains("host2.example.com"));
}

/// Test check subcommand with JSON output
#[test]
fn test_soma_check_subcommand_json() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.args(&["check", "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"status_reports\":"))
        .stdout(predicate::str::contains("\"hostname\":"))
        .stdout(predicate::str::contains("\"status\":"))
        .stdout(predicate::str::contains("\"health\":"))
        .stdout(predicate::str::contains("\"last_seen\":"));
}

/// Test check subcommand with CSV output
#[test]
fn test_soma_check_subcommand_csv() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.args(&["check", "--csv"])
        .assert()
        .success()
        .stdout(predicate::str::contains("hostname,status,health,last_seen"))
        .stdout(predicate::str::contains("online,healthy"));
}

/// Test check subcommand with noaction flag
#[test]
fn test_soma_check_subcommand_noaction() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.args(&["--noaction", "check", "host1.example.com"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Running in no-action mode"))
        .stdout(predicate::str::contains("Would check status of hosts"));
}

/// Test invalid subcommand
#[test]
fn test_soma_invalid_subcommand() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.arg("invalid")
        .assert()
        .failure()
        .stderr(predicate::str::contains("error:"))
        .stderr(predicate::str::contains("Found argument 'invalid'"));
}

/// Test subcommand with invalid flags
#[test]
fn test_soma_subcommand_invalid_flag() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.args(&["list", "--invalid"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("error:"))
        .stderr(predicate::str::contains("Found argument '--invalid'"));
}

/// Test combining JSON and CSV flags (should work but CSV might take precedence)
#[test]
fn test_soma_list_json_and_csv() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.args(&["list", "--json", "--csv"]).assert().success(); // Should succeed, implementation determines which format wins
}

/// Test flags with subcommands in different order
#[test]
fn test_soma_flags_subcommand_order() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.args(&["--verbose", "--noaction", "list", "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Running in verbose mode"))
        .stdout(predicate::str::contains("Running in no-action mode"))
        .stdout(predicate::str::contains("Would list all managed hosts"));
}

/// Test help for individual subcommands
#[test]
fn test_soma_list_help() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.args(&["list", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "List all hosts that can be managed",
        ))
        .stdout(predicate::str::contains("--json"))
        .stdout(predicate::str::contains("--csv"));
}

/// Test help for scan subcommand
#[test]
fn test_soma_scan_help() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.args(&["scan", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Scan the network for hosts"))
        .stdout(predicate::str::contains("--json"))
        .stdout(predicate::str::contains("--csv"));
}

/// Test help for check subcommand
#[test]
fn test_soma_check_help() {
    let mut cmd = Command::cargo_bin("soma").unwrap();
    cmd.args(&["check", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Request a status report"))
        .stdout(predicate::str::contains("--json"))
        .stdout(predicate::str::contains("--csv"))
        .stdout(predicate::str::contains("<hosts>..."));
}
