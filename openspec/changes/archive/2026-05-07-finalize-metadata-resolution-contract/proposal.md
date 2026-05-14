## Why

KMMは外部metadataをMarkdown本文へ埋め込まない。

editor保存時にmetadata targetが移動、消失、衝突した場合、downstreamが独自状態を作らずに扱えるpublic contractが必要である。

## What Changes

- metadata解決結果に `Conflict` を追加する。
- editor保存時に必要なrequest/result DTOを固定する。
- unresolved metadataを削除しない契約をテストで固定する。
- PDFページング、LLM注釈、AST単位copy/editをmetadata用途としてfixture化する。

## Impact

- `src/metadata/types.rs`
- `src/metadata/resolver.rs`
- `src/lib.rs`
- `tests/metadata_resolution.rs`
- downstream handoff条件
