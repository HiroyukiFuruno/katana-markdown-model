use super::emoji::EmojiParser;
use super::inline::InlineScanner;
use crate::{
    EmojiNode, InlineCodeNode, InlineMathNode, InlineSpan, KmmNode, KmmNodeKind, SourceSpan,
};

impl<F> InlineScanner<'_, F>
where
    F: Fn(usize, usize) -> SourceSpan,
{
    pub(super) fn strong(&mut self) -> Option<KmmNode> {
        self.delimited("**", |text| KmmNodeKind::Strong(InlineSpan { text }))
    }

    pub(super) fn emphasis(&mut self) -> Option<KmmNode> {
        self.delimited("*", |text| KmmNodeKind::Emphasis(InlineSpan { text }))
    }

    pub(super) fn strikethrough(&mut self) -> Option<KmmNode> {
        self.delimited("~~", |text| KmmNodeKind::Strikethrough(InlineSpan { text }))
    }

    pub(super) fn inline_code(&mut self) -> Option<KmmNode> {
        self.delimited("`", |code| KmmNodeKind::InlineCode(InlineCodeNode { code }))
    }

    pub(super) fn inline_math(&mut self) -> Option<KmmNode> {
        self.delimited("$", |expression| {
            KmmNodeKind::InlineMath(InlineMathNode { expression })
        })
    }

    pub(super) fn emoji(&mut self) -> Option<KmmNode> {
        let start = self.offset;
        if let Some((emoji, end)) = EmojiParser::shortcode_at(self.raw, start) {
            return Some(self.take_node(KmmNodeKind::Emoji(emoji), start, end));
        }
        let character = self.raw[start..].chars().next()?;
        if !EmojiParser::unicode(character) {
            return None;
        }
        let emoji = EmojiNode {
            value: character.to_string(),
            shortcode: None,
        };
        Some(self.take_node(
            KmmNodeKind::Emoji(emoji),
            start,
            start + character.len_utf8(),
        ))
    }
}
