use super::block::BlockParser;
use super::engine::ParserCursor;
use super::list::ListBlockParser;
use super::table::TableParser;
use crate::{DollarMathBlockNode, FootnoteDefinitionNode, KmmNodeKind};

impl ParserCursor<'_> {
    pub(super) fn code_block(&mut self) -> KmmNodeKind {
        let role = BlockParser::code_block_role(&self.current().text);
        self.line += 1;
        while self.line < self.index.lines().len() {
            let text = self.current().text.trim_start().to_string();
            self.line += 1;
            if text.starts_with("```") {
                break;
            }
        }
        KmmNodeKind::CodeBlock(role)
    }

    pub(super) fn html_block(&mut self) -> KmmNodeKind {
        let start = self.line;
        self.line += 1;
        while self.line < self.index.lines().len() && !self.html_closed(start, self.line) {
            self.line += 1;
        }
        let raw = self.raw_text(start, self.line);
        KmmNodeKind::HtmlBlock(BlockParser::html_role(&raw))
    }

    pub(super) fn table(&mut self) -> KmmNodeKind {
        let mut rows = Vec::new();
        while self.line < self.index.lines().len() && self.current().text.contains('|') {
            let line = self.current();
            rows.push(TableParser::table_row(
                &line.text,
                line.start,
                |start, end| {
                    self.index
                        .source_span_for_byte_range(self.source, start, end)
                },
            ));
            self.line += 1;
        }
        KmmNodeKind::Table(TableParser::table_node(rows))
    }

    pub(super) fn block_quote(&mut self) -> KmmNodeKind {
        let mut lines = Vec::new();
        while self.line < self.index.lines().len()
            && self.current().text.trim_start().starts_with('>')
        {
            lines.push(self.current().text.clone());
            self.line += 1;
        }
        BlockParser::alert_label(&lines)
            .map(|label| KmmNodeKind::Alert { label })
            .unwrap_or(KmmNodeKind::BlockQuote)
    }

    pub(super) fn description_list(&mut self) -> KmmNodeKind {
        let mut lines = Vec::new();
        while self.line + 1 < self.index.lines().len() && self.next_line_starts_description() {
            lines.push(self.current().text.clone());
            lines.push(self.index.lines()[self.line + 1].text.clone());
            self.line += 2;
            self.skip_description_gap();
        }
        KmmNodeKind::DescriptionList {
            items: BlockParser::description_items(&lines),
        }
    }

    pub(super) fn list(&mut self) -> KmmNodeKind {
        let mut lines = Vec::new();
        while self.line < self.index.lines().len() {
            let text = &self.current().text;
            if !self.belongs_to_list_block(text) {
                break;
            }
            lines.push(self.current().clone());
            self.line += 1;
        }
        KmmNodeKind::List(ListBlockParser::new(self.source, self.index).parse(&lines))
    }

    pub(super) fn paragraph(&mut self) -> KmmNodeKind {
        self.line += 1;
        KmmNodeKind::Paragraph
    }

    pub(super) fn footnote_definition(&mut self) -> KmmNodeKind {
        let text = self.current().text.trim_start();
        let label_end = text.find("]:").expect("footnote definition is prechecked");
        let label = text[2..label_end].to_string();
        let body = text[label_end + 2..].trim().to_string();
        self.line += 1;
        KmmNodeKind::FootnoteDefinition(FootnoteDefinitionNode { label, text: body })
    }

    pub(super) fn dollar_math_block(&mut self) -> KmmNodeKind {
        self.line += 1;
        let mut expression = Vec::new();
        while self.line < self.index.lines().len() {
            let text = self.current().text.trim();
            if text == "$$" {
                self.line += 1;
                break;
            }
            expression.push(self.current().text.clone());
            self.line += 1;
        }
        KmmNodeKind::DollarMathBlock(DollarMathBlockNode {
            expression: expression.join("\n"),
        })
    }

    pub(super) fn is_table_start(&self) -> bool {
        self.line + 1 < self.index.lines().len()
            && self.current().text.contains('|')
            && TableParser::table_separator(&self.index.lines()[self.line + 1].text)
    }

    pub(super) fn is_description_start(&self) -> bool {
        self.line + 1 < self.index.lines().len() && self.next_line_starts_description()
    }

    pub(super) fn is_footnote_definition(&self) -> bool {
        let trimmed = self.current().text.trim_start();
        trimmed.starts_with("[^") && trimmed.contains("]:")
    }

    pub(super) fn is_html_start(&self, line: &str) -> bool {
        let trimmed = line.trim_start();
        trimmed.starts_with("<p")
            || trimmed.starts_with("<h1")
            || trimmed.starts_with("<details")
            || trimmed.starts_with("<div")
    }

    fn skip_description_gap(&mut self) {
        if self.line + 2 >= self.index.lines().len() || !self.current().text.trim().is_empty() {
            return;
        }
        if self.index.lines()[self.line + 2]
            .text
            .trim_start()
            .starts_with(':')
        {
            self.line += 1;
        }
    }

    fn next_line_starts_description(&self) -> bool {
        self.index.lines()[self.line + 1]
            .text
            .trim_start()
            .starts_with(':')
    }

    fn html_closed(&self, start: usize, end: usize) -> bool {
        let raw = self.raw_text(start, end);
        raw.contains("</p>")
            || raw.contains("</h1>")
            || raw.contains("</details>")
            || raw.contains("</div>")
    }

    fn belongs_to_list_block(&self, text: &str) -> bool {
        if BlockParser::unordered_list_line(text) || BlockParser::ordered_list_line(text) {
            return true;
        }
        if text.trim().is_empty() {
            return self.line + 1 < self.index.lines().len()
                && self.index.lines()[self.line + 1].text.starts_with("  ");
        }
        text.starts_with("  ")
    }
}
