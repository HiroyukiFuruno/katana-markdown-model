use katana_markdown_model::{
    ContextAnchor, KatanaMarkdownModel, KmmDocument, KmmNode, KmmNodeId, MarkdownInput,
    MetadataDocument, MetadataEntry, MetadataReconcileRequest, MetadataTarget,
};
use serde_json::json;
use std::path::{Path, PathBuf};

const OLD_METADATA_SOURCE: &str = "# Title\n\nSame\n\nGone\n";
const NEW_METADATA_SOURCE: &str = "# Title\n\nSame\n\nSame\n";

pub struct MetadataDemo;

impl MetadataDemo {
    pub fn request() -> MetadataReconcileRequest {
        let old_document = KatanaMarkdownModel::parse(MarkdownInput::from_content(
            "metadata-harness.md",
            OLD_METADATA_SOURCE,
        ))
        .expect("old metadata fixture must parse");
        let new_document = KatanaMarkdownModel::parse(MarkdownInput::from_content(
            "metadata-harness.md",
            NEW_METADATA_SOURCE,
        ))
        .expect("new metadata fixture must parse");
        let markdown_path = PathBuf::from("metadata-harness.md");

        MetadataReconcileRequest {
            metadata: MetadataDocument {
                markdown_path: markdown_path.clone(),
                entries: metadata_entries(&old_document, &markdown_path),
            },
            old_document,
            new_document,
        }
    }
}

fn metadata_entries(document: &KmmDocument, path: &Path) -> Vec<MetadataEntry> {
    let title = node_containing(document, "# Title");
    let same = node_containing(document, "Same");
    let gone = node_containing(document, "Gone");

    vec![
        metadata_entry("resolved-title", path, title, title.id.clone()),
        metadata_entry(
            "moved-title",
            path,
            title,
            KmmNodeId("old-title".to_string()),
        ),
        metadata_entry("unresolved-gone", path, gone, gone.id.clone()),
        metadata_entry(
            "conflict-same",
            path,
            same,
            KmmNodeId("old-same".to_string()),
        ),
    ]
}

fn node_containing<'a>(document: &'a KmmDocument, expected: &str) -> &'a KmmNode {
    document
        .nodes
        .iter()
        .find(|node| node.source.raw.text.contains(expected))
        .expect("manual harness metadata node must exist")
}

fn metadata_entry(key: &str, path: &Path, node: &KmmNode, node_id: KmmNodeId) -> MetadataEntry {
    MetadataEntry {
        key: key.to_string(),
        target: MetadataTarget {
            file_path: path.to_path_buf(),
            node_id,
            byte_range: node.source.byte_range,
            line_column_range: node.source.line_column_range,
            text_fingerprint: node.source.raw.fingerprint(),
            context: ContextAnchor {
                before: String::new(),
                after: String::new(),
            },
        },
        payload: json!({ "kind": "manual-harness" }),
    }
}
