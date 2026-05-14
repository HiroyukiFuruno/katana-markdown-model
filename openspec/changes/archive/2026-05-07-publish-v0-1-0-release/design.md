## Context

KMM `Cargo.toml` のversionは `0.1.0` である。

初回リリースでは、public DTO、metadata API、parser adapter境界、fixture test、downstream handoff条件が完了している必要がある。

## Goals

- `v0.1.0` の公開手順を固定する。
- 全change完了後にだけ公開する。
- crates.io公開後の検証を行う。
- KML同様のbranch戦略をKMM向けに正式採用する。

## Non-Goals

- 全change完了前の先行公開。
- 製品CLIの追加。
- KMLの多チャネル配布の導入。

## Branch Policy

`v0.1.0` 以降のbranch戦略:

- 既定ブランチ: `master`
- release統合ブランチ: `release/vX.Y.Z`
- 補助ブランチ: `feature/vX.Y.Z-<short-slug>`
- release PR: `release/vX.Y.Z` から `master`
- `fix/vX.Y.Z-*`、`chore/vX.Y.Z-*`、`release-vX.Y.Z` は使わない
- merge時に `--admin` は使わない
- merge後にbranch hygieneを行う

## Decision

`v0.1.0` の公開手順の正本は `docs/release-runbook.md` とする。

GitHub Actionsの `release` workflowは手動実行とし、`publish_crate=false` ではrelease前検査だけを行う。`publish_crate=true` のときだけ、crates.io公開とGitHub Release作成を行う。

release noteの正本は `docs/release-notes/v0.1.0.md` とする。

## Verification

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
scripts/openspec validate "publish-v0-1-0-release" --strict
just release-check
```
