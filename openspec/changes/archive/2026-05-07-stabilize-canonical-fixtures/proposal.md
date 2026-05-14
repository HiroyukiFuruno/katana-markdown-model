## Why

KMM v0はCommonMark完全準拠より、現在KatanAで実現できているMarkdown挙動の踏襲を優先する。

軽量fixtureだけを正本にすると、KatanA現行挙動との差分を見落とす。

## What Changes

- KatanA現行fixtureをKMM側のcanonical fixtureとして扱う。
- README badge、alert、description listを必須fixtureとして固定する。
- node種別、source range、raw snippet、fingerprintをテストで固定する。
- canonical fixtureの同期方法を決める。

## Impact

- `tests/fixtures/**`
- `tests/document_model.rs`
- fixture同期手順の文書
- KMM public DTOの安定化
