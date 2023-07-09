use assert_cmd::Command;

#[test]
fn runs() {
 let mut cmd = Command::cargo_bin("chapter_1").unwrap();
 cmd.assert().success();
}