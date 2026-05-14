use crate::{ByteRange, LineColumn, LineColumnRange, RawSnippet, SourceSpan};

#[derive(Debug, Clone)]
pub(crate) struct SourceLine {
    pub number: usize,
    pub start: usize,
    pub end: usize,
    pub text: String,
}

#[derive(Debug, Clone)]
pub(crate) struct LineIndex {
    lines: Vec<SourceLine>,
}

impl LineIndex {
    pub fn new(source: &str) -> Self {
        let mut lines = Vec::new();
        let mut offset = 0;
        for (index, segment) in source.split_inclusive('\n').enumerate() {
            let text = segment.trim_end_matches('\n').to_string();
            let end = offset + text.len();
            lines.push(SourceLine {
                number: index + 1,
                start: offset,
                end,
                text,
            });
            offset += segment.len();
        }
        if source.is_empty() {
            lines.push(SourceLine {
                number: 1,
                start: 0,
                end: 0,
                text: String::new(),
            });
        }
        Self { lines }
    }

    pub fn lines(&self) -> &[SourceLine] {
        &self.lines
    }

    pub fn source_span(&self, source: &str, start_line: usize, end_line: usize) -> SourceSpan {
        let first = &self.lines[start_line];
        let last = &self.lines[end_line - 1];
        let end = last.end;
        self.source_span_for_byte_range(source, first.start, end)
    }

    pub fn source_span_for_byte_range(
        &self,
        source: &str,
        start_offset: usize,
        end_offset: usize,
    ) -> SourceSpan {
        let start = self.line_column(start_offset);
        let end = self.line_column(end_offset);
        SourceSpan {
            byte_range: ByteRange {
                start: start_offset,
                end: end_offset,
            },
            line_column_range: LineColumnRange { start, end },
            raw: RawSnippet::new(source[start_offset..end_offset].to_string()),
        }
    }

    fn line_column(&self, offset: usize) -> LineColumn {
        let line = self
            .lines
            .iter()
            .find(|line| line.start <= offset && offset <= line.end)
            .unwrap_or_else(|| self.lines.last().expect("line index must not be empty"));
        LineColumn {
            line: line.number,
            column: source_column(&line.text, offset - line.start),
        }
    }
}

fn source_column(line: &str, byte_offset: usize) -> usize {
    line[..byte_offset].chars().count() + 1
}
