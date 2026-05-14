## ADDED Requirements

### Requirement: KMM defines canonical fixtures

KMM SHALL define canonical fixtures for current KatanA-supported Markdown behavior.

#### Scenario: Parse canonical sample

- **WHEN** KMM parses the canonical KatanA sample fixture
- **THEN** the resulting document contains the expected KMM-owned node kinds
- **THEN** each asserted node keeps source range, raw snippet, and fingerprint

### Requirement: KMM avoids absolute-path test dependency

KMM MUST NOT require a developer-specific absolute path to run standard fixture tests.

#### Scenario: Run fixture tests in a clean checkout

- **WHEN** fixture tests run in the KMM repository
- **THEN** the tests use repository-local fixture inputs or a documented sync result
- **THEN** tests do not require `/Users/hiroyuki_furuno/works/private/katana` to exist
