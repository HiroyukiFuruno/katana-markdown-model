use super::emoji::EmojiParser;

pub(super) fn destination_title(raw: &str) -> Option<(String, Option<String>)> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }
    let Some((destination, title)) = trimmed.split_once(' ') else {
        return Some((trimmed.to_string(), None));
    };
    let parsed_title = title.trim().trim_matches('"').to_string();
    Some((destination.to_string(), Some(parsed_title)))
}

pub(super) fn autolink_destination(value: &str) -> bool {
    value.starts_with("https://") || value.starts_with("http://") || value.starts_with("mailto:")
}

pub(super) fn html_end(raw: &str, start: usize) -> Option<usize> {
    if !raw[start..].starts_with('<') {
        return None;
    }
    let tag_end = raw[start..].find('>')? + start + 1;
    let name = raw[start + 1..tag_end - 1]
        .chars()
        .take_while(|it| it.is_ascii_alphanumeric())
        .collect::<String>();
    if name.is_empty() {
        return Some(tag_end);
    }
    let closing = format!("</{name}>");
    raw[tag_end..]
        .find(&closing)
        .map(|relative| tag_end + relative + closing.len())
        .or(Some(tag_end))
}

pub(super) fn special_start(raw: &str, offset: usize, character: char) -> bool {
    matches!(character, '*' | '~' | '`' | '<' | '[' | '!' | '$' | ':')
        || EmojiParser::unicode(character)
        || raw[offset..].starts_with("![")
}
