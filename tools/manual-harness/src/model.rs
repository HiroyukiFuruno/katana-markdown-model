use katana_markdown_model::{KmmDocument, KmmError, TargetResolutionKind};
use serde::Serialize;
use std::path::PathBuf;

pub struct HarnessDocument {
    pub path: String,
    pub source: String,
    pub nodes: Vec<HarnessNode>,
    pub metadata: Vec<String>,
}

#[derive(Serialize)]
pub struct HarnessNode {
    pub id: String,
    pub kind: String,
    pub byte_start: usize,
    pub byte_end: usize,
    pub start_line: usize,
    pub start_column: usize,
    pub end_line: usize,
    pub end_column: usize,
    pub raw: String,
    pub fingerprint: String,
}

#[derive(Debug)]
pub enum HarnessError {
    CurrentDirectory(std::io::Error),
    CreateDirectory(PathBuf, std::io::Error),
    ReadFixture(PathBuf, std::io::Error),
    WriteHtml(PathBuf, std::io::Error),
    Parse(KmmError),
}

impl HarnessDocument {
    pub fn new(
        path: PathBuf,
        source: String,
        document: KmmDocument,
        metadata: katana_markdown_model::MetadataReconcileResult,
    ) -> Self {
        Self {
            path: path.display().to_string(),
            source,
            nodes: document.nodes.iter().map(HarnessNode::from).collect(),
            metadata: metadata
                .resolutions
                .iter()
                .map(|resolution| {
                    format!("{}: {}", resolution.key, resolution_label(&resolution.kind))
                })
                .collect(),
        }
    }
}

impl From<&katana_markdown_model::KmmNode> for HarnessNode {
    fn from(node: &katana_markdown_model::KmmNode) -> Self {
        Self {
            id: node.id.0.clone(),
            kind: node.kind.label().to_string(),
            byte_start: node.source.byte_range.start,
            byte_end: node.source.byte_range.end,
            start_line: node.source.line_column_range.start.line,
            start_column: node.source.line_column_range.start.column,
            end_line: node.source.line_column_range.end.line,
            end_column: node.source.line_column_range.end.column,
            raw: node.source.raw.text.clone(),
            fingerprint: node.source.raw.fingerprint().value,
        }
    }
}

impl std::fmt::Display for HarnessError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CurrentDirectory(error) => write!(formatter, "current directory error: {error}"),
            Self::CreateDirectory(path, error) => {
                write!(formatter, "failed to create {}: {error}", path.display())
            }
            Self::ReadFixture(path, error) => {
                write!(formatter, "failed to read {}: {error}", path.display())
            }
            Self::WriteHtml(path, error) => {
                write!(formatter, "failed to write {}: {error}", path.display())
            }
            Self::Parse(error) => write!(formatter, "parse error: {error}"),
        }
    }
}

impl std::error::Error for HarnessError {}

fn resolution_label(kind: &TargetResolutionKind) -> String {
    match kind {
        TargetResolutionKind::Resolved { node_id } => format!("Resolved -> {}", node_id.0),
        TargetResolutionKind::Moved {
            previous_node_id,
            node_id,
        } => format!("Moved {} -> {}", previous_node_id.0, node_id.0),
        TargetResolutionKind::Conflict(conflict) => {
            format!("Conflict candidates={}", conflict.candidate_node_ids.len())
        }
        TargetResolutionKind::Unresolved(unresolved) => {
            format!("Unresolved {}", unresolved.reason)
        }
    }
}
