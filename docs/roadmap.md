# KMM v0.1.0 Roadmap

## 結論

`katana-markdown-model`（KMM）は、`v0.1.0` 初回リリースまで小さなOpenSpec changeを順番に完了する。

`v0.1.0` まではcrates.ioへ公開しない。すべての実装、検証、受け渡し条件が完了した後にだけ、`v0.1.0` を公開する。

## KMMの責務

KMMはMarkdownをHTMLへ変換する部品ではない。

KMMは、KatanA ecosystemで共有するMarkdown文書モデル、source range、line-column、raw snippet、fingerprint、外部metadata解決を所有するlibraryである。これらはKLEの保存時metadata同期、KDVのviewer/export、KatanAのeditor-viewer同期制御に使われる材料である。

KMMは次を所有しない。

- 製品CLI
- 製品UI
- HTML/PDF/PNG/JPG出力
- Floem、egui、KatanA workspace state
- viewer/editor同期制御
- KDV、KLE、KCF、KatanA本体の統合処理

## v0.1.0までのchange順序

## v0.1.0 release PR準備チェック表

この表は、`release/v0.1.0` から `master` へ出すrelease PRで何を完了し、何をmerge後に残すかを固定する。

| change | 状態 | release PRに含める内容 | PR後に残す内容 |
| --- | --- | --- | --- |
| `stabilize-release-readiness-gates` | 完了済み | ブランチ保護（branch protection）の実設定確認 | check名が変わった場合の保護設定更新 |
| `stabilize-canonical-fixtures` | 完了済み | なし | fixture更新が発生した場合の同期PR |
| `finalize-metadata-resolution-contract` | 完了済み | なし | downstream採用時の追加用途fixture |
| `lock-parser-adapter-strategy` | PR内で完了 | parser境界、contract test、parser評価メモ | parser engine差し替えは将来change |
| `prepare-manual-harness` | PR内で再整理 | 構造確認補助tool、package混入確認 | release必須の目視GUIから外す |
| `prepare-downstream-handoff-contract` | PR内で再整理 | KDV/KCF/KLE/KatanA受け渡し境界、KDV改名方針 | downstream repo側の実装PR |
| `publish-v0-1-0-release` | PR準備のみ | runbook、secret、PR、公開、verify、branch hygiene手順 | GitHub Release、crates.io公開、公開後verify、branch hygiene |

release PR作成前に、次をすべて満たす。

- active changeのOpenSpec検証が通っている
- `just check` が通っている
- `just release-check` が通っている
- fixture testで代表構造、metadata、同期材料を検証できる
- `cargo package --locked --allow-dirty --list` に開発用harnessが含まれていない
- `master` のブランチ保護（branch protection）が `Test and Build (macos-latest)`、`Test and Build (ubuntu-latest)`、`Test and Build (windows-latest)`、`preflight` を必須checkにしている
- `release/v0.1.0` から `master` へのrelease PRが作成済みで、必須checkが通っている

実リリース前に、次をすべて満たす。

- release PRが `master` へmergeされている
- `CARGO_REGISTRY_TOKEN` がGitHub secretとして登録されている
- `v0.1.0` GitHub Release作成手順を実行できる
- crates.io publish手順を実行できる
- 公開後verify手順を実行できる

## v0.1.0に含めない精度向上

parser精度、対応Markdown構文の拡張、metadata照合の追加推定ロジック（heuristic）は `v0.1.0` へ混ぜない。

`v0.1.0` はKMMの公開境界を固定するリリース（release）である。公開後にKDV、KLE、KatanAが採用した結果として見つかる精度改善は、`v0.1.1` 以降の小さなchangeで扱う。

`v0.1.1` 候補:

- canonical fixtureで見つかったsource range、line-column、raw snippet、fingerprintのズレ修正
- footnote、image、link、HTML inline、math inlineの専用DTO化が必要になった場合の追加
- metadata target移動判定の精度向上
- editor-viewer同期で不足したanchor材料の追加

ただし、KMMがviewer、export、同期制御を持たない境界は変えない。

### 1. `stabilize-release-readiness-gates`

CI、release前検査、branch protection、`just check` / `release-check` の対応を固定する。

このchangeでは、KMLのrelease前検査を参考にする。ただしKMMはlibrary-onlyなので、npm、PyPI、Homebrew、binary artifact、MCPB、editor extensionの検査は持ち込まない。

完了条件:

