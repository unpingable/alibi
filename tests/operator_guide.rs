use std::process::Command;

#[test]
fn guide_selects_existing_accusations_without_collecting_a_receipt() {
    let output = Command::new(env!("CARGO_BIN_EXE_alibi"))
        .arg("guide")
        .output()
        .unwrap();
    assert!(output.status.success());
    let text = String::from_utf8(output.stdout).unwrap();
    for evaluator in alibi::eval::registry() {
        assert!(text.contains(evaluator.accusation()));
    }
    assert!(text.contains("does not diagnose"));
    assert!(text.contains("DNS"));
    assert!(text.contains("UNKNOWN"));
    assert!(!text.contains("ALIBI host="));
    let invalid = Command::new(env!("CARGO_BIN_EXE_alibi"))
        .args(["guide", "extra"])
        .output()
        .unwrap();
    assert_eq!(invalid.status.code(), Some(64));
}
