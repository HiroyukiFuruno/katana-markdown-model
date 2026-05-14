# KMM v0.1.1 Roadmap

## Summary

`v0.1.1` is a precision-hardening release. It improves issues found during KDV, KLE, and KatanA adoption without breaking the public contract fixed in `v0.1.0`.

KMM still does not own rendering, export, or editor-viewer synchronization control in `v0.1.1`.

## Goal

`v0.1.0` fixed the initial public boundary.

`v0.1.1` handles only precision problems that should be fixed on the KMM side after downstream adoption.

## Scope

- source range, line-column, raw snippet, and fingerprint drift found by canonical fixtures
- decision on whether footnote, image, link, HTML inline, or math inline needs dedicated DTOs
- metadata target move-detection precision improvements
- evaluation of missing anchor material for KatanA editor-viewer synchronization

## Out of Scope

- Markdown rendering in KMM
- HTML/PDF/PNG/JPG export
- product CLI
- product UI
- KatanA workspace state
- scroll, selection, or highlight commands to viewers or editors
- direct dependencies on KDV, KLE, KCF, or KatanA itself

## Process

### 1. Triage Downstream Adoption Results

Collect adoption results from KDV, KLE, and KatanA using `katana-markdown-model = "0.1.0"`.

KMM handles only issues related to KMM public DTOs, source mapping, metadata resolution, or synchronization materials.

### 2. Reproduce With Fixtures

Before changing implementation, reproduce each issue with a fixture test inside KMM.

Do not add tests that depend on absolute paths. When using a KatanA fixture, sync it into `tests/fixtures/canonical/` in this repository first.

### 3. Preserve the Public Contract

Do not expose third-party parser ASTs through public DTOs.

When adding a new DTO, record in OpenSpec why KDV, KLE, or KatanA cannot handle the case with existing DTOs.

### 4. Release

Collect `v0.1.1` on `release/v0.1.1` and create a release PR to `master`.

Run `just release-check` before publication.

## Completion Conditions

- OpenSpec tasks in `prepare-v0-1-1-precision-hardening` are complete
- added or changed behavior is fixed by fixture tests
- KMM remains library-only
- third-party parser ASTs do not leak into the public contract
- KMM does not own rendering, export, or synchronization control
- `just check` passes
- `just release-check` passes

## Decision Criteria

Fixes accepted for `v0.1.1` must satisfy all of the following:

- the root cause is in KMM parsing results or metadata resolution, not downstream convenience
- the issue can be reproduced with a fixture
- public DTO compatibility is preserved
- KMM responsibility boundaries do not expand

## Follow-Up Candidates

Precision work that does not fit into `v0.1.1` moves to `v0.1.2` or later.

If scope grows, create an OpenSpec change first and confirm that the responsibility belongs to KMM.
