## ADDED Requirements

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
