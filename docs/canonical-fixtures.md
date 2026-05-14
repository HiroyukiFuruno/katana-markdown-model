# Canonical Fixtures

## 結論

KMMのcanonical fixtureは、KatanA現行挙動をKMM内で再現するための正本である。

標準テストは `/Users/hiroyuki_furuno/works/private/katana` の絶対パスへ依存しない。KatanA側の現行fixtureを確認したうえで、KMM repository内の `tests/fixtures/canonical/` に同期したファイルをテスト入力にする。

## 同期元と同期先

| 同期元 | 同期先 | 用途 |
| --- | --- | --- |
| `/Users/hiroyuki_furuno/works/private/katana/assets/fixtures/sample.md` | `tests/fixtures/canonical/katana_sample.md` | KatanA現行の総合Markdown fixture |
| `/Users/hiroyuki_furuno/works/private/katana/assets/fixtures/sample_basic.md` | `tests/fixtures/canonical/katana_sample_basic.md` | 基本Markdownとalert fixture |
| `/Users/hiroyuki_furuno/works/private/katana/README.md` | `tests/fixtures/canonical/katana_readme.md` | README headerとbadge列 |
| `tests/fixtures/description_list.md` | `tests/fixtures/canonical/description_list.md` | description list正本 |

## 同期手順

KatanA側fixtureの更新をKMMへ取り込む場合は、差分を確認してから同期する。

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
cp /Users/hiroyuki_furuno/works/private/katana/assets/fixtures/sample.md tests/fixtures/canonical/katana_sample.md
cp /Users/hiroyuki_furuno/works/private/katana/assets/fixtures/sample_basic.md tests/fixtures/canonical/katana_sample_basic.md
cp /Users/hiroyuki_furuno/works/private/katana/README.md tests/fixtures/canonical/katana_readme.md
cp tests/fixtures/description_list.md tests/fixtures/canonical/description_list.md
```

同期後は次を実行する。

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
scripts/openspec validate "stabilize-canonical-fixtures" --strict
just check
```

## 期待する主要構造

`katana_sample.md` は次を固定する。

- heading
- centered HTML block
- README badge row
- listとtask marker
- code block
- Mermaid / PlantUML / DrawIo / math code block
- tableとalignment
- blockquote
- legacy note block
- thematic break
- emoji

`katana_sample_basic.md` は次を固定する。

- heading
- listとtask marker
- tableとalignment
- legacy note block
- GFM alert block
- footnote記法を含むparagraph
- special characterとemoji

`katana_readme.md` は次を固定する。

- centered icon block
- centered heading
- centered description
- centered link row
- badge row
- language selector

`description_list.md` は次を固定する。

- term
- description
- description list node

## 現時点で構造化対象外の要素

次の要素はcanonical fixtureに含めるが、`stabilize-canonical-fixtures` では専用DTOへ固定しない。

- footnote
- image
- link
- HTML inline
- math inline

これらはraw snippet、source range、fingerprintを保持したうえで、後続のparser strategyまたはmetadata contractで必要に応じて専用DTO化する。
