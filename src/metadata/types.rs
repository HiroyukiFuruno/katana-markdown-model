use crate::{ByteRange, KmmDocument, KmmNodeId, LineColumnRange, TextFingerprint};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetadataDocument {
    pub markdown_path: PathBuf,
    pub entries: Vec<MetadataEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetadataEntry {
    pub key: String,
    pub target: MetadataTarget,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetadataTarget {
    pub file_path: PathBuf,
    pub node_id: KmmNodeId,
    pub byte_range: ByteRange,
    pub line_column_range: LineColumnRange,
    pub text_fingerprint: TextFingerprint,
    pub context: ContextAnchor,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextAnchor {
    pub before: String,
    pub after: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetResolution {
    pub key: String,
    pub kind: TargetResolutionKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetadataReconcileRequest {
    pub old_document: KmmDocument,
    pub new_document: KmmDocument,
    pub metadata: MetadataDocument,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetadataReconcileResult {
    pub metadata: MetadataDocument,
    pub resolutions: Vec<TargetResolution>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TargetResolutionKind {
    Resolved {
        node_id: KmmNodeId,
    },
    Moved {
        previous_node_id: KmmNodeId,
        node_id: KmmNodeId,
    },
    Conflict(ConflictedTarget),
    Unresolved(UnresolvedTarget),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConflictedTarget {
    pub previous_node_id: KmmNodeId,
    pub candidate_node_ids: Vec<KmmNodeId>,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnresolvedTarget {
    pub node_id: KmmNodeId,
    pub reason: String,
}
