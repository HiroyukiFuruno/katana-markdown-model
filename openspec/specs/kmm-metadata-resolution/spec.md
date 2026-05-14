# kmm-metadata-resolution Specification

## Purpose
TBD - created by archiving change finalize-metadata-resolution-contract. Update Purpose after archive.
## Requirements
### Requirement: KMM exposes complete metadata resolution states

KMM SHALL expose metadata resolution states that downstream repositories can consume without inventing a parallel schema.

#### Scenario: Resolve metadata after save

- **WHEN** KMM compares old Markdown, new Markdown, and metadata targets
- **THEN** each target is returned as resolved, moved, unresolved, or conflicted
- **THEN** the result uses KMM-owned public DTOs

### Requirement: KMM preserves unresolved metadata

KMM MUST preserve unresolved metadata entries.

#### Scenario: Target cannot be matched

- **WHEN** a metadata target cannot be resolved after Markdown changes
- **THEN** KMM returns an unresolved result
- **THEN** KMM does not delete the metadata entry

### Requirement: KMM reports ambiguous metadata as conflict

KMM SHALL report ambiguous target recovery as a conflict.

#### Scenario: Multiple candidate nodes match a target

- **WHEN** KMM finds multiple plausible nodes for one metadata target
- **THEN** KMM returns a conflict result
- **THEN** KMM does not silently choose one candidate
