## ADDED Requirements

### Requirement: KMM parses Japanese nested list items inside alerts without panic

KMM SHALL parse GitHub alert blockquotes containing Japanese nested list items without slicing UTF-8 strings at invalid byte boundaries.

#### Scenario: Alert contains Japanese list item

- **WHEN** Markdown contains `> [!NOTE]` with a nested Japanese list item
- **THEN** KMM returns an alert node
- **THEN** KMM returns nested list content under the alert
- **THEN** KMM does not panic

### Requirement: KMM exposes nested inline span children

KMM SHALL expose nested inline spans through `KmmNode.children`.

#### Scenario: Strong span contains emphasis span

- **WHEN** Markdown contains strong text with nested emphasis
- **THEN** KMM returns a strong child node
- **THEN** that strong node contains emphasis and text child nodes
- **THEN** KDV can build nested HTML without reparsing the strong text

### Requirement: KMM exposes one-line dollar math blocks

KMM SHALL expose one-line dollar math blocks as `DollarMathBlock`.

#### Scenario: One-line dollar math block is parsed

- **WHEN** Markdown contains `$$ expression $$` as a whole line
- **THEN** KMM returns a `DollarMathBlock`
- **THEN** the expression excludes the surrounding `$$` delimiters

### Requirement: KMM keeps KatanA Japanese sample as copied AST regression fixture

KMM SHALL verify KatanA Japanese sample AST behavior using a copied fixture inside the KMM repository.

#### Scenario: Copied Japanese sample fixture covers KDV export regressions

- **WHEN** the regression test reads `tests/fixtures/canonical/katana_sample_ja.md`
- **THEN** KMM parses the fixture without relying on sibling repository relative paths
- **THEN** the fixture exposes nested inline span and one-line dollar math DTOs
