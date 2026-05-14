## Context

KMMは製品UIを持たない。

文書モデル、source mapping、metadata解決はfixture testで保証する。人間が確認する画面は補助であり、release前の必須品質ゲートにはしない。

## Goals

- `just harness-up` を任意の構造確認補助toolとして維持する。
- Markdown本文とKMM出力を同時に確認できる。
- metadata解決結果を確認できる。
- harnessを公開crateから分離する。

## Non-Goals

- 製品UIの実装。
- KMM本体へのbinary target追加。
- HTML/PDF出力の実装。
- downstream UIの代替。

## Harness View

画面上では次を確認できるようにする。

- Markdown本文
- KMMが解釈したnode一覧
- 選択nodeのsource range
- 選択nodeのline-column
- raw snippet
- fingerprint
- metadata解決状態

これは構造確認の補助であり、表示そのものを製品仕様にはしない。

## Decision

`v0.1.0` では、開発用の構造確認harnessを提供する。

画面はMarkdown原文、KMM node一覧、選択nodeのsource mapping、metadata解決結果を表示する。Markdown描画やviewer/exportの確認はKDV側の責務であり、KMM harnessの完了条件にしない。

配置は `tools/manual-harness` とし、KMM本体へ製品binary targetは追加しない。公開crateには `tools/**` を含めない。

目視確認結果は `docs/release-readiness/<version>-manual-harness.md` に記録する。

既定表示対象は `/Users/hiroyuki_furuno/works/private/katana/assets/fixtures/sample.md` とする。存在しない環境ではKMM内のcanonical fixtureへfallbackする。

## Verification

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
scripts/openspec validate "prepare-manual-harness" --strict
just harness-check
just harness-up /Users/hiroyuki_furuno/works/private/katana/assets/fixtures/sample.md
cargo package --locked --allow-dirty --list
```
