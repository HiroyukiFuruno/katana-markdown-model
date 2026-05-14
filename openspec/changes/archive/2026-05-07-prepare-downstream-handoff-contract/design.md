## Context

KMMはP1の分離対象である。

downstream連携は、KMM public DTOとmetadata APIが固定されてから始める。

## Goals

- downstreamごとの受け渡し条件を固定する。
- KMM内部parser型への依存を禁止する。
- 独自metadata schemaの発生を防ぐ。
- KDVとKCFの責務分離を明確にする。
- KatanAがeditor-viewer同期制御を担う条件を明確にする。

## Non-Goals

- downstream repositoryの実装。
- KDV、KLE、KCF、KatanA側のOpenSpec更新。
- UI widgetの実装。

## Handoff Targets

- KDV: KMM public DTOをviewer/export inputとして使う。
- KLE: 保存時metadata同期でKMM APIを使う。
- KCF: Mermaid、Draw.io、PlantUML、mathなどの外部描画を担う。既存exportはKDV移譲まで維持する。
- KatanA: fixture authority、統合順序、editor-viewer同期制御を管理する。

## Decision

受け渡し境界の正本は `docs/downstream-handoff.md` とする。

downstreamが使ってよい入口は、`KatanaMarkdownModel::parse`、`KatanaMarkdownModel::reconcile`、`KatanaMarkdownModel::reconcile_targets` に限定する。

KCFの新規export計画はKDVへ移譲する。KCF側で維持する既存exportは、KDVが同等機能を持った後に削除する。

KatanAはKMMのnode id、source range、line-column、raw snippet、fingerprintを使ってeditorとviewerを対応付け、viewerまたはeditorへscroll、selection、highlightなどの命令を送る。KMMへ表示制御の命令は送らない。

## Verification

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
scripts/openspec validate "prepare-downstream-handoff-contract" --strict
just check
```
