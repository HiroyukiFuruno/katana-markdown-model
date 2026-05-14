---
name: commit_and_push
description: katana-markdown-model の変更を検証後にcommitとpushへ進める。
---

# Commit And Push

## 前提

ユーザーがcommitまたはpushを明示している場合だけ実行します。

## 手順

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
git status --short
git diff --stat
just check
```

検証が通ったら、関心事ごとにstageしてcommitします。

## 禁止

- `--no-verify` によるhook回避
- 失敗したlint、AST lint、test、OpenSpec検証の無視
- 無関係な未追跡ファイルの混入
