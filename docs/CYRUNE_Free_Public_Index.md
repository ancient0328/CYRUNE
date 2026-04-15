# CYRUNE Free v0.1 Public Index

**状態**: Current accepted public entry  
**主語**: CYRUNE Free v0.1 public-only corpus の入口  
**目的**: 第三者が internal operational docs や historical docs を読まずに、current accepted public truth だけを辿れるようにする

---

## 1. この文書の役割

この文書は、CYRUNE Free v0.1 の **public-only corpus** を読むための入口である。  
ここで案内するのは current accepted public truth だけであり、task-level roadmap、gate operation、raw proof payload、historical / draft 文書は入口に含めない。

この文書自体は product overview ではなく、**public-safe authority list** である。

## 2. 最初に読む順序

1. `CYRUNE.md`
2. `CYRUNE_ProblemStatement-ja.md` または `CYRUNE_ProblemStatement-En.md`
3. `CYRUNE-Free_Canonical.md`
4. `free/v0.1/dev-docs/summary/00-SUMMARY_INDEX.md`
5. `free/v0.1/dev-docs/summary/01-SYSTEM_AND_SCOPE.md`
6. `free/v0.1/dev-docs/summary/02-ARCHITECTURE_AND_RUNTIME_LINES.md`
7. `free/v0.1/dev-docs/summary/03-CANONICAL_CONTRACTS_AND_DATA_MODELS.md`
8. `free/v0.1/dev-docs/summary/07-CURRENT_STATE_AND_OPERATIONAL_GUIDE.md`

## 3. 補助的に読む文書

必要に応じて次を読む。

1. `free/v0.1/dev-docs/00-TARGET_SYSTEM.md`
2. `free/v0.1/dev-docs/03-architecture/ARCHITECTURE_OVERVIEW.md`
3. `CYRUNE_Free_v0.1_Diagrams.html`
4. `docs/mermaid/` 配下の diagram source
5. `docs/ENGINEERING_SPEC.md`
6. `docs/USER_GUIDE.md`
7. `free/v0.1/dev-docs/90-reports/20260410-terminal-D6-native-outer-launcher-proof.md`
8. `free/v0.1/dev-docs/90-reports/20260411-terminal-D7-terminal-bundle-productization-proof.md`
9. `free/v0.1/dev-docs/90-reports/20260412-terminal-EVID-D7RC1D-1-external-release-concretization-closeout.md`

上記 7-9 は supporting file list であり、current public authority surface の direct link set ではない。

## 4. この入口で authority として扱う truth

この入口で authority として扱ってよいのは次である。

1. CYRUNE Free v0.1 current accepted product truth
2. `cyr` single-entry
3. `BUNDLE_ROOT` single authority
4. `CYRUNE_HOME` non-authority
5. fail-closed family の存在
6. `D5`、`D6`、`D7`、`D7-RC1` complete
7. `current accepted next executable scope = none`
8. concrete release value は residual detail であり current accepted public claim に採用していないこと

## 5. この入口で authority として扱ってはいけないもの

次は current accepted public truth の authority として扱ってはならない。

1. task-level roadmap
2. current inventory の全量
3. exact manifest
4. raw proof / raw validation output
5. organization-owned variable handling detail
6. historical / draft / superseded 文書
7. concrete reverse-DNS bundle identifier
8. concrete installer / archive filename
9. concrete upstream revision
10. concrete signing identity value
11. concrete notarization provider value

## 6. historical / non-authoritative 文書の扱い

次は current accepted public truth の入口に含めない。

1. `CYRUNE_Terminal.md`
2. `CYRUNE_Terminal_CanonicalDraft.md`
3. `CYRUNE_Free_v0.1_StructurePack.md`
4. `CYRUNE構図.md`
5. `CITADEL_CYRUNE.md`

これらは historical / non-authoritative corpus であり、背景や初期構想の参照に限定する。

## 7. deferred-publication 文書の扱い

次は current Free v0.1 public truth に自動採用しない。

1. `CYRUNE_ProductTierCanonical.md`
2. `CYRUNE-Pro_Canonical.md`
3. `CYRUNE-Pro+_Canonical.md`
4. `CYRUNE-Enterprise_Canonical.md`
5. `CITADEL.md`
6. `CITADEL_ThreatModel.md`
7. `CYRUNE_TierReusableAssetInventory.md`
8. `CRANE_3層メモリ.md`

これらは historical ではないが、別の publication decision が必要である。

## 8. 現時点の一文結論

CYRUNE Free v0.1 public corpus は、standalone summary と public-safe final closeout family によって、第三者が internal operational docs を読まずに current accepted product truth を理解できる状態にある。  
public-ready logical closeout は完了している。  
ただしこれは public boundary / public corpus 側の logical closeout であり、Free v0.1 ship goal complete を意味しない。  
physical export / repo split / concrete residual value の fixed value 化は別 scope であり、まだ完了していない。
