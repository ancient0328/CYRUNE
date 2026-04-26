# CYRUNE Free v0.1 Public Docs Language Envelope Main Update

**作成日時 (JST)**: 2026-04-26 09:36:20 JST  
**分類**: `証跡`  
**時間相**: `現在との差分を比較する段階`  
**Correlation ID**: `CYRUNE-PUBDOC-20260426-0936-JST`  
**Reason**: `PUBLIC_DOC_LANGUAGE_ENVELOPE`  
**対象**: `Distro/CYRUNE/public/free-v0.1/`  

## 1. Scope

この証跡は、GitHub public repository `ancient0328/CYRUNE` の `main` を latest public surface として扱い、`v0.1.0` immutable snapshot tag を移動せず、公開 docs の言語方針・claim boundary・source-side path 境界を整える作業を対象にする。

対象外は、runtime 機能追加、`v0.1.0` tag / release の移動、release asset の再作成、Pro / Enterprise / CITADEL scope の実装である。

## 2. Applied Changes

1. root `README.md` に `README.ja.md` と `docs/ja/` への Japanese companion routing を固定した。
2. `docs/CYRUNE_Free_Public_Index.md` を English primary public index として整理し、Japanese companion、source-side path boundary、main / SemVer tag publication model、non-authority scope を明示した。
3. `docs/USER_GUIDE.md` と `docs/ENGINEERING_SPEC.md` を GitHub repository root 起点の English primary docs にした。
4. Japanese companion docs を `docs/ja/` に集約した。
5. `docs/current/`、`docs/deferred/`、`docs/historical/`、`free/v0.1/dev-docs/summary/` に public repository reading caveat と signed update non-claim を維持した。
6. `free/v0.1/0/scripts/export_public_corpus.py` の generated README template を current README language / versioning wording に合わせた。

## 3. Verification Commands

以下を `/tmp/cyrune-publish-9ViAx3I6/CYRUNE` で実行した。

```bash
bash -n scripts/prepare-public-run.sh && bash -n scripts/doctor.sh && bash -n scripts/first-success.sh
python3 -m py_compile free/v0.1/0/scripts/export_public_corpus.py free/v0.1/0/scripts/publish_release_package_to_github.py
python3 - <<'PY'
from __future__ import annotations
import re
from pathlib import Path
root = Path('.')
files = [p for p in root.rglob('*.md') if '.git' not in p.parts and 'target' not in p.parts]
missing = []
pattern = re.compile(r'\[[^\]]+\]\(([^)]+)\)')
for path in files:
    text = path.read_text(encoding='utf-8')
    for m in pattern.finditer(text):
        raw = m.group(1).strip()
        if not raw or raw.startswith(('#', 'http://', 'https://', 'mailto:')):
            continue
        target = raw.split('#', 1)[0]
        if not target:
            continue
        if target.startswith('<') and target.endswith('>'):
            target = target[1:-1]
        candidate = (root / target.lstrip('/')) if target.startswith('/') else (path.parent / target)
        if not candidate.exists():
            missing.append(f'{path}:{m.start()+1}: {raw}')
if missing:
    print('\n'.join(missing))
    raise SystemExit(1)
print(f'checked {len(files)} markdown files; missing relative links: 0')
PY
git diff --check
cargo fmt --manifest-path free/v0.1/0/Cargo.toml --all -- --check
cargo check --manifest-path free/v0.1/0/Cargo.toml --workspace --all-targets
cargo clippy --manifest-path free/v0.1/0/Cargo.toml --workspace --all-targets -- -D warnings
cargo build --manifest-path free/v0.1/0/Cargo.toml --workspace --all-targets
```

## 4. Verification Results

1. Shell script parse: `pass`
2. Python compile: `pass`
3. Markdown relative link check: `checked 49 markdown files; missing relative links: 0`
4. Git diff whitespace check: `pass`
5. Rust format check: `pass`
6. Rust workspace check: `pass`
7. Rust clippy with warnings denied: `pass`
8. Rust workspace build: `pass`

