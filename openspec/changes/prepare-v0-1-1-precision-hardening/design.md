## Context

KMM `v0.1.0` publishes renderer-neutral document models, source mapping, metadata resolution, and materials that can be used for synchronization. It does not publish rendering or synchronization control.

When KDV, KLE, and KatanA adopt `v0.1.0`, they may find gaps in real Markdown display, save-time metadata reconciliation, or editor-viewer synchronization.

`v0.1.1` does not move every downstream gap back into KMM. It handles only issues caused by KMM parsing results or public DTOs.

## Goals

- Improve precision without breaking the `v0.1.0` public contract.
- Convert downstream adoption results into fixture tests.
- Add regression tests for source range, line-column, raw snippet, and fingerprint drift.
- Improve metadata target move detection without weakening the unresolved-preservation or conflict-return contracts.
- Keep synchronization control in KatanA while adding only the minimal anchor material KatanA can use.

## Non-Goals

- Markdown rendering.
- HTML/PDF/PNG/JPG export.
- Product CLI.
- Product UI.
- KatanA workspace state.
- Scroll, selection, or highlight commands to viewers or editors.
- Dependencies on KDV, KLE, KCF, or KatanA itself.
- One-shot implementation of the full CommonMark specification.

## Work Streams

### 1. Downstream Adoption Triage

Collect adoption results from KDV, KLE, and KatanA using `katana-markdown-model = "0.1.0"`.

Classify each issue into one of four buckets:

- KMM parsing precision issue
- KMM metadata resolution issue
- missing synchronization material returned by KMM
- issue owned by downstream responsibility

Do not add issues outside KMM responsibility to KMM tasks.

### 2. Fixture-First Fixes

Before fixing implementation, reproduce the failure with a KMM fixture test.

Do not add tests that depend on absolute paths. When using a KatanA fixture, sync it into this repository's canonical fixtures before testing.

### 3. Markdown Contract Gaps

Footnote, image, link, HTML inline, and math inline were triaged in `v0.1.0` as current KatanA behavior.

In `v0.1.1`, consider dedicated DTOs only if downstream repositories cannot handle the case with existing DTOs. If a dedicated DTO is added, add it without breaking existing DTOs.

### 4. Metadata Precision

Metadata target move detection uses node ids, fingerprints, source ranges, and surrounding context.

`v0.1.1` must not silently choose one target when the result is ambiguous. Ambiguous recovery returns `Conflict`; unrecoverable recovery returns `Unresolved`.

### 5. Sync Anchor Materials

KMM does not own synchronization control.

If existing node ids, source ranges, line-column positions, raw snippets, and fingerprints are insufficient for KatanA to map editor and viewer nodes, KMM may add renderer-neutral anchor material to the public DTO.

Even then, KMM does not send commands to viewers or editors.

## Release Policy

Collect `v0.1.1` on `release/v0.1.1` and create a release PR to `master`.

Before publication, run:

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
scripts/openspec validate "prepare-v0-1-1-precision-hardening" --strict
just check
just release-check
```
