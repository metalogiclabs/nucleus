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
    let bytes = std::fs::read(root.join("tests/fixtures/native32-identity").join(format!("{name}.ndjson"))).unwrap();
    let mut child = cmd.spawn().unwrap();
    child.stdin.take().unwrap().write_all(&bytes).unwrap();
    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(),Some(expected),"{name}: {}",String::from_utf8_lossy(&output.stderr));
}

#[test]
fn indexed_identity_native_and_negative_controls() {
    check("043_eqType",0);
    check("069_eqRec",0);
    check("078_boolRecEqns",0);
    check("079_prodRecEqns",0);
    check("missing-rule",2);
    check("forged-rule",2);
    check("forged-recursor-type",2);
    check("forged-constructor-result",1);
    check("wrong-owner",2);
    check("wrong-index-count",2);
    check("fake-projection",1);
    check("false-k-agreement",1);
    check("identity-true",0);
    check("identity-false",2);
}
