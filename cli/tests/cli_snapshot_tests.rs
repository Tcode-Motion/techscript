//! # TechScript CLI E2E Snapshot Tests
//!
//! Spawns tsc compiler driver binary and asserts output patterns.

#[test]
fn test_cli_version_output() {
    let mut cmd = std::process::Command::new(env!("CARGO_BIN_EXE_tsc"));
    cmd.arg("version");
    let output = cmd.output().expect("Failed to execute tsc binary");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("TECHSCRIPT 2.0 TOOLCHAIN VERSION"));
    assert!(stdout.contains("Compiler Driver:"));
    assert!(stdout.contains("Language Standard:"));
}

#[test]
fn test_cli_doctor_output() {
    let mut cmd = std::process::Command::new(env!("CARGO_BIN_EXE_tsc"));
    cmd.arg("doctor");
    let output = cmd.output().expect("Failed to execute tsc binary");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("Checking TechScript 2.0 Environment Health..."));
    assert!(stdout.contains("Compiler version check"));
    assert!(stdout.contains("Standard library integrity"));
}

#[test]
fn test_cli_fuzzy_suggestions() {
    let mut cmd = std::process::Command::new(env!("CARGO_BIN_EXE_tsc"));
    cmd.arg("doctorr");
    let output = cmd.output().expect("Failed to execute tsc binary");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(stderr.contains("Unknown command: 'doctorr'"));
    assert!(stderr.contains("Did you mean?"));
    assert!(stderr.contains("doctor"));
}
