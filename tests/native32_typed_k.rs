use std::io::Write;
use std::process::{Command, Stdio};
use std::path::Path;

fn check(name: &str, expected: i32) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let flags: std::collections::HashMap<String,String> =
        serde_json::from_slice(&std::fs::read(root.join("arena-flags.json")).unwrap()).unwrap();
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_metatron-kernel"));
    for (key, _) in std::env::vars_os() {
        let s = key.to_string_lossy();
        if s.starts_with("NUCLEUS_") || s.starts_with("METATRON_") {
            cmd.env_remove(key);
        }
    }
    cmd.envs(flags).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped());
    let bytes = std::fs::read(root.join("tests/fixtures/native32-typed-k").join(format!("{name}.ndjson"))).unwrap();
    let mut child = cmd.spawn().unwrap();
    child.stdin.take().unwrap().write_all(&bytes).unwrap();
    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(),Some(expected),"{name}: {}",String::from_utf8_lossy(&output.stderr));
}

#[test]
fn typed_k_native_and_negative_controls() {
    check("099_ruleK",0);
    check("100_ruleKbad",2);
    check("129_etaRuleK",0);
    check("075_typeSingletonRecReduction",2);
    check("041_kEqK",2);
    check("constructor-major-good",0);
    check("constructor-major-wrong-index",2);
    check("withdraw-k",1);
    check("forged-rhs",2);
    check("wrong-index-metadata",2);
    check("wrong-ctor",2);
}
