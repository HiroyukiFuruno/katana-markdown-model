# Project Rules

- This repository is library-only. Do not add a CLI or binary target unless an OpenSpec change explicitly changes that boundary.
- KMM owns renderer-neutral document model DTOs. Do not expose third-party parser AST types as public contract.
- KMM must not depend on KatanA, kcf, kdp, kle, or UI frameworks.
- Use `katana-ast-lint` for shared AST lint. Do not add repository-local bypass lint as the baseline.
- Use `scripts/openspec` instead of calling a bare `openspec` command from agent instructions or local workflow docs.
- Run `just check` before commit, push, or handoff.
- Do not bypass `lefthook`, Clippy, AST lint, tests, or OpenSpec validation.
