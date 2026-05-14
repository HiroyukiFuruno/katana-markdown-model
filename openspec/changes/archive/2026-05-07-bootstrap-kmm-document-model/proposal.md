## Why

KatanAのMarkdown viewer、editor支援、exportは、現状では既存parserやrendererの都合に寄っている。

そのため、表（table/grid）、README badge、alert、description list、脚注、絵文字、diagram、math、metadata、PDFページング、LLM注釈を同じ文書仕様として扱いにくい。

KMMはMarkdownをHTMLへ変換する部品ではない。KMMは、KatanA ecosystemが共有するMarkdown文書モデルの正本を持つlibraryである。

この変更では、KMM repositoryを「別セッションが継続開発できる初期状態」まで立ち上げる。

## What Changes

### 初期構築で完了させること

- `katana-markdown-model` をRust library crateとして成立させる。
- CLIを提供せず、public APIをlibraryに限定する。
- `KmmDocument`、`KmmNode`、`KmmNodeKind`、`SourceSpan`、`MetadataDocument`、`MetadataTarget`、`TargetResolution` をpublic DTOとして定義する。
- 内部parser moduleを作るが、parser内部型をpublic contractへ出さない。
- source range、line-column、raw snippet、text fingerprintをnodeへ持たせる。
- heading、paragraph、HTML block、README badge row、list、code block、diagram role、table、blockquote、alert、description listを初期nodeとして扱う。
- metadataをMarkdown本文へ埋め込まず、外部metadata documentとして扱う。
- metadata targetをstable node idまたはfingerprintで再解決し、unresolvedを削除せず返す。
- `katana-ast-lint` をKMMのAST lint品質ゲートとして採用する。
- `just check`、`lefthook`、CI、repo-local skillを横展開し、開発開始できる状態にする。

### 初期構築では完了させないこと

- CommonMark完全準拠。
- parser engineの最終選定。
- full fixtureの完全モデル化。
- table cell単位のsource range。
- emoji shortcodeとUnicodeの専用node。
- release workflowとcrates.io公開。
- KDV、KLE、KCF、KatanAへの組み込み。

## Capabilities

### New Capabilities

- `kmm-document-model`: Markdown文書をrenderer-neutralなKMM所有DTOとして表現する。
- `kmm-source-mapping`: nodeごとにsource range、line-column、raw snippet、fingerprintを保持する。
- `kmm-metadata-target`: 外部metadata targetを文書modelへ解決する。
- `kmm-quality-gate`: KAL AST lint、Clippy、tests、OpenSpecをまとめて検証する。
- `kmm-handoff-ready`: 別セッションがOpenSpecから次作業を判断できる。

## Impact

- 新規crate構成を追加する。
- `.github/workflows/test-and-build.yml` と `release-preflight.yml` を追加する。
- `just check` をKMM標準品質ゲートにする。
- KAL `katana-ast-lint = "0.1.0"` をdev-dependencyとして使う。
- KMM public DTOが、後続のkdp、kle、kcf、KatanA統合の参照点になる。
