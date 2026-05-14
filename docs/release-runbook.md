# v0.1.0 Release Runbook

## Summary

Publish `v0.1.0` only after the release PR has been merged into `master`.

The release PR itself does not publish.

## Release PR

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
git switch release/v0.1.0
just release-check
git push -u origin release/v0.1.0
gh pr create --base master --head release/v0.1.0
```

The PR body must state that publication has not been performed yet. It must also include the `just release-check` result, fixture-test evidence, and any optional structure-inspection evidence.

## Secret Registration

Publishing to crates.io requires the `CARGO_REGISTRY_TOKEN` GitHub secret.

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
gh secret set CARGO_REGISTRY_TOKEN \
  --repo HiroyukiFuruno/katana-markdown-model \
  --body "<crates.io token>"
gh secret list --repo HiroyukiFuruno/katana-markdown-model | rg '^CARGO_REGISTRY_TOKEN'
```

## Pre-Merge Checks

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
scripts/openspec validate "publish-v0-1-0-release" --strict
just check
just release-check
just harness-up
```

`just harness-up` is an optional structure-inspection aid. Fixture tests and `just release-check` remain the authoritative release gates.

## Publication Through GitHub Actions

After the PR is merged, manually dispatch the `release` workflow from `master`.

- `version`: `v0.1.0`
- `publish_crate`: `true`

When `publish_crate=false`, the workflow runs `just release-check` only. It does not create a GitHub Release or publish to crates.io.

## Manual Fallback

Use this path only if GitHub Actions cannot be used.

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
git switch master
git pull --ff-only origin master
just release-check
cargo publish --locked
gh release create v0.1.0 \
  --repo HiroyukiFuruno/katana-markdown-model \
  --target master \
  --title "katana-markdown-model v0.1.0" \
  --notes-file docs/release-notes/v0.1.0.md
```

## Failure Policy

Do not reuse a version that has already been published to crates.io.

- If publication fails before crates.io upload, fix the issue and rerun the same `v0.1.0` release.
- If GitHub Release creation fails after crates.io publication, recreate the GitHub Release with the same tag.
- If a content issue is found after crates.io publication, fix it in the next patch version.

## Post-Publication Verification

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
gh release view v0.1.0 --repo HiroyukiFuruno/katana-markdown-model
cargo search katana-markdown-model --limit 1
cargo info katana-markdown-model@0.1.0
```

## Branch Hygiene

After release, inspect and clean local branches, remote branches, and worktrees.

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
git switch master
git pull --ff-only origin master
git branch --list
git worktree list
git branch -d release/v0.1.0
git push origin --delete release/v0.1.0
```

If the remote release branch was already deleted by PR merge, `git push origin --delete release/v0.1.0` is unnecessary.
