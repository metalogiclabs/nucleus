# Contributing to Nucleus

Small reproductions and narrowly scoped changes are welcome.

## Reporting a checker result

Include the Nucleus commit, Rust version, launch command, export format,
input SHA-256, raw exit code and stderr. Supply the smallest export that
demonstrates the behavior and explain the expected typing or reduction rule.
Distinguish an incorrect decision from an unsupported input or timeout.

## Testing changes

```sh
python3 scripts/verify_source_import.py
python3 scripts/test.py
python3 -m unittest discover -s tests_py
```

A new admission rule should include a positive case and an adversarial
variant. Preserve existing decisive results, and retain UNKNOWN where a
required check remains unresolved. Do not change expected outcomes merely
to make a test pass.

The runtime identity manifest binds the current implementation to its
qualified snapshot. A deliberate runtime change needs corresponding
qualification evidence and an explicit baseline update.

Runtime decisions must not depend on fixture names, known input hashes,
expected test outcomes or calls to another checker. External qualification
tools may use input names and hashes to bind results to test data.

See [qualification](docs/QUALIFICATION.md) for the executed input sets and
verification boundaries.
