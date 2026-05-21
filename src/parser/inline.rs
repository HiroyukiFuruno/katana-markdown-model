use super::inline_syntax::special_start;
use crate::{KmmNode, KmmNodeKind, SourceSpan, TextSpan};
use std::collections::HashMap;

pub(crate) struct InlineParser;

impl InlineParser {
    pub(crate) fn nodes(
        raw: &str,
        span_for_match: impl Fn(usize, usize) -> SourceSpan,
    ) -> Vec<KmmNode> {
        InlineScanner::new(raw, &span_for_match).nodes()
    }
}

pub(super) struct InlineScanner<'a> {
    pub(super) raw: &'a str,
    pub(super) offset: usize,
    pub(super) ordinals: HashMap<&'static str, usize>,
    pub(super) span_for_match: &'a dyn Fn(usize, usize) -> SourceSpan,
}

impl<'a> InlineScanner<'a> {
    fn new(raw: &'a str, span_for_match: &'a dyn Fn(usize, usize) -> SourceSpan) -> Self {
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
        let bounds = self.delimited_bounds(delimiter)?;
        Some(self.take_node(
            kind(self.raw[bounds.content_start..bounds.content_end].to_string()),
            bounds.start,
            bounds.end,
        ))
    }

    pub(super) fn nested_delimited(
        &mut self,
        delimiter: &str,
        kind: impl Fn(String) -> KmmNodeKind,
    ) -> Option<KmmNode> {
        let bounds = self.delimited_bounds(delimiter)?;
        let content = &self.raw[bounds.content_start..bounds.content_end];
        let children = InlineParser::nodes(content, |start, end| {
            (self.span_for_match)(bounds.content_start + start, bounds.content_start + end)
        });
        let mut node = self.take_node(kind(content.to_string()), bounds.start, bounds.end);
        node.children = children;
        Some(node)
    }

    fn delimited_bounds(&self, delimiter: &str) -> Option<DelimitedBounds> {
        let start = self.offset;
        let content_start = start + delimiter.len();
        if !self.raw[start..].starts_with(delimiter) {
            return None;
        }
        let content_end = self.raw[content_start..].find(delimiter)? + content_start;
        if content_start == content_end {
            return None;
        }
        Some(DelimitedBounds {
            start,
            content_start,
            content_end,
            end: content_end + delimiter.len(),
        })
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

struct DelimitedBounds {
    start: usize,
    content_start: usize,
    content_end: usize,
    end: usize,
}
