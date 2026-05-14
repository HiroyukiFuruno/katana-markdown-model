## Context

KMM `v0.1.0` は、描画や同期制御ではなく、renderer-neutralな文書モデル、source mapping、metadata解決、同期に使える材料を公開する。

KDV、KLE、KatanAが `v0.1.0` を採用すると、実際のMarkdown表示、保存時metadata同期、editor-viewer同期で不足が見つかる可能性がある。

その不足をすべてKMMへ戻すのではなく、KMMの解析結果やpublic DTOが原因のものだけを `v0.1.1` で扱う。

## Goals

- `v0.1.0` の公開契約（public contract）を壊さずに精度を上げる。
- downstream採用結果をfixture testへ変換する。
- source range、line-column、raw snippet、fingerprintのズレを回帰テスト化する。
- metadata target移動判定の改善を、unresolved保持とconflict返却の契約を壊さずに行う。
- 同期制御はKatanAに残したまま、KatanAが使えるanchor材料だけを必要最小限で追加する。

## Non-Goals

- Markdown描画。
- HTML/PDF/PNG/JPG export。
- 製品CLI。
- 製品UI。
- KatanA workspace state。
- viewerまたはeditorへのscroll、selection、highlight命令。
- KDV、KLE、KCF、KatanA本体への依存追加。
- CommonMark全仕様の一括実装。

## Work Streams

### 1. Downstream adoption triage

KDV、KLE、KatanAで `katana-markdown-model = "0.1.0"` を採用した結果を集める。

分類は次の4つにする。

- KMM解析精度の問題
- KMM metadata解決の問題
- KMMが返す同期材料の不足
- downstream側の責務として扱う問題

KMMで扱わないものは、KMMのtaskへ混ぜない。

### 2. Fixture-first fixes

修正前に、KMM内のfixture testで失敗を再現する。

絶対パスに依存するtestは追加しない。KatanA fixtureを使う場合は、KMM repository内のcanonical fixtureへ同期したうえでtestにする。

### 3. Markdown contract gaps

footnote、image、link、HTML inline、math inlineは、`v0.1.0` ではKatanA現行仕様として棚卸し済みである。

`v0.1.1` では、downstreamが既存DTOでは扱えない場合だけ専用DTO化を検討する。専用DTOを追加する場合は、既存DTOを壊さず追加する。

### 4. Metadata precision

metadata target移動判定は、node id、fingerprint、source range、前後文脈を使う。

`v0.1.1` では、誤って1候補に決める修正を禁止する。曖昧な場合は `Conflict`、復元できない場合は `Unresolved` を返す契約を維持する。

### 5. Sync anchor materials

KMMは同期制御を持たない。

KatanAがeditorとviewerを対応付けるために既存のnode id、source range、line-column、raw snippet、fingerprintだけでは不足する場合、KMMは追加のanchor材料をpublic DTOとして返すことを検討する。

追加する場合も、KMMはviewerやeditorへ命令しない。

## Release Policy

`v0.1.1` は `release/v0.1.1` でまとめ、`master` へrelease PRを作る。

公開前には次を通す。

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
scripts/openspec validate "prepare-v0-1-1-precision-hardening" --strict
just check
just release-check
```
