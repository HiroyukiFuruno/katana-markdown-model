---
name: openspec-apply-change
description: Read a katana-markdown-model OpenSpec change and implement it according to tasks.md.
---

# OpenSpec Apply Change

## Steps

1. Read `openspec/changes/<change>/proposal.md`, `design.md`, `tasks.md`, and `specs/**/spec.md`.
2. Mark only completed tasks as `[x]`.
3. Record new decisions or unresolved issues in OpenSpec.
4. Verify after implementation.

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
scripts/openspec validate "<change>" --strict
just check
```

## Notes

- KMM is library-only.
- Do not expose existing parser ASTs through public DTOs.
- Use KAL AST lint as the quality gate.
