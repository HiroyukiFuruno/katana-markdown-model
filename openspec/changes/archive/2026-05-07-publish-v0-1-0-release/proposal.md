## Why

KMMは `v0.1.0` が初回公開版である。

全ての分割changeが完了する前に公開すると、downstreamへ未固定のpublic DTOやmetadata APIを渡すことになる。

## What Changes

- `v0.1.0` 公開前の完了条件を固定する。
- GitHub Releaseとcrates.io公開手順を定義する。
- `CARGO_REGISTRY_TOKEN` など必要なsecretを明記する。
- 公開済みversionを再利用しない方針を明記する。
- 公開後verifyを定義する。

## Impact

- release runbook
- `just release-check`
- release workflow
- GitHub secrets
- branch strategy

このchangeは、前段の全changeが完了してから実施する。
