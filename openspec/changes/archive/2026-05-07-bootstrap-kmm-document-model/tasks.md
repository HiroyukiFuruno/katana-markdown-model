# Tasks: bootstrap-kmm-document-model

## 0. 全体見直し

### Definition of Ready

- [x] KMMがMarkdown仕様の中核であり、viewer、editor、exportがKMM文書モデルを消費する方針が確認済みである
- [x] KCFは外部描画へ縮小し、既存exportはKDV移譲まで維持する方針が確認済みである
- [x] 親OpenSpec `katana/openspec/changes/establish-kmm-markdown-platform` に、KMMを出発点にした実装順序とrepo別DoR/DoDが反映されている
- [x] 周辺repoのOpenSpecに、KMM完了前に進めてよい範囲とpending条件が反映されている

### Tasks

- [x] 0.1 KMMの責務を「HTML変換器ではなく文書モデルの正本」として再確認する
- [x] 0.2 KCFを外部描画へ縮小し、既存exportをKDV実装まで維持する条件を明記する
- [x] 0.3 KMMから見たcross-repo実装順序をhandoffに固定する
- [x] 0.4 `katana-ui-widget` が未作成であることをリスクとして明記する
- [x] 0.5 KDV / KLE / KCF / KatanA統合のDoRがKMM完了条件を参照しているか確認する

### Definition of Done

- [x] 別セッションがKMM OpenSpecだけを読んでも、次に何を実装するか判断できる
- [x] 親計画とKMM計画の順序が矛盾していない
- [x] KDV/KLE/KCF/KatanAがKMMより先に独自metadataや独自文書モデルを作らないことが明記されている

## 1. Repository Baseline

### Definition of Ready

- [x] `katana/openspec/changes/establish-kmm-markdown-platform` でKMMの責務が定義されている
- [x] P0 `katana-ast-lint` v0.1.0 がcrates.ioで利用可能である

### Tasks

- [x] 1.1 KMM crate構成を決める
- [x] 1.2 public DTOと内部parser moduleの境界を決める
- [x] 1.3 KCF/KDV/KatanA/editorへ依存しないことを検証する
- [x] 1.4 `katana-ast-lint = "0.1.0"` をKMM品質ゲートへ接続する
- [x] 1.5 `just check`、`lefthook`、CI、repo-local skillを横展開する
- [x] 1.6 GitHub repositoryを作成し、`master` をdefault branchにする
- [x] 1.7 KML相当のbranch protectionを `release-preflight.yml` push後に再設定する

### Definition of Done

- [x] KMMが単独repositoryとして成立している
- [x] public contractに既存parser ASTが漏れていない
- [x] KMM固有の一時AST lintや除外設定を品質ゲートにしていない
- [x] `just check` がPASSする

## 2. Document Model Bootstrap

### Definition of Ready

- [x] 初期fixture contractが決まっている
- [x] canonical fixtureの取り込み方法が決まっている

### Tasks

- [x] 2.1 `sample.md` の主要nodeを軽量fixtureでモデル化する
- [x] 2.2 README badgeをHTML block / badge rowとしてモデル化する
- [x] 2.3 alertとdescription listをモデル化する
- [x] 2.4 tableに行、列、cell、alignmentを持たせる
- [x] 2.5 table/gridのcell単位source rangeを持たせる
- [x] 2.6 emojiを削除せず、Unicodeとshortcode情報を保持する専用nodeを追加する
- [x] 2.7 footnote、image、link、HTML inline、math inlineをKatanA現行仕様として棚卸しする
- [x] 2.8 `katana/assets/fixtures/sample.md`、`sample_basic.md`、`katana/README.md` badgeをcanonical fixtureとして同期する

### Definition of Done

- [x] KatanA現行fixtureの主要構造がKMM nodeとして固定されている
- [x] node種別、source range、raw snippet、fingerprintがfixture testで固定されている
- [x] KMMのpublic DTOだけでKDV / KLE / KCF / KatanAが参照できる

## 3. Metadata Target Resolution

### Definition of Ready

- [x] metadataをMarkdown本文へ埋め込まない方針が決まっている

### Tasks

