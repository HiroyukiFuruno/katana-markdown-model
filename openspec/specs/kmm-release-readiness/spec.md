# kmm-release-readiness Specification

## Purpose
KMM の release 前検証、release PR preflight、merge 後 publication automation、手動 rerun 経路を固定する。

## Requirements
### Requirement: KMM defines a release readiness gate

KMM SHALL provide a release readiness gate before crates.io publication.

#### Scenario: Run release readiness locally

- **WHEN** a developer runs `just release-check`
- **THEN** KMM runs the normal quality gate
- **THEN** KMM packages the crate with `cargo package --locked --allow-dirty`
- **THEN** KMM runs `cargo publish --dry-run --locked --allow-dirty`
- **THEN** KMM does not publish the crate

### Requirement: KMM keeps release preflight wired into CI

KMM SHALL run release readiness checks in GitHub Actions before release branches can merge.

#### Scenario: Release pull request is opened

- **WHEN** a pull request targets `master`
- **THEN** the release preflight workflow runs the release readiness gate
- **THEN** branch protection can require the workflow check by name

### Requirement: KMM uses library-only release checks

KMM MUST NOT add distribution checks that belong to non-library channels unless a later OpenSpec changes that boundary.

#### Scenario: Release readiness is expanded

- **WHEN** KMM adds release readiness checks
- **THEN** npm, PyPI, Homebrew, binary artifact, MCPB, and editor extension checks are out of scope
- **THEN** KMM keeps the release gate focused on the Rust library crate

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
