# KMM v0.1.0 Roadmap

## Summary

`katana-markdown-model` (KMM) reaches the first `v0.1.0` release by completing small OpenSpec changes in sequence.

Do not publish `v0.1.0` until implementation, verification, and handoff conditions are complete.

## KMM Responsibility

KMM is not a Markdown-to-HTML converter.

KMM owns the shared Markdown document model, source ranges, line-column positions, raw snippets, fingerprints, and external metadata resolution used across the KatanA ecosystem. These outputs are materials for KLE save-time metadata reconciliation, KDV viewer/export behavior, and KatanA editor-viewer synchronization.

KMM does not own:

- product CLI
- product UI
- HTML/PDF/PNG/JPG output
- Floem, egui, or KatanA workspace state
- viewer/editor synchronization control
- integration logic for KDV, KLE, KCF, or KatanA itself

## Change Order Through v0.1.0

## v0.1.0 Release PR Checklist

This table fixes what must be completed in the release PR from `release/v0.1.0` to `master`, and what remains after merge.

| change | status | included in release PR | left after PR |
| --- | --- | --- | --- |
| `stabilize-release-readiness-gates` | complete | branch protection settings verified | update protection if check names change |
| `stabilize-canonical-fixtures` | complete | none | fixture sync PRs when fixtures change |
| `finalize-metadata-resolution-contract` | complete | none | additional usage fixtures from downstream adoption |
| `lock-parser-adapter-strategy` | complete in PR | parser boundary, contract tests, parser evaluation notes | future parser-engine replacement change |
| `prepare-manual-harness` | reorganized in PR | structure-inspection aid and package exclusion check | keep it out of mandatory visual release gates |
| `prepare-downstream-handoff-contract` | reorganized in PR | KDV/KCF/KLE/KatanA handoff boundary and KDV rename policy | downstream repository implementation PRs |
| `publish-v0-1-0-release` | PR preparation only | runbook, secret, PR, publication, verification, and branch hygiene procedure | GitHub Release, crates.io publication, post-publication verification, branch hygiene |

Before creating the release PR, all of the following must be true:

- active OpenSpec change validation passes
- `just check` passes
- `just release-check` passes
- fixture tests verify representative structure, metadata, and synchronization materials
- `cargo package --locked --allow-dirty --list` does not include the development harness
- `master` branch protection requires `Test and Build (macos-latest)`, `Test and Build (ubuntu-latest)`, `Test and Build (windows-latest)`, and `preflight`
- release PR from `release/v0.1.0` to `master` exists and required checks pass

Before the actual release, all of the following must be true:

- release PR is merged into `master`
- `CARGO_REGISTRY_TOKEN` is registered as a GitHub secret
- GitHub Release creation procedure is executable for `v0.1.0`
- crates.io publication procedure is executable
- post-publication verification procedure is executable

## Precision Work Excluded From v0.1.0

Parser precision, expanded Markdown syntax coverage, and additional metadata matching heuristics are not included in `v0.1.0`.

`v0.1.0` fixes KMM's public boundary. Precision improvements found after KDV, KLE, and KatanA adoption are handled by smaller `v0.1.1` or later changes.

`v0.1.1` candidates:

- source range, line-column, raw snippet, and fingerprint drift found by canonical fixtures
- dedicated DTOs for footnotes, images, links, HTML inline, or math inline if downstream usage proves they are required
- metadata target move-detection precision improvements
- additional editor-viewer synchronization anchor material when existing fields are insufficient

KMM still must not take ownership of viewer, export, or synchronization control.

### 1. `stabilize-release-readiness-gates`

Fix the mapping among CI, release preflight, branch protection, `just check`, and `release-check`.

This change follows the KML release-preflight pattern, but KMM is library-only. Do not bring in npm, PyPI, Homebrew, binary artifact, MCPB, or editor extension checks.

Completion conditions:

