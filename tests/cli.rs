use assert_cmd::Command;

#[test]
fn test_cli() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("3").assert().code(3);
}

#[test]
fn test_cli_no_args() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.assert().code(0);
}

#[test]
fn test_cli_invalid_args() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("abc").assert().code(0);
}