Runtime first-success was not rerun in this report because this change did not modify Rust runtime code, shell runtime scripts, release carrier pins, or release asset contents. The first-success expected contract remains documented in `docs/FIRST_SUCCESS_EXPECTED.md`.

## 5. Closed Gate Report

### Gate 1. 個別事案固定性

**判定**: Strong Yes  
**理由**: 対象は `Distro/CYRUNE/public/free-v0.1/` の public docs envelope update に固定され、`v0.1.0` tag / release asset movement は対象外として明示されている。  
**直接根拠**: `README.md`, `docs/CYRUNE_Free_Public_Index.md`, this report section 1  
**崩れる条件**: `v0.1.0` tag / release を移動する作業、または `Distro/CYRUNE/public/free-v0.1/` 以外の product maturity review を同じ完了判定へ混入した場合。

### Gate 2. fail-closed

**判定**: Strong Yes  
**理由**: docs は non-claim boundary、source-side path boundary、supplementary docs caveat を明示し、古い dev-docs / deferred docs が public alpha claim を上書きしない閉じ方にしている。  
**直接根拠**: `docs/CYRUNE_Free_Public_Index.md`, `free/v0.1/dev-docs/summary/00-SUMMARY_INDEX.md`, `free/v0.1/dev-docs/summary/04-ROADMAP_AND_EXECUTION_HISTORY.md`, `free/v0.1/dev-docs/summary/07-CURRENT_STATE_AND_OPERATIONAL_GUIDE.md`  
**崩れる条件**: historical / deferred / summary docs が current public truth として読める導線を追加した場合。

### Gate 3. 根拠の接続と範囲

**判定**: Strong Yes  
**理由**: public claim boundary は README / Public Index / companion docs / summary caveat に同じ範囲で接続され、operational pins、SemVer tag、source-side path、runtime authority を混線させていない。  
**直接根拠**: `README.md`, `docs/CYRUNE_Free_Public_Index.md`, `docs/USER_GUIDE.md`, `docs/ENGINEERING_SPEC.md`  
**崩れる条件**: carrier URL / SHA256 や historical roadmap を product identity truth として扱う文言を追加した場合。

### Gate 4. 構造・責務・意味論整合

**判定**: Strong Yes  
**理由**: English primary entry path、Japanese companion shelf、current / deferred / historical shelves、dev-docs supplementary role が分離され、公開 reader の root は GitHub repository root に統一されている。  
**直接根拠**: `README.md`, `README.ja.md`, `docs/ja/README.md`, `docs/current/README.md`, `docs/deferred/README.md`, `docs/historical/README.md`  
**崩れる条件**: `docs/ja/` を authority surface として扱う文言、または public user path と source-side staging path を同一視する文言を追加した場合。

### Gate 5. 時間軸整合

**判定**: Strong Yes  
**理由**: `main` は latest public surface、`v0.1.0` は immutable snapshot tag、`v0.1` は compatibility tag として区別され、過去の publication branch wording は history / then-current として限定されている。  
**直接根拠**: `README.md`, `docs/CYRUNE_Free_Public_Index.md`, `free/v0.1/dev-docs/summary/04-ROADMAP_AND_EXECUTION_HISTORY.md`, `free/v0.1/dev-docs/summary/07-CURRENT_STATE_AND_OPERATIONAL_GUIDE.md`  
**崩れる条件**: `v0.1` branch を current publication model とする文言、または `v0.1.0` tag を latest mutable surface と扱う文言を追加した場合。

### Gate 6. 未証明採用の不在

**判定**: Strong Yes  
**理由**: first-success runtime capability、native distribution、OS sandbox enforcement、classification / MAC completion、signed update delivery は未実施 / 非対象のまま public alpha claim に採用していない。docs / script parse / compile / format / check / lint / build の検証結果だけを採用している。  
**直接根拠**: `README.md`, `docs/FIRST_SUCCESS_EXPECTED.md`, this report sections 3 and 4  
**崩れる条件**: 未実行の runtime first-success、release signing、installer、OS sandbox、classification / MAC enforcement を完了済み claim に含めた場合。

## 6. Result

`main` public docs update は、GitHub 公開リポジトリの latest surface に反映するための commit / push 対象として扱える。
