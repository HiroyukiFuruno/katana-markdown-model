---
name: commit_and_push
description: Commit and push katana-markdown-model changes after verification.
---

# Commit And Push

## Preconditions

Run this only when the user explicitly asks to commit or push.

## Steps

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
git status --short
git diff --stat
just check
```

After verification passes, stage and commit by concern.

## Prohibited

- bypassing hooks with `--no-verify`
- ignoring failed lint, AST lint, tests, or OpenSpec validation
- mixing unrelated untracked files into the commit
