<p align="center">
  <img src="assets/kmm-icon.png" width="128" alt="katana-markdown-model icon">
</p>

<h1 align="center">katana-markdown-model</h1>

<p align="center">
  KatanA系列（ecosystem）でMarkdown文書モデル、外部メタデータ、位置解決を共有するための描画方式に依存しないライブラリ（renderer-neutral library）です。
</p>

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License: MIT"></a>
  <a href="https://github.com/HiroyukiFuruno/katana-markdown-model/actions/workflows/test-and-build.yml"><img src="https://github.com/HiroyukiFuruno/katana-markdown-model/actions/workflows/test-and-build.yml/badge.svg" alt="CI"></a>
  <a href="https://github.com/HiroyukiFuruno/katana-markdown-model/releases/latest"><img src="https://img.shields.io/github/v/release/HiroyukiFuruno/katana-markdown-model" alt="Latest Release"></a>
  <a href="https://crates.io/crates/katana-markdown-model"><img src="https://img.shields.io/crates/v/katana-markdown-model.svg" alt="crates.io"></a>
</p>

`katana-markdown-model`（KMM）は、KatanA系列（ecosystem）のMarkdown文書モデル、外部メタデータ（metadata）、位置解決を担うライブラリ（library）です。

KMMはHTML変換器ではありません。KatanAの表示機能（viewer）、編集機能（editor）、出力機能（export）が同じ文書解釈を共有するための中核です。

分離優先順位ではP1です。P0 `katana-ast-lint` の共通品質ゲートを前提にして進めます。

## 初期方針

- KMM v0はMarkdown共通仕様（CommonMark）の完全準拠より、現在KatanAで実現できている挙動の踏襲を優先します。
- 主な検証用入力（fixture）は `/Users/hiroyuki_furuno/works/private/katana/assets/fixtures/sample.md` です。
- READMEバッジ（badge）、注意枠（alert）、説明リスト（description list）も必須の検証用入力（fixture）にします。
- メタデータ（metadata）はMarkdown本文へ埋め込まず、外部ファイルとして扱います。
- KMMはKCF、KDV、KatanA、editorへ依存しません。
- KMM固有の一時lintを作らず、共通AST lintを品質ゲートにします。

## 開発入口

```bash
just check
```

`just check` は整形確認（format）、Clippy、KAL AST lint、テスト（test）、OpenSpec検証を実行します。

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

- 描画方式に依存しない（renderer-neutral） `KmmDocument` / `KmmNode` / `KmmNodeKind`
- 位置範囲（source range）、行列位置（line-column）、元テキスト断片（raw snippet）、テキスト指紋（text fingerprint）
- 見出し（heading）、段落（paragraph）、HTMLブロック（HTML block）、バッジ行（badge row）、リスト（list）、コードブロック（code block）、図表ブロック（diagram block）、表（table）、引用（blockquote）、注意枠（alert）、説明リスト（description list）
- 外部メタデータ対象（metadata target）と再解決API
- `katana-ast-lint` を使ったrepository AST lint
