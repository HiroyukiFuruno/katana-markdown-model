## Context

KMM の Release workflow は `workflow_dispatch` だけを持っていた。

この設計では、release PR を merge しても crates.io publish と GitHub Release 作成は自動では始まらない。`impl-release` の publish phase では手動起動を前提にしていたが、通常の release merge 操作から見ると「merge したのに release が流れない」状態になる。

## Goals

- release PR merge 後の `master` push で Release workflow を起動する。
- publish 対象 version は `Cargo.toml` から解決する。
- 手動 dispatch は残し、障害時の再実行経路を維持する。
- GitHub Release title は `vX.Y.Z` のみとする。

## Non-Goals

- crates.io token の管理方法を変えること。
- release PR の required check を増やすこと。
- KMM 以外の repo release workflow を変更すること。

## Workflow Direction

`release.yml` は次の 2 系統で起動する。

- `push` to `master`: release merge 後の通常 publication 経路
- `workflow_dispatch`: 障害時や明示的な再実行経路

push 起動では、`Cargo.toml` の `package.version` を読み、`v${version}` を release version とする。

手動起動では、input の `version` を使い、`Cargo.toml` と一致することを確認する。

GitHub Release がすでに存在する場合、publish step は skip する。これにより、workflow ファイルや release note の後続修正で同じ version を再 publish しない。

## Verification

- `scripts/openspec validate "auto-run-release-after-merge" --strict`
- `just check`
- `just release-check`
