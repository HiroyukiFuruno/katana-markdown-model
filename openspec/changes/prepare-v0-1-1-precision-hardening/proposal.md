## Why

`v0.1.0` fixes KMM's initial public boundary.

Parser precision, metadata matching precision, and additional synchronization anchor material can move the public boundary, so they are not mixed into `v0.1.0`.

`v0.1.1` handles only the KMM-side precision issues found after KDV, KLE, and KatanA adopt `v0.1.0`.

## What Changes

- Plan `v0.1.1` as a precision-hardening release.
- Keep KMM responsibility limited to parsing, document structure, source mapping, metadata resolution, and synchronization materials.
- Convert downstream adoption findings into fixture tests before fixing implementation.
- Decide whether footnote, image, link, HTML inline, or math inline needs dedicated DTOs.
- Improve metadata target move detection and missing synchronization anchor material within KMM's public contract.

## Impact

- `docs/roadmap-v0.1.1.md`
- `tests/fixtures/**`
- `tests/canonical_fixtures.rs`
- `tests/parser_adapter_contract.rs`
- `tests/metadata_resolution.rs`
- `src/document/**`
- `src/metadata/**`
- `src/parser/**`
