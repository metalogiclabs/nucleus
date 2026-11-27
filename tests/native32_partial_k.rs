use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

fn check(fixture: &str, expected: i32) {
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
    let bytes = std::fs::read(root.join("tests/fixtures").join(fixture)).unwrap();
    let mut child = cmd.spawn().unwrap();
    child.stdin.take().unwrap().write_all(&bytes).unwrap();
    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(), Some(expected), "{fixture}: {}", String::from_utf8_lossy(&output.stderr));
}

#[test]
fn partial_k_uses_a_typed_fresh_major() {
    check("native32-typed-k/129_etaRuleK.ndjson", 0);
}

#[test]
fn wrong_indices_and_untyped_arguments_cannot_license_k() {
    for name in ["partial-wrong-index", "partial-untyped-minor", "full-wrong-major"] {
        check(&format!("native32-partial-k/{name}.ndjson"), 2);
    }
}
