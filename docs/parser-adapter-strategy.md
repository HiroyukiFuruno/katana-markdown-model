# Parser Adapter Strategy

## 結論

`v0.1.0` では現行parserを維持する。

目的はCommonMark完全準拠ではなく、KMM public DTOとsource mapping契約を固定することである。parser engineの差し替えは後続で検討できるが、public contractとcontract testを壊してはならない。

## 境界

KMMのpublic APIはKMM所有DTOだけを返す。

- `KmmDocument`
- `KmmNode`
- `KmmNodeKind`
- `SourceSpan`
- `MetadataDocument`
- `TargetResolution`

`src/parser/**` は内部実装である。downstreamはparser内部型へ依存しない。

## v0.1.0で固定するparse contract

- tableはrow、cell、alignment、cell source rangeを保持する
- README badge rowは `HtmlBlockRole::BadgeRow` として保持する
- centered HTML blockは `HtmlBlockRole::Centered` として保持する
- legacy note blockとGFM alert blockは `Alert` labelを保持する
- description listはtermとdescriptionを保持する
- footnote記法は専用DTO化しないが、paragraph raw snippetとして保持する
- Mermaid、PlantUML、DrawIoはdiagram code blockとして保持する
- math fenced blockはmath code blockとして保持する
- inline mathは専用DTO化しないが、paragraph raw snippetとして保持する
- Unicode emojiとshortcode emojiはKMM nodeとして保持する

## parser候補評価

現時点では、Comrak、pulldown-cmark、markdown-rsなどのthird-party parserを採用しない。

理由は、次の条件をKMMのcanonical fixtureとcontract testで確認してからでないと、差し替えによる退行を判断できないためである。

- source rangeとraw snippetをKMMの `SourceSpan` に安定して変換できる
- README badge、alert、description list、table、diagram、mathを壊さない
- Unicode emojiとshortcode emojiを削除しない
- parser AST型をpublic APIへ漏らさない
- OSやfontに依存して文書モデルが変わらない

## 差し替え条件

parser engineを差し替える場合は、先に次を満たす。

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
cargo test --test parser_adapter_contract --locked
cargo test --test canonical_fixtures --locked
just check
```

この検証に失敗するparser候補は採用しない。
