# Tasks: stabilize-release-readiness-gates

## Definition of Ready

- [x] KMMがlibrary-onlyであることが確認済みである
- [x] KMLのrelease前検査を参考にするが、多チャネル配布は持ち込まない方針が確認済みである
- [x] KMMの既定ブランチは `master` 維持である

## Tasks

- [x] 1.1 `release-check` の責務を `docs/quality-gates.md` に追記する
- [x] 1.2 `release-check` targetを追加し、`just check`、package、dry-run publishを実行する
- [x] 1.3 `release-preflight` が `release-check` 相当を実行するように整理する
- [x] 1.4 `master` branch protectionで要求するcheck名を文書化する
- [x] 1.5 `v0.1.0` 公開前に必要な `CARGO_REGISTRY_TOKEN` と停止条件を文書化する
- [x] 1.6 `release-check` が実公開を行わないことを確認する

## Definition of Done

- [x] `just release-check` がlocalで成功する
- [x] `release-preflight` がrelease前検査を実行する
- [x] local targetとGitHub Actionsの検査内容が矛盾していない
- [x] `v0.1.0` までは実公開しない条件が明記されている

## Verification

- [x] `scripts/openspec validate "stabilize-release-readiness-gates" --strict`
- [x] `just release-check`
