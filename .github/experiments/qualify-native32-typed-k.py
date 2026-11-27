#!/usr/bin/env python3
"""Replay immutable cohorts and countercontrols through the public launcher."""
import collections
import hashlib
import io
import json
import os
from pathlib import Path
import subprocess
import tarfile
import zipfile

OUT = Path("evidence-run")
BASE_SHA256 = "72b3db6e2d59d4025fcfbc8e140b5dfa9d057cb206c4cc6625b4740ee8e2698b"
TOOLCHAIN = "leanprover/lean4-pr-releases:pr-release-16595-cb63311"
EXPECTED = {
    "099_ruleK": 0, "100_ruleKbad": 2, "129_etaRuleK": 0,
    "forged-rule": 2, "indexed-iota-bad": 2, "indexed-iota-good": 0,
    "missing-rule": 2, "partial-untyped-minor": 2, "partial-wrong-index": 2,
    "withdraw-k": 1, "wrong-index-count": 2,
}


def digest(raw):
    return hashlib.sha256(raw).hexdigest()


def archive(raw):
    z = zipfile.ZipFile(io.BytesIO(raw))
    assert z.testzip() is None
    return z


def inputs(raw):
    z = archive(raw)
    with tarfile.open(fileobj=io.BytesIO(z.read("lean-arena-tests.tar.gz"))) as t:
        return {m.name.split("/", 1)[1].removesuffix(".ndjson"): t.extractfile(m).read()
                for m in t.getmembers() if m.isfile() and m.name.endswith(".ndjson")}


def run(raw, baseline=False):
    if baseline:
        env = {k: v for k, v in os.environ.items()
               if not k.startswith(("NUCLEUS_", "METATRON_"))}
        env.update(json.loads(Path("arena-flags.json").read_text()))
        command = [str((OUT / "baseline").resolve())]
    else:
        env = None
        command = ["python3", "arena.py"]
    p = subprocess.run(command, input=raw, capture_output=True, env=env, timeout=120)
    return p.returncode, p.stderr.decode(errors="replace")


def save(name, value):
    (OUT / name).write_text(json.dumps(value, indent=2, sort_keys=True) + "\n")


def main():
    raw = (OUT / "base.zip").read_bytes()
    assert digest(raw) == BASE_SHA256
    z = archive(raw)
    iota = archive(z.read("base.zip"))
    composition = archive(iota.read("composition.zip"))
    ordinary = archive(composition.read("base.zip"))
    old = archive(ordinary.read("prior-qualification.zip"))
    corpus = {"current": inputs(ordinary.read("test-tarball.zip")),
              "prior": inputs(old.read("tarball.zip"))}
    for cohort, ins in corpus.items():
        before = {r["name"]: r for r in json.loads(z.read(cohort + "-rows.json"))}
        assert len(ins) == 356 and set(ins) == set(before)
        rows = []
        for name, data in ins.items():
            row = before[name]
            assert digest(data) == row["sha256"], name
            base_code, _ = run(data, baseline=True)
            assert base_code == row["exit"], (cohort, name, base_code, row)
            code, err = run(data)
            result = {**row, "before": base_code, "exit": code, "stderr": err}
            rows.append(result)
            assert code == base_code or (
                base_code == 2 and code == {"accept": 0, "reject": 1}[row["expected"]]
            ), (cohort, result)
        changes = [r for r in rows if r["before"] != r["exit"]]
        summary = {"input_count": len(rows), "counts": dict(collections.Counter(r["exit"] for r in rows)),
                   "changes": changes}
        save(cohort + "-rows.json", rows)
        save(cohort + "-summary.json", summary)
        print(cohort, json.dumps(summary), flush=True)
        if cohort == "current":
            assert {r["name"] for r in changes} == {
                "tutorial/099_ruleK", "renovation/129_etaRuleK"}
            assert all(r["before"] == 2 and r["exit"] == 0 for r in changes)

    preserved = []
    for row in json.loads(iota.read("native-ab.json")):
        data = iota.read("native/" + row["name"] + ".ndjson")
        assert digest(data) == row["sha256"]
        code, err = run(data)
        assert code == row["after"], (row, code, err)
        preserved.append({"group": "iota", "name": row["name"], "sha256": digest(data), "exit": code})
    for row in json.loads(iota.read("iota-controls.json")):
        data = iota.read("native/mut-" + row["name"] + ".ndjson")
        assert digest(data) == row["sha256"]
        code, err = run(data)
        assert code == row["exit"], (row, code, err)
        preserved.append({"group": "iota", "name": row["name"], "sha256": digest(data), "exit": code})
    for row in json.loads(z.read("controls.json")):
        data = z.read("controls/" + row["name"] + ".ndjson")
        assert digest(data) == row["sha256"]
        code, err = run(data)
        assert code == row["exit"], (row, code, err)
        preserved.append({"group": "identity", "name": row["name"], "sha256": digest(data), "exit": code})
    for row in json.loads(ordinary.read("new-regressions.json")):
        data = ordinary.read("new-inputs/" + row["case"])
        assert digest(data) == row["sha256"]
        code, err = run(data)
        assert code == row["exit"], (row, code, err)
        preserved.append({"group": "external", "name": row["case"], "sha256": digest(data), "exit": code})
    assert collections.Counter(r["group"] for r in preserved) == {"iota": 12, "identity": 10, "external": 4}
    save("preserved-controls.json", preserved)

    fixtures = Path("tests/fixtures/native32-typed-k")
    assert {p.stem for p in fixtures.glob("*.ndjson")} == set(EXPECTED)
    controls = []
    for name, expected in EXPECTED.items():
        path = fixtures / (name + ".ndjson")
        data = path.read_bytes()
        before, _ = run(data, baseline=True)
        code, err = run(data)
        assert code == expected, (name, expected, code, err)
        if name in ("099_ruleK", "129_etaRuleK", "indexed-iota-good"):
            assert before == 2
        official = subprocess.run(["elan", "run", TOOLCHAIN, "leanchecker", "--from-export", str(path)],
                                  capture_output=True, timeout=120)
        assert (official.returncode == 0) == (expected == 0), (
            name, official.returncode, official.stdout, official.stderr)
        controls.append({"name": name, "sha256": digest(data), "before": before,
                         "exit": code, "stderr": err, "official_exit": official.returncode,
                         "official_stdout": official.stdout.decode(errors="replace"),
                         "official_stderr": official.stderr.decode(errors="replace")})
    save("controls.json", controls)
    runtime = {str(p): {"sha256": digest(p.read_bytes()),
                       "git_blob": hashlib.sha1(b"blob " + str(p.stat().st_size).encode() + b"\0" + p.read_bytes()).hexdigest()}
               for p in sorted(Path("src").rglob("*.rs"))}
    save("runtime-source.json", runtime)
    print("QUALIFIED_TYPED_K", json.dumps({"controls": controls, "preserved": len(preserved)}), flush=True)


if __name__ == "__main__":
    main()
