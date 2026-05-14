use super::types::{
    ConflictedTarget, MetadataDocument, MetadataEntry, MetadataReconcileRequest,
    MetadataReconcileResult, TargetResolution, TargetResolutionKind, UnresolvedTarget,
};
use crate::{KmmDocument, KmmNode, TextFingerprint};

pub struct MetadataResolver;

impl MetadataResolver {
    pub fn new() -> Self {
        Self
    }

    pub fn reconcile(
        &self,
        old_document: &KmmDocument,
        new_document: &KmmDocument,
        metadata: &MetadataDocument,
    ) -> Vec<TargetResolution> {
        metadata
            .entries
            .iter()
            .map(|entry| self.resolve_entry(old_document, new_document, entry))
            .collect()
    }

    pub fn reconcile_request(&self, request: MetadataReconcileRequest) -> MetadataReconcileResult {
        let resolutions = self.reconcile(
            &request.old_document,
            &request.new_document,
            &request.metadata,
        );
        MetadataReconcileResult {
            metadata: request.metadata,
            resolutions,
        }
    }

    fn resolve_entry(
        &self,
        old_document: &KmmDocument,
        new_document: &KmmDocument,
        entry: &MetadataEntry,
    ) -> TargetResolution {
        if let Some(node) = new_document.node_by_id(&entry.target.node_id) {
            return Self::resolved(entry, node);
        }
        let candidates = self.find_by_fingerprint(new_document, &entry.target.text_fingerprint);
        match candidates.as_slice() {
            [node] => return Self::moved(entry, node),
            [_, _, ..] => return Self::conflict(entry, &candidates),
            [] => {}
        }
        Self::unresolved(old_document, entry)
    }

    fn find_by_fingerprint<'a>(
        &self,
        document: &'a KmmDocument,
        fingerprint: &TextFingerprint,
    ) -> Vec<&'a KmmNode> {
        document
            .nodes
            .iter()
            .filter(|node| &node.source.raw.fingerprint() == fingerprint)
            .collect()
    }

    fn resolved(entry: &MetadataEntry, node: &KmmNode) -> TargetResolution {
        TargetResolution {
            key: entry.key.clone(),
            kind: TargetResolutionKind::Resolved {
                node_id: node.id.clone(),
            },
        }
    }

    fn moved(entry: &MetadataEntry, node: &KmmNode) -> TargetResolution {
        TargetResolution {
            key: entry.key.clone(),
            kind: TargetResolutionKind::Moved {
                previous_node_id: entry.target.node_id.clone(),
                node_id: node.id.clone(),
            },
        }
    }

    fn conflict(entry: &MetadataEntry, nodes: &[&KmmNode]) -> TargetResolution {
        TargetResolution {
            key: entry.key.clone(),
            kind: TargetResolutionKind::Conflict(ConflictedTarget {
                previous_node_id: entry.target.node_id.clone(),
                candidate_node_ids: nodes.iter().map(|node| node.id.clone()).collect(),
                reason: "multiple fingerprint matches".to_string(),
            }),
        }
    }

    fn unresolved(old_document: &KmmDocument, entry: &MetadataEntry) -> TargetResolution {
        let old_exists = old_document.node_by_id(&entry.target.node_id).is_some();
        let reason = if old_exists {
            "target disappeared"
        } else {
            "target unknown"
        };
        TargetResolution {
            key: entry.key.clone(),
            kind: TargetResolutionKind::Unresolved(UnresolvedTarget {
                node_id: entry.target.node_id.clone(),
                reason: reason.to_string(),
            }),
        }
    }
}

impl Default for MetadataResolver {
    fn default() -> Self {
        Self::new()
    }
}
