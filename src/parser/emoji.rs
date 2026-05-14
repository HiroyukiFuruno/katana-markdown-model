use crate::{EmojiNode, KmmNode, KmmNodeKind, SourceSpan};

pub(crate) struct EmojiParser;

impl EmojiParser {
    pub(crate) fn emoji_nodes(
        raw: &str,
        span_for_match: impl Fn(usize, usize) -> SourceSpan,
    ) -> Vec<KmmNode> {
        let mut nodes = shortcode_emoji_nodes(raw, &span_for_match);
        nodes.extend(unicode_emoji_nodes(raw, span_for_match));
        nodes
    }
}

fn shortcode_emoji_nodes(
    raw: &str,
    span_for_match: &impl Fn(usize, usize) -> SourceSpan,
) -> Vec<KmmNode> {
    let mut nodes = Vec::new();
    let mut offset = 0;
    while let Some(start) = raw[offset..].find(':') {
        let absolute_start = offset + start;
        let search_from = absolute_start + 1;
        let Some(end) = raw[search_from..].find(':') else {
            break;
        };
        let absolute_end = search_from + end + 1;
        let candidate = &raw[absolute_start + 1..absolute_end - 1];
        if shortcode_name(candidate) {
            let source = span_for_match(absolute_start, absolute_end);
            nodes.push(emoji_node(
                raw[absolute_start..absolute_end].to_string(),
                Some(candidate.to_string()),
                nodes.len(),
                source,
            ));
        }
        offset = absolute_end;
    }
    nodes
}

fn unicode_emoji_nodes(
    raw: &str,
    span_for_match: impl Fn(usize, usize) -> SourceSpan,
) -> Vec<KmmNode> {
    raw.char_indices()
        .filter(|(_, character)| unicode_emoji(*character))
        .enumerate()
        .map(|(ordinal, (start, character))| {
            let end = start + character.len_utf8();
            emoji_node(
                character.to_string(),
                None,
                ordinal,
                span_for_match(start, end),
            )
        })
        .collect()
}

fn emoji_node(
    value: String,
    shortcode: Option<String>,
    ordinal: usize,
    source: SourceSpan,
) -> KmmNode {
    let kind = KmmNodeKind::Emoji(EmojiNode { value, shortcode });
    KmmNode::new(kind, &source.raw.text.clone(), ordinal, source)
}

fn shortcode_name(candidate: &str) -> bool {
    !candidate.is_empty()
        && candidate.chars().all(|it| {
            it.is_ascii_lowercase() || it.is_ascii_digit() || matches!(it, '_' | '-' | '+')
        })
}

fn unicode_emoji(character: char) -> bool {
    matches!(character as u32, 0x1F300..=0x1FAFF | 0x2600..=0x27BF)
}
