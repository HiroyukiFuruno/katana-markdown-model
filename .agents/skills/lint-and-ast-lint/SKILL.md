---
name: lint-and-ast-lint
description: Run katana-markdown-model lint, KAL AST lint, tests, and OpenSpec validation.
---

# Lint And AST Lint

Use this skill to verify the KMM quality gate.

## Execution Order

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
just check
```

`just check` runs:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
- `cargo test --test repository_ast_lint --locked -- --nocapture`
- `cargo test --workspace --locked`
- `scripts/openspec validate <active-change> --strict`

## Rules

- Do not skip failed checks.
- AST lint uses `katana-ast-lint`.
- Do not replace the shared gate with KMM-local temporary lint rules or exclusions.
