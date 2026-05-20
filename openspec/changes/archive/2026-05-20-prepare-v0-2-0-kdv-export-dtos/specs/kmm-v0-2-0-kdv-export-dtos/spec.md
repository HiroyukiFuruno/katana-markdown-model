## ADDED Requirements

### Requirement: KMM exposes renderer-neutral inline spans

KMM SHALL expose inline Markdown spans as renderer-neutral public DTOs.

#### Scenario: KDV reads paragraph inline spans

- **WHEN** a paragraph contains strong, emphasis, strikethrough, inline code, inline HTML, or plain text
- **THEN** KMM returns child nodes that identify those spans without requiring raw Markdown reparsing
- **THEN** each child node includes its own source span

### Requirement: KMM exposes link, image, footnote, and math DTOs

KMM SHALL expose link, image, footnote, inline math, and dollar math block structures as public DTOs.

#### Scenario: KDV exports links, images, footnotes, and math

- **WHEN** Markdown contains an inline link, autolink, image, footnote reference, footnote definition, inline math, or dollar math block
- **THEN** KMM returns renderer-neutral DTOs for those structures
- **THEN** KDV can choose HTML output without reinterpreting raw Markdown

### Requirement: KMM exposes list item body and nested blocks

KMM SHALL expose list item body, marker, task marker, and nested block children as public DTOs.

#### Scenario: KDV exports nested lists and list code blocks

- **WHEN** Markdown contains a nested list or a code block inside a list item
- **THEN** KMM returns the list item marker and body inline spans
- **THEN** KMM returns nested block children under the list item

### Requirement: KMM exposes blockquote children

KMM SHALL expose blockquote children as public DTOs.

#### Scenario: KDV exports blockquote contents

- **WHEN** Markdown contains a blockquote with decorated text, nested list, or code block
- **THEN** KMM returns the blockquote as a block node with child blocks
- **THEN** KMM does not require KDV to parse blockquote raw text

### Requirement: KMM keeps rendering outside its responsibility

KMM MUST NOT add rendering, export, UI, or synchronization-control behavior while adding v0.2.0 DTOs.

#### Scenario: A downstream repository needs HTML output

- **WHEN** a downstream repository creates HTML, PDF, PNG, JPG, scroll commands, selection, or highlight behavior
- **THEN** KMM only supplies renderer-neutral document data
- **THEN** the downstream repository owns the output or control behavior
