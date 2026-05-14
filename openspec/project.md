# katana-markdown-model OpenSpec

## Project

`katana-markdown-model`（KMM）は、KatanA ecosystem のMarkdown文書モデル、外部メタデータ（metadata）、位置解決を担うlibrary。

KMMはHTML変換器ではない。KatanA、katana-document-viewer、katana-language-editor、katana-canvas-forge が同じMarkdown解釈を共有するための中核である。

分離優先順位はP1。P0 `katana-ast-lint` による共通品質ゲートを前提にする。

## Design Principles

- KMMはKCF、KDV、KatanA、editorへ依存しない。
- KMM固有の一時AST lintを作らず、`katana-ast-lint` の共通ruleとadapterを使う。
- 既存Markdown libraryのASTをpublic contractにしない。
- Markdown本文へKatanA専用metadataを埋め込まない。
- 現在KatanAで実現できているMarkdown挙動をv0互換ラインにする。
- 絵文字は削除しない。Unicodeとshortcode情報を保持し、描画は利用側へ委譲する。
- Mermaid、draw.io、PlantUML、mathの描画はKCFへ委譲できる構造として保持し、KMMでは描画しない。
- editor-viewer同期制御はKatanAが所有する。KMMは同期制御を持たず、同期に使えるnode id、source range、line-column、raw snippet、fingerprintを返す。

## Canonical Fixtures

- `/Users/hiroyuki_furuno/works/private/katana/assets/fixtures/sample.md`
- `/Users/hiroyuki_furuno/works/private/katana/README.md` 冒頭のbadge列
- `/Users/hiroyuki_furuno/works/private/katana/assets/fixtures/sample_basic.md` のalert記法
- description list fixture

## Consumers

- KatanA
- katana-document-viewer
- katana-language-editor
- katana-canvas-forge
