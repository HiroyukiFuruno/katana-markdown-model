# Downstream Handoff Contract

## Summary

The downstream handoff boundary for KMM `v0.1.0` is limited to KMM public DTOs and metadata APIs.

Downstream repositories must not depend on `src/parser/**`, third-party parser ASTs, or KMM internal implementation types.

KMM does not own rendering, export, or editor-viewer synchronization control. KatanA owns synchronization control and sends commands to viewers or editors.

## Public API Boundary

Downstream repositories may use only these entry points:

- `KatanaMarkdownModel::parse(MarkdownInput) -> Result<KmmDocument, KmmError>`
- `KatanaMarkdownModel::reconcile(MetadataReconcileRequest) -> MetadataReconcileResult`
- `KatanaMarkdownModel::reconcile_targets(&KmmDocument, &KmmDocument, &MetadataDocument) -> Vec<TargetResolution>`

Downstream repositories may reference only these DTOs:

- `KmmDocument`
- `KmmNode`
- `KmmNodeKind`
- `SourceSpan`
- `RawSnippet`
- `TextFingerprint`
- `MetadataDocument`
- `MetadataEntry`
- `MetadataTarget`
- `MetadataReconcileRequest`
- `MetadataReconcileResult`
- `TargetResolution`
- `TargetResolutionKind`

## KDV

KDV is the abbreviation for `katana-document-viewer`. The existing `katana-document-preview` repository is unreleased and not adopted, so the plan is to rename it to KDV.

KDV receives `KmmDocument` as viewer input and owns Markdown viewer behavior, hit-test behavior, node selection, and HTML/PDF/PNG/JPG export.

Hit-test and export metadata use these fields from KMM nodes:

- `KmmNodeId`
- `KmmNodeKind`
- `SourceSpan.byte_range`
- `SourceSpan.line_column_range`
- `SourceSpan.raw`
- `SourceSpan.raw.fingerprint()`

KDV must not parse Markdown again. HTML conversion output must not become an alternative KMM contract.

Viewer display and export use the same render pipeline inside KDV.

## KLE

KLE passes the old document, new document, and metadata document to KMM during save.

The standard save-time entry point is `KatanaMarkdownModel::reconcile(MetadataReconcileRequest)`.

KLE treats `TargetResolutionKind` as these states:

- `Resolved`: target remains on the same node
- `Moved`: fingerprint identifies the moved target node
- `Unresolved`: target cannot be recovered
- `Conflict`: multiple fingerprint candidates exist, and KMM does not choose one silently

KLE must not delete metadata entries for `Unresolved` or `Conflict`.

## KUW Or Widget Boundary

If KUW exists, metadata display UI belongs to KUW.

If KUW does not exist yet, do not silently add shared widget responsibility to KatanA or KDP. If a substitute is needed, create an explicit OpenSpec widget boundary.

Only this information is passed to metadata display:

- `MetadataEntry.key`
- `MetadataEntry.payload`
- `TargetResolutionKind`
- target node `KmmNodeId`
- target node `SourceSpan`

## KCF

KCF owns external rendering for Mermaid, Draw.io, PlantUML, and math.

Existing HTML/PDF/PNG/JPG export remains in KCF until KDV has equivalent functionality. After KDV implements it, export planning and implementation move from KCF to KDV and are removed from KCF.

KCF must not expand the metadata schema before KMM. If a new metadata use is required, update KMM fixtures and OpenSpec first.

The v0 payload-use boundary for KCF is defined by these `kind` values in `tests/fixtures/metadata_uses.json`:

- `pdf-page`
- `llm-annotation`
- `ast-edit`

## KatanA

KatanA owns integration order and fixture authority.

KMM canonical fixtures under `tests/fixtures/canonical/**` are the source of truth. When KatanA fixtures change, update KMM fixture sync, contract tests, and inspection results in the same change.

After `v0.1.0` publication, downstream repositories use the crates.io dependency `katana-markdown-model = "0.1.0"` as the baseline. Before publication, integration verification must reference a release branch or explicit git revision.

## KatanA Synchronization Control

KatanA is the only coordinator for editor-viewer synchronization.

KatanA maps editor and viewer content using KMM node ids, source ranges, line-column positions, raw snippets, and fingerprints. KMM, KLE, and KDV do not know about one another.

KatanA sends commands to viewers or editors. It does not send scroll, selection, or highlight commands to KMM.

## Conditions For Keeping Existing KCF Export

Move existing KCF export to KDV only after all of the following are true:

- KMM `v0.1.0` is published
- KMM public DTO and metadata APIs are fixed as described in this document
- KUW or another explicit widget boundary is defined
- KDV can adopt `KmmDocument` as viewer/export input
- KLE can adopt `MetadataReconcileRequest` / `MetadataReconcileResult` as the save-time contract

## Prohibited

- exposing KMM parser internals through downstream public APIs
- creating independent downstream metadata schemas
- deleting unresolved metadata during save
- letting KCF define new export or paging metadata contracts before KMM/KDV
- moving editor-viewer synchronization control into KMM, KLE, or KDV
- silently absorbing shared metadata widget responsibility into KatanA itself
