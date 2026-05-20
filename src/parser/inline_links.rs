use super::inline::InlineScanner;
use super::inline_syntax::{autolink_destination, destination_title, html_end};
use crate::{FootnoteReferenceNode, ImageNode, InlineHtmlNode, KmmNode, KmmNodeKind, LinkNode};

impl InlineScanner<'_> {
    pub(super) fn image(&mut self) -> Option<KmmNode> {
        let start = self.offset;
        let rest = &self.raw[start..];
        let label_end = rest.strip_prefix("![")?.find(']')? + start + 2;
        let target_start = label_end + 2;
        if !self.raw[label_end..].starts_with("](") {
            return None;
        }
        let target_end = self.raw[target_start..].find(')')? + target_start;
        let (src, title) = destination_title(&self.raw[target_start..target_end])?;
        let kind = KmmNodeKind::Image(ImageNode {
            alt: self.raw[start + 2..label_end].to_string(),
            src,
            title,
        });
        Some(self.take_node(kind, start, target_end + 1))
    }

    pub(super) fn footnote_reference(&mut self) -> Option<KmmNode> {
        let start = self.offset;
        let rest = self.raw[start..].strip_prefix("[^")?;
        let end = rest.find(']')? + start + 2;
        let label = self.raw[start + 2..end].to_string();
        Some(self.take_node(
            KmmNodeKind::FootnoteReference(FootnoteReferenceNode { label }),
            start,
            end + 1,
        ))
    }

    pub(super) fn link(&mut self) -> Option<KmmNode> {
        let start = self.offset;
        if !self.raw[start..].starts_with('[') {
            return None;
        }
        let label_end = self.raw[start + 1..].find(']')? + start + 1;
        let target_start = label_end + 2;
        if !self.raw[label_end..].starts_with("](") {
            return None;
        }
        let target_end = self.raw[target_start..].find(')')? + target_start;
        let (destination, title) = destination_title(&self.raw[target_start..target_end])?;
        let kind = KmmNodeKind::Link(LinkNode {
            label: self.raw[start + 1..label_end].to_string(),
            destination,
            title,
            autolink: false,
        });
        Some(self.take_node(kind, start, target_end + 1))
    }

    pub(super) fn autolink(&mut self) -> Option<KmmNode> {
        let start = self.offset;
        let end = self.raw[start..].strip_prefix('<')?.find('>')? + start + 1;
        let destination = &self.raw[start + 1..end];
        if destination.contains(' ') || !autolink_destination(destination) {
            return None;
        }
        let kind = KmmNodeKind::Link(LinkNode {
            label: destination.to_string(),
            destination: destination.to_string(),
            title: None,
            autolink: true,
        });
        Some(self.take_node(kind, start, end + 1))
    }

    pub(super) fn inline_html(&mut self) -> Option<KmmNode> {
        let start = self.offset;
        let end = html_end(self.raw, start)?;
        let kind = KmmNodeKind::InlineHtml(InlineHtmlNode {
            html: self.raw[start..end].to_string(),
        });
        Some(self.take_node(kind, start, end))
    }
}
