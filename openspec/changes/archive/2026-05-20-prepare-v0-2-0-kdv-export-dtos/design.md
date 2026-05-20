## Context

`v0.1.0` の KMM は block 単位の文書構造、source range、raw snippet、fingerprint、metadata resolution を公開した。

KDV の HTML 書き出し（HTML export）では、paragraph や list の raw text だけでは inline decoration、link、image、footnote、math、nested list、blockquote children を失う。KDV 側で補うと、KMM とは別の Markdown 解釈が必要になる。

## Goals

- KDV が raw Markdown を再パースせず HTML を生成できる公開データ型（public DTO）を追加する。
- KMM は描画方式に依存しない（renderer-neutral）構造だけを返す。
- 既存の `KmmDocument` / `KmmNode` の基本形を残し、追加構造は `kind` と `children` で表す。
- third-party parser AST を公開契約へ漏らさない。

## Non-Goals

- KMM 内で HTML / PDF / PNG / JPG を生成すること。
- KMM 内で viewer / editor の scroll、selection、highlight を制御すること。
- KDV、KatanA、KCF、KLE、UI framework へ依存すること。
- CommonMark / GFM の全仕様を一度に実装すること。

## Public DTO Direction

### Inline spans

Inline span は `KmmNode.children` に child node として入れる。

対象:

- `Text`
- `Strong`
- `Emphasis`
- `Strikethrough`
- `InlineCode`
- `InlineHtml`
- `Link`
- `Image`
- `FootnoteReference`
- `InlineMath`

各 child node は必ず `SourceSpan` を持つ。KDV は `source.raw.text` に依存して再パースせず、`KmmNodeKind` と専用 DTO の値を使う。

### Dollar math block

`$$ ... $$` は fenced code block ではないため、dedicated block node として扱う。

### List item

`ListNode` は list 全体の要約を維持しつつ、`ListItemNode` の配列を持つ。

`ListItemNode` は次を持つ。

- marker
- ordered number
- task marker
- inline children
- nested block children

Nested list や list 内 code block は `ListItemNode.children` に入れる。

### Blockquote

`BlockQuoteNode` は blockquote 内の子 block を持つ。alert は従来どおり専用 label を維持しつつ、本文 children を持てる形にする。

## Compatibility

`KmmDocument` と `KmmNode` の基本形は維持する。

ただし `KmmNodeKind` と一部 DTO は追加される。公開契約が広がるため、この変更は `v0.2.0` とする。

## Verification

- issue #1 の代表構文を repository-local fixture test にする。
- `scripts/openspec validate "prepare-v0-2-0-kdv-export-dtos" --strict`
- `just check`
- `just release-check`