- `just check` が標準品質ゲートとして維持されている
- `release-check` が `just check`、`cargo package --locked --allow-dirty`、`cargo publish --dry-run --locked --allow-dirty` を含む
- GitHub Actionsの `release-preflight` が同じ検査を実行する
- `master` branch protectionの必須checkが文書化されている

### 2. `stabilize-canonical-fixtures`

KatanA現行fixture、README badge、alert、description listをKMMの正本fixtureとして固定する。

完了条件:

- canonical fixtureの同期方法が決まっている
- node種別、source range、raw snippet、fingerprintがテストで固定されている
- 絶対パスに依存しない検証になっている

### 3. `finalize-metadata-resolution-contract`

metadata解決のpublic contractを固定する。

完了条件:

- `Resolved`、`Moved`、`Unresolved`、`Conflict` がpublic DTOとして定義されている
- editor保存時に必要なrequest/result DTOがKMM側で固定されている
- unresolved metadataを削除しない契約がテストで固定されている

### 4. `lock-parser-adapter-strategy`

parser内部型をpublic contractへ出さない境界と、主要Markdown構造のparse contractを固定する。

完了条件:

- parser engineを差し替えてもpublic DTOが壊れない
- table、badge、alert、description list、footnote、diagram、math、emojiのcontract testがある
- OS依存emojiを壊すparser候補を採用しない判断基準がある

### 5. `prepare-manual-harness`

`just harness-up` は、KMM出力の構造確認を補助する開発用toolとして扱う。

KMMのrelease判定は、目視GUIではなくfixture testを正本にする。画面上の確認は、Markdown本文とKMMが解釈したnode一覧、選択nodeの位置情報、raw snippet、fingerprint、metadata解決状態を見る補助に限定する。

これは製品UIではなく、release必須の手動品質ゲートでもない。

完了条件:

- fixture testでKDV/KLE/KatanAが必要とする構造、metadata、同期材料を検証できる
- `just harness-up` は任意の構造確認補助toolとして起動できる
- KMM本体はlibrary-onlyを維持している
- 開発用harnessのbinaryやUI依存が公開crateへ混入しない

### 6. `prepare-downstream-handoff-contract`

KDV、KLE、KCF、KatanAへ渡すpublic DTO境界と受け渡し条件を固定する。

完了条件:

- downstreamがKMM内部parser型へ依存しない条件が明文化されている
- downstreamが独自metadata schemaを作らない条件が明文化されている
- KDVがviewer/exportを担うこと、KCF exportはKDV実装まで維持することが明文化されている
- KatanAがeditor-viewer同期制御を担うことが明文化されている

### 7. `publish-v0-1-0-release`

全change完了後に `v0.1.0` のGitHub Releaseとcrates.io公開を実施する。

完了条件:

- 全OpenSpec changeが完了している
- `just check` が通っている
- `release-check` が通っている
- `cargo package --locked --allow-dirty` が通っている
- `cargo publish --dry-run --locked --allow-dirty` が通っている
- `CARGO_REGISTRY_TOKEN` がGitHub secretとして登録されている
- GitHub Releaseとcrates.io公開後のverify手順が通っている

## v0.1.0以降のbranch戦略

KMMは `v0.1.0` 以降、KML同様のbranch戦略を正式採用する。ただしKMMの既定ブランチは `master` を維持する。

- 既定ブランチ: `master`
- release統合ブランチ: `release/vX.Y.Z`
- 補助ブランチ: `feature/vX.Y.Z-<short-slug>`
- release PR: `release/vX.Y.Z` から `master` へ作成する
- `fix/vX.Y.Z-*`、`chore/vX.Y.Z-*`、`release-vX.Y.Z` は使わない
- merge時に `--admin` は使わない
- merge後はbranch hygieneを行う

## リリース前の必須検証

各change完了時:

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
scripts/openspec validate "<change-name>" --strict
just check
```

`v0.1.0` 公開直前:

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
just check
cargo package --locked --allow-dirty
cargo publish --dry-run --locked --allow-dirty
```

任意の構造確認:

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
just harness-up
```

## 進めないこと

- 全change完了前に `v0.1.0` を公開しない
- KMMに製品CLIを追加しない
- KMMに製品UIを追加しない
- KMMでHTML/PDF出力を担当しない
- KMMでeditor-viewer同期制御を担当しない
- downstream側の都合でKMM public DTOへparser内部型を漏らさない
