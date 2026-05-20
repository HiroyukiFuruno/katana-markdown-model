use super::line_index::SourceLine;

const TASK_MARKER_WIDTH: usize = 4;

#[derive(Clone)]
pub(super) struct ListMarker {
    pub(super) indent: usize,
    pub(super) marker: String,
    pub(super) ordered_number: Option<usize>,
    pub(super) body_start: usize,
}

pub(super) fn list_marker(line: &str) -> Option<ListMarker> {
    let indent = leading_spaces(line);
    let trimmed = &line[indent..];
    if trimmed.starts_with("- ") {
        return Some(ListMarker {
            indent,
            marker: "-".to_string(),
            ordered_number: None,
            body_start: indent + 2,
        });
    }
    let (number, rest) = trimmed.split_once('.')?;
    if number.is_empty() || !number.chars().all(|it| it.is_ascii_digit()) || !rest.starts_with(' ')
    {
        return None;
    }
    Some(ListMarker {
        indent,
        marker: format!("{number}."),
        ordered_number: number.parse().ok(),
        body_start: indent + number.len() + 2,
    })
}

pub(super) fn next_item_offset(lines: &[SourceLine], start: usize, base_indent: usize) -> usize {
    lines[start..]
        .iter()
        .position(|line| list_marker(&line.text).is_some_and(|marker| marker.indent == base_indent))
        .map(|relative| start + relative)
        .unwrap_or(lines.len())
}

pub(super) fn nested_list_end(
    lines: &[SourceLine],
    start: usize,
    nested_indent: usize,
    base_indent: usize,
) -> usize {
    lines[start..]
        .iter()
        .position(|line| {
            line.text.trim().is_empty()
                || list_marker(&line.text).is_some_and(|marker| marker.indent < nested_indent)
                || leading_spaces(&line.text) <= base_indent
        })
        .map(|relative| start + relative)
        .unwrap_or(lines.len())
}

pub(super) fn task_and_body(raw: &str) -> (Option<String>, usize) {
    task_marker(raw)
        .map(|marker| (Some(marker), TASK_MARKER_WIDTH))
        .unwrap_or((None, 0))
}

pub(super) fn task_marker(raw: &str) -> Option<String> {
    ["[x]", "[ ]", "[-]", "[/]"]
        .iter()
        .find(|marker| raw.starts_with(**marker))
        .map(|marker| (*marker).to_string())
}

fn leading_spaces(line: &str) -> usize {
    line.chars().take_while(|it| *it == ' ').count()
}
