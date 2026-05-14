# Quality Gates

## Local Targets

| Target | Responsibility | Blocking |
| --- | --- | --- |
| `just fmt-check` | Verify rustfmt output is committed | Yes |
| `just lint` | Run Clippy with zero warnings | Yes |
| `just ast-lint` | Run shared AST lint through KAL | Yes |
| `just test` | Run unit and integration tests | Yes |
| `just openspec-check` | Validate active OpenSpec changes | Yes |
| `just check` | Run all local gates above | Yes |
| `just release-check` | Run local release-readiness checks without publishing | Yes for release branches |
| `preflight` | Run PR release-readiness checks in GitHub Actions | Yes |

## AST Lint

KMM uses `katana-ast-lint` as the shared quality gate. The repository entry point is the one-line KAL runner in `tests/repository_ast_lint.rs`.

KMM must not introduce a separate local lint baseline to avoid the shared rules.

## CI Required Checks

Branch protection for `master` must require:

- `Test and Build (macos-latest)`
- `Test and Build (ubuntu-latest)`
- `Test and Build (windows-latest)`
- `preflight`

If workflow job names change, update branch protection in the same change. A passing local target is not enough when GitHub no longer requires the matching check.

As of 2026-05-15, GitHub branch protection for `master` requires the four checks above.

Pushes to `master` and pull requests run `Test and Build` for changes to Rust, Justfile, agent instructions, workflows, README, docs, OpenSpec, scripts, tests, and tools.
If a required check name changes, update GitHub branch protection for `master` in the same change.

## Release Readiness

KMM has a release-readiness preflight and a manual `release` workflow for the actual publication step. Keep `just release-check` green locally. It runs `just check`, `cargo package --locked --allow-dirty`, and `cargo publish --dry-run --locked --allow-dirty`.

`just release-check` and the `preflight` workflow must stay aligned. KMM is library-only, so npm, PyPI, Homebrew, binary artifact, MCPB, and editor extension release checks are out of scope unless a later OpenSpec changes that boundary.

`CARGO_REGISTRY_TOKEN` must be registered as a GitHub secret before publication. The release workflow must be dispatched from `master`.
