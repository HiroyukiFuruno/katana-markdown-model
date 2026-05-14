## Why

KMMのpublic contractはKMM所有DTOでなければならない。

parser engineを先に固定しすぎると差し替えが難しくなる。一方で、KatanA現行挙動を壊すparserを採用してはいけない。

## What Changes

- 現行parser維持かparser engine差し替えかを評価する。
- parser内部型をpublic DTOへ出さないadapter境界を固定する。
- table、badge、alert、description list、footnote、diagram、math、emojiのparse contractをテストで固定する。
- OS依存emojiを壊さない判断基準を持つ。

## Impact

- `src/parser/**`
- `src/document/types.rs`
- `tests/document_model.rs`
- parser評価メモ
