## ADDED Requirements

### Requirement: KMM publishes after release merge

KMM SHALL start the Release workflow after a release PR is merged into `master`.

#### Scenario: Release commit reaches master

- **WHEN** a `master` push contains the release workflow, crate version, lockfile, or release note paths
- **THEN** KMM starts the Release workflow
- **THEN** KMM resolves the release version from `Cargo.toml`
- **THEN** KMM publishes the crate and creates the GitHub Release when that release does not already exist

### Requirement: KMM keeps manual release dispatch

KMM SHALL keep manual release dispatch for explicit reruns and failure recovery.

#### Scenario: Release is manually dispatched

- **WHEN** a developer starts the Release workflow manually
- **THEN** KMM uses the provided `version` input
- **THEN** KMM verifies that `Cargo.toml` matches the requested version
- **THEN** KMM respects `publish_crate=false` as a dry-run-only execution

### Requirement: KMM uses version-only GitHub Release titles

KMM SHALL use the release tag as the GitHub Release title.

#### Scenario: GitHub Release is created

- **WHEN** KMM creates a GitHub Release for `vX.Y.Z`
- **THEN** the GitHub Release title is `vX.Y.Z`
- **THEN** the title does not include the crate name
