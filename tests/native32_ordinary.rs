use std::io::Cursor;
use metatron_kernel::{run, verdict::Verdict};

const HEADER: &str = "{\"meta\":{\"exporter\":{\"name\":\"lean4export\",\"version\":\"3.2.0\"},\"format\":{\"version\":\"3.2.0\"},\"lean\":{\"githash\":\"synthetic-control\",\"version\":\"control\"}}}\n";
const TABLES: &str = "{\"in\":1,\"str\":{\"pre\":0,\"str\":\"a\"}}\n{\"il\":1,\"succ\":0}\n{\"ie\":0,\"sort\":1}\n{\"ie\":1,\"sort\":0}\n";
const GOOD: &str = "{\"def\":{\"all\":[1],\"hints\":\"opaque\",\"levelParams\":[],\"name\":1,\"safety\":\"safe\",\"type\":0,\"value\":1}}\n";
fn replay(body:&str)->Verdict {
    run(Cursor::new(format!("{HEADER}{body}").into_bytes()))
}
#[test]
fn native_definitions_check_actual_types() {
    assert_eq!(replay(&format!("{TABLES}{GOOD}")),Verdict::Accept);
    let bad=GOOD.replace("\"type\":0,\"value\":1","\"type\":1,\"value\":0");
    assert_eq!(replay(&format!("{TABLES}{bad}")),Verdict::Reject);
}
#[test]
fn valid_axiom_does_not_authorize_acceptance() {
    let tail="{\"in\":2,\"str\":{\"pre\":0,\"str\":\"b\"}}\n{\"axiom\":{\"isUnsafe\":false,\"levelParams\":[],\"name\":2,\"type\":0}}\n";
    assert_eq!(replay(&format!("{TABLES}{GOOD}{tail}")),Verdict::Unknown);
}
#[test]
fn native_unsafe_and_unsupported_decline() {
    let bad=GOOD.replace("\"safe\"","\"unsafe\"");
    assert_eq!(replay(&format!("{TABLES}{bad}")),Verdict::Unknown);
    let duplicate="{\"ie\":1,\"sort\":0}\n";
    assert_eq!(replay(&format!("{TABLES}{duplicate}{GOOD}")),Verdict::Error);
}
#[test]
fn native_let_is_checked() {
    let body="{\"in\":2,\"str\":{\"pre\":0,\"str\":\"x\"}}\n{\"ie\":2,\"bvar\":0}\n{\"ie\":3,\"letE\":{\"body\":2,\"name\":2,\"nondep\":false,\"type\":0,\"value\":1}}\n";
    let d=GOOD.replace("\"value\":1","\"value\":3");
    assert_eq!(replay(&format!("{TABLES}{body}{d}")),Verdict::Accept);
}
