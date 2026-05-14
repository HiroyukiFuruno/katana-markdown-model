# Tasks: prepare-downstream-handoff-contract

## Definition of Ready

- [x] KMM public DTOとmetadata APIのv0.1.0境界が固定されている
- [x] downstreamがKMM内部parser型へ依存しない方針である

## Tasks

- [x] 1.1 KDVへ渡すviewer/export inputとhit-test metadataを定義する
- [x] 1.2 KLEへ渡すsave-time metadata sync contractを定義する
- [x] 1.3 KUWまたは明示的widget境界へ渡すmetadata/unresolved表示DTOを定義する
- [x] 1.4 KCFを外部描画へ縮小し、既存exportをKDV移譲まで維持する条件を定義する
- [x] 1.5 KatanA統合で必要なfixture authorityとdependency version policyを定義する
- [x] 1.6 KatanAがeditor-viewer同期制御を担い、viewerまたはeditorへ命令する条件を文書化する

## Definition of Done

- [x] downstream repoがKMM内部parser型へ依存しない
- [x] downstream repoが独自metadata schemaを作らない
- [x] KDV、KCF、KLE、KatanAの責務境界が明確である

## Verification

- [x] `scripts/openspec validate "prepare-downstream-handoff-contract" --strict`
- [x] `just check`
