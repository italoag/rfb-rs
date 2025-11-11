/// End-to-end integration tests
use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("rfb").unwrap();
    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("RFB-RS"))
        .stdout(predicate::str::contains("download"))
        .stdout(predicate::str::contains("transform"))
        .stdout(predicate::str::contains("api"));
}

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("rfb").unwrap();
    cmd.arg("--version");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("rfb"));
}

#[test]
fn test_download_command_help() {
    let mut cmd = Command::cargo_bin("rfb").unwrap();
    cmd.arg("download").arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Download data files"))
        .stdout(predicate::str::contains("--directory"))
        .stdout(predicate::str::contains("--parallel"));
}

#[test]
fn test_transform_command_help() {
    let mut cmd = Command::cargo_bin("rfb").unwrap();
    cmd.arg("transform").arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Transform downloaded data"))
        .stdout(predicate::str::contains("--directory"))
        .stdout(predicate::str::contains("--privacy"));
}

#[test]
fn test_db_command_help() {
    let mut cmd = Command::cargo_bin("rfb").unwrap();
    cmd.arg("db").arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Database commands"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("drop"));
}

#[test]
fn test_api_command_help() {
    let mut cmd = Command::cargo_bin("rfb").unwrap();
    cmd.arg("api").arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Start API server"))
        .stdout(predicate::str::contains("--port"))
        .stdout(predicate::str::contains("--host"));
}

#[test]
fn test_check_command_help() {
    let mut cmd = Command::cargo_bin("rfb").unwrap();
    cmd.arg("check").arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Check integrity"))
        .stdout(predicate::str::contains("--directory"));
}

#[test]
fn test_check_nonexistent_directory() {
    let mut cmd = Command::cargo_bin("rfb").unwrap();
    cmd.arg("check").arg("--directory").arg("/nonexistent/path");

    cmd.assert().failure();
}

#[test]
fn test_check_empty_directory() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("rfb").unwrap();
    cmd.arg("check")
        .arg("--directory")
        .arg(temp_dir.path().to_str().unwrap());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Checked 0 files"));
}

#[test]
fn test_db_create_without_url() {
    let mut cmd = Command::cargo_bin("rfb").unwrap();
    cmd.arg("db").arg("create");

    // Should fail without DATABASE_URL
    cmd.assert().failure();
}

#[test]
fn test_api_invalid_port() {
    let mut cmd = Command::cargo_bin("rfb").unwrap();
    cmd.arg("api").arg("--port").arg("99999");

    // Port out of range should fail
    cmd.assert().failure();
}

#[test]
fn test_download_invalid_parallel() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("rfb").unwrap();
    cmd.arg("download")
        .arg("--directory")
        .arg(temp_dir.path().to_str().unwrap())
        .arg("--parallel")
        .arg("0")
        .timeout(std::time::Duration::from_secs(5));

    // Parallel 0 is invalid and should fail immediately with error message
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("parallel"));
}
