use katana_markdown_model::{
    ContextAnchor, KatanaMarkdownModel, KmmNodeKind, MarkdownInput, MetadataDocument,
    MetadataEntry, MetadataReconcileRequest, MetadataTarget, TargetResolutionKind,
};
use serde_json::json;
use std::path::PathBuf;

#[test]
fn resolves_metadata_target_by_stable_node_id() {
    let document = sample_document("# Title\n\nBody text\n");
    let heading = document
        .nodes_by_kind(|kind| matches!(kind, KmmNodeKind::Heading(_)))
        .remove(0);
    let metadata = metadata_for_node("title-note", &document.path, heading);

    let resolutions = KatanaMarkdownModel::reconcile_targets(&document, &document, &metadata);

    assert!(matches!(
        &resolutions[0].kind,
        TargetResolutionKind::Resolved { node_id } if node_id == &heading.id
    ));
}

#[test]
fn unresolved_metadata_is_returned_without_deletion() {
    let old_document = sample_document("# Title\n\nBody text\n");
    let new_document = sample_document("# Other\n\nBody text\n");
    let heading = old_document
        .nodes_by_kind(|kind| matches!(kind, KmmNodeKind::Heading(_)))
        .remove(0);
    let metadata = metadata_for_node("title-note", &old_document.path, heading);

    let resolutions =
        KatanaMarkdownModel::reconcile_targets(&old_document, &new_document, &metadata);

    assert!(matches!(
        &resolutions[0].kind,
        TargetResolutionKind::Unresolved(unresolved) if unresolved.node_id == heading.id
    ));
}

#[test]
fn resolves_metadata_target_by_fingerprint_when_node_id_changes() {
    let old_document = sample_document("# Title\n\nBody text\n");
    let new_document = sample_document("# Title\n\nBody text\n");
    let heading = old_document
        .nodes_by_kind(|kind| matches!(kind, KmmNodeKind::Heading(_)))
        .remove(0);
    let mut metadata = metadata_for_node("title-note", &old_document.path, heading);
    metadata.entries[0].target.node_id.0 = "kmm-old-id".to_string();

    let resolutions =
        KatanaMarkdownModel::reconcile_targets(&old_document, &new_document, &metadata);

    assert!(matches!(
        &resolutions[0].kind,
        TargetResolutionKind::Moved { previous_node_id, .. } if previous_node_id.0 == "kmm-old-id"
    ));
}

#[test]
fn returns_conflict_when_fingerprint_matches_multiple_new_nodes() {
    let old_document = sample_document("# Title\n\nBody text\n");
    let new_document = sample_document("# Title\n\n# Title\n");
    let heading = old_document
        .nodes_by_kind(|kind| matches!(kind, KmmNodeKind::Heading(_)))
        .remove(0);
    let mut metadata = metadata_for_node("title-note", &old_document.path, heading);
    metadata.entries[0].target.node_id.0 = "kmm-old-id".to_string();

    let resolutions =
        KatanaMarkdownModel::reconcile_targets(&old_document, &new_document, &metadata);

    assert!(matches!(
        &resolutions[0].kind,
        TargetResolutionKind::Conflict(conflict)
            if conflict.previous_node_id.0 == "kmm-old-id" && conflict.candidate_node_ids.len() == 2
    ));
}

#[test]
fn reconciles_save_time_metadata_request_without_deleting_entries() {
    let old_document = sample_document("# Title\n\nBody text\n");
    let new_document = sample_document("# Other\n\nBody text\n");
    let heading = old_document
        .nodes_by_kind(|kind| matches!(kind, KmmNodeKind::Heading(_)))
        .remove(0);
    let heading_id = heading.id.clone();
    let metadata = metadata_for_node("title-note", &old_document.path, heading);

    let result = KatanaMarkdownModel::reconcile(MetadataReconcileRequest {
        old_document,
        new_document,
        metadata,
    });

    assert_eq!(result.metadata.entries.len(), 1);
    assert_eq!(result.resolutions.len(), 1);
    assert!(matches!(
        &result.resolutions[0].kind,
        TargetResolutionKind::Unresolved(unresolved) if unresolved.node_id == heading_id
    ));
}

#[test]
fn metadata_use_fixture_covers_v0_use_cases() {
    let metadata: MetadataDocument =
        serde_json::from_str(include_str!("fixtures/metadata_uses.json")).unwrap();
    let payload_kinds = metadata
        .entries
        .iter()
        .map(|entry| entry.payload["kind"].as_str().unwrap())
        .collect::<Vec<&str>>();

    assert_eq!(
        payload_kinds,
        vec!["pdf-page", "llm-annotation", "ast-edit"]
    );
}

fn sample_document(content: &str) -> katana_markdown_model::KmmDocument {
    KatanaMarkdownModel::parse(MarkdownInput::from_content("README.md", content)).unwrap()
}

fn metadata_for_node(
    key: &str,
    path: &std::path::Path,
    node: &katana_markdown_model::KmmNode,
) -> MetadataDocument {
    MetadataDocument {
        markdown_path: path.to_path_buf(),
        entries: vec![MetadataEntry {
            key: key.to_string(),
            target: MetadataTarget {
                file_path: PathBuf::from(path),
                node_id: node.id.clone(),
                byte_range: node.source.byte_range,
                line_column_range: node.source.line_column_range,
                text_fingerprint: node.source.raw.fingerprint(),
                context: ContextAnchor {
                    before: String::new(),
                    after: String::new(),
                },
            },
            payload: json!({ "kind": "note" }),
        }],
    }
}
