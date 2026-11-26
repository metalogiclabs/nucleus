# Qualification

The results below describe specific source, configuration and input sets.
They establish bounded test behavior, rather than universal soundness or
complete Lean support.

## Runtime identity and regression checks

The qualified Rust source tree is
`362242c265ee2b1758645c78ad141ad78bf6f8c2`. The
[identity manifest](../evidence/source-import-20261126.json) binds all 28
runtime files to the original qualified snapshot.

[CI](https://github.com/metalogiclabs/nucleus/actions/workflows/ci.yml)
checks that identity, runs the Rust and Python regression suites, builds the
locked release configuration, and exercises the public launcher.

```sh
python3 scripts/verify_source_import.py
python3 scripts/test.py
python3 -m unittest discover -s tests_py
```

The Rust test runner supplies the three capability flags and stack allowance
used in the source qualification. The public launcher uses the separately
frozen `arena-flags.json` configuration.

## Frozen input verification

| Scope | ACCEPT | REJECT | UNKNOWN |
| --- | ---: | ---: | ---: |
| 356 downloadable beta inputs | 59 | 51 | 246 |
| 138 format-3.2.0 inputs, included above | 29 | 34 | 75 |
| Separate 212-input legacy replay | 122 | 84 | 6 |

These scopes overlap and their totals must not be added. The
[original qualification](https://github.com/heathsanchez/test/actions/runs/45681083460)
observed no incorrect decisive results or crashes. The standalone-layout
replay preserved all 356 raw exit codes, with per-input identities recorded
in the [migration receipt](../evidence/standalone-migration-20261126.json).

To reproduce the standalone replay, obtain the original
`nucleus-beta-entry-evidence` artifact, ID `15053582604`, and run:

```sh
python3 scripts/replay_recorded_beta.py \
  --archive /path/to/nucleus-original-qualification.zip \
  --out /tmp/nucleus-migration-replay.json
```

The harness verifies the artifact SHA-256, ZIP CRCs, every input SHA-256 and
every recorded raw exit code. Expected archive SHA-256:

```text
d8a2cb8b5c92bb14dea63fb019086e6c7f8a6b6183d804681f53a50fb8c145ee
```

## Official Arena qualification

The [source-relocation qualification](https://github.com/leanprover/lean-kernel-arena-beta/actions/runs/45738423426)
passed all five jobs. Its 364 result rows contain 110 correct prescribed
decisions, three permitted policy decisions and 251 declines, with zero
incorrect results, errors or timeouts. The results archive was independently
SHA-256 and CRC verified.

The Arena's CI omissions and policy cases differ from the frozen input sets
above; their counts must not be added.

## Supported boundary

Inputs larger than 10 MiB are declined by the generic allowance. Unsupported
declarations and conversions return UNKNOWN. Large-input declines are not
successful proof checks, and full Mathlib acceptance is not established.

The abstract Lean model under `formal/` establishes limited environment laws
under explicit typing assumptions. It does not establish refinement or
soundness of the Rust implementation.
