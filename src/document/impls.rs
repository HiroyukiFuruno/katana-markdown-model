use super::types::{KmmDocument, KmmNode, KmmNodeId, KmmNodeKind};
use crate::TextFingerprint;

impl KmmDocument {
    pub fn nodes_by_kind(&self, predicate: impl Fn(&KmmNodeKind) -> bool) -> Vec<&KmmNode> {
        self.nodes
            .iter()
            .filter(|node| predicate(&node.kind))
            .collect()
    }

    pub fn node_by_id(&self, id: &KmmNodeId) -> Option<&KmmNode> {
        self.nodes.iter().find(|node| &node.id == id)
    }
}

impl KmmNode {
    pub(crate) fn new(
        kind: KmmNodeKind,
        raw: &str,
        ordinal: usize,
        source: crate::SourceSpan,
    ) -> Self {
        Self {
            id: KmmNodeId::from_parts(&kind, raw, ordinal),
            kind,
            source,
            children: Vec::new(),
        }
    }
}

impl KmmNodeId {
    pub(crate) fn from_parts(kind: &KmmNodeKind, raw: &str, ordinal: usize) -> Self {
        let seed = format!("{}\0{}\0{}", kind.label(), raw, ordinal);
        Self(format!("kmm-{}", TextFingerprint::for_text(&seed).value))
    }
}

impl KmmNodeKind {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Heading(_) => "heading",
            Self::Paragraph => "paragraph",
            Self::Emoji(_) => "emoji",
            Self::HtmlBlock(_) => "html-block",
            Self::List(_) => "list",
            Self::CodeBlock(_) => "code-block",
            Self::Table(_) => "table",
            Self::BlockQuote => "blockquote",
            Self::Alert { .. } => "alert",
            Self::DescriptionList { .. } => "description-list",
            Self::ThematicBreak => "thematic-break",
            Self::RawBlock { .. } => "raw-block",
        }
    }
}
