# Tasks: prepare-manual-harness

## Definition of Ready

- [x] KMM本体はlibrary-onlyを維持する方針である
- [x] release前の正本検証はfixture testで行う方針である
- [x] KCU/KCFの `just harness-up` を参考にする方針である

## Tasks

- [x] 1.1 harnessを公開crateへ含めない配置に決める
- [x] 1.2 `just harness-up` の補助入口を追加する
- [x] 1.3 Markdown本文とKMM node一覧を表示する
- [x] 1.4 選択nodeのsource range、line-column、raw snippet、fingerprintを表示する
- [x] 1.5 metadata解決状態を表示する
- [x] 1.6 代表fixtureごとの目視確認手順を文書化する
- [x] 1.7 目視確認結果の記録方法を文書化する
- [x] 1.8 package対象にharnessが混入しないことを確認する

## Definition of Done

- [x] `just harness-up` で構造確認補助環境が起動する
- [x] KMM本体に製品binary targetが追加されていない
- [x] fixtureごとの目視確認手順がある
- [x] release前品質ゲートとして扱える

## Verification

- [x] `scripts/openspec validate "prepare-manual-harness" --strict`
- [x] `just harness-up`
- [x] `cargo package --locked --allow-dirty --list` でharness混入がないことを確認する
