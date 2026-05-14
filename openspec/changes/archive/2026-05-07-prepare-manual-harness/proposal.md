## Why

KMMはlibrary-onlyであり、構造保証はfixture testを正本にする。

`just harness-up` は、文書モデルとmetadata解決結果を補助的に確認する開発用入口として扱う。Markdown描画、viewer、exportはKMMの責務ではない。

## What Changes

- `just harness-up` を任意の構造確認入口として維持する。
- Markdown本文、KMM node一覧、source range、raw snippet、fingerprint、metadata解決状態を確認できる開発用環境を用意する。
- KMM本体はlibrary-onlyを維持する。
- harnessのUI依存やbinaryが公開crateへ混入しない境界を固定する。
- fixtureごとの目視確認手順と確認結果の記録方法を用意する。

## Impact

- `tools/**` など公開crate外の開発用harness
- `Justfile`
- release前検証手順
- OpenSpec tasks
