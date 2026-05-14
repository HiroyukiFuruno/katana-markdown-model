# KMM Responsibility Boundary

## Summary

KMM parses Markdown and returns shared document structure, source mapping, and metadata target resolution for the KatanA ecosystem.

KMM does not own rendering, export, or editor-viewer synchronization control. Synchronization control always belongs to the consuming application, KatanA, which sends commands to viewers or editors.

## KMM Owns

- Markdown document model
- stable node id
- source range
- line-column position
- raw snippet
- text fingerprint
- metadata schema
- metadata target resolution
- parser adapter boundary

These are materials for KLE save-time metadata reconciliation, KDV viewer/export behavior, and KatanA editor-viewer synchronization.

## KMM Does Not Own

- Markdown rendering
- Floem widget
- viewer state
- editor state
- scroll state
- viewport
- hit-test policy
- HTML/PDF/PNG/JPG export
- external rendering for Mermaid, Draw.io, PlantUML, or math
- KatanA workspace state

## Repository Boundary

| repository | responsibility |
| --- | --- |
| `katana-markdown-model` | Markdown parsing, document structure, source mapping, metadata target resolution |
| `katana-document-viewer` | viewer, hit-test, and HTML/PDF/PNG/JPG export from KMM DTOs |
| `katana-canvas-forge` | external rendering for Mermaid, Draw.io, PlantUML, and math |
| `katana-language-editor` | editing surface and save-time metadata reconciliation |
| `katana` | viewer/editor/export integration and editor-viewer synchronization control |

`katana-document-preview` is unreleased and not adopted, so the planned repository name is `katana-document-viewer` (KDV).

## Synchronization Boundary

KMM does not perform synchronization.

KMM returns stable node id, source range, line-column position, raw snippet, and fingerprint data that KatanA can use for synchronization decisions.

KatanA uses that data to send scroll, selection, or highlight commands to viewers or editors. KMM does not know viewer state, editor state, or KatanA integration state.
