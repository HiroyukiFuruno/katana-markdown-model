## Context

既存の `tests/fixtures/` には軽量fixtureがある。

次に必要なのは、KatanA現行fixtureをKMMが再現すべき正本として固定することである。

## Goals

- canonical fixtureの同期方法を決める。
- KatanA現行fixtureの主要構造をKMM nodeとして固定する。
- source range、raw snippet、fingerprintを回帰テストにする。

## Non-Goals

- CommonMark全仕様の実装。
- downstream repositoryへの組み込み。
- parser engineの最終選定。

## Fixture Sources

- `/Users/hiroyuki_furuno/works/private/katana/assets/fixtures/sample.md`
- `/Users/hiroyuki_furuno/works/private/katana/assets/fixtures/sample_basic.md`
- `/Users/hiroyuki_furuno/works/private/katana/README.md` 冒頭のbadge列
- description list fixture

KMMの標準テストは絶対パスへ依存しない。

## Verification

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
scripts/openspec validate "stabilize-canonical-fixtures" --strict
just check
```
