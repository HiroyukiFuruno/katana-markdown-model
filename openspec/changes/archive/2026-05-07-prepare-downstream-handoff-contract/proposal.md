## Why

KMMのpublic DTOとmetadata APIが固定されたら、KDV、KLE、KCF、KatanAへ受け渡す必要がある。

受け渡し条件が曖昧だと、downstreamがKMM parserを再実装したり、独自metadata schemaを作ったりする危険がある。

## What Changes

- KDV、KLE、KCF、KatanAが参照するKMM public DTO境界をまとめる。
- KMM内部parser型へ依存しない条件を明記する。
- downstreamが独自metadata schemaを作らない条件を明記する。
- KDVがviewer/exportを担い、KCFが外部描画を担う条件をKMM側から固定する。
- KatanAがeditor-viewer同期制御を担い、KMMが同期材料だけを返す条件を明記する。

## Impact

- `docs/roadmap.md`
- `openspec/changes/**/handoff.md` 相当の受け渡し文書
- downstream側OpenSpecのDoR
