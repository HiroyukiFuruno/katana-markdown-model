## Context

初期parserはpublic DTOとsource mappingの契約を固定するための最小実装である。

後続でparser engineを差し替えても、public DTOとtest contractを壊してはならない。

## Goals

- parser adapter境界を固定する。
- third-party parser ASTをpublic contractへ出さない。
- KatanA現行挙動を表すparse contractをテストで固定する。

## Non-Goals

- CommonMark全仕様の実装。
- renderer実装。
- downstream統合。

## Evaluation Policy

parser候補は次の条件で評価する。

- public DTOへ内部AST型が漏れない
- source rangeとraw snippetを保持できる
- README badge、alert、description list、table、diagram、mathを壊さない
- Unicode emojiとshortcode emojiを削除しない
- OSやfontに依存してKMMの文書モデルが変わらない

## Decision

`v0.1.0` では現行parserを維持する。

Comrak、pulldown-cmark、markdown-rsなどのthird-party parserは、canonical fixtureとcontract testでsource range、raw snippet、emoji、table、badge、alert、description list、diagram、mathを維持できることを確認するまで採用しない。

詳細な評価方針は `docs/parser-adapter-strategy.md` を正とする。

## Contract Tests

`tests/parser_adapter_contract.rs` は、parser engine差し替え時にも壊してはいけない最低限のcontractを固定する。

対象:

- table
- README badge row
- alert
- description list
- footnote記法のraw保持
- diagram fenced block
- math fenced blockとinline mathのraw保持
- Unicode emojiとshortcode emoji

## Verification

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
cargo test --test parser_adapter_contract --locked
scripts/openspec validate "lock-parser-adapter-strategy" --strict
just check
```
