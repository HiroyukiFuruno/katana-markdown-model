# Manual Harness

## Summary

`just harness-up` is a development-only entry point for human inspection of KMM output structure.

It is not a product CLI or product UI. KMM remains library-only.

Fixture tests are the authoritative release gate. The harness is not used for validating Markdown rendering, viewer behavior, or export behavior.

## What the Screen Shows

The screen shows:

- Markdown input
- KMM node list
- selected node source range
- selected node line-column position
- selected node raw snippet
- selected node fingerprint
- metadata resolution state

Metadata resolution states show `Resolved`, `Moved`, `Unresolved`, and `Conflict` on the same screen.

## Start

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
just harness-up
```

By default, it opens this KatanA fixture:

```text
/Users/hiroyuki_furuno/works/private/katana/assets/fixtures/sample.md
```

Pass a path to inspect another Markdown file.

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
just harness-up /Users/hiroyuki_furuno/works/private/katana/assets/fixtures/sample.md
```

## Fixture Inspection Procedure

1. Confirm that the Markdown input shows KatanA `sample.md`.
2. Confirm that the node list includes headings, alerts, tables, description lists, diagrams, math, and emoji.
3. Select a node and confirm source range, line-column position, raw snippet, and fingerprint.
4. Confirm that metadata resolution states include `Resolved`, `Moved`, `Unresolved`, and `Conflict`.
5. Record the result in `docs/release-readiness/<version>-manual-harness.md`.

## Package Check

Before release, verify that the development harness is not included in the published crate.

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
cargo package --locked --allow-dirty --list
```

The output must not include `tools/manual-harness`.
