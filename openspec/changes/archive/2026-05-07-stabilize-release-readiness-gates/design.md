## Context

KMMには既に `just check` と `release-preflight` がある。

現状はcrates.io公開が未接続であり、release前検査も `just check` との関係が十分に固定されていない。

## Goals

- release前検査を `release-check` として明示する。
- local targetとGitHub Actionsの検査内容をずらさない。
- branch protectionで要求するcheck名を文書化する。
- `v0.1.0` まで実公開しない停止条件を明記する。

## Non-Goals

- GitHub Release作成。
- crates.ioへの実公開。
- npm、PyPI、Homebrew、binary artifact、MCPB、editor extension検査の導入。

## Design

KMMのrelease前検査はKMLを参考にするが、library-onlyに絞る。

標準入口:

- `just check`: 日常開発の品質ゲート。
- `release-check`: release前の追加検査。

`release-check` は次を含む。

- `just check`
- `cargo package --locked --allow-dirty`
- `cargo publish --dry-run --locked --allow-dirty`

GitHub Actionsの `release-preflight` は、PR上でも同じrelease前検査が動くように保つ。

## Verification

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
scripts/openspec validate "stabilize-release-readiness-gates" --strict
just release-check
```
