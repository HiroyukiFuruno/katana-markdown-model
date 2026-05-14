#![deny(warnings, clippy::all)]
#![allow(
    missing_docs,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions
)]

pub mod document;
pub mod error;
pub mod input;
pub mod metadata;
pub mod source;

mod parser;

pub use document::{
    CodeBlockRole, DescriptionItem, DiagramKind, EmojiNode, HeadingNode, HtmlBlockRole,
    KmmDocument, KmmNode, KmmNodeId, KmmNodeKind, ListNode, TableAlignment, TableCell, TableNode,
    TableRow,
};
pub use error::KmmError;
pub use input::MarkdownInput;
pub use metadata::{
    ConflictedTarget, ContextAnchor, MetadataDocument, MetadataEntry, MetadataReconcileRequest,
    MetadataReconcileResult, MetadataResolver, MetadataTarget, TargetResolution,
    TargetResolutionKind, UnresolvedTarget,
};
pub use source::{ByteRange, LineColumn, LineColumnRange, RawSnippet, SourceSpan, TextFingerprint};

pub struct KatanaMarkdownModel;

impl KatanaMarkdownModel {
    pub fn parse(input: MarkdownInput) -> Result<KmmDocument, KmmError> {
        parser::MarkdownParser::new().parse(input)
    }

    pub fn reconcile_targets(
        old_document: &KmmDocument,
        new_document: &KmmDocument,
        metadata: &MetadataDocument,
    ) -> Vec<TargetResolution> {
        MetadataResolver::new().reconcile(old_document, new_document, metadata)
    }

    pub fn reconcile(request: MetadataReconcileRequest) -> MetadataReconcileResult {
        MetadataResolver::new().reconcile_request(request)
    }
}
