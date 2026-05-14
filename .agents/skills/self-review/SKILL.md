---
name: self-review
description: Self-review katana-markdown-model diffs before commit or handoff.
---

# Self Review

Review the current diff from these perspectives.

## Scope Check

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
git status --short
git diff --stat
```

## Design Check

- KMM does not depend on KatanA, KCF, KDP, KLE, or UI frameworks.
- Third-party parser ASTs do not leak into the public contract.
- Metadata is not embedded into Markdown content.
- Renderer or exporter responsibilities are not moved into KMM.

## Quality Check

```bash
just check
```

Do not commit, push, or hand off while this fails.
