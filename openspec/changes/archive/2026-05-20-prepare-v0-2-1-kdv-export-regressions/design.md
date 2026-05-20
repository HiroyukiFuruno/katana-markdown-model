## Context

`v0.2.0` で KDV HTML 書き出し（HTML export）向け DTO を追加したが、KDV の RED tests が残り 2 種類の KMM-owned gap を示した。

KDV は raw Markdown を独自再パースしない方針であり、HTML output は KDV の責務だが、Markdown 解釈と renderer-neutral structure は KMM が返す必要がある。

## Goals

- 日本語を含む blockquote / alert nested list を panic なしに parse する。
- nested inline span を `KmmNode.children` で表現する。
- one-line dollar math block を multiline dollar math block と同じ DTO で表現する。
- KatanA の日本語 sample fixture は KMM repo 内へコピーした canonical fixture を使って検証する。
- KMM は HTML 生成や UI 制御を持たない。

## Non-Goals

- KMM 内で HTML を生成すること。
- CommonMark / GFM の全 nested inline 仕様を一度に実装すること。
- KDV、KatanA、KCF、KLE、UI framework に依存すること。

## Design Notes

### UTF-8 source spans in quote children

Quote 内 list を unquoted text として `ListBlockParser` に渡す場合、`SourceLine.start` は元行の `>` marker 位置ではなく、unquoted text の開始 byte offset に合わせる。

これにより list item body の end offset が日本語文字の途中を指さない。

### Nested inline children

`Strong` / `Emphasis` / `Strikethrough` node は従来どおり text を保持しつつ、inner content を `children` に持つ。

KDV は親 node の kind と child node list から HTML を組み立てられる。KMM は HTML tag を生成しない。

### One-line dollar math

行全体が `$$ ... $$` の形なら `DollarMathBlock` とする。

Inline の `$...$` は従来どおり `InlineMath` とする。

### Copied KatanA fixture

`/Users/hiroyuki_furuno/works/private/katana/assets/fixtures/sample.ja.md` は評価元だが、KMM の test は sibling repository に依存しない。

内容を `tests/fixtures/canonical/katana_sample_ja.md` へコピーし、`include_str!` で読み込む。

## Verification

- `cargo test --test kdv_export_v0_2_1_regressions --locked`
- `scripts/openspec validate "prepare-v0-2-1-kdv-export-regressions" --strict`
- `just check`
- `just release-check`
