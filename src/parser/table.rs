use crate::{SourceSpan, TableAlignment, TableCell, TableNode, TableRow};

pub(crate) struct TableParser;

impl TableParser {
    pub(crate) fn table_node(rows: Vec<TableRow>) -> TableNode {
        let alignments = rows
            .get(1)
            .map(|row| row.cells.iter().map(|cell| alignment(&cell.text)).collect())
            .unwrap_or_default();
        TableNode { alignments, rows }
    }

    pub(crate) fn table_row(
        line: &str,
        line_start: usize,
        cell_span: impl Fn(usize, usize) -> SourceSpan,
    ) -> TableRow {
        TableRow {
            cells: split_table_cell_ranges(line)
                .into_iter()
                .map(|range| {
                    let text = line[range.start..range.end].trim().to_string();
                    TableCell {
                        text,
                        source: cell_span(line_start + range.start, line_start + range.end),
                    }
                })
                .collect(),
        }
    }

    pub(crate) fn table_separator(line: &str) -> bool {
        split_table_cells(line)
            .iter()
            .all(|cell| cell.chars().all(|it| matches!(it, '-' | ':' | ' ')))
    }
}

struct CellRange {
    start: usize,
    end: usize,
}

fn split_table_cell_ranges(line: &str) -> Vec<CellRange> {
    let content_start = usize::from(line.starts_with('|'));
    let content_end = line
        .strip_suffix('|')
        .map(str::len)
        .unwrap_or_else(|| line.len());
    let mut ranges = Vec::new();
    let mut start = content_start;
    for (index, character) in line[content_start..content_end].char_indices() {
        if character == '|' {
            ranges.push(CellRange {
                start,
                end: content_start + index,
            });
            start = content_start + index + character.len_utf8();
        }
    }
    ranges.push(CellRange {
        start,
        end: content_end,
    });
    ranges
}

fn split_table_cells(line: &str) -> Vec<String> {
    split_table_cell_ranges(line)
        .into_iter()
        .map(|range| line[range.start..range.end].trim().to_string())
        .collect()
}

fn alignment(cell: &str) -> TableAlignment {
    match (cell.starts_with(':'), cell.ends_with(':')) {
        (true, true) => TableAlignment::Center,
        (true, false) => TableAlignment::Left,
        (false, true) => TableAlignment::Right,
        (false, false) => TableAlignment::Unspecified,
    }
}
