use crate::EmojiNode;

pub(crate) struct EmojiParser;

impl EmojiParser {
    pub(crate) fn shortcode_at(raw: &str, offset: usize) -> Option<(EmojiNode, usize)> {
        if !raw[offset..].starts_with(':') {
            return None;
        }
        let search_from = offset + 1;
        let end = raw[search_from..].find(':')?;
        let absolute_end = search_from + end + 1;
        let candidate = &raw[offset + 1..absolute_end - 1];
        if shortcode_name(candidate) {
            return Some((
                EmojiNode {
                    value: raw[offset..absolute_end].to_string(),
                    shortcode: Some(candidate.to_string()),
                },
                absolute_end,
            ));
        }
        None
    }

    pub(crate) fn unicode(character: char) -> bool {
        matches!(character as u32, 0x1F300..=0x1FAFF | 0x2600..=0x27BF)
    }
}

fn shortcode_name(candidate: &str) -> bool {
    !candidate.is_empty()
        && candidate.chars().all(|it| {
            it.is_ascii_lowercase() || it.is_ascii_digit() || matches!(it, '_' | '-' | '+')
        })
}
