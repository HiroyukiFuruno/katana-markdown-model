# katana-markdown-model — UI 分離計画 抜粋

作成日: 2026-05-17  
canonical: [`katana/docs/architecture/ui-separation/detailed-design-and-tasks.md`](../../katana/docs/architecture/ui-separation/detailed-design-and-tasks.md)

## このファイルの位置付け

本ファイルは KatanA ecosystem の **UI 分離構想 master** から `katana-markdown-model` (KMM) 担当部分を抜粋したもの。task ID は master と同一。**master が単一情報源**。

## Repository の役割

`katana-markdown-model` (KMM) は **canonical renderer-neutral Markdown model** として位置付ける。

- KatanA viewers / editors / export flows が共有する common interpretation layer。
- 既存の `katana-markdown-engine` (KME) は migration / compatibility / deprecation target とする。canonical は KMM。
- KDV (`katana-document-viewer`) は KMM を input として扱う。
- KLE (`katana-language-editor`) は KMM に **依存しない**。markdown 用 port adapter (`katana-language-editor-md`) が間接的に KMM を使う。
- KUC (`katana-ui-core`) は KMM 型に直接依存しない。表示 DTO のみ受ける。

詳細: master [`1.7 katana-markdown-model / katana-markdown-engine`](../../katana/docs/architecture/ui-separation/detailed-design-and-tasks.md#17-katana-markdown-model--katana-markdown-engine)

## 担当 Phase

- **Phase 6**: KMM canonical 化 (本 repo のメイン作業)
- **横断**: P0-B (naming policy ADR)

## Task list (master 抜粋)

### P6-A. Canonical switch

- [ ] P6-A-001: `katana-markdown-model` README を canonical model として更新する。
- [ ] P6-A-002: `katana-markdown-engine` README に compatibility / migration notice を追加する。
- [ ] P6-A-003: KatanA OpenSpec `adopt-kme-in-katana` を `adopt-kmm-in-katana` に読み替える提案を作る。
- [ ] P6-A-004: KDV から KMM を参照する dependency を追加する。
- [ ] P6-A-005: KLE は KMM に **hard dependency を持たない**。markdown source mapping は `SourceAnchorAdapter` trait 経由で受け取り、KMM 用 implementation は `katana-language-editor-md` crate に閉じる。この方針を ADR `docs/adr/kle-kmm-dependency.md` (KLE repo) に記録する。
- [ ] P6-A-006: KUC が KMM に依存しないことを guardrail に追加する。
- [ ] P6-A-007: KMM document model fixture を KDV forge fixture に共有する。
- [ ] P6-A-008: KMM metadata target を viewer unresolved 表示 DTO に変換する。
- [ ] P6-A-009: KMM metadata target を editor anchor DTO に変換する。
- [ ] P6-A-010: KMM reconciliation を editor save flow に接続する。

### P8-A (README 反映)

- [ ] P8-A-008: `katana-markdown-model` README に canonical model 方針を追加する。

## 前提 (depends on) / 出力 (provides)

- **前提 (P0 完了)**:
  - `katana-markdown-model` を canonical model crate として ADR に記録 (P0-B-001)
  - `katana-markdown-engine` を migration / compatibility target として ADR に記録 (P0-B-002)

- **出力**:
  - canonical README (KMM)
  - migration notice README (KME)
  - KDV 経由で参照される共通 fixture
  - KMM metadata target -> viewer unresolved DTO 変換
  - KMM metadata target -> editor anchor DTO 変換 (KLE 直接ではなく md adapter 経由で消費)

## Done criteria

本 repo に関する master 9 章 Done criteria のうち、該当項目:

- [ ] KMM が canonical document model になる
- [ ] KME が migration / compatibility target として README に記録されている
- [ ] KDV / KatanA / md adapter が KMM を介して markdown を解釈する
- [ ] KUC / KLE が KMM に直接依存していない (P3-E-007 / P6-A-006 guardrail で検査)

## 重要な非該当事項

- 本 repo の document model 自体には大きな変更を加えない。主な作業は canonical naming への切替と他 repo との接続。
- KME の deprecation は P9-B-003 で window を決めるが、本 repo (KMM) 側ではなく KME repo 側のタスク。

## drift 検出

- 本ファイルの task ID は master と完全一致する。
- P8-A-001 の CI script が master と本ファイルの task ID 一致を検査する。

## 参照リンク

- [master detailed-design-and-tasks.md](../../katana/docs/architecture/ui-separation/detailed-design-and-tasks.md)
- [master principles.md](../../katana/docs/architecture/ui-separation/principles.md)
- [overview README](../../katana/docs/architecture/ui-separation/README.md)
- [KLE repo の Phase 3 抜粋 (md adapter 配置)](../../katana-language-editor/docs/ui-separation-plan.md)
- [KDV repo の Phase 2 抜粋 (KMM input conversion)](../../katana-document-viewer/docs/ui-separation-plan.md)
- [既存 docs/roadmap.md](roadmap.md)
- [既存 docs/responsibility-boundary.md](responsibility-boundary.md)
