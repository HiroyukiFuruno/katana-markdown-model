## Why

KDV の downstream RED test から、KMM v0.2.0 の公開データ型（public DTO）と parser に 2 件の regression が見つかった。

- issue #3: GitHub alert blockquote 内の日本語 list item で UTF-8 byte boundary panic が起きる。
- issue #4: nested inline span と one-line dollar math block を KDV が raw Markdown 再パースなしに扱えない。

どちらも KMM 側の renderer-neutral DTO / parser responsibility に属するため、`v0.2.1` patch release として対応する。

## What Changes

- GitHub alert blockquote 内の日本語 list item を panic なしに parse する。
- blockquote / alert children の nested list source span を UTF-8 char boundary に合わせる。
- strong / emphasis / strikethrough の nested inline children を公開 DTO として返す。
- `$$ ... $$` の one-line dollar math block を `DollarMathBlock` として返す。
- KatanA の `sample.ja.md` を `tests/fixtures/canonical/katana_sample_ja.md` へコピーし、外部パス参照なしの AST regression fixture として扱う。
- `Cargo.toml` と release note を `v0.2.1` に更新する。

## Impact

- `Cargo.toml`
- `Cargo.lock`
- `src/parser/**`
- `tests/**`
- `docs/release-notes/v0.2.1.md`
- `docs/release-readiness/v0.2.1-kdv-export-regressions.md`
