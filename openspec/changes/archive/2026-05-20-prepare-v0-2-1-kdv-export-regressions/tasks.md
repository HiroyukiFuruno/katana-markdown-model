# Tasks: prepare-v0-2-1-kdv-export-regressions

## Definition of Ready

- [x] issue #3 が KMM parser panic として報告されている
- [x] issue #4 が KMM public DTO gap として報告されている
- [x] KDV は raw Markdown を独自に再パースしない
- [x] `v0.2.1` は最新 `v0.2.0` から見て自然な patch release

## Tasks: Implementation

- [x] 1.1 issue #3 の GitHub alert + 日本語 list item を regression test にする
- [x] 1.2 copied KatanA Japanese sample fixture で nested inline span を regression test にする
- [x] 1.3 copied KatanA Japanese sample fixture で one-line dollar math block を regression test にする
- [x] 1.4 quote 内 list child の UTF-8 source span を修正する
- [x] 1.5 nested inline children を `KmmNode.children` で返す
- [x] 1.6 one-line dollar math block を `DollarMathBlock` として返す
- [x] 1.7 `Cargo.toml` と release note を `v0.2.1` に更新する
- [x] 1.8 release readiness を記録する

## Definition of Done

- [x] issue #3 の reproduction が panic なしに parse される
- [x] issue #4 の nested inline が raw Markdown 再パースなしに取得できる
- [x] issue #4 の one-line dollar math が `DollarMathBlock` として取得できる
- [x] KMM remains library-only
- [x] KMM does not own HTML rendering or UI control
- [x] `scripts/openspec validate "prepare-v0-2-1-kdv-export-regressions" --strict` passes
- [x] `just check` passes
- [x] `just release-check` passes

## Verification

- [x] `cargo test --test kdv_export_v0_2_1_regressions --locked`
- [x] `scripts/openspec validate "prepare-v0-2-1-kdv-export-regressions" --strict`
- [x] `just check`
- [x] `just release-check`
