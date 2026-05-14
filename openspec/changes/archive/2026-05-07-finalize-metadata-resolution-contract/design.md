## Context

現状のmetadata解決結果は `Resolved`、`Moved`、`Unresolved` を持つ。

kleなどのdownstreamが保存時に必要とする状態をKMM側で固定しないと、各repoが独自metadata schemaを作る危険がある。

## Goals

- metadata解決のpublic DTOをv0.1.0境界として固定する。
- `Conflict` を未定義のままdownstreamへ流さない。
- unresolved targetを削除せず保持する。

## Non-Goals

- metadataをMarkdown本文へ埋め込むこと。
- editor UIを実装すること。
- PDF pagingやLLM注釈の実処理をKMMで実装すること。

## Contract

KMMは保存前source、保存後source、metadata documentを受け取り、targetごとの解決状態を返す。

最低限の状態:

- `Resolved`: 直接解決できた
- `Moved`: 同じ意味のtargetへ再対応できた
- `Unresolved`: 再対応できなかったがmetadataは保持する
- `Conflict`: 複数候補や曖昧な再対応があり、自動決定できない

## Conflict Policy

最小のconflict判定は、targetのnode idで直接解決できず、同じfingerprintを持つ新document nodeが複数ある場合である。

この場合、KMMは候補を1つに決めない。`previous_node_id` と `candidate_node_ids` を返し、downstream側がユーザー確認や別の判断材料で扱える状態にする。

## Save-time DTO

editor保存時は `MetadataReconcileRequest` を入口にする。

requestは次を持つ。

- 保存前document
- 保存後document
- metadata document

resultは次を持つ。

- metadata document
- targetごとのresolution

KMMはunresolved metadataを削除しない。resultにmetadata documentを保持し、downstreamが勝手に消さない契約を明示する。

## Metadata Use Fixture

`tests/fixtures/metadata_uses.json` は、v0.1.0で想定するmetadata用途を固定する。

- PDFページング
- LLM注釈
- AST単位copy/edit

## Verification

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
scripts/openspec validate "finalize-metadata-resolution-contract" --strict
just check
```
