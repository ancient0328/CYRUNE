# ENGINEERING_SPEC

This document explains the engineering-facing structure and execution contract of the CYRUNE Free v0.1 public repository.

It does not replace the public authority surface. Start with the root `README.md`, `docs/GETTING_STARTED.md`, and `docs/CYRUNE_Free_Public_Index.md`.

Japanese companion material is available at `docs/ja/ENGINEERING_SPEC.md`.

## 1. Scope

This document covers the repository root contents and the package-level implementation contract for:

- `README.md`
- `docs/`
- `scripts/`
- `free/v0.1/0/`

It is meant to answer:

1. what physical surfaces are published,
2. what root each public script uses,
3. what state is generated under `target/public-run/`,
4. what changes can break public-run behavior,
5. what predicates should be checked during maintenance,
6. what runtime / product scope this public alpha does not claim.

This document does not redefine current public truth, task roadmaps, native distribution, OS-level sandbox enforcement, completed classification / MAC, upper-tier features, signing, notarization, or installer distribution.

## 2. Public Surfaces

The public repository has four surfaces:

1. discovery / authority surface:
   `README.md`, `docs/GETTING_STARTED.md`, `docs/FIRST_SUCCESS_EXPECTED.md`, `docs/TROUBLESHOOTING.md`, `docs/CYRUNE_Free_Public_Index.md`
2. current public docs:
   `docs/current/` plus the guide and engineering docs under `docs/`
3. separated reference shelves:
   `docs/historical/`, `docs/deferred/`, `docs/ja/`
4. runnable source tree:
   `free/v0.1/0/`

## 3. Repository Publication Model

GitHub publication uses `main` as the latest public repository surface and immutable SemVer tags as snapshots.

For CYRUNE Free v0.1 public alpha:

- `main` points to the latest public surface.
- `v0.1.0` is the published immutable snapshot tag for the Free v0.1 public alpha.
- Existing `v0.1` is a version marker / compatibility tag.
- A `v0.1` branch is not used.
- Future v0.1 maintenance, if needed, must use a non-conflicting branch name such as `release/v0.1`.

## 4. Topology

Top-level layout:

- `README.md`
- `README.ja.md`
- `docs/`
- `scripts/`
- `free/`

`docs/current/` contains current public truth references.
`docs/deferred/` contains future-publication or upper-tier material that is not adopted into Free v0.1 alpha claims.
`docs/historical/` contains non-authoritative historical material.
`docs/ja/` contains Japanese companion documents.

`free/v0.1/0/` contains the runnable source tree.

Important implementation families:

1. `crates/cyrune-runtime-cli/`: `cyr` command family and user-facing runtime surface.
2. `crates/cyrune-daemon/`: daemon / host execution surface.
3. `crates/cyrune-control-plane/`: request validation, Working rebuild, policy gate, citation validation, and ledger commit.
4. `crates/cyrune-core-contract/`: request / result / denial / ID contract types.
5. `resources/bundle-root/embedding/`: shipping embedding pin and static payload references.

## 5. Script Root Chain

All public scripts are called from the repository root:

```bash
./scripts/prepare-public-run.sh
./scripts/doctor.sh
./scripts/first-success.sh
```

They derive:

1. `SCRIPT_DIR`: `scripts/`
2. `PUBLIC_ROOT`: repository root
3. `FREE_ROOT`: `free/v0.1/0`
4. `STATE_ROOT`: `free/v0.1/0/target/public-run`
5. `CYRUNE_HOME`: `free/v0.1/0/target/public-run/home`

## 6. prepare-public-run Contract

`scripts/prepare-public-run.sh` must:

1. recreate `target/public-run/`,
2. download the configured release carrier,
3. verify filename, size, and SHA256,
4. reject unsafe tar members,
5. require the expected carrier manifest,
6. extract the home template,
7. build `cyrune-runtime-cli` and `cyrune-daemon`,
8. install `cyr` and `cyrune-daemon` under `target/public-run/bin/`.

The concrete carrier URL / filename / size / SHA256 values are operational pins. They are not product identity authority.

## 7. doctor Contract

`scripts/doctor.sh` must run only against prepared public-run state.

Expected success:

- exit code `0`
- JSON output
- `"status": "healthy"`

If public-run state is missing or invalid, it must fail instead of constructing hidden fallback state.

## 8. first-success Contract

`scripts/first-success.sh` must run the no-LLM first-success request through the prepared `cyr` binary.

Expected success:

- exit code `0`
- JSON output
- `policy_pack_id` is `cyrune-free-default`
- an `evidence_id` is returned
- evidence files exist under `CYRUNE_HOME/ledger/evidence/<evidence_id>/`
- `CYRUNE_HOME/working/working.json` exists

## 9. Change Impact Map

Changes to the following affect public-run behavior directly:

- root resolution in the public scripts
- carrier URL / size / SHA256 pins
- tar member safety validation
- binary names and installation paths
- `CYRUNE_HOME` layout
- `cyr run --no-llm` output contract
- evidence ledger paths
- `working/working.json`

Changes to the following affect public-reader interpretation:

- root README claim boundary
- public index reading order
- current / deferred / historical shelf placement
- Japanese companion routing
- release/tag wording

## 10. Non-Claims

This public alpha does not claim:

- native distributable release
- installer packaging
- concrete signing / notarization values
- OS-level sandbox enforcement
- enforcement-complete classification / MAC lattice
- Pro / Pro+ / Enterprise / CITADEL feature surface

## 11. Validation

The public CI checks:

- public shell scripts parse,
- Rust formatting,
- Rust workspace check,
- Rust lint with warnings denied.

Runtime first-success validation is documented in `docs/FIRST_SUCCESS_EXPECTED.md` and produces local evidence under `free/v0.1/0/target/public-run/home/`.
