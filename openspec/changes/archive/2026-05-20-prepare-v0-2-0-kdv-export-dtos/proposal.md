## Why

KDV の HTML 書き出し（HTML export）で、KMM の公開データ型（public DTO）に出ていない Markdown 構造を正しく扱えないことが issue #1 で確定した。

KDV が raw Markdown を独自に再パースすると、Markdown 解釈の正本が KMM と KDV に分かれる。KMM は HTML を生成しないが、KDV が再パースなしで書き出せるだけの描画方式に依存しない構造（renderer-neutral structure）を返す必要がある。

この変更は公開データ型を広げるため、`v0.2.0` で扱う。

## What Changes

- inline span の公開データ型を追加する。
  - strong
  - emphasis
  - strikethrough
  - inline code
  - inline HTML
  - text
- link / image / footnote / inline math / dollar math block の公開データ型を追加する。
- list item の本文、marker、task marker、子 block を公開データ型で取得できるようにする。
- blockquote の子 block を公開データ型で取得できるようにする。
- KDV が raw Markdown 再パースなしで HTML を組み立てられることを fixture test で固定する。

## Impact

- `Cargo.toml`
- `src/document/**`
- `src/parser/**`
- `tests/**`
- `docs/release-notes/v0.2.0.md`
- `docs/release-readiness/v0.2.0-*.md`
