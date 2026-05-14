## ADDED Requirements

### Requirement: KMM exposes a renderer-neutral document model

KMM SHALL parse supported Markdown fixtures into a renderer-neutral document model.

#### Scenario: Parse canonical fixture

- **WHEN** KMM parses KatanA `assets/fixtures/sample.md`
- **THEN** the resulting model contains block and inline nodes for current KatanA-supported Markdown behavior
- **THEN** each node has a stable id, source range, line-column range, and raw snippet

### Requirement: KMM uses shared AST lint governance

KMM SHALL use the shared P0 AST lint governance as its quality gate.

#### Scenario: Run KMM quality gate

- **WHEN** KMM runs AST lint
- **THEN** it uses `katana-ast-lint` common rules through a KMM repository adapter
- **THEN** KMM does not introduce a repository-local lint drift as the baseline

### Requirement: KMM keeps metadata external

KMM MUST treat metadata as an external document associated with Markdown source.

#### Scenario: Resolve metadata target

- **WHEN** KMM receives Markdown source and `README.md.metadata.json`
- **THEN** KMM resolves metadata targets to document nodes where possible
- **THEN** unresolved targets are returned without deletion

### Requirement: KMM does not expose parser internals

KMM MUST NOT expose third-party parser AST types as its public contract.

#### Scenario: Downstream consumes KMM model

- **WHEN** KDV、KLE、KCF、またはKatanAがKMM outputを消費する
- **THEN** it receives KMM-owned DTOs
- **THEN** it does not depend on Comrak, pulldown-cmark, markdown-rs, or vendor parser internals

### Requirement: KMM is the cross-repository starting point

KMM SHALL define the document model and metadata contracts before downstream viewer, editor, export, external rendering, or integration work proceeds.

#### Scenario: Downstream repository starts KMM adoption

- **WHEN** KDV、KLE、KCF、またはKatanAがKMM adoption workを始める
- **THEN** KMM public DTOs are available
- **THEN** KMM metadata target resolution is available when metadata behavior is required
- **THEN** the downstream repository does not create its own document model or metadata schema as a substitute

### Requirement: KCF remains external-rendering focused after KDV owns export

KMM SHALL treat KCF as the owner of Mermaid, Draw.io, PlantUML, math, and other external rendering, while KDV owns viewer/export.

#### Scenario: KCF work resumes after KDV boundary is defined

- **WHEN** KCF resumes KMM-related work
- **THEN** it treats KDV as the owner of HTML/PDF/PNG/JPG export
- **THEN** it keeps existing export only until KDV provides equivalent behavior
- **THEN** it does not define KMM metadata, viewer, editor, or UI widget behavior on its own

### Requirement: KMM does not own editor-viewer synchronization control

KMM SHALL provide source mapping and fingerprint data that KatanA can use for synchronization without owning synchronization state or commands.

#### Scenario: KatanA coordinates viewer and editor

- **WHEN** KatanA aligns viewer and editor state
- **THEN** KatanA uses KMM node id, source range, line-column, raw snippet, and fingerprint
- **THEN** KatanA sends commands to the viewer or editor
- **THEN** KMM does not know viewer state, editor state, scroll position, selection, or highlight state

### Requirement: KMM tracks missing KUW as a planning risk

KMM SHALL track `katana-ui-widget` absence as a cross-repository planning risk.

#### Scenario: Viewer or KatanA integration needs shared UI parts

- **WHEN** viewer, metadata display, tabs, toolbar, copy, or edit affordance work is planned
- **THEN** the work identifies whether it belongs in KUW
- **THEN** KatanA or KDV does not silently absorb shared widget responsibilities

### Requirement: KMM handoff is implementation-ready

KMM SHALL provide enough OpenSpec detail for another session to continue without relying on chat history.

#### Scenario: New session starts KMM work

- **WHEN** a new session begins work in `katana-markdown-model`
- **THEN** it can read `handoff.md`, `tasks.md`, and `spec.md`
- **THEN** it can identify current completion state, next tasks, blockers, and verification commands
