use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

fn check(name: &str, expected: i32) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let flags: std::collections::HashMap<String, String> =
        serde_json::from_slice(&std::fs::read(root.join("arena-flags.json")).unwrap()).unwrap();
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_metatron-kernel"));
    for (key, _) in std::env::vars_os() {
        let s = key.to_string_lossy();
        if s.starts_with("NUCLEUS_") || s.starts_with("METATRON_") {
            cmd.env_remove(key);
        }
    }
    cmd.envs(flags).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped());
    let bytes = std::fs::read(root.join("tests/fixtures/native32-unary-recursion").join(format!("{name}.ndjson"))).unwrap();
    let mut child = cmd.spawn().unwrap();
    child.stdin.take().unwrap().write_all(&bytes).unwrap();
    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(), Some(expected), "{name}: {}", String::from_utf8_lossy(&output.stderr));
}

#[test]
fn native_unary_recursive_signatures_are_reconstructed() {
    check("044_natDef", 0);
    check("070_nRec", 0);
}

#[test]
fn native_unary_computation_preserves_the_actual_induction_hypothesis() {
    check("080_nRecReduction", 0);
}

#[test]
fn erased_or_corrupt_induction_hypotheses_do_not_supply_authority() {
    for name in ["erased-ih", "wrong-recursive-target", "swapped-step-ih", "forged-zero-rule"] {
        check(name, 2);
    }
}
