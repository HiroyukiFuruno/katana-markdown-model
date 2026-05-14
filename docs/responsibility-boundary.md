# KMM Responsibility Boundary

## 結論

KMMはMarkdownを解析し、KatanA ecosystemで共有する文書構造、source mapping、metadata target解決を返すlibraryである。

KMMは描画、export、editor-viewer同期制御を持たない。同期制御は常に利用側アプリであるKatanAが担い、KatanAがviewerやeditorへ命令する。

## KMMが所有するもの

- Markdown document model
- stable node id
- source range
- line-column
- raw snippet
- text fingerprint
- metadata schema
- metadata target resolution
- parser adapter boundary

これらは、KLEの保存時metadata同期、KDVのviewer/export、KatanAのeditor-viewer同期に使える材料である。

## KMMが所有しないもの

- Markdown描画
- Floem widget
- viewer state
- editor state
- scroll state
- viewport
- hit-test方針
- HTML/PDF/PNG/JPG export
- Mermaid / Draw.io / PlantUML / math の外部描画
- KatanA workspace state

## Repository境界

| repository | 責務 |
| --- | --- |
| `katana-markdown-model` | Markdown解析、文書構造、source mapping、metadata target解決 |
| `katana-document-viewer` | KMM DTOを入力にしたviewer、hit-test、HTML/PDF/PNG/JPG export |
| `katana-canvas-forge` | Mermaid / Draw.io / PlantUML / math 等の外部描画 |
| `katana-language-editor` | 編集面と保存時metadata同期 |
| `katana` | viewer/editor/export統合、editor-viewer同期制御 |

`katana-document-preview` は未リリース・未取り込みのため、計画上は `katana-document-viewer`（KDV）へ改名する。

## 同期境界

KMMは同期を実行しない。

KMMはKatanAが同期判断に使えるnode id、source range、line-column、raw snippet、fingerprintを安定して返す。

KatanAはその情報を使い、viewerまたはeditorへscroll、selection、highlightなどの命令を送る。KMMはviewer、editor、KatanAの統合状態を知らない。
