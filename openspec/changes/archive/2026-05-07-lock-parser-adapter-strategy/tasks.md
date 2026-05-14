# Tasks: lock-parser-adapter-strategy

## Definition of Ready

- [x] canonical fixtureの主要node一覧がある
- [x] KMM public DTOへparser内部型を出さない方針である

## Tasks

- [x] 1.1 現行parser維持かparser engine差し替えかを評価する
- [x] 1.2 parser候補のsource range保持能力を確認する
- [x] 1.3 parser候補がUnicode emojiとshortcode emojiを壊さないか確認する
- [x] 1.4 parser内部型をpublic DTOへ出さないadapter境界を固定する
- [x] 1.5 table、badge、alert、description list、footnote、diagram、math、emojiのparse contractをテスト化する
- [x] 1.6 採用しないparser候補がある場合は理由を記録する

## Definition of Done

- [x] parser engineを差し替えてもpublic DTOが壊れない
- [x] KatanA現行挙動を再現できないparserを選ばない
- [x] 主要Markdown構造のcontract testがある

## Verification

- [x] `scripts/openspec validate "lock-parser-adapter-strategy" --strict`
- [x] `just check`
