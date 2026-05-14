---
name: impl-release
description: katana-markdown-model で指定バージョンのOpenSpec実装、release準備、release PR、GitHub Release、crates.io公開、事後整理までを進めるときに使う。/impl-release vX.Y.Z 相当のKMM向けリリース実装ワークフロー。
---

# impl-release

KMMの `vX.Y.Z` リリースを、実装から公開後整理まで進める入口です。

## 前提

- 対象repositoryは `/Users/hiroyuki_furuno/works/private/katana-markdown-model`。
- default branchは `master`。
- KMMはlibrary-onlyです。CLI、UI、binary artifact、npm、PyPI、Homebrew、MCPB、editor extensionのrelease処理を持ち込みません。
- 公開対象は `katana-markdown-model` crate、GitHub Release、`docs/release-notes/<version>.md` です。
- release統合branchは `release/vX.Y.Z`、補助branchは `feature/vX.Y.Z-<short-slug>` に統一します。
- release PRは `release/vX.Y.Z` から `master` へ作ります。

## 停止ルール

次に当たる場合は作業を止め、ユーザーに確認します。

- 指定versionが最新tag、GitHub Release、crates.io版から見て不自然に飛んでいる。
- OpenSpec tasksにないAPI変更、公開DTO変更、互換性破壊、責務境界変更が必要になった。
- KMMにCLI、描画、export、UI framework、downstream repository依存を追加しそうになった。
- `CARGO_REGISTRY_TOKEN` や公開権限など、ユーザー側操作が必要な状態になった。
- commit、push、merge、公開など、外部に見える操作へ進む前。

## 参照するもの

- `.agents/skills/openspec-apply-change/SKILL.md`: OpenSpec changeの実装。
- `.agents/skills/lint-and-ast-lint/SKILL.md`: `just check` の品質gate確認。
- `.agents/skills/self-review/SKILL.md`: handoff前の自己レビュー。
- `.agents/skills/commit_and_push/SKILL.md`: ユーザー承認後のcommit/push。
- `docs/release-runbook.md`: 公開手順。
- `docs/quality-gates.md`: release前gate。
- `docs/roadmap.md` と `docs/roadmap-v0.1.1.md`: release範囲と責務境界。
- `branch-hygiene` skill: merge後、release後のbranch/worktree整理。

## Phase 0: 状態把握

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
git status --short --branch
git fetch origin --prune --tags
```

確認すること:

- 既存差分を混ぜない。
- `Cargo.toml` のversionが対象versionと一致するか。
- 最新tag、最新GitHub Release、crates.io最新versionから見て、指定versionが自然な次版か。
- `docs/release-notes/vX.Y.Z.md` があるか。
- 対象versionのOpenSpec changeがあるか。

versionが不自然な場合は、ユーザー指定を正として扱わず停止します。

## Phase 1: 実装

1. `release/vX.Y.Z` を作るか、既存の同名branchを継続します。
2. 対象OpenSpec changeの `proposal.md`、`design.md`、`tasks.md`、`specs/**/spec.md` を読みます。
3. `tasks.md` の未完了taskを上から実装します。
4. bug fixは回帰testを先に追加します。
5. 新しい判断、制約、未解決事項はOpenSpecまたはrelease関連docsへ日本語で記録します。
6. task完了直後に `tasks.md` のcheckboxを更新します。

KMMの責務境界:

- 第三者parser ASTをpublic contractへ出さない。
- KatanA、KCF、KDV、KLE、UI frameworkへ依存しない。
- 描画、export、scroll、selection、highlight制御をKMMへ入れない。
- fixture testをrelease判断の正本にする。

## Phase 2: 検証

対象changeごとに実行します。

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
scripts/openspec validate "<change-name>" --strict
just check
```

release PR前に実行します。

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
just release-check
```

`just release-check` は `just check`、`cargo package --locked --allow-dirty`、`cargo publish --dry-run --locked --allow-dirty` を含みます。

失敗した検証をskipしません。lint除外、hook回避、`--no-verify` は使いません。

## Phase 3: release PR準備

確認対象:

- `Cargo.toml` のversion。
- `Cargo.lock` が必要な場合の更新。
- `docs/release-notes/vX.Y.Z.md`。
- `docs/release-readiness/<version>-*.md`。
- 完了したOpenSpec changeのarchive。

PR作成前にユーザーへ、変更内容と検証結果を報告して停止します。

ユーザー承認後、PRは次の形で作ります。

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
git push -u origin release/vX.Y.Z
gh pr create --base master --head release/vX.Y.Z --title "Prepare vX.Y.Z release"
```

PR本文には、公開未実施であること、`just release-check` の結果、fixture test結果、任意の構造確認結果を書きます。

## Phase 4: merge前確認

merge前に確認します。

- `release-preflight` が通っている。
- `Test and Build (macos-latest)` が通っている。
- `Test and Build (ubuntu-latest)` が通っている。
- `Test and Build (windows-latest)` が通っている。
- review commentが残っていない。
- `--admin` を使わない。

mergeはユーザー承認後にだけ行います。

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
gh pr merge --merge --delete-branch <PR-number-or-url>
git switch master
git pull --ff-only origin master
```

## Phase 5: 公開

公開は、PRが `master` へmergeされた後にだけ行います。

GitHub Actionsの `release` workflowを `master` から手動実行します。

- `version`: `vX.Y.Z`
- `publish_crate`: `true`

Actionsが使えない場合だけ、`docs/release-runbook.md` の手動公開手順を使います。

公開前に、`CARGO_REGISTRY_TOKEN` がGitHub secretとして登録済みか確認します。

## Phase 6: 公開後verify

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
gh release view vX.Y.Z --repo HiroyukiFuruno/katana-markdown-model
cargo search katana-markdown-model --limit 1
cargo info katana-markdown-model@X.Y.Z
```

crates.io公開済みversionは再利用しません。

- crates.io公開前に失敗した場合は、修正後に同じversionで再実行できます。
- crates.io公開後にGitHub Release作成だけ失敗した場合は、同じtagでGitHub Release作成を再実行します。
- crates.io公開後に内容不備が見つかった場合は、次のpatch versionで修正します。

## Phase 7: 事後整理

`branch-hygiene` skillを使います。

報告すること:

- GitHub Release URL。
- crates.io version。
- 削除したlocal branch。
- 削除したremote branch。
- 削除したworktree。
- 残したbranchと理由。
- 次versionへ回す課題。

## 完了条件

- 対象OpenSpec changeのtaskが完了している。
- `scripts/openspec validate "<change-name>" --strict` が通っている。
- `just check` が通っている。
- `just release-check` が通っている。
- release PRが `master` にmergeされている。
- GitHub Actionsの `release` workflowが成功している。
- GitHub Releaseとcrates.ioの公開後verifyが通っている。
- branch hygieneが完了している。