- `just check` remains the standard quality gate
- `release-check` includes `just check`, `cargo package --locked --allow-dirty`, and `cargo publish --dry-run --locked --allow-dirty`
- GitHub Actions `release-preflight` runs the same checks
- required checks for `master` branch protection are documented

### 2. `stabilize-canonical-fixtures`

Fix current KatanA fixtures, README badges, alerts, and description lists as KMM canonical fixtures.

Completion conditions:

- canonical fixture sync procedure is defined
- node kinds, source ranges, raw snippets, and fingerprints are fixed by tests
- verification does not depend on absolute paths

### 3. `finalize-metadata-resolution-contract`

Fix the public contract for metadata resolution.

Completion conditions:

- `Resolved`, `Moved`, `Unresolved`, and `Conflict` are public DTO states
- request/result DTOs required for editor save-time behavior are fixed in KMM
- tests fix the contract that unresolved metadata is not deleted

### 4. `lock-parser-adapter-strategy`

Fix the boundary that parser internals never leak into the public contract, and define the parse contract for major Markdown structures.

Completion conditions:

- public DTOs remain stable even if the parser engine is replaced
- contract tests cover tables, badges, alerts, description lists, footnotes, diagrams, math, and emoji
- parser candidates that break OS-dependent emoji handling are rejected

### 5. `prepare-manual-harness`

`just harness-up` is a development-only aid for inspecting KMM output structure.

KMM release decisions are based on fixture tests, not manual GUI inspection. Visual inspection is limited to confirming Markdown input, parsed node lists, selected-node position data, raw snippets, fingerprints, and metadata resolution states.

This is not a product UI and not a mandatory manual release gate.

Completion conditions:

- fixture tests verify structure, metadata, and synchronization materials needed by KDV, KLE, and KatanA
- `just harness-up` can be used as an optional structure-inspection aid
- KMM remains library-only
- development harness binaries and UI dependencies are excluded from the published crate

### 6. `prepare-downstream-handoff-contract`

Fix the public DTO handoff boundary and handoff conditions for KDV, KLE, KCF, and KatanA.

Completion conditions:

- downstream repositories are explicitly prohibited from depending on KMM parser internals
- downstream repositories are explicitly prohibited from creating independent metadata schemas
- KDV owns viewer/export behavior, and KCF export remains only until KDV provides equivalent functionality
- KatanA owns editor-viewer synchronization control

### 7. `publish-v0-1-0-release`

Publish `v0.1.0` to GitHub Releases and crates.io after all changes are complete.

Completion conditions:

- all OpenSpec changes are complete
- `just check` passes
- `release-check` passes
- `cargo package --locked --allow-dirty` passes
- `cargo publish --dry-run --locked --allow-dirty` passes
- `CARGO_REGISTRY_TOKEN` is registered as a GitHub secret
- GitHub Release and crates.io post-publication verification passes

## Branch Strategy After v0.1.0

After `v0.1.0`, KMM formally adopts the KML-style branch strategy while keeping `master` as the default branch.

- default branch: `master`
- release integration branch: `release/vX.Y.Z`
- supporting branch: `feature/vX.Y.Z-<short-slug>`
- release PR: from `release/vX.Y.Z` to `master`
- do not use `fix/vX.Y.Z-*`, `chore/vX.Y.Z-*`, or `release-vX.Y.Z`
- do not use `--admin` during merge
- run branch hygiene after merge

## Required Verification Before Release

For each completed change:

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
scripts/openspec validate "<change-name>" --strict
just check
```

Immediately before publishing `v0.1.0`:

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
just check
cargo package --locked --allow-dirty
cargo publish --dry-run --locked --allow-dirty
```

Optional structure inspection:

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
just harness-up
```

## Do Not Proceed With

- publishing `v0.1.0` before all required changes are complete
- adding a product CLI to KMM
- adding a product UI to KMM
- making KMM own HTML/PDF output
- making KMM own editor-viewer synchronization control
- leaking parser internals into KMM public DTOs for downstream convenience
