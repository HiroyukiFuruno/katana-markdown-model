## ADDED Requirements

### Requirement: KMM releases v0.1.0 only after prerequisite changes are complete

KMM MUST publish `v0.1.0` only after all prerequisite OpenSpec changes are complete.

#### Scenario: v0.1.0 release is requested early

- **WHEN** any prerequisite OpenSpec change still has incomplete release-blocking tasks
- **THEN** KMM does not publish `v0.1.0`
- **THEN** the remaining prerequisite is reported instead

### Requirement: KMM uses master-based release branches after v0.1.0

KMM SHALL use KML-style release branches with `master` as the default branch.

#### Scenario: Prepare a release after v0.1.0

- **WHEN** KMM prepares a release
- **THEN** the integration branch is named `release/vX.Y.Z`
- **THEN** any auxiliary branch is named `feature/vX.Y.Z-<short-slug>`
- **THEN** the release pull request targets `master`

### Requirement: KMM verifies publication after release

KMM SHALL verify GitHub Release and crates.io publication after `v0.1.0` is published.

#### Scenario: v0.1.0 publication completes

- **WHEN** the release workflow finishes
- **THEN** KMM verifies the GitHub Release target
- **THEN** KMM verifies that crates.io exposes `katana-markdown-model` version `0.1.0`

### Requirement: KMM documents release execution before publication

KMM SHALL document release execution before `v0.1.0` is published.

#### Scenario: Release PR is prepared

- **WHEN** `release/v0.1.0` is prepared for `master`
- **THEN** `docs/release-runbook.md` defines the release PR, secret, publication, failure, verify, and branch hygiene steps
- **THEN** `docs/release-notes/v0.1.0.md` exists
- **THEN** real publication tasks remain incomplete until after merge
