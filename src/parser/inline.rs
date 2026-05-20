use super::inline_syntax::special_start;
use crate::{KmmNode, KmmNodeKind, SourceSpan, TextSpan};
use std::collections::HashMap;

pub(crate) struct InlineParser;

impl InlineParser {
    pub(crate) fn nodes(
        raw: &str,
        span_for_match: impl Fn(usize, usize) -> SourceSpan,
    ) -> Vec<KmmNode> {
        InlineScanner::new(raw, span_for_match).nodes()
    }
}

pub(super) struct InlineScanner<'a, F>
where
    F: Fn(usize, usize) -> SourceSpan,
{
    pub(super) raw: &'a str,
    pub(super) offset: usize,
    pub(super) ordinals: HashMap<&'static str, usize>,
    pub(super) span_for_match: F,
}

impl<'a, F> InlineScanner<'a, F>
where
    F: Fn(usize, usize) -> SourceSpan,
{
    fn new(raw: &'a str, span_for_match: F) -> Self {
        Self {
            raw,
            offset: 0,
            ordinals: HashMap::new(),
            span_for_match,
        }
    }

    fn nodes(&mut self) -> Vec<KmmNode> {
        let mut nodes = Vec::new();
        while self.offset < self.raw.len() {
            let node = self.special_node().unwrap_or_else(|| self.text_node());
            nodes.push(node);
        }
        nodes
    }

    fn special_node(&mut self) -> Option<KmmNode> {
        self.image()
            .or_else(|| self.footnote_reference())
            .or_else(|| self.link())
            .or_else(|| self.autolink())
            .or_else(|| self.strong())
            .or_else(|| self.emphasis())
            .or_else(|| self.strikethrough())
            .or_else(|| self.inline_code())
            .or_else(|| self.inline_html())
            .or_else(|| self.inline_math())
            .or_else(|| self.emoji())
    }

    pub(super) fn delimited(
        &mut self,
        delimiter: &str,
        kind: impl Fn(String) -> KmmNodeKind,
    ) -> Option<KmmNode> {
        let start = self.offset;
        let content_start = start + delimiter.len();
        if !self.raw[start..].starts_with(delimiter) {
            return None;
        }
        let content_end = self.raw[content_start..].find(delimiter)? + content_start;
        if content_start == content_end {
            return None;
        }
        Some(self.take_node(
            kind(self.raw[content_start..content_end].to_string()),
            start,
            content_end + delimiter.len(),
        ))
    }

    fn text_node(&mut self) -> KmmNode {
        let start = self.offset;
        let end = self.text_end();
        let text = self.raw[start..end].to_string();
        self.take_node(KmmNodeKind::Text(TextSpan { text }), start, end)
    }

    fn text_end(&self) -> usize {
        let first = self.raw[self.offset..]
            .chars()
            .next()
            .expect("offset points to a character");
        let from = self.offset + first.len_utf8();
        self.raw[from..]
            .char_indices()
            .find_map(|(relative, character)| {
                let absolute = from + relative;
                special_start(self.raw, absolute, character).then_some(absolute)
            })
            .unwrap_or(self.raw.len())
    }

    pub(super) fn take_node(&mut self, kind: KmmNodeKind, start: usize, end: usize) -> KmmNode {
        let source = (self.span_for_match)(start, end);
        let label = kind.label();
        let ordinal = *self
            .ordinals
            .entry(label)
            .and_modify(|it| *it += 1)
            .or_insert(0);
        self.offset = end;
        KmmNode::new(kind, &source.raw.text.clone(), ordinal, source)
    }
}
