# kmm-v0-1-1-precision-hardening Specification

## Purpose
KMM v0.1.1 計画時点で定義した precision hardening の判断基準を保存する。後続の v0.2.0 / v0.2.1 で実装済みの DTO / parser regression とは別に、KMM が renderer-neutral 境界を保ちながら downstream adoption 結果を分類するための基準として扱う。

## Requirements
### Requirement: KMM treats v0.1.1 as precision hardening

KMM SHALL use `v0.1.1` for precision improvements discovered after `v0.1.0` adoption.

#### Scenario: Downstream adoption finds a parsing issue

- **WHEN** KDV, KLE, or KatanA finds a KMM parsing mismatch after adopting `v0.1.0`
- **THEN** KMM records the issue as a `v0.1.1` candidate
- **THEN** KMM reproduces it with a repository-local fixture before changing implementation

### Requirement: KMM keeps the renderer-neutral boundary

KMM MUST NOT add renderer, export, UI, or synchronization control responsibilities in `v0.1.1`.

#### Scenario: A downstream repository needs rendered output

- **WHEN** rendered Markdown, HTML, PDF, PNG, JPG, scroll, selection, or highlight behavior is required
- **THEN** KMM does not implement that behavior
- **THEN** KMM only returns document model data, source mapping, metadata resolution, or synchronization materials

### Requirement: KMM preserves metadata resolution safety

KMM SHALL improve metadata target matching without weakening unresolved or conflict handling.

#### Scenario: Metadata target recovery is ambiguous

- **WHEN** KMM finds multiple plausible targets for one metadata entry
- **THEN** KMM returns conflict
- **THEN** KMM does not silently choose one candidate

### Requirement: KMM evaluates new sync anchor materials conservatively

KMM SHALL add synchronization anchor materials only when existing public DTO fields are insufficient.

#### Scenario: KatanA cannot map editor and viewer nodes with existing fields

- **WHEN** node id, source range, line-column, raw snippet, and fingerprint are insufficient for synchronization mapping
- **THEN** KMM may add renderer-neutral anchor material to the public DTO
- **THEN** KMM still does not control editor or viewer synchronization
