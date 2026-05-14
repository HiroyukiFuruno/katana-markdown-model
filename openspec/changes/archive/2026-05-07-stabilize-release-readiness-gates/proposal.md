## Why

KMMは `v0.1.0` 公開前に、通常検査とrelease前検査の境界を固定する必要がある。

KMLでは、local targetだけではなくGitHub Actionsとbranch protectionまで含めてrelease前検査を扱っている。KMMでも同じ考え方を採用する。

ただしKMMはlibrary-onlyなので、KMLの多チャネル配布検査は持ち込まない。

## What Changes

- `release-check` をKMMのrelease前検査入口として定義する。
- `release-check` は `just check`、`cargo package --locked --allow-dirty`、`cargo publish --dry-run --locked --allow-dirty` を含む。
- GitHub Actionsの `release-preflight` が `release-check` 相当を実行する。
- `master` branch protectionで要求するcheck名を文書化する。
- `v0.1.0` 公開前に必要なsecretと実公開前の停止条件を明記する。

## Impact

- `Justfile` / `just/**`
- `.github/workflows/release-preflight.yml`
- `docs/quality-gates.md`
- OpenSpec tasks

このchangeでは実公開は行わない。
