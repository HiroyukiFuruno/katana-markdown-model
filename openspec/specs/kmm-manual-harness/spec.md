# kmm-manual-harness Specification

## Purpose
TBD - created by archiving change prepare-manual-harness. Update Purpose after archive.
## Requirements
### Requirement: KMM provides a structure inspection harness

KMM SHALL provide a development-only harness for structure inspection.

#### Scenario: Developer starts the harness

- **WHEN** a developer runs `just harness-up`
- **THEN** a structure inspection environment starts
- **THEN** the environment shows Markdown input, KMM nodes, selected node source mapping, and metadata resolution details
- **THEN** the default input is `/Users/hiroyuki_furuno/works/private/katana/assets/fixtures/sample.md` when it exists

### Requirement: KMM keeps the harness outside the published library

KMM MUST keep the manual harness outside the published library contract.

#### Scenario: KMM is packaged

- **WHEN** `cargo package --locked --allow-dirty --list` is run
- **THEN** product CLI and product UI targets are not added to the KMM crate
- **THEN** harness-only files do not become KMM public API

### Requirement: KMM uses harness confirmation as a release quality gate

KMM SHALL treat fixture tests as the release readiness source of truth.

#### Scenario: v0.1.0 release is prepared

- **WHEN** KMM prepares the `v0.1.0` release
- **THEN** representative fixtures are checked through automated tests
- **THEN** `just harness-up` remains optional inspection support

