# Public Beta Terminal Evidence Release-Contract Closed Gate Report

## 1. 判定メタ情報

| Item | Value |
|---|---|
| 判定日時 | 2026-05-01 01:34:42 JST |
| 対象 | CYRUNE Free v0.1 public beta terminal evidence and release-contract verifier evidence |
| 対象フェーズ / タスク | Evidence Phase E1-E6 |
| 時間相固定 | implementation / harness / test 後、public CI shipping-home binding fix 後の final active local candidate に対する evidence closeout |
| 判定スコープ | final active local candidate root `20260501T012515+0900` の E1-E5 evidence |
| build / test / runtime verification | 実施済み: public-run prepare, doctor, first-success semantic verifier, release verifier, cargo fmt/test/check/clippy/build, Python unittest |

## 2. 今回の目的

今回成立させる対象は、public beta claim を機械的に支える terminal evidence artifact chain です。具体的には、final active local candidate に対して、first-success が wrapper exit 0 ではなく semantic verifier report として `verified: true` / `outcome: accepted` / evidence files / terminal binding / working hash / candidate-root binding へ接続され、release verifier が同じ candidate root と同じ first-success report を検査できることを対象にします。

今回まだ成立させない対象は、既存 immutable release asset の bytes が今回の final local candidate bytes と一致すること、既存 `v0.1.1-beta.1` asset が今回の E5 STOP correction または public CI shipping-home binding fix を含むこと、native installer、Pro / Enterprise / CITADEL scope、production-ready claim です。

## 3. 未完了だが正常なもの

| 未完了項目 | 正常とみなす理由 | Owner | 本文採用 |
|---|---|---|---|
| final local candidate と既存 immutable asset の byte-equivalence proof | 今回の E6 は final local candidate evidence と remote identity tuple / first-success root binding の検証であり、既公開 asset bytes の再発行や同一性主張を対象にしていない | future publication identity / release task | 採用しない |
| final commit/push 後の新 remote main SHA / CI run adoption | final commit/push は E6 後の工程であり、今回の E6 は push 後 remote state を先取りしない | post-E finalization task | 採用しない |

## 4. 初回 findings と補正

| Finding | 補正 | 補正後の残存 |
|---|---|---|
| `first-success.sh` が `cyr run` wrapperであり accepted-run semantic verifier ではなかった | `cyr verify first-success` を実装し、public wrapper が verifier report を所定 path に保存するよう変更 | 残存なし |
| accepted / rejected response payload が field-set 推測であり明示 outcome discriminator を持たなかった | `RunAccepted` / `RunRejected` に `outcome` を追加し、schema tests を追加 | 残存なし |
| accepted evidence commit と visible working projection terminal state の不整合余地があった | accepted finalization を terminal binding / working hash 再計算 / failure-code paths へ再設計し、post-commit failure tests を追加 | 残存なし |
| release verifier が first-success report を candidate root に再束縛しない余地があった | release verifier に `state_root` / `cyrune_home` / evidence hash / terminal binding / working hash root binding checks を追加 | 残存なし |
| E5 で public package candidate cargo test が source-only embedding artifact assumption により失敗した | test-only helper を primary source fixture, NotFound-only fallback to absolute `CYRUNE_TEST_SHIPPING_HOME_ROOT` に補正し、新 candidate で E1-E5 を再実行 | 残存なし |
| public CI `Check Rust tests` が generated shipping home を明示束縛していなかった | public CI に `CYRUNE_TEST_SHIPPING_HOME_ROOT: ${{ github.workspace }}/free/v0.1/0/target/public-run/home` を追加し、public Python test を 20 件へ更新して assertion を追加した。修正後に final candidate `20260501T012515+0900` を作り直し、E1-E5 を再実行 | 残存なし |

## 5. Final Candidate Evidence

