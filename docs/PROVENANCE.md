# Source provenance

Nucleus is maintained at [metalogiclabs/nucleus](https://github.com/metalogiclabs/nucleus).
The implementation originated in the `metatron-kernel/` directory of
[`heathsanchez/test`](https://github.com/heathsanchez/test/tree/578697991a903ab78d344aa71165d465bef3c3d7/metatron-kernel).
Its source history is retained in Git.

| Qualified component | Identity |
| --- | --- |
| Original source commit | `578697991a903ab78d344aa71165d465bef3c3d7` |
| Rust source tree | `362242c265ee2b1758645c78ad141ad78bf6f8c2` |
| Rust toolchain | `1.99.1` |
| Runtime identity manifest | [source-import-20261126.json](../evidence/source-import-20261126.json) |

The manifest records SHA-256 and Git blob identities for all 28 runtime
files: the Rust implementation, Cargo manifest and lockfile, launcher and
frozen capability flags. Verify them with:

```sh
python3 scripts/verify_source_import.py
```

The executable retains its historical `metatron-kernel` name. Use `arena.py`
for the documented launch configuration.

See [qualification](QUALIFICATION.md) for the tested input sets, reproducible
results and limits of the checker.
