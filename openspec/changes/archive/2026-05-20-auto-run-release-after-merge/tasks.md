# Tasks: auto-run-release-after-merge

## Definition of Ready

- [x] PR #2 merge 後に Release workflow が起動していないことを確認した
- [x] `.github/workflows/release.yml` が `workflow_dispatch` のみであることを確認した
- [x] `CARGO_REGISTRY_TOKEN` secret が存在することを確認した

## Tasks: Implementation

- [x] 1.1 `master` push で Release workflow が起動するようにする
- [x] 1.2 push 起動時に `Cargo.toml` から release version を解決する
- [x] 1.3 既存 release がある version は publish を skip する
- [x] 1.4 GitHub Release title を `vX.Y.Z` のみにする
- [x] 1.5 release runbook を自動起動前提へ更新する
- [x] 1.6 v0.1.0 publication 記録を現在の title に合わせる

## Definition of Done

- [x] release merge 後の `master` push が Release workflow を起動できる
- [x] 手動 dispatch 経路が残っている
- [x] release title が `vX.Y.Z` のみになる
- [x] `scripts/openspec validate "auto-run-release-after-merge" --strict` passes
- [x] `just check` passes

## Verification

- [x] `scripts/openspec validate "auto-run-release-after-merge" --strict`
- [x] `just check`
