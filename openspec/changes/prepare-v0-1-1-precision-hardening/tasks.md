# Tasks: prepare-v0-1-1-precision-hardening

## Definition of Ready

- [x] `v0.1.0` はKMMの初回公開境界を固定するリリース（release）である
- [x] `v0.1.0` にはparser精度、metadata照合精度、同期anchor材料の追加を混ぜない方針である
- [x] KMMは描画、export、同期制御を持たない方針である

## Tasks: 計画

- [x] 1.1 `v0.1.1` を精度向上releaseとして位置付ける
- [x] 1.2 `v0.1.1` の対象と対象外を文書化する
- [x] 1.3 downstream採用結果をKMM責務へ分類する流れを定義する
- [x] 1.4 fixture-firstで修正する方針を定義する
- [x] 1.5 `v0.1.1` のrelease policyを定義する

## Tasks: 実装

- [ ] 2.1 KDV、KLE、KatanAで `katana-markdown-model = "0.1.0"` を採用した結果を棚卸しする
- [ ] 2.2 KMM解析精度、metadata解決、同期材料、downstream責務へ問題を分類する
- [ ] 2.3 KMMで扱う問題をfixture testで再現する
- [ ] 2.4 source range、line-column、raw snippet、fingerprintのズレを修正する
- [ ] 2.5 footnote、image、link、HTML inline、math inlineの専用DTO要否を判断する
- [ ] 2.6 専用DTOが必要な場合、既存public DTOを壊さず追加する
- [ ] 2.7 metadata target移動判定の精度を改善する
- [ ] 2.8 曖昧なmetadata復元では `Conflict` を返す回帰テストを維持する
- [ ] 2.9 復元不能なmetadataでは `Unresolved` を返す回帰テストを維持する
- [ ] 2.10 editor-viewer同期で不足したanchor材料があれば、KMM public DTOとして必要最小限で追加する
- [ ] 2.11 KMMがviewerまたはeditorへ命令しないことを文書とtestで確認する

## Definition of Done

- [ ] downstream採用結果のうち、KMMで扱う問題だけがtask化されている
- [ ] 追加・修正した挙動がfixture testで固定されている
- [ ] KMMがlibrary-onlyを維持している
- [ ] 公開契約（public contract）に第三者parser ASTが漏れていない
- [ ] KMMが描画、export、同期制御を持っていない
- [ ] `just check` が成功している
- [ ] `just release-check` が成功している

## Verification

- [x] `scripts/openspec validate "prepare-v0-1-1-precision-hardening" --strict`
- [x] `just check`
- [x] `just release-check`
