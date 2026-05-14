---
name: lint-and-ast-lint
description: katana-markdown-model の通常lint、KAL AST lint、test、OpenSpec検証を実行する。
---

# Lint And AST Lint

KMMの品質ゲートを確認します。

## 実行順序

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
just check
```

`just check` は以下を実行します。

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
- `cargo test --test repository_ast_lint --locked -- --nocapture`
- `cargo test --workspace --locked`
- `scripts/openspec validate "bootstrap-kmm-document-model" --strict`

## ルール

- 失敗した検査をskipしない。
- AST lintは `katana-ast-lint` を使う。
- KMM固有の一時lintや除外設定で代替しない。
