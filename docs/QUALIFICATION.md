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


## Native NDJSON 3.2.0 ordinary-core migration (2026-11-27)

[Qualified run 45938613850](https://github.com/metalogiclabs/nucleus/actions/runs/45938613850)
and [retained artifact 15172160773](https://github.com/metalogiclabs/nucleus/actions/runs/45938613850/artifacts/15172160773)
establish the exact runtime file hashes in
[the separate qualification manifest](../evidence/native32-ordinary-qualification-20261127.json).
The original import manifest remains historical evidence.

Two separate 356-input replays (current native-export cohort and earlier
preview cohort) each yielded 62 ACCEPT, 51 REJECT, and 243 UNKNOWN, with
no incorrect decisive results. All 38 decisions lost in the exporter
migration were recovered; three additional universe cases were accepted.
The core accepts only strictly validated safe ordinary definitions,
opaque theorems and dependent-form lets. An ill-formed final axiom may
be rejected; a valid axiom does not authorize acceptance. No new
inductive, projection, quotient or literal authority is introduced.

Four independently sourced regressions yielded UNKNOWN, UNKNOWN,
REJECT, UNKNOWN respectively (valid and invalid casesOn, invalid
Nat.log2, missing Nat.succ). The invalid casesOn transport error was
eliminated, but its semantic rejection and broader recursive checking
remain unresolved. These are bounded local qualification results, not
a new official Arena ranking or a whole-Mathlib result.


## Certified nonrecursive composition and computation (2026-11-27)

[Composition qualification](https://github.com/metalogiclabs/nucleus/actions/runs/45940578030)
recovered 20 correct ACCEPTs on the current native 356-input cohort,
and one on the earlier cohort. The [iota qualification](https://github.com/metalogiclabs/nucleus/actions/runs/45943776855)
preserved every one of these verdicts: current 82A/51R/223U,
earlier 63A/51R/242U. The cohorts overlap.

Four newly generated native 3.2 exports exercising constructor
selection, field substitution and surplus arguments changed from
UNKNOWN to ACCEPT; the official pinned Lean kernel accepted each.
Three invalid computation witnesses stayed UNKNOWN. Five malformed
rule controls produced two REJECTs and three UNKNOWNs. Exact source,
fixtures, A/B and both protected replays are retained in
[artifact 15173687615](https://github.com/metalogiclabs/nucleus/actions/runs/45943776855/artifacts/15173687615)
and the [new source manifest](../evidence/native32-computation-qualification-20261127.json).
These are bounded local results, not an updated official Arena score.
General indexed equality, recursion, eta, K and full Mathlib remain
outside the established new capability.


## Native indexed identity reconstruction (2026-11-27)

[Qualification 45963087833](https://github.com/metalogiclabs/nucleus/actions/runs/45963087833)
established ten new correct native ACCEPTs, including eqType,
eqRec, boolRecEqns and prodRecEqns. The current cohort has
92 ACCEPT, 51 REJECT and 213 UNKNOWN (356 inputs). The prior cohort
remains 63A/51R/242U. No protected decisive verdict was lost.
All twelve earlier native iota controls and four external fixtures
were preserved. Ten new controls yielded 1 ACCEPT, 3 REJECT and
6 UNKNOWN (including false Boolean equality); no false ACCEPT.

The indexed certificate reconstructs the entire identity family,
constructor, recursor telescope and rule body. It promotes only
opaque signatures, not indexed reduction or K authority. The
exact runtime manifest and original fixtures are retained.
Full indexed/recursive support and whole-Mathlib acceptance are
not established.


## Native typed K and indexed iota (2026-11-27)

[Qualification 45968074994](https://github.com/metalogiclabs/nucleus/actions/runs/45968074994)
compared three builds: opaque signatures, indexed constructor iota
without K, and typed K. The authentic ruleK proof changed 2/2/0;
ruleKbad stayed 2/2/2. A local concrete constructor separator
changed 2/0/0, while its wrong-index counterpart stayed 2/2/2.
The valid trace proves constructor/major-domain compatibility;
the negative trace refutes it without granting reduction.

Current native corpus 93A/51R/212U; earlier corpus 63A/51R/242U.
All previously decided rows and all 26 external/native controls
preserved. No false ACCEPT observed. This is an acceptance-only
bounded expansion for fully reconstructed identity families, not
general indexed or recursive verification. Partial recursor eta
and recursive dependencies still decline. Exact source and inputs
are retained in the qualification manifest and artifact.
# Typed partial K follow-up

[Qualification 45970879447](https://github.com/metalogiclabs/nucleus/actions/runs/45970879447)
extends exact PR #4 source `61baa9e5` with a typed eta opening of precisely
one missing major premise. Both functions' actual captured argument types,
complete function types and fresh-local support are checked before the
existing constructor/major-domain K compatibility check can execute.
The [separate manifest](../evidence/native32-partial-k-qualification-20261127.json)
retains the parent qualification unchanged.

The authentic `renovation/129_etaRuleK` changes UNKNOWN to ACCEPT in both
overlapping 356-input cohorts. Current: 94 ACCEPT / 51 REJECT / 211 UNKNOWN;
earlier: 64 / 51 / 241. All previously decisive verdicts are preserved.
The preceding full-K gain belongs to PR #4 and is not counted again.

Rust: 371 passed, 17 ignored; Python: 27 passed. Fourteen native controls
were compared with Lean at `cb633112`: this includes existing valid
unsupported cases and three new invalid index/argument controls, all of
which remain UNKNOWN in Nucleus. Source family reconstruction, flags,
launcher and prior manifests are unchanged. False Boolean equality,
neutral singleton structure eta and recursive inductives remain unresolved.

The [retained artifact 15185455830](https://github.com/metalogiclabs/nucleus/actions/runs/45970879447/artifacts/15185455830)
was independently SHA-256 and CRC verified (22 entries); its 25 source files
reconstruct source tree `1f0d18bf3ca366d357f76f72290916ee85d0617a`.
