# katana-markdown-model OpenSpec

## Project

`katana-markdown-model` (KMM) owns the Markdown document model, external metadata, and source-position resolution for the KatanA ecosystem.

KMM is not an HTML converter. It is the shared interpretation layer used by KatanA, katana-document-viewer, katana-language-editor, and katana-canvas-forge.

KMM is the P1 separation target and assumes the shared P0 quality gate provided by `katana-ast-lint`.

## Design Principles

- KMM does not depend on KCF, KDV, KatanA, or editor implementations.
- KMM does not introduce repository-local temporary AST lint; it uses shared `katana-ast-lint` rules and adapters.
- Existing Markdown library ASTs do not become public contracts.
- KatanA-specific metadata is not embedded into Markdown content.
- Current KatanA Markdown behavior defines the v0 compatibility line.
- Emoji is not removed. KMM preserves Unicode and shortcode information, while rendering remains the consumer's responsibility.
- Mermaid, draw.io, PlantUML, and math are preserved in a structure that KCF can render; KMM does not render them.
- KatanA owns editor-viewer synchronization control. KMM does not synchronize anything; it returns node ids, source ranges, line-column positions, raw snippets, and fingerprints that consumers can use.

## Canonical Fixtures

- `/Users/hiroyuki_furuno/works/private/katana/assets/fixtures/sample.md`
- badge row at the beginning of `/Users/hiroyuki_furuno/works/private/katana/README.md`
- alert syntax in `/Users/hiroyuki_furuno/works/private/katana/assets/fixtures/sample_basic.md`
- description-list fixture

## Consumers

- KatanA
- katana-document-viewer
- katana-language-editor
- katana-canvas-forge
