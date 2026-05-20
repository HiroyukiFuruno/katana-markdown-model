use crate::{SourceSpan, TextFingerprint};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KmmNodeId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KmmDocument {
    pub path: PathBuf,
    pub fingerprint: TextFingerprint,
    pub nodes: Vec<KmmNode>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KmmNode {
    pub id: KmmNodeId,
    pub kind: KmmNodeKind,
    pub source: SourceSpan,
    pub children: Vec<KmmNode>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum KmmNodeKind {
    Heading(HeadingNode),
    Paragraph,
    Text(TextSpan),
    Strong(InlineSpan),
    Emphasis(InlineSpan),
    Strikethrough(InlineSpan),
    InlineCode(InlineCodeNode),
    InlineHtml(InlineHtmlNode),
    Link(LinkNode),
    Image(ImageNode),
    FootnoteReference(FootnoteReferenceNode),
    FootnoteDefinition(FootnoteDefinitionNode),
    InlineMath(InlineMathNode),
    DollarMathBlock(DollarMathBlockNode),
    Emoji(EmojiNode),
    HtmlBlock(HtmlBlockRole),
    List(ListNode),
    CodeBlock(CodeBlockRole),
    Table(TableNode),
    BlockQuote,
    Alert { label: String },
    DescriptionList { items: Vec<DescriptionItem> },
    ThematicBreak,
    RawBlock { reason: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeadingNode {
    pub level: u8,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmojiNode {
    pub value: String,
    pub shortcode: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextSpan {
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InlineSpan {
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InlineCodeNode {
    pub code: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InlineHtmlNode {
    pub html: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinkNode {
    pub label: String,
    pub destination: String,
    pub title: Option<String>,
    pub autolink: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageNode {
    pub alt: String,
    pub src: String,
    pub title: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FootnoteReferenceNode {
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FootnoteDefinitionNode {
    pub label: String,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InlineMathNode {
    pub expression: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DollarMathBlockNode {
    pub expression: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HtmlBlockRole {
    Generic,
    Centered,
    BadgeRow,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ListNode {
    pub ordered: bool,
    pub task_markers: Vec<String>,
    pub items: Vec<ListItemNode>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ListItemNode {
    pub marker: String,
    pub ordered_number: Option<usize>,
    pub task_marker: Option<String>,
    pub body: Vec<KmmNode>,
    pub children: Vec<KmmNode>,
    pub source: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CodeBlockRole {
    Plain { language: Option<String> },
    Diagram { kind: DiagramKind },
    Math,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagramKind {
    Mermaid,
    DrawIo,
    PlantUml,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TableNode {
    pub alignments: Vec<TableAlignment>,
    pub rows: Vec<TableRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TableAlignment {
    Left,
    Center,
    Right,
    Unspecified,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TableRow {
    pub cells: Vec<TableCell>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TableCell {
    pub text: String,
    pub source: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DescriptionItem {
    pub term: String,
    pub description: String,
}