| Item | Value |
|---|---|
| final active candidate root | `/Users/ancient0328/Development/GitHub/CRANE-project/Distro/CYRUNE/target/evidence-public-free-v0.1-candidate-20260501T012515+0900` |
| superseded candidate roots | `20260501T004527+0900`, `20260501T010226+0900` |
| final candidate reset reason | public CI shipping-home binding fix after the previous E6 report |
| first-success outcome | `accepted` |
| first-success verified | `true` |
| first-success correlation_id | `RUN-20260430-3713` |
| first-success run_id | `RUN-20260430-3713-R01` |
| evidence_id | `EVID-1` |
| policy_pack_id | `cyrune-free-default` |
| citation_bundle_id | `CB-20260430-3713` |
| working_hash_after | `sha256:3191f32dbcec8f883b540fe953bd8bfd7a8895749a932569fbb159b4fbb7d887` |
| working_json_hash | `sha256:3191f32dbcec8f883b540fe953bd8bfd7a8895749a932569fbb159b4fbb7d887` |
| release verifier verified | `true` |
| release verifier first_success_root_binding | `candidate_root` |
| release verifier repository | `ancient0328/CYRUNE` |
| release verifier tag | `v0.1.1-beta.1` |
| release verifier source_sha | `062cd58548e9f66e2371f580edae8f641d0d05f7` |
| release verifier tag_target | `61eb4c68630600d9b1a7f325fd6d06759ede846c` |
| release verifier release_id | `313683966` |
| release verifier asset_id | `405673798` |
| release verifier asset_digest | `sha256:73654922f0f1c170ce34001d6f1021b72ec9eb8c28aa8a81a3d572ccde00c938` |
| release verifier ci_run_id | `24947529643` |
| public Python tests | `Ran 20 tests`, `OK` |
| Rust source checks | `cargo fmt`, `cargo test`, `cargo clippy -D warnings`, `cargo build`: exit `0` |
| Public embedded checks | Python unittest, `cargo fmt`, `cargo check`, `cargo clippy -D warnings`, `cargo build`, shipping-home-bound `cargo test`: exit `0` |

## 6. 6 Gate 判定

### Gate 1. 個別事案固定性

| Item | Value |
|---|---|
| 判定 | Strong Yes |
| 判定理由 | 判定対象を final active local candidate root `20260501T012515+0900` と E1-E5 evidence に固定している。旧 candidate roots `20260501T004527+0900` と `20260501T010226+0900` は historical-only とし、current evidence へ採用していない。 |
| 直接根拠 | final candidate E1-E5 output, public CI shipping-home binding fix record, evidence reset after public CI binding fix |
| この判定が崩れる条件 | 旧 candidate output を current evidence として採用する、または今回対象を既存 immutable asset byte identity / production-ready claim へ拡張する |

### Gate 2. fail-closed

| Item | Value |
|---|---|
| 判定 | Strong Yes |
| 判定理由 | first-success verifier は accepted outcome / ids / policy pack / evidence hashes / terminal binding / working hash を満たさない場合に failure code を返す。release verifier は mutable input, missing args, GitHub surface mismatch, first-success root mismatch を closed failure code で reject する。E5 STOP correction の fallback は primary source `NotFound` の場合だけで、env unset / empty / relative / fallback read failure / primary non-NotFound read error は panic で失敗する。public CI Rust test step も generated shipping home を env で束縛する。 |
| 直接根拠 | final first-success verifier output, final release verifier output, test-only helper correction, public CI binding fix, final E5 command results |
| この判定が崩れる条件 | `first-success.sh` が `cyr run` wrapperへ戻る、release verifier が candidate root mismatch を許容する、test helper が permission/corruption等を fallback で吸収する、CI Rust test step が generated shipping home binding を失う、または failed command output を成功 evidence として採用する |

### Gate 3. 根拠の接続と範囲

| Item | Value |
|---|---|
| 判定 | Strong Yes |
| 判定理由 | runtime evidence は final candidate root の E2 `target/public-run` で生成された `first-success-report.json` に接続され、release verifier は同じ candidate root と同じ report path を入力にしている。remote identity tuple は GitHub release/tag/asset/CI surface の検査であり、local candidate bytes と既公開 asset bytes の同一性証明としては採用していない。 |
| 直接根拠 | final candidate E2 first-success report, final candidate E3 release verifier output, final candidate E5 output |
| この判定が崩れる条件 | final local candidate を既公開 asset bytes と同一と記述する、旧 `.memory/613` / `.memory/618` / `.memory/619` を current candidate evidence として採用する、または report path を別 root からコピーする |

### Gate 4. 構造・責務・意味論整合

