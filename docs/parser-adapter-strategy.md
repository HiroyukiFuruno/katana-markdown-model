# Parser Adapter Strategy

## Summary

`v0.1.0` keeps the current parser.

The goal is not full CommonMark coverage. The goal is to fix the KMM public DTO and source-mapping contract. A later parser-engine replacement is possible, but it must not break the public contract or contract tests.

## Boundary

KMM public APIs return only KMM-owned DTOs.

- `KmmDocument`
- `KmmNode`
- `KmmNodeKind`
- `SourceSpan`
- `MetadataDocument`
- `TargetResolution`

`src/parser/**` is internal implementation. Downstream repositories must not depend on parser internals.

## Parse Contract Fixed in v0.1.0

- tables preserve rows, cells, alignment, and cell source ranges
- README badge rows are preserved as `HtmlBlockRole::BadgeRow`
- centered HTML blocks are preserved as `HtmlBlockRole::Centered`
- legacy note blocks and GFM alert blocks preserve the `Alert` label
- description lists preserve terms and descriptions
- footnote syntax is not modeled by a dedicated DTO, but remains in paragraph raw snippets
- Mermaid, PlantUML, and DrawIo are preserved as diagram code blocks
- math fenced blocks are preserved as math code blocks
- inline math is not modeled by a dedicated DTO, but remains in paragraph raw snippets
- Unicode emoji and shortcode emoji are preserved in KMM nodes

## Parser Candidate Evaluation

KMM does not currently adopt third-party parsers such as Comrak, pulldown-cmark, or markdown-rs.

Any replacement parser must first be evaluated against canonical fixtures and contract tests for these conditions:

- stable conversion of source ranges and raw snippets into KMM `SourceSpan`
- no regression for README badges, alerts, description lists, tables, diagrams, or math
- Unicode emoji and shortcode emoji are preserved
- parser AST types do not leak into public APIs
- the document model does not vary by OS or font

## Replacement Conditions

Before replacing the parser engine, run:

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
cargo test --test parser_adapter_contract --locked
cargo test --test canonical_fixtures --locked
just check
```

Parser candidates that fail this verification are rejected.