- [x] 3.1 `README.md.metadata.json` のschema初期案を定義する
- [x] 3.2 file path、node id、byte range、line-column、fingerprint、前後文脈をtargetに含める
- [x] 3.3 旧本文と新本文からtarget移動を判定する
- [x] 3.4 復元できないtargetをunresolvedとして返す
- [x] 3.5 conflict状態をpublic DTOとして追加する
- [x] 3.6 PDFページング、LLM注釈、AST単位copy/editをmetadata用途としてfixture化する
- [x] 3.7 editor保存時に必要なrequest/result DTOをkleがそのまま使える形へ固定する

### Definition of Done

- [x] 保存時metadata更新に必要なAPIがkleから利用できる
- [x] unresolved targetが削除されず保持される
- [x] conflictが未定義のままdownstreamへ流れない

## 4. Parser Strategy

### Definition of Ready

- [x] KatanA現行fixtureとbadge fixtureの必須node一覧がある

### Tasks

- [x] 4.1 現行の初期parserを維持するか、parser engineを差し替えるかを評価する
- [x] 4.2 候補parserがOS依存emojiを壊さないか確認する
- [x] 4.3 parser内部型をpublic DTOへ出さないadapter境界を固定する
- [x] 4.4 table/grid、HTML badge、alert、description list、footnote、diagram、mathのparse contractをtestで固定する

### Definition of Done

- [x] parser engineを差し替えてもpublic DTOが壊れない
- [x] KatanA現行挙動を再現できないparserを選ばない

## 5. Cross-repo Handoff

### Definition of Ready

- [x] KMM public DTOとmetadata APIのv0境界が固定されている

### Tasks

- [x] 5.1 KDVへ渡すviewer/export inputとhit-test metadataを定義する
- [x] 5.2 KLEへ渡すsave-time metadata sync contractを定義する
- [x] 5.3 KUWへ渡すmetadata/unresolved表示DTOを定義する
- [x] 5.4 KCFを外部描画へ縮小し、既存exportをKDV移譲まで維持する条件を定義する
- [x] 5.5 KatanA統合で必要なfixture authorityとdependency version policyを定義する
- [x] 5.6 KatanAがeditor-viewer同期制御を担い、viewerまたはeditorへ命令する条件を文書化する

### Definition of Done

- [x] downstream repoがKMM内部parser型へ依存しない
- [x] downstream repoが独自metadata schemaを作らない
- [x] KDV、KCF、KLE、KatanAの責務境界が明確である

## 6. Verification

- [x] 6.1 `just check` を実行する
- [x] 6.2 KMM fixture testsを実行する
- [x] 6.3 KAL AST lintのKMM adapterで検査できることを確認する
- [x] 6.4 `scripts/openspec validate "bootstrap-kmm-document-model" --strict` を実行する
- [x] 6.5 親OpenSpecと周辺repo OpenSpecを再検証する

## 7. v0.1.0分割計画

`bootstrap-kmm-document-model` は初期立ち上げの親計画として残す。

未完了項目は、`docs/roadmap.md` と次のOpenSpec changeへ分割して進める。

- `stabilize-release-readiness-gates`
  - 1.7 branch protection
  - release前検査
  - `just check` / `release-check`
- `stabilize-canonical-fixtures`
  - 2.7 footnote、image、link、HTML inline、math inlineの棚卸し
  - 2.8 canonical fixture同期
- `finalize-metadata-resolution-contract`
  - 3.5 conflict状態
  - 3.6 metadata用途fixture
  - 3.7 editor保存時DTO
- `lock-parser-adapter-strategy`
  - 4.1から4.4のparser strategy
- `prepare-manual-harness`
  - 構造確認補助tool
  - `just harness-up`
- `prepare-downstream-handoff-contract`
  - 5.1から5.6のcross-repo handoff
- `publish-v0-1-0-release`
  - 全change完了後の `v0.1.0` GitHub Releaseとcrates.io公開

### Tasks

- [x] 7.1 `docs/roadmap.md` へ `v0.1.0` までの順序とbranch戦略を明記する
- [x] 7.2 未完了項目を小さなOpenSpec changeへ分割する
- [x] 7.3 `v0.1.0` までは全change完了前に公開しない条件を明記する
- [x] 7.4 `release/v0.1.0` PR準備範囲と実リリース後に残す範囲を分離する
- [x] 7.5 PR merge後にGitHub Release、crates.io公開、公開後verify、branch hygieneを実施する

7.5は実リリース後の作業である。`v0.1.0` の公開境界を広げる精度向上は含めない。
