# v0.1.0 Release Runbook

## 結論

`v0.1.0` は、`release/v0.1.0` のPRが `master` へmergeされた後にだけ公開する。

このPRでは公開しない。

## release PR

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
git switch release/v0.1.0
just release-check
git push -u origin release/v0.1.0
gh pr create --base master --head release/v0.1.0
```

PR本文には、実公開が未実施であること、`just release-check` の結果、fixture testと任意の構造確認結果を明記する。

## secret登録

crates.io公開には、GitHub secretの `CARGO_REGISTRY_TOKEN` が必要である。

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
gh secret set CARGO_REGISTRY_TOKEN \
  --repo HiroyukiFuruno/katana-markdown-model \
  --body "<crates.io token>"
gh secret list --repo HiroyukiFuruno/katana-markdown-model | rg '^CARGO_REGISTRY_TOKEN'
```

## merge前確認

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
scripts/openspec validate "publish-v0-1-0-release" --strict
just check
just release-check
just harness-up
```

`just harness-up` は任意の構造確認補助である。release判断はfixture testと `just release-check` を正本にする。

## GitHub Actionsでの公開

PR merge後、GitHub Actionsの `release` workflowを `master` から手動実行する。

- `version`: `v0.1.0`
- `publish_crate`: `true`

`publish_crate=false` の場合は、`just release-check` だけを実行し、GitHub Releaseとcrates.io公開は行わない。

## 手動公開の代替手順

GitHub Actionsが使えない場合だけ、次を手動で実行する。

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
git switch master
git pull --ff-only origin master
just release-check
cargo publish --locked
gh release create v0.1.0 \
  --repo HiroyukiFuruno/katana-markdown-model \
  --target master \
  --title "katana-markdown-model v0.1.0" \
  --notes-file docs/release-notes/v0.1.0.md
```

## 失敗時方針

crates.ioへ公開済みのversionは再利用しない。

- crates.io公開前に失敗した場合: 修正後に同じ `v0.1.0` で再実行できる
- crates.io公開後にGitHub Release作成だけ失敗した場合: 同じtagでGitHub Release作成を再実行する
- crates.io公開後に内容不備が見つかった場合: `v0.1.1` 以降で修正する

## 公開後verify

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
gh release view v0.1.0 --repo HiroyukiFuruno/katana-markdown-model
cargo search katana-markdown-model --limit 1
cargo info katana-markdown-model@0.1.0
```

## branch hygiene

release後は、local branch、remote branch、worktreeを確認して整理する。

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
git switch master
git pull --ff-only origin master
git branch --list
git worktree list
git branch -d release/v0.1.0
git push origin --delete release/v0.1.0
```

remote branchをPR merge時に削除済みの場合、`git push origin --delete release/v0.1.0` は不要である。
