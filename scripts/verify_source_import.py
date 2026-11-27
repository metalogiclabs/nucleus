#!/usr/bin/env python3
"""Verify runtime bytes against the frozen, qualified source import."""
import hashlib
import json
from pathlib import Path
import sys


def main():
    root = Path(__file__).resolve().parents[1]
    original_path = root / "evidence/source-import-20261126.json"
    original = json.loads(original_path.read_text())
    qualified_path = root / "evidence/native32-ordinary-qualification-20261127.json"
    manifest = original
    if qualified_path.exists():
        manifest = json.loads(qualified_path.read_text())
        if (manifest.get("status") != "WARRANTED_BOUNDED"
                or manifest.get("base_manifest_sha256") != hashlib.sha256(original_path.read_bytes()).hexdigest()
                or set(manifest["runtime_files"]) != set(original["runtime_files"])):
            print("Invalid qualified runtime lineage", file=sys.stderr)
            return 1
        changed = sorted(k for k in manifest["runtime_files"]
                         if manifest["runtime_files"][k] != original["runtime_files"][k])
        if changed != manifest["changed_runtime_files"]:
            print("Qualified runtime delta differs from manifest", file=sys.stderr)
            return 1
    computation_path = root / "evidence/native32-computation-qualification-20261127.json"
    if computation_path.exists():
        previous = manifest
        if not qualified_path.exists():
            print("Missing parent qualification manifest", file=sys.stderr)
            return 1
        manifest = json.loads(computation_path.read_text())
        added = {"src/renovation/nullary/composite.rs"}
        changed = {"src/renovation.rs", "src/renovation/nullary.rs",
                   "src/renovation/nullary/parametric.rs"} | added
        if (manifest.get("status") != "WARRANTED_BOUNDED"
                or manifest.get("parent_manifest_sha256") != hashlib.sha256(qualified_path.read_bytes()).hexdigest()
                or set(manifest["added_runtime_files"]) != added
                or set(manifest["changed_runtime_files"]) != changed
                or set(manifest["runtime_files"]) != set(previous["runtime_files"]) | added
                or {k for k in manifest["runtime_files"] if manifest["runtime_files"][k] != previous["runtime_files"].get(k)} != changed):
            print("Invalid computation qualification lineage", file=sys.stderr)
            return 1
    identity_path = root / "evidence/native32-identity-qualification-20261127.json"
    if identity_path.exists():
        previous = manifest
        if not computation_path.exists():
            print("Missing parent computation qualification", file=sys.stderr)
            return 1
        manifest = json.loads(identity_path.read_text())
        changed = {"src/renovation/nullary/parametric.rs"}
        if (manifest.get("status") != "WARRANTED_BOUNDED"
                or manifest.get("parent_manifest_sha256") != hashlib.sha256(computation_path.read_bytes()).hexdigest()
                or set(manifest["runtime_files"]) != set(previous["runtime_files"])
                or set(manifest["changed_runtime_files"]) != changed
                or {k for k in manifest["runtime_files"] if manifest["runtime_files"][k] != previous["runtime_files"][k]} != changed):
            print("Invalid indexed identity qualification lineage", file=sys.stderr)
            return 1
    k_path = root / "evidence/native32-typed-k-qualification-20261127.json"
    if k_path.exists():
        previous = manifest
        if not identity_path.exists():
            print("Missing parent indexed identity qualification", file=sys.stderr)
            return 1
        manifest = json.loads(k_path.read_text())
        changed = {"src/renovation/nullary/parametric.rs"}
        if (manifest.get("status") != "WARRANTED_BOUNDED"
                or manifest.get("parent_manifest_sha256") != hashlib.sha256(identity_path.read_bytes()).hexdigest()
                or set(manifest["runtime_files"]) != set(previous["runtime_files"])
                or set(manifest["changed_runtime_files"]) != changed
                or {k for k in manifest["runtime_files"] if manifest["runtime_files"][k] != previous["runtime_files"][k]} != changed):
            print("Invalid typed K qualification lineage", file=sys.stderr)
            return 1
    partial_k_path = root / "evidence/native32-partial-k-qualification-20261127.json"
    if partial_k_path.exists():
        previous = manifest
        if not k_path.exists():
            print("Missing parent typed K qualification", file=sys.stderr)
            return 1
        manifest = json.loads(partial_k_path.read_text())
        changed = {"src/convert.rs", "src/typecheck.rs"}
        if (manifest.get("status") != "WARRANTED_BOUNDED"
                or manifest.get("parent_manifest_sha256") != hashlib.sha256(k_path.read_bytes()).hexdigest()
                or set(manifest["runtime_files"]) != set(previous["runtime_files"])
                or set(manifest["changed_runtime_files"]) != changed
                or {k for k in manifest["runtime_files"] if manifest["runtime_files"][k] != previous["runtime_files"][k]} != changed):
            print("Invalid partial K qualification lineage", file=sys.stderr)
            return 1
    unary_path = root / "evidence/native32-unary-recursion-qualification-20261127.json"
    if unary_path.exists():
        previous = manifest
        if not partial_k_path.exists():
            print("Missing parent partial K qualification", file=sys.stderr)
            return 1
        manifest = json.loads(unary_path.read_text())
        changed = {"src/renovation/nullary/parametric.rs"}
        if (manifest.get("status") != "WARRANTED_BOUNDED"
                or manifest.get("parent_manifest_sha256") != hashlib.sha256(partial_k_path.read_bytes()).hexdigest()
                or set(manifest["runtime_files"]) != set(previous["runtime_files"])
                or set(manifest["changed_runtime_files"]) != changed
                or {k for k in manifest["runtime_files"] if manifest["runtime_files"][k] != previous["runtime_files"][k]} != changed):
            print("Invalid unary recursion qualification lineage", file=sys.stderr)
            return 1
    expected = manifest["runtime_files"]
    actual = {p.relative_to(root).as_posix() for p in (root / "src").rglob("*") if p.is_file()}
    actual.update(("Cargo.toml", "Cargo.lock", "arena.py", "arena-flags.json"))
    failures = []
    for name in sorted(set(expected) | actual):
        path = root / name
        if name not in expected or name not in actual or not path.is_file():
            failures.append(name + ": missing or unexpected runtime file")
        elif hashlib.sha256(path.read_bytes()).hexdigest() != expected[name]["sha256"]:
            failures.append(name + ": differs from qualified source")
    if failures:
        print("\n".join(failures), file=sys.stderr)
        return 1
    print(f"Verified {len(expected)} runtime files against {manifest.get('qualification_run', manifest['original_commit'])}")
    print("Original src tree:", manifest["original_src_tree"])
    return 0


if __name__ == "__main__":
    sys.exit(main())
