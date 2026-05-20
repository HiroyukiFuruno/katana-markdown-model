# Tasks: prepare-v0-2-0-kdv-export-dtos

## Definition of Ready

- [x] issue #1 が KMM の公開データ型（public DTO）不足として確定している
- [x] KDV は raw Markdown を独自に再パースしない
- [x] KMM は HTML 書き出し（HTML export）そのものを持たない
- [x] `v0.2.0` は最新 `v0.1.0` から見て不自然ではない

## Tasks: Implementation

- [x] 1.1 issue #1 の代表構文を KMM fixture test として追加する
- [x] 1.2 inline span の公開データ型を追加する
- [x] 1.3 link / image / footnote / inline math / dollar math block の公開データ型を追加する
- [x] 1.4 list item body / marker / task marker / nested block を公開データ型で返す
- [x] 1.5 blockquote children を公開データ型で返す
- [x] 1.6 KMM が third-party parser AST、HTML 生成、UI 制御を公開契約に含めないことを確認する
- [x] 1.7 `Cargo.toml` と release note を `v0.2.0` に更新する
- [x] 1.8 release readiness を記録する

## Definition of Done

- [x] KDV が raw Markdown 再パースなしに strong / emphasis / strikethrough / inline code / link / image / footnote / math / nested list / blockquote children を取得できる
- [x] 追加・変更挙動は fixture test で固定されている
- [x] KMM remains library-only
- [x] third-party parser ASTs do not leak into the public contract
- [x] KMM does not own rendering, export, scroll, selection, or highlight control
- [x] `scripts/openspec validate "prepare-v0-2-0-kdv-export-dtos" --strict` passes
- [x] `just check` passes
- [x] `just release-check` passes

## Verification

- [x] `scripts/openspec validate "prepare-v0-2-0-kdv-export-dtos" --strict`
- [x] `just check`
- [x] `just release-check`
