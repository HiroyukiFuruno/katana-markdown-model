# Quality Gates

## Local Targets

| Target | Responsibility | Blocking |
| --- | --- | --- |
| `just fmt-check` | Verify rustfmt output is committed | Yes |
| `just lint` | Run Clippy with zero warnings | Yes |
| `just ast-lint` | Run shared AST lint through KAL | Yes |
| `just test` | Run unit and integration tests | Yes |
| `just openspec-check` | Validate active OpenSpec change | Yes |
| `just check` | Run all local gates above | Yes |
| `just release-check` | Run local release-readiness checks without publishing | Yes for release branches |
| `preflight` | Run PR release-readiness checks in GitHub Actions | Yes |

## AST Lint

KMMは共有品質ゲートとして `katana-ast-lint` を使う。repository側の実行入口は
`tests/repository_ast_lint.rs` の1行KAL runnerとする。

KMM must not introduce a separate local lint baseline to avoid the shared rules.

## CI Required Checks

Branch protection for `master` should require:

- `Test and Build (macos-latest)`
- `Test and Build (ubuntu-latest)`
- `Test and Build (windows-latest)`
- `preflight`

If workflow job names change, update branch protection in the same change. A
passing local target is not enough when GitHub no longer requires the matching
check.

2026-05-15時点で、GitHubの `master` ブランチ保護（branch protection）は上記4checkを必須にしている。

`master` へのpushとPRでは、Rust、Justfile、agent手順、workflow、README、docs、OpenSpec、scripts、tests、toolsの変更で `Test and Build` を起動する。
必須check名を変える場合は、同じ変更内でGitHubの `master` ブランチ保護（branch protection）も更新する。

## Release Readiness

KMM has a release-readiness preflight and a manual `release` workflow for the
actual publication step. Keep `just release-check` green locally. It runs
`just check`, `cargo package --locked --allow-dirty` and
`cargo publish --dry-run --locked --allow-dirty`.

`just release-check` and the `preflight` workflow must stay aligned. KMM is
library-only, so npm, PyPI, Homebrew, binary artifact, MCPB, and editor
extension release checks are out of scope unless a later OpenSpec changes that
boundary.

Before `v0.1.0` publication, register `CARGO_REGISTRY_TOKEN` as a GitHub secret.
Do not publish until the prerequisite OpenSpec changes in `docs/roadmap.md` are
complete. The release workflow must be dispatched from `master`.
