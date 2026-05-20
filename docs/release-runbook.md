# Release Runbook

## Summary

Publish `vX.Y.Z` only after the release PR has been merged into `master`.

The release PR itself does not publish. Publication starts from the Release workflow after the release commit reaches `master`.

## Release PR

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
git switch release/vX.Y.Z
just release-check
git push -u origin release/vX.Y.Z
gh pr create --base master --head release/vX.Y.Z
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

After the PR is merged, the `release` workflow runs from `master` when the release commit changes release-relevant paths.

Automatic release inputs are resolved as follows:

- `version`: `vX.Y.Z` from `Cargo.toml`
- `publish_crate`: `true`
- GitHub Release title: `vX.Y.Z`

If the same GitHub Release already exists, publication is skipped.

Use manual dispatch only for explicit reruns or failure recovery.

- `version`: `vX.Y.Z`
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
gh release create vX.Y.Z \
  --repo HiroyukiFuruno/katana-markdown-model \
  --target master \
  --title "vX.Y.Z" \
  --notes-file docs/release-notes/vX.Y.Z.md
```

## Failure Policy

Do not reuse a version that has already been published to crates.io.

- If publication fails before crates.io upload, fix the issue and rerun the same `vX.Y.Z` release.
- If GitHub Release creation fails after crates.io publication, recreate the GitHub Release with the same tag.
- If a content issue is found after crates.io publication, fix it in the next patch version.

## Post-Publication Verification

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
gh release view vX.Y.Z --repo HiroyukiFuruno/katana-markdown-model
cargo search katana-markdown-model --limit 1
cargo info katana-markdown-model@X.Y.Z
```

## Branch Hygiene

After release, inspect and clean local branches, remote branches, and worktrees.

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
git switch master
git pull --ff-only origin master
git branch --list
git worktree list
git branch -d release/vX.Y.Z
git push origin --delete release/vX.Y.Z
```

If the remote release branch was already deleted by PR merge, `git push origin --delete release/vX.Y.Z` is unnecessary.