| Item | Value |
|---|---|
| 判定 | Strong Yes |
| 判定理由 | accepted-run response, accepted evidence, visible working projection, terminal binding, public wrapper, release verifier, CI harness, and tests are separated by responsibility. Runtime semantic validation is performed by `cyr verify first-success`; release-contract validation is performed by the release verifier; E5 test helper correction stays inside test-only module and does not alter production runtime semantics; CI shipping-home binding is expressed in harness and asserted by public Python test. |
| 直接根拠 | outcome discriminator tests, terminal binding tests, working projection binding tests, first-success verifier tests, release verifier tests, public CI workflow test, final runtime and release verifier output |
| この判定が崩れる条件 | test-only helper logic is moved into production runtime path, public wrapper stops using the semantic verifier, release verifier starts accepting reports not rooted in the candidate root, or CI test binding becomes implicit again |

### Gate 5. 時間軸整合

| Item | Value |
|---|---|
| 判定 | Strong Yes |
| 判定理由 | E5 STOP correction 後 candidate と public CI binding fix 後 final candidate を分離し、public CI binding fix 後の source/public package state から final candidate を作り直して E1-E5 を再実行している。今回の E6 は final local candidate evidence として閉じ、final commit/push 後 remote main / CI run / future release identity を先取りしない。 |
| 直接根拠 | old candidate historical-only note, public CI binding fix record, evidence reset after public CI binding fix, final candidate E1-E5 output |
| この判定が崩れる条件 | public CI binding fix 前の candidate output を fix 後 evidence と混ぜる、final push 後の remote SHA/CI run を未観測のまま採用する、または既存 release asset が今回の patch を含むと記述する |

### Gate 6. 未証明採用の不在

| Item | Value |
|---|---|
| 判定 | Strong Yes |
| 判定理由 | 未証明の asset byte equivalence、未実行の post-push CI、native installer、production-ready scope は根拠に採用していない。採用した根拠は final candidate E1-E5 の実行結果、current code, current public wrapper / verifier / tests, and public CI binding assertion に限定している。 |
| 直接根拠 | final candidate E1-E5 records, public CI binding fix record, evidence reset after public CI binding fix |
| この判定が崩れる条件 | 未実行の remote post-push CI や unverified asset bytes を current closeout 根拠に入れる、または superseded candidate output を final evidence として採用する |

## 7. 総括

| Item | Value |
|---|---|
| `No` の有無 | なし |
| `Provisional Yes` の有無 | なし |
| 最終結論 | final active local candidate `20260501T012515+0900` に対する public beta terminal evidence / first-success semantic verifier / release-contract verifier evidence package is closed for the scoped E1-E6 evidence phase |
| 次に進むべき owner / task | post-E finalization: public embedded report sync, external public repository sync, existing-test/reference drift check, commit/push, and post-push publication identity follow-up if required |

This report does not claim that the already-published immutable `cyrune-free-v0.1.1-beta.1.tar.gz` asset contains the E5 STOP correction or the public CI shipping-home binding fix. It also does not claim production-ready status or native installer readiness.

## Reference Map

| Label | Path |
|---|---|
| Final candidate E1-E5 output | `.memory/629-20260501-final-candidate-evidence-output.md` |
| Evidence reset after public CI binding fix | `.memory/628-20260501-evidence-reset-after-public-ci-binding-fix.md` |
| Public CI shipping-home binding fix | `.memory/627-20260501-public-ci-shipping-home-test-binding-fix.md` |
| E5 STOP root cause | `.memory/614-20260501-e5-candidate-test-stop-root-cause.md` |
| E5 correction edit inventory | `.memory/616-20260501-e5-stop-correction-edit-inventory.md` |
| Closed Gate format | `/Users/ancient0328/Development/GitHub/CRANE-project/CLOSED_GATE_REPORTING_FORMAT.md` |
| Source test helper | `free/v0.1/0/crates/cyrune-daemon/src/command.rs` |
| Public embedded test helper | `public/free-v0.1/free/v0.1/0/crates/cyrune-daemon/src/command.rs` |
| First-success wrapper | `public/free-v0.1/scripts/first-success.sh` |
| Release verifier | `public/free-v0.1/scripts/verify-beta-release-contract.py` |
| Public CI harness | `public/free-v0.1/.github/workflows/public-ci.yml` |
