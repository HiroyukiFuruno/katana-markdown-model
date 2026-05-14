---
name: impl-release
description: Use this for katana-markdown-model release implementation workflows, including OpenSpec implementation, release preparation, release PRs, GitHub Releases, crates.io publication, and post-release cleanup. Equivalent to /impl-release vX.Y.Z for KMM.
---

# impl-release

This is the entry point for moving a KMM `vX.Y.Z` release from implementation through publication and cleanup.

## Preconditions

- Target repository: `/Users/hiroyuki_furuno/works/private/katana-markdown-model`.
- Default branch: `master`.
- KMM is library-only. Do not bring in CLI, UI, binary artifact, npm, PyPI, Homebrew, MCPB, or editor extension release flows.
- Publication targets are the `katana-markdown-model` crate, GitHub Release, and `docs/release-notes/<version>.md`.
- Release integration branches use `release/vX.Y.Z`; support branches use `feature/vX.Y.Z-<short-slug>`.
- Release PRs are created from `release/vX.Y.Z` to `master`.

## Stop Rules

Stop and ask the user when any of these are true:

- The requested version looks unnatural compared with the latest tag, GitHub Release, or crates.io version.
- An API change, public DTO change, compatibility break, or responsibility-boundary change is needed but not covered by OpenSpec tasks.
- The work would add CLI, rendering, export, UI framework, or downstream repository dependencies to KMM.
- User-side setup or credentials are required, such as `CARGO_REGISTRY_TOKEN` or publication permission.
- The next step is externally visible: commit, push, merge, or publish.

## References

- `.agents/skills/openspec-apply-change/SKILL.md`: OpenSpec change implementation.
- `.agents/skills/lint-and-ast-lint/SKILL.md`: `just check` quality gate.
- `.agents/skills/self-review/SKILL.md`: self-review before handoff.
- `.agents/skills/commit_and_push/SKILL.md`: commit/push after user approval.
- `docs/release-runbook.md`: publication procedure.
- `docs/quality-gates.md`: release preflight gates.
- `docs/roadmap.md` and `docs/roadmap-v0.1.1.md`: release scope and responsibility boundary.
- `branch-hygiene` skill: branch/worktree cleanup after merge or release.

## Phase 0: Inspect State

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
git status --short --branch
git fetch origin --prune --tags
```

Check:

- do not mix existing unrelated diffs
- whether `Cargo.toml` version matches the target version
- whether the requested version is the natural next version compared with tags, GitHub Releases, and crates.io
- whether `docs/release-notes/vX.Y.Z.md` exists
- whether an OpenSpec change exists for the target version

If the version looks unnatural, do not treat the user request as automatically correct. Stop and ask.

## Phase 1: Implement

1. Create `release/vX.Y.Z` or continue the existing branch.
2. Read `proposal.md`, `design.md`, `tasks.md`, and `specs/**/spec.md` for the target OpenSpec change.
3. Implement pending tasks from `tasks.md` in order.
4. For bug fixes, add the regression test first.
5. Record new decisions, constraints, and unresolved items in OpenSpec or release docs.
6. Update task checkboxes immediately after completing each task.

KMM responsibility boundary:

- Do not expose third-party parser ASTs through the public contract.
- Do not depend on KatanA, KCF, KDV, KLE, or UI frameworks.
- Do not add rendering, export, scroll, selection, or highlight control to KMM.
- Treat fixture tests as the source of truth for release decisions.

## Phase 2: Verify

Run this for each target change:

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
scripts/openspec validate "<change-name>" --strict
just check
```

Run this before the release PR:

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
just release-check
```

`just release-check` includes `just check`, `cargo package --locked --allow-dirty`, and `cargo publish --dry-run --locked --allow-dirty`.

Do not skip failed verification. Do not bypass lint, hooks, or checks with exclusions or `--no-verify`.

## Phase 3: Prepare Release PR

Check:

- `Cargo.toml` version
- `Cargo.lock` update, if needed
- `docs/release-notes/vX.Y.Z.md`
- `docs/release-readiness/<version>-*.md`
- completed OpenSpec changes are archived

Before creating the PR, report the changes and verification results to the user and stop.

After user approval, create the PR:

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
git push -u origin release/vX.Y.Z
gh pr create --base master --head release/vX.Y.Z --title "Prepare vX.Y.Z release"
```

The PR body must state that publication has not been performed yet, and include `just release-check`, fixture-test, and optional structure-inspection results.

## Phase 4: Pre-Merge Check

Before merge, verify:

- `release-preflight` passes
- `Test and Build (macos-latest)` passes
- `Test and Build (ubuntu-latest)` passes
- `Test and Build (windows-latest)` passes
- no review comments remain unresolved
- `--admin` is not used

Merge only after user approval.

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
gh pr merge --merge --delete-branch <PR-number-or-url>
git switch master
git pull --ff-only origin master
```

## Phase 5: Publish

Publish only after the PR has been merged into `master`.

Manually dispatch the GitHub Actions `release` workflow from `master`.

- `version`: `vX.Y.Z`
- `publish_crate`: `true`

Use the manual path in `docs/release-runbook.md` only if GitHub Actions cannot be used.

Before publication, confirm that `CARGO_REGISTRY_TOKEN` is registered as a GitHub secret.

## Phase 6: Verify Publication

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
gh release view vX.Y.Z --repo HiroyukiFuruno/katana-markdown-model
cargo search katana-markdown-model --limit 1
cargo info katana-markdown-model@X.Y.Z
```

Do not reuse a crates.io version that has already been published.

- If failure occurs before crates.io publication, fix it and rerun the same version.
- If only GitHub Release creation fails after crates.io publication, recreate the GitHub Release with the same tag.
- If a content issue is found after crates.io publication, fix it in the next patch version.

## Phase 7: Cleanup

Use the `branch-hygiene` skill.

Report:

- GitHub Release URL
- crates.io version
- deleted local branches
- deleted remote branches
- deleted worktrees
- retained branches and reasons
- issues deferred to the next version

## Completion Conditions

- target OpenSpec tasks are complete
- `scripts/openspec validate "<change-name>" --strict` passes
- `just check` passes
- `just release-check` passes
- release PR is merged into `master`
- GitHub Actions `release` workflow succeeds
- GitHub Release and crates.io post-publication verification passes
- branch hygiene is complete
