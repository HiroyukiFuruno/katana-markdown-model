## ADDED Requirements

### Requirement: KMM defines downstream handoff contracts

KMM SHALL define handoff contracts for downstream repositories after the v0.1.0 API boundary is stable.

#### Scenario: Downstream starts KMM adoption

- **WHEN** KDV、KLE、KCF、またはKatanAがKMM adoptionを始める
- **THEN** it consumes KMM-owned public DTOs
- **THEN** it does not depend on KMM parser internals
- **THEN** it uses `docs/downstream-handoff.md` as the handoff source

### Requirement: KMM prevents parallel metadata schemas

KMM SHALL define metadata contracts before downstream repositories implement metadata behavior.

#### Scenario: Editor or export needs metadata

- **WHEN** KLE、KDV、KCF、またはKatanAがmetadata target behaviorを必要とする
- **THEN** it uses the KMM metadata contract
- **THEN** it does not create a substitute schema
- **THEN** unresolved or conflicted metadata is not deleted automatically

### Requirement: KMM separates KDV viewer/export from KCF external rendering

KMM SHALL define KDV as the Markdown viewer/export consumer and KCF as the external rendering consumer.

#### Scenario: KDV and KCF consume KMM

- **WHEN** KDV consumes KMM
- **THEN** KDV uses `KmmDocument` as viewer/export input
- **WHEN** KCF consumes KMM-related information
- **THEN** KCF only receives external rendering inputs or KMM metadata purposes needed for external rendering
- **THEN** KCF does not add new HTML/PDF/PNG/JPG export ownership after the KDV boundary is defined

### Requirement: KatanA owns editor-viewer synchronization control

KMM SHALL provide synchronization materials without owning synchronization control.

#### Scenario: KatanA coordinates editor and viewer

- **WHEN** KatanA needs to align editor and viewer state
- **THEN** KatanA uses KMM node id, source range, line-column, raw snippet, and fingerprint
- **THEN** KatanA sends scroll, selection, or highlight commands to the viewer or editor
- **THEN** KatanA does not send viewer/editor control commands to KMM

### Requirement: KMM defines repository-specific downstream inputs

KMM SHALL define the minimum downstream input per repository.

#### Scenario: Downstream repository consumes KMM

- **WHEN** KDV consumes KMM
- **THEN** KDV uses `KmmDocument` as viewer/export input
- **WHEN** KLE consumes KMM
- **THEN** KLE uses `MetadataReconcileRequest` and `MetadataReconcileResult` for save-time metadata sync
- **WHEN** KCF consumes KMM-related inputs
- **THEN** KCF treats Mermaid、Draw.io、PlantUML、math as external rendering inputs
