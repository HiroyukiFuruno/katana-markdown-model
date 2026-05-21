use super::block::BlockParser;
use super::inline::InlineParser;
use super::line_index::{LineIndex, SourceLine};
use super::list::ListBlockParser;
use crate::{KmmNode, KmmNodeKind, SourceSpan};

pub(crate) struct BlockQuoteParser<'a> {
    source: &'a str,
    index: &'a LineIndex,
    span: &'a SourceSpan,
}

impl<'a> BlockQuoteParser<'a> {
    pub(crate) fn new(source: &'a str, index: &'a LineIndex, span: &'a SourceSpan) -> Self {
        Self {
            source,
            index,
            span,
        }
    }

    pub(crate) fn children(&self) -> Vec<KmmNode> {
        let lines = self.quote_lines();
        let mut children = Vec::new();
        let mut offset = self.skip_alert_label(&lines);
        while offset < lines.len() {
            if lines[offset].text.trim().is_empty() {
                offset += 1;
                continue;
            }
            if lines[offset].text.trim_start().starts_with("```") {
                let (node, next) = self.code_child(&lines, offset);
                children.push(node);
                offset = next;
                continue;
            }
            if BlockParser::unordered_list_line(&lines[offset].text)
                || BlockParser::ordered_list_line(&lines[offset].text)
            {
                let end = self.list_end(&lines, offset + 1);
                children.push(self.list_child(&lines[offset..end], children.len()));
                offset = end;
                continue;
            }
            children.push(self.paragraph_child(&lines[offset], children.len()));
            offset += 1;
        }
        children
    }

    fn quote_lines(&self) -> Vec<QuoteLine> {
        let mut lines = Vec::new();
        let mut offset = 0;
        for segment in self.span.raw.text.split_inclusive('\n') {
            let raw_line = segment.trim_end_matches('\n');
            let line_start = self.span.byte_range.start + offset;
            let line_end = line_start + raw_line.len();
            lines.push(quote_line(raw_line, line_start, line_end));
            offset += segment.len();
        }
        lines
    }

    fn skip_alert_label(&self, lines: &[QuoteLine]) -> usize {
        lines
            .first()
            .filter(|line| line.text.trim_start().starts_with("[!"))
            .map(|_| 1)
            .unwrap_or(0)
    }

    fn paragraph_child(&self, line: &QuoteLine, ordinal: usize) -> KmmNode {
        let span = self
            .index
            .source_span_for_byte_range(self.source, line.start, line.end);
        let mut node = KmmNode::new(
            KmmNodeKind::Paragraph,
            &span.raw.text.clone(),
            ordinal,
            span,
        );
        node.children = InlineParser::nodes(&line.text, |start, end| {
            self.index.source_span_for_byte_range(
                self.source,
                line.text_start + start,
                line.text_start + end,
            )
        });
        node
    }

    fn code_child(&self, lines: &[QuoteLine], start: usize) -> (KmmNode, usize) {
        let mut end = start + 1;
        while end < lines.len() {
            let text = lines[end].text.trim_start();
            end += 1;
            if text.starts_with("```") {
                break;
            }
        }
        let span = self.original_span(&lines[start..end]);
        let role = BlockParser::code_block_role(lines[start].text.trim_start());
        let raw = span.raw.text.clone();
        (
            KmmNode::new(KmmNodeKind::CodeBlock(role), &raw, 0, span),
            end,
        )
    }

    fn list_child(&self, lines: &[QuoteLine], ordinal: usize) -> KmmNode {
        let source_lines = lines
            .iter()
            .map(|line| SourceLine {
                number: 0,
                start: line.text_start,
                end: line.end,
                text: line.text.clone(),
            })
            .collect::<Vec<SourceLine>>();
        let span = self.original_span(lines);
        let raw = span.raw.text.clone();
        KmmNode::new(
            KmmNodeKind::List(ListBlockParser::new(self.source, self.index).parse(&source_lines)),
            &raw,
            ordinal,
            span,
        )
    }

    fn list_end(&self, lines: &[QuoteLine], start: usize) -> usize {
        lines[start..]
            .iter()
            .position(|line| {
                line.text.trim().is_empty()
                    || line.text.trim_start().starts_with("```")
                    || !(BlockParser::unordered_list_line(&line.text)
                        || BlockParser::ordered_list_line(&line.text)
                        || line.text.starts_with("  "))
            })
            .map(|relative| start + relative)
            .unwrap_or(lines.len())
    }

    fn original_span(&self, lines: &[QuoteLine]) -> SourceSpan {
        self.index.source_span_for_byte_range(
            self.source,
            lines.first().expect("quote child needs first line").start,
            lines.last().expect("quote child needs last line").end,
        )
    }
}

struct QuoteLine {
    start: usize,
    end: usize,
    text_start: usize,
    text: String,
}

fn quote_line(raw_line: &str, start: usize, end: usize) -> QuoteLine {
    let marker_start = raw_line.find('>').unwrap_or(0);
    let mut text_start = start + marker_start + 1;
    if raw_line[text_start - start..].starts_with(' ') {
        text_start += 1;
    }
    QuoteLine {
        start,
        end,
        text_start,
        text: raw_line[text_start - start..].to_string(),
    }
}
