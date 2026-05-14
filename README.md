# katana-markdown-model

`katana-markdown-model`（KMM）は、KatanA ecosystem のMarkdown文書モデル、外部メタデータ（metadata）、位置解決を担うlibraryです。

KMMはHTML変換器ではありません。KatanAのviewer、編集機能（editor）、出力（export）が同じ文書解釈を共有するための中核です。

分離優先順位ではP1です。P0 `katana-ast-lint` の共通品質ゲートを前提にして進めます。

## 初期方針

- KMM v0はCommonMark完全準拠より、現在KatanAで実現できている挙動の踏襲を優先します。
- 主fixtureは `/Users/hiroyuki_furuno/works/private/katana/assets/fixtures/sample.md` です。
- README badge、alert、description listも必須fixtureにします。
- metadataはMarkdown本文へ埋め込まず、外部ファイルとして扱います。
- KMMはKCF、KDV、KatanA、editorへ依存しません。
- KMM固有の一時lintを作らず、共通AST lintを品質ゲートにします。

## 開発入口

```bash
just check
```

`just check` はformat、Clippy、KAL AST lint、test、OpenSpec検証を実行します。

## 最小利用例

```rust
use katana_markdown_model::{KatanaMarkdownModel, MarkdownInput};

let document = KatanaMarkdownModel::parse(MarkdownInput::from_content(
    "README.md",
    "# Title\n\nBody\n",
))?;
assert_eq!(document.nodes.len(), 2);
# Ok::<(), katana_markdown_model::KmmError>(())
```

## 現在の初期実装範囲

- renderer-neutralな `KmmDocument` / `KmmNode` / `KmmNodeKind`
- source range、line-column、raw snippet、text fingerprint
- heading、paragraph、HTML block、badge row、list、code block、diagram block、table、blockquote、alert、description list
- 外部metadata targetと再解決API
- `katana-ast-lint` を使ったrepository AST lint
