use assert_cmd::Command;

#[test]
fn true_ok() {
    let mut cmd = Command::cargo_bin("true_rs").unwrap();
    cmd.assert().success();
}

#[test]
fn false_not_ok() {
    let mut cmd = Command::cargo_bin("false_rs").unwrap();
    cmd.assert().failure();
}

