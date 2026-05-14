# KMM v0.1.1 Roadmap

## 結論

`v0.1.1` は、`v0.1.0` で固定した公開契約（public contract）を壊さず、KDV、KLE、KatanA採用時に見つかった解析精度を小さく改善するリリース（release）である。

KMMは描画、export、editor-viewer同期制御を持たない。この境界は `v0.1.1` でも変えない。

## 目的

`v0.1.0` は初回公開境界を固定するリリース（release）である。

`v0.1.1` では、公開後の利用結果からKMM側で直すべき精度問題だけを扱う。

## 対象

- canonical fixtureで見つかったsource range、line-column、raw snippet、fingerprintのズレ修正
- footnote、image、link、HTML inline、math inlineの追加DTOが必要かの判断
- metadata target移動判定の精度向上
- KatanAのeditor-viewer同期で不足したanchor材料の追加検討

## 対象外

- KMMによるMarkdown描画
- HTML/PDF/PNG/JPG export
- 製品CLI
- 製品UI
- KatanA workspace state
- viewerまたはeditorへのスクロール（scroll）、選択（selection）、強調表示（highlight）命令
- KDV、KLE、KCF、KatanA本体への直接依存

## 進め方

### 1. downstream採用結果の棚卸し

KDV、KLE、KatanAで `katana-markdown-model = "0.1.0"` を使った結果を集める。

KMMで扱うのは、KMM公開DTO（public DTO）、source mapping、metadata解決、同期材料に関する問題だけである。

### 2. fixtureで再現する

修正前に、KMM内のfixture testで問題を再現する。

絶対パスに依存するtestは追加しない。KatanA fixtureを使う場合は、KMM repository内の `tests/fixtures/canonical/` へ同期してから検証する。

### 3. 公開契約（public contract）を守って修正する

第三者parser ASTをpublic DTOへ出さない。

新しいDTOを追加する場合は、KDV、KLE、KatanAが既存DTOでは扱えない理由をOpenSpecへ記録する。

### 4. releaseする

`v0.1.1` は `release/v0.1.1` でまとめ、`master` へリリース変更依頼（release PR）を作る。

公開前には `just release-check` を必須にする。

## 完了条件

- `prepare-v0-1-1-precision-hardening` のOpenSpec taskが完了している
- 追加・修正した挙動がfixture testで固定されている
- KMMがlibrary-onlyを維持している
- 公開契約（public contract）に第三者parser ASTが漏れていない
- KMMが描画、export、同期制御を持っていない
- `just check` が通っている
- `just release-check` が通っている

## 判断基準

`v0.1.1` で採用する修正は、次を満たすものに限定する。

- downstream側の都合ではなく、KMMの解析結果またはmetadata解決結果に原因がある
- fixtureで再現できる
- 公開DTO（public DTO）の互換性を壊さない
- KMMの責務境界を広げない

## 後続候補

`v0.1.1` で扱いきれない精度改善は、`v0.1.2` 以降へ分ける。

対象を広げる場合も、まずOpenSpec changeを作り、KMMが持つ責務かどうかを確認する。
