# Tasks: stabilize-canonical-fixtures

## Definition of Ready

- [x] KMM v0はKatanA現行挙動の踏襲を優先する方針である
- [x] canonical fixture候補が `openspec/project.md` に列挙されている

## Tasks

- [x] 1.1 canonical fixtureの同期方法を決める
- [x] 1.2 KatanA `sample.md` の主要node期待値を棚卸しする
- [x] 1.3 KatanA `sample_basic.md` のalert期待値を棚卸しする
- [x] 1.4 README badge列の期待値を棚卸しする
- [x] 1.5 description list fixtureを正本fixtureとして固定する
- [x] 1.6 node種別、source range、raw snippet、fingerprintをテストへ追加する
- [x] 1.7 絶対パスに依存しないfixture運用を文書化する

## Definition of Done

- [x] KatanA現行fixtureの主要構造がKMM nodeとして固定されている
- [x] source range、raw snippet、fingerprintの回帰テストがある
- [x] downstreamが参照するfixture authorityが明確である

## Verification

- [x] `scripts/openspec validate "stabilize-canonical-fixtures" --strict`
- [x] `just check`
