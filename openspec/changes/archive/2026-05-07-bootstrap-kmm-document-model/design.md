## Context

KMM v0は、CommonMark完全準拠を先に狙わない。

優先するのは、現在KatanAで実現できているMarkdown挙動を文書モデルとして失わず、viewer、editor、exportが同じ解釈を共有できることである。

KMMはP1の分離対象であり、P0 `katana-ast-lint` の共通品質ゲートを前提にする。

## Goals

- KMMを単独repositoryのRust library crateとして成立させる。
- `sample.md`、README badge、alert、description listを初期fixtureとして扱う。
- 全nodeにstable id、source range、line-column、byte offset、raw snippet、fingerprintを持たせる。
- metadata targetをnodeへ解決し、resolved、moved、unresolvedを返す。
- Mermaid、draw.io、PlantUML、math、emojiを最終的に構造として保持できるDTO境界を作る。
- 既存parserのASTをpublic contractにしない。
- `katana-ast-lint` を品質ゲートとして使う。
- 別セッションがOpenSpecを読んで、次に何を実装するか判断できる状態にする。

## Non-Goals

- KMM v0でMarkdown全仕様を完全実装すること。
- HTML/PDF/PNG/JPG出力をKMMが直接担当すること。
- Floem、egui、KCF、KDV、KLE、KatanAへ依存すること。
- metadataをMarkdown本文へ埋め込むこと。
- CLIを提供すること。

## Current Bootstrap State

初期構築では、以下の実装を入れる。

- crate: `katana-markdown-model`
- library name: `katana_markdown_model`
- public entry point:
  - `KatanaMarkdownModel::parse(MarkdownInput) -> Result<KmmDocument, KmmError>`
  - `KatanaMarkdownModel::reconcile_targets(old, new, metadata) -> Vec<TargetResolution>`
- module boundary:
  - `document`: public document DTO
  - `source`: source mapping DTO
  - `metadata`: external metadata DTO and resolver
  - `parser`: internal parser implementation
  - `input`: flexible input entry
  - `error`: public error type
- quality:
  - `just check`
  - KAL AST lint via `tests/repository_ast_lint.rs`
  - `lefthook`
  - CI matrix
  - release preflight job

## Public Contract

KMM public DTOはKMMが所有する。第三者parserのAST型を公開しない。

### Document DTO

- `KmmDocument`
  - `path`
  - `fingerprint`
  - `nodes`
- `KmmNode`
  - `id`
  - `kind`
  - `source`
  - `children`
- `KmmNodeKind`
  - `Heading`
  - `Paragraph`
  - `HtmlBlock`
  - `List`
  - `CodeBlock`
  - `Table`
  - `BlockQuote`
  - `Alert`
  - `DescriptionList`
  - `ThematicBreak`
  - `RawBlock`

### Source DTO

- `ByteRange`
- `LineColumn`
- `LineColumnRange`
- `RawSnippet`
- `TextFingerprint`
- `SourceSpan`

### Metadata DTO

- `MetadataDocument`
- `MetadataEntry`
- `MetadataTarget`
- `ContextAnchor`
- `TargetResolution`
- `TargetResolutionKind`
- `UnresolvedTarget`

## Parser Boundary

初期parserは最終仕様ではない。

初期parserの責務は、public DTOとsource mappingの契約を固定し、fixture testを通せる最小の構造化を行うことである。

後続でparser engineを差し替えても、public DTOとtest contractを壊してはならない。

## Fixture Strategy

初期構築では軽量fixtureを `tests/fixtures/` に置く。

後続では、以下をcanonical fixtureとして固定する。

- `katana/assets/fixtures/sample.md`
- `katana/README.md` 冒頭のbadge列
- `katana/assets/fixtures/sample_basic.md` のalert記法
- description list fixture

canonical fixtureは、コピー、submodule、sync scriptのいずれかで再現可能にする。絶対パスに依存したtestをrepository標準にしない。

## Metadata Resolution

metadataはMarkdown本文へ埋め込まない。

targetは以下を持つ。

- file path
- node id
- byte range
- line-column range
- text fingerprint
- before / after context

KMMは、旧document、新document、metadataを受け取り、以下を返す。

- `Resolved`: node idで直接解決できた
- `Moved`: node idは変わったがfingerprintで再対応できた
- `Unresolved`: 再対応できなかった。metadata entryは削除しない

## Quality Gate

`just check` を標準入口にする。

`just check` は以下を実行する。

- rustfmt check
- Clippy
- KAL AST lint
- unit / integration tests
- OpenSpec strict validation

GitHub branch protectionで要求するcheckはKML相当にする。

- `Test and Build (macos-latest)`
- `Test and Build (ubuntu-latest)`
- `Test and Build (windows-latest)`
- `preflight`

## Next Session Contract

次セッションは、最初に以下を読む。

1. `openspec/changes/bootstrap-kmm-document-model/handoff.md`
2. `openspec/changes/bootstrap-kmm-document-model/tasks.md`
3. `openspec/changes/bootstrap-kmm-document-model/specs/kmm-document-model/spec.md`
4. `docs/quality-gates.md`

次セッションの最初の実装候補は以下。

1. table/gridのcell単位source range
2. emoji shortcodeとUnicode保持の専用node
3. canonical fixture同期
4. release workflow

## Risks

- OpenSpecが薄いと、別セッションがparser精度、metadata、品質ゲートのどれを進めるべきか判断できない。
- parser内部型を公開すると、後続でparser差し替えができなくなる。
- fixtureを軽量fixtureのまま正本扱いすると、KatanA現行挙動の踏襲を検証できない。
- metadataをnode idだけで解決すると、保存後の移動に弱い。
- KAL AST lintを外すと、repository分離後に品質基準が割れる。
