# Tasks: prepare-v0-1-1-precision-hardening

## Definition of Ready

- [x] `v0.1.0` fixes KMM's initial public boundary
- [x] parser precision, metadata matching precision, and additional synchronization anchor materials are intentionally excluded from `v0.1.0`
- [x] KMM does not own rendering, export, or synchronization control

## Tasks: Planning

- [x] 1.1 Position `v0.1.1` as a precision-hardening release
- [x] 1.2 Document `v0.1.1` scope and non-scope
- [x] 1.3 Define how to classify downstream adoption results into KMM responsibility
- [x] 1.4 Define the fixture-first fix policy
- [x] 1.5 Define the `v0.1.1` release policy

## Tasks: Implementation

- [ ] 2.1 Inventory adoption results from KDV, KLE, and KatanA using `katana-markdown-model = "0.1.0"`
- [ ] 2.2 Classify issues into KMM parsing precision, metadata resolution, synchronization material, and downstream responsibility
- [ ] 2.3 Reproduce KMM-owned issues with fixture tests
- [ ] 2.4 Fix source range, line-column, raw snippet, and fingerprint drift
- [ ] 2.5 Decide whether footnote, image, link, HTML inline, or math inline needs dedicated DTOs
- [ ] 2.6 If a dedicated DTO is needed, add it without breaking existing public DTOs
- [ ] 2.7 Improve metadata target move-detection precision
- [ ] 2.8 Preserve regression tests that return `Conflict` for ambiguous metadata recovery
- [ ] 2.9 Preserve regression tests that return `Unresolved` for unrecoverable metadata
- [ ] 2.10 If editor-viewer synchronization needs more anchor material, add only the minimum renderer-neutral KMM public DTO fields
- [ ] 2.11 Confirm in docs and tests that KMM does not command viewers or editors

## Definition of Done

- [ ] only KMM-owned issues from downstream adoption are taskized
- [ ] added or changed behavior is fixed by fixture tests
- [ ] KMM remains library-only
- [ ] third-party parser ASTs do not leak into the public contract
- [ ] KMM does not own rendering, export, or synchronization control
- [ ] `just check` passes
- [ ] `just release-check` passes

## Verification

- [x] `scripts/openspec validate "prepare-v0-1-1-precision-hardening" --strict`
- [x] `just check`
- [x] `just release-check`
