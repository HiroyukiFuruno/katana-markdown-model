# Downstream Handoff Contract

## 結論

KMM `v0.1.0` のdownstream受け渡し境界は、KMM public DTOとmetadata APIだけで固定する。

downstreamは `src/parser/**`、third-party parser AST、KMM内部実装型へ依存しない。

KMMは描画、export、editor-viewer同期制御を持たない。同期制御はKatanAが担い、KatanAがviewerやeditorへ命令する。

## Public API境界

downstreamが使ってよい入口は次に限定する。

- `KatanaMarkdownModel::parse(MarkdownInput) -> Result<KmmDocument, KmmError>`
- `KatanaMarkdownModel::reconcile(MetadataReconcileRequest) -> MetadataReconcileResult`
- `KatanaMarkdownModel::reconcile_targets(&KmmDocument, &KmmDocument, &MetadataDocument) -> Vec<TargetResolution>`

downstreamが参照してよいDTOは次に限定する。

- `KmmDocument`
- `KmmNode`
- `KmmNodeKind`
- `SourceSpan`
- `RawSnippet`
- `TextFingerprint`
- `MetadataDocument`
- `MetadataEntry`
- `MetadataTarget`
- `MetadataReconcileRequest`
- `MetadataReconcileResult`
- `TargetResolution`
- `TargetResolutionKind`

## KDV

KDVは `katana-document-viewer` の略称である。既存 `katana-document-preview` は未リリース・未取り込みのため、計画上はKDVへ改名する。

KDVは `KmmDocument` をviewer inputとして受け取り、Markdown viewer、hit-test、node選択、HTML/PDF/PNG/JPG exportを担う。

hit-testやexport metadataは、KMM nodeから次を参照する。

- `KmmNodeId`
- `KmmNodeKind`
- `SourceSpan.byte_range`
- `SourceSpan.line_column_range`
- `SourceSpan.raw`
- `SourceSpan.raw.fingerprint()`

KDVはMarkdownを再parseしない。HTML変換結果をKMMの代替contractにしない。

viewer表示とexportは、KDV内の同じrender pipelineを使う。

## kle

kleは保存時に、old document、new document、metadata documentをKMMへ渡す。

保存時の標準入口は `KatanaMarkdownModel::reconcile(MetadataReconcileRequest)` とする。

kleは `TargetResolutionKind` を次の状態として扱う。

- `Resolved`: targetは同じnodeに残っている
- `Moved`: fingerprintで移動先nodeを特定できた
- `Unresolved`: targetを復元できない
- `Conflict`: fingerprint候補が複数あり、KMMは1つに決めない

kleは `Unresolved` と `Conflict` のmetadata entryを削除しない。

## KUWまたはwidget境界

KUWが存在する場合、metadata表示UIはKUW側に置く。

KUWが未作成の場合でも、KatanAやkdpへ暗黙に共通widget責務を増やさない。代替する場合は、OpenSpecで明示的なwidget境界を作る。

metadata表示に渡す情報は次に限定する。

- `MetadataEntry.key`
- `MetadataEntry.payload`
- `TargetResolutionKind`
- 対象nodeの `KmmNodeId`
- 対象nodeの `SourceSpan`

## KCF

KCFはMermaid、Draw.io、PlantUML、mathなどの外部描画を担う。

既存HTML/PDF/PNG/JPG exportは、KDV側に同等機能が入るまで維持する。KDV実装後、KCFのexport関連計画と実装はKDVへ移譲し、KCF側から削除する。

KCFはKMMより先にmetadata schemaを増やさない。新しいmetadata用途が必要な場合は、KMM側のfixtureとOpenSpecを先に更新する。

kcfが参照するpayload用途は、`tests/fixtures/metadata_uses.json` の次のkindをv0境界にする。

- `pdf-page`
- `llm-annotation`
- `ast-edit`

## KatanA

KatanA本体は統合順序とfixture authorityを管理する。

KMMのcanonical fixtureは `tests/fixtures/canonical/**` を正とする。KatanA側のfixtureが変わる場合、KMM側のfixture同期、contract test、目視確認結果を同じchangeで更新する。

`v0.1.0` 公開後、downstreamは crates.io の `katana-markdown-model = "0.1.0"` を基準に採用する。公開前の統合検証だけは、release branchまたはgit revisionを明示して扱う。

## KatanAの同期制御

KatanAはeditor-viewer同期の唯一のcoordinatorである。

KatanAはKMMのnode id、source range、line-column、raw snippet、fingerprintを使ってeditorとviewerを対応付ける。KMM、KLE、KDVは互いを知らない。

KatanAが命令する先はviewerまたはeditorであり、KMMへscroll、selection、highlightなどの命令は送らない。

## KCF既存export維持条件

KCFの既存exportをKDVへ移譲する判断は、次をすべて満たした後に行う。

- KMM `v0.1.0` が公開されている
- KMM public DTOとmetadata APIがこの文書どおり固定されている
- KUWまたは明示的widget境界が定義されている
- KDVが `KmmDocument` をviewer/export inputとして採用できる
- kleが `MetadataReconcileRequest` / `MetadataReconcileResult` を保存時contractとして採用できる

## 禁止事項

- KMM内部parser型をdownstream public APIへ出す
- downstreamで独自metadata schemaを作る
- unresolved metadataを保存時に削除する
- KCFがKMM/KDVより先に新規export/paging metadata contractを固定する
- KMM、KLE、KDVへeditor-viewer同期制御を持たせる
- KatanA本体へ共通metadata widget責務を暗黙に吸収させる
