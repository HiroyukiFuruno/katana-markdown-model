# Tasks: publish-v0-1-0-release

## Definition of Ready

- [x] `stabilize-release-readiness-gates` が完了している
- [x] `stabilize-canonical-fixtures` が完了している
- [x] `finalize-metadata-resolution-contract` が完了している
- [x] `lock-parser-adapter-strategy` が完了している
- [x] `prepare-manual-harness` が完了している
- [x] `prepare-downstream-handoff-contract` が完了している
- [x] 初回公開versionは `v0.1.0` である
- [x] 既定ブランチは `master` 維持である

## Tasks: PR準備

- [x] 1.1 `v0.1.0` release runbookを作成する
- [x] 1.2 `CARGO_REGISTRY_TOKEN` の登録手順を明記する
- [x] 1.3 `release/v0.1.0` から `master` へのrelease PR手順を明記する
- [x] 1.4 GitHub Release作成手順を実装または文書化する
- [x] 1.5 crates.io publish手順を実装または文書化する
- [x] 1.6 公開済みversionを再利用しない失敗時方針を明記する
- [x] 1.7 公開後verify手順を実装または文書化する
- [x] 1.8 release後のbranch hygiene手順を明記する

## Definition of Done: PR準備

- [x] release runbookがある
- [x] release workflowがdry-runとpublishを分けている
- [x] release noteがある
- [x] 実公開に関する完了Taskが未チェックのまま残っている

## Definition of Done: 実リリース

- [x] 実公開前に必要なOpenSpec changeのtaskが完了している
- [x] `just release-check` が成功している
- [x] `v0.1.0` GitHub Releaseが作成されている
- [x] `v0.1.0` がcrates.ioへ公開されている
- [x] 公開後verifyが成功している
- [x] release後のbranch hygieneが完了している

精度向上は `v0.1.0` の実公開条件に含めない。公開後に見つかったparser精度、metadata照合精度、同期anchor材料の追加は `v0.1.1` 以降で扱う。

## Verification

- [x] `scripts/openspec validate "publish-v0-1-0-release" --strict`
- [x] `just release-check`
- [x] 公開後verify
