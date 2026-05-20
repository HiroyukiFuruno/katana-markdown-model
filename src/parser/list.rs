use super::block::BlockParser;
use super::inline::InlineParser;
use super::line_index::{LineIndex, SourceLine};
use super::list_syntax::{
    ListMarker, list_marker, nested_list_end, next_item_offset, task_and_body, task_marker,
};
use crate::{KmmNode, KmmNodeKind, ListItemNode, ListNode, SourceSpan};

pub(crate) struct ListBlockParser<'a> {
    source: &'a str,
    index: &'a LineIndex,
}

impl<'a> ListBlockParser<'a> {
    pub(crate) fn new(source: &'a str, index: &'a LineIndex) -> Self {
        Self { source, index }
    }

    pub(crate) fn parse(&self, lines: &[SourceLine]) -> ListNode {
        ListNode {
            ordered: lines
                .iter()
                .any(|line| BlockParser::ordered_list_line(&line.text)),
            task_markers: lines
                .iter()
                .filter_map(|line| task_marker(line.text.trim_start()))
                .collect(),
            items: self.items(lines),
        }
    }

    fn items(&self, lines: &[SourceLine]) -> Vec<ListItemNode> {
        let Some(base_marker) = lines.first().and_then(|line| list_marker(&line.text)) else {
            return Vec::new();
        };
        let mut items = Vec::new();
        let mut offset = 0;
        while offset < lines.len() {
            let Some(marker) = list_marker(&lines[offset].text) else {
                offset += 1;
                continue;
            };
            if marker.indent != base_marker.indent {
                offset += 1;
                continue;
            }
            let end = next_item_offset(lines, offset + 1, marker.indent);
            items.push(self.item(&lines[offset..end], marker));
            offset = end;
        }
        items
    }

    fn item(&self, lines: &[SourceLine], marker: ListMarker) -> ListItemNode {
        let first = &lines[0];
        let (task, body_start) = task_and_body(&first.text[marker.body_start..]);
        let body_offset = marker.body_start + body_start;
        let body_text = first.text[body_offset..].to_string();
        ListItemNode {
            marker: marker.marker,
            ordered_number: marker.ordered_number,
            task_marker: task,
            body: self.inline_body(first, body_offset, &body_text),
            children: self.children(&lines[1..], marker.indent),
            source: self.line_span(lines),
        }
    }

    fn inline_body(&self, line: &SourceLine, offset: usize, text: &str) -> Vec<KmmNode> {
        InlineParser::nodes(text, |start, end| {
            self.index.source_span_for_byte_range(
                self.source,
                line.start + offset + start,
                line.start + offset + end,
            )
        })
    }

    fn children(&self, lines: &[SourceLine], base_indent: usize) -> Vec<KmmNode> {
        let mut children = Vec::new();
        let mut offset = 0;
        while offset < lines.len() {
            let text = lines[offset].text.trim_start();
            if text.is_empty() {
                offset += 1;
                continue;
            }
            if text.starts_with("```") {
                let (node, next) = self.code_child(lines, offset);
                children.push(node);
                offset = next;
                continue;
            }
            if let Some(marker) = list_marker(&lines[offset].text)
                && marker.indent > base_indent
            {
                let end = nested_list_end(lines, offset + 1, marker.indent, base_indent);
                children.push(self.list_child(&lines[offset..end], children.len()));
                offset = end;
                continue;
            }
            offset += 1;
        }
        children
    }

    fn code_child(&self, lines: &[SourceLine], start: usize) -> (KmmNode, usize) {
        let mut end = start + 1;
        while end < lines.len() {
            let text = lines[end].text.trim_start();
            end += 1;
            if text.starts_with("```") {
                break;
            }
        }
        let span = self.line_span(&lines[start..end]);
        let role = BlockParser::code_block_role(lines[start].text.trim_start());
        let raw = span.raw.text.clone();
        (
            KmmNode::new(KmmNodeKind::CodeBlock(role), &raw, 0, span),
            end,
        )
    }

    fn list_child(&self, lines: &[SourceLine], ordinal: usize) -> KmmNode {
        let span = self.line_span(lines);
        let raw = span.raw.text.clone();
        KmmNode::new(KmmNodeKind::List(self.parse(lines)), &raw, ordinal, span)
    }

    fn line_span(&self, lines: &[SourceLine]) -> SourceSpan {
        self.index.source_span_for_byte_range(
            self.source,
            lines.first().expect("line span needs first line").start,
            lines.last().expect("line span needs last line").end,
        )
    }
}
