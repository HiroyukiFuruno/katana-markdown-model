# Canonical Fixtures

## Summary

KMM canonical fixtures are the source of truth for reproducing current KatanA behavior inside KMM.

Standard tests must not depend on absolute paths under `/Users/hiroyuki_furuno/works/private/katana`. After reviewing current KatanA fixtures, sync the needed inputs into `tests/fixtures/canonical/` in this repository and use the synced files as test inputs.

## Sources and Destinations

| source | destination | purpose |
| --- | --- | --- |
| `/Users/hiroyuki_furuno/works/private/katana/assets/fixtures/sample.md` | `tests/fixtures/canonical/katana_sample.md` | comprehensive current KatanA Markdown fixture |
| `/Users/hiroyuki_furuno/works/private/katana/assets/fixtures/sample_basic.md` | `tests/fixtures/canonical/katana_sample_basic.md` | basic Markdown and alert fixture |
| `/Users/hiroyuki_furuno/works/private/katana/README.md` | `tests/fixtures/canonical/katana_readme.md` | README header and badge row |
| `tests/fixtures/description_list.md` | `tests/fixtures/canonical/description_list.md` | description-list source of truth |

## Sync Procedure

When importing KatanA fixture updates into KMM, inspect the diff before syncing.

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
cp /Users/hiroyuki_furuno/works/private/katana/assets/fixtures/sample.md tests/fixtures/canonical/katana_sample.md
cp /Users/hiroyuki_furuno/works/private/katana/assets/fixtures/sample_basic.md tests/fixtures/canonical/katana_sample_basic.md
cp /Users/hiroyuki_furuno/works/private/katana/README.md tests/fixtures/canonical/katana_readme.md
cp tests/fixtures/description_list.md tests/fixtures/canonical/description_list.md
```

After syncing, run:

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-model
scripts/openspec validate "stabilize-canonical-fixtures" --strict
just check
```

## Expected Major Structures

`katana_sample.md` fixes:

- heading
- centered HTML block
- README badge row
- lists and task markers
- code block
- Mermaid / PlantUML / DrawIo / math code block
- table and alignment
- blockquote
- legacy note block
- thematic break
- emoji

`katana_sample_basic.md` fixes:

- heading
- lists and task markers
- table and alignment
- legacy note block
- GFM alert block
- paragraph containing footnote syntax
- special characters and emoji

`katana_readme.md` fixes:

- centered icon block
- centered heading
- centered description
- centered link row
- badge row
- language selector

`description_list.md` fixes:

- term
- description
- description-list node

## Elements Not Structured Yet

The following elements are included in canonical fixtures but are not fixed as dedicated DTOs by `stabilize-canonical-fixtures`:

- footnote
- image
- link
- HTML inline
- math inline

They keep raw snippets, source ranges, and fingerprints. Later parser strategy or metadata contract changes can add dedicated DTOs if required.
