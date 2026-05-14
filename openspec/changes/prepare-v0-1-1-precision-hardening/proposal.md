## Why

`v0.1.0` はKMMの初回公開境界を固定するリリース（release）である。

parser精度、metadata照合精度、同期anchor材料の追加は、公開境界を動かす可能性があるため `v0.1.0` には混ぜない。

`v0.1.1` では、KDV、KLE、KatanAが `v0.1.0` を採用した結果から、KMM側で直すべき精度問題だけを小さく扱う。

## What Changes

- `v0.1.1` を精度向上releaseとして計画する。
- KMMの責務を解析、文書構造、source mapping、metadata解決、同期材料に限定したまま維持する。
- downstream採用結果をfixture testへ落とし込んでから修正する。
- footnote、image、link、HTML inline、math inlineの専用DTOが必要かを判断する。
- metadata target移動判定と同期目印（anchor）材料の不足を、KMMの公開契約（public contract）内で改善する。

## Impact

- `docs/roadmap-v0.1.1.md`
- `tests/fixtures/**`
- `tests/canonical_fixtures.rs`
- `tests/parser_adapter_contract.rs`
- `tests/metadata_resolution.rs`
- `src/document/**`
- `src/metadata/**`
- `src/parser/**`
