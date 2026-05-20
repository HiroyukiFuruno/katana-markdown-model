## Why

PR #2 を `master` に merge しても Release workflow が起動しなかった。

原因は `.github/workflows/release.yml` が `workflow_dispatch` のみで、`master` への release merge を起動条件にしていなかったため。runbook も手動 dispatch 前提で、merge 後に publish される期待と workflow の実体がずれていた。

また GitHub Release の title が `katana-markdown-model v0.1.0` のように crate name を含むため、latest 表示が冗長になっている。release title は tag と同じ `vX.Y.Z` のみにする。

## What Changes

- `release.yml` を `master` push でも起動する。
- push 起動時は `Cargo.toml` の version から `vX.Y.Z` を解決する。
- 手動起動時は従来どおり input version を使う。
- 既存 GitHub Release がある version は publish を skip する。
- GitHub Release title は `vX.Y.Z` のみにする。
- runbook を merge 後自動起動前提へ更新する。

## Impact

- `.github/workflows/release.yml`
- `docs/release-runbook.md`
- `docs/release-readiness/v0.1.0-publication.md`
- `openspec/specs/kmm-release-readiness/spec.md`
