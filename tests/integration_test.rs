use std::process::Command;

#[test]
fn test_cli_usage() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "staxup", "--", "invalid"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to run command");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Usage: staxup"));
}

#[test]
fn test_cli_update() {
    // Since check_and_apply is stubbed, this should succeed
    let output = Command::new("cargo")
        .args(&["run", "--bin", "staxup", "--", "update", "testtool"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to run command");

    // Should exit with 0 since check_and_apply returns Ok(())
    assert!(output.status.success());
}