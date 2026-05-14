---
name: openspec-apply-change
description: katana-markdown-model の OpenSpec change を読み、tasks.md に沿って実装する。
---

# OpenSpec Apply Change

## 手順

1. `openspec/changes/<change>/proposal.md`、`design.md`、`tasks.md`、`specs/**/spec.md` を読む。
2. 完了したtaskだけを `[x]` にする。
3. 新しい判断や未解決事項はOpenSpecへ追記する。
4. 実装後に検証する。

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
scripts/openspec validate "<change>" --strict
just check
```

## 注意

- KMMはlibrary-onlyです。
- 既存parser ASTをpublic DTOにしません。
- KALのAST lintを品質ゲートにします。
