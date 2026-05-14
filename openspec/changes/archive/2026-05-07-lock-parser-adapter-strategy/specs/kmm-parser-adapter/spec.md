## ADDED Requirements

### Requirement: KMM hides parser internals behind an adapter

KMM MUST NOT expose parser engine AST types as public API.

#### Scenario: Downstream consumes parsed document

- **WHEN** downstream code uses KMM parse output
- **THEN** it receives KMM-owned DTOs
- **THEN** it cannot depend on parser engine AST types through the public contract

### Requirement: KMM locks parser behavior with contract tests

KMM SHALL protect KatanA-supported Markdown behavior with parser contract tests.

#### Scenario: Parser engine changes

- **WHEN** parser implementation changes
- **THEN** table, badge, alert, description list, footnote, diagram, math, and emoji contract tests still pass
- **THEN** source range and raw snippet assertions remain stable

### Requirement: KMM preserves emoji model data

KMM SHALL preserve Unicode emoji and shortcode emoji as document model data.

#### Scenario: Markdown contains emoji

- **WHEN** Markdown contains Unicode emoji or shortcode emoji
- **THEN** KMM keeps the emoji information in KMM-owned DTOs
- **THEN** KMM does not make rendering or font fallback decisions
