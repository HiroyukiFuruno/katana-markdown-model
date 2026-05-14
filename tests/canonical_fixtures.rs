use katana_markdown_model::{
    CodeBlockRole, DescriptionItem, DiagramKind, HtmlBlockRole, KatanaMarkdownModel, KmmDocument,
    KmmNode, KmmNodeKind, MarkdownInput, TableAlignment,
};
#[test]
fn parses_katana_sample_major_structures() {
    let document = canonical_document(
        "katana_sample.md",
        include_str!("fixtures/canonical/katana_sample.md"),
    );

    assert_source_contract(&document);
    assert_heading_exists(&document, "1. HTML Centering");
    assert_heading_exists(&document, "15. Consecutive Diagrams");
    assert!(has_node(&document, |kind| matches!(
        kind,
        KmmNodeKind::HtmlBlock(HtmlBlockRole::BadgeRow)
    )));
    assert!(has_node(&document, |kind| matches!(
        kind,
        KmmNodeKind::ThematicBreak
    )));
    assert!(has_node(&document, |kind| matches!(
        kind,
        KmmNodeKind::BlockQuote
    )));
    assert!(has_node(&document, |kind| matches!(
        kind,
        KmmNodeKind::List(_)
    )));
    assert!(
        nodes(&document)
            .iter()
            .filter(|node| matches!(node.kind, KmmNodeKind::Table(_)))
            .count()
            >= 6
    );
    assert_diagram_exists(&document, DiagramKind::Mermaid);
    assert_diagram_exists(&document, DiagramKind::PlantUml);
    assert_diagram_exists(&document, DiagramKind::DrawIo);
    assert!(has_node(&document, |kind| {
        matches!(kind, KmmNodeKind::CodeBlock(CodeBlockRole::Math))
    }));

    let alert_labels = alert_labels(&document);
    assert!(
        ["NOTE", "TIP", "IMPORTANT", "WARNING", "CAUTION"]
            .iter()
            .all(|label| alert_labels.contains(&label.to_string()))
    );
}

#[test]
fn parses_katana_basic_alerts_tables_and_footnote_text() {
    let document = canonical_document(
        "katana_sample_basic.md",
        include_str!("fixtures/canonical/katana_sample_basic.md"),
    );

    assert_source_contract(&document);
    assert_heading_exists(&document, "8. Alert Blocks");
    assert!(
        alert_labels(&document)
            .windows(5)
            .any(|labels| labels == ["NOTE", "TIP", "IMPORTANT", "WARNING", "CAUTION"])
    );
    assert!(nodes(&document).iter().any(|node| {
        matches!(
            &node.kind,
            KmmNodeKind::Paragraph if node.source.raw.text.contains("footnote[^1]")
        )
    }));

    let aligned_table = nodes(&document)
        .into_iter()
        .find_map(|node| match &node.kind {
            KmmNodeKind::Table(table)
                if table.alignments
                    == vec![
                        TableAlignment::Left,
                        TableAlignment::Center,
                        TableAlignment::Right,
                    ] =>
            {
                Some(table)
            }
            _ => None,
        });
    let table = aligned_table.expect("canonical aligned table must exist");
    assert_eq!(table.rows[3].cells[2].text, "12345");
    assert_eq!(table.rows[3].cells[2].source.raw.text, " 12345 ");
}

#[test]
fn parses_readme_header_badge_row() {
    let document = canonical_document(
        "katana_readme.md",
        include_str!("fixtures/canonical/katana_readme.md"),
    );

    assert_source_contract(&document);
    let html_roles = nodes(&document)
        .iter()
        .filter_map(|node| match &node.kind {
            KmmNodeKind::HtmlBlock(role) => Some(role),
            _ => None,
        })
        .take(6)
        .collect::<Vec<&HtmlBlockRole>>();

    assert_eq!(
        html_roles,
        vec![
            &HtmlBlockRole::Centered,
            &HtmlBlockRole::Centered,
            &HtmlBlockRole::Centered,
            &HtmlBlockRole::Centered,
            &HtmlBlockRole::BadgeRow,
            &HtmlBlockRole::Centered,
        ]
    );
    assert!(nodes(&document).iter().any(|node| {
        matches!(node.kind, KmmNodeKind::HtmlBlock(HtmlBlockRole::BadgeRow))
            && node.source.raw.text.contains("homebrew-cask")
    }));
}

#[test]
fn parses_canonical_description_list_fixture() {
    let document = canonical_document(
        "description_list.md",
        include_str!("fixtures/canonical/description_list.md"),
    );

    assert_source_contract(&document);
    let lists = nodes(&document)
        .into_iter()
        .filter_map(|node| match &node.kind {
            KmmNodeKind::DescriptionList { items } => Some(items),
            _ => None,
        })
        .collect::<Vec<&Vec<DescriptionItem>>>();

    assert_eq!(lists.len(), 1);
    assert_eq!(lists[0][0].term, "KMM");
    assert_eq!(
        lists[0][0].description,
        "Markdown document model owned by KatanA ecosystem."
    );
}

fn canonical_document(path: &str, source: &str) -> KmmDocument {
    KatanaMarkdownModel::parse(MarkdownInput::from_content(path, source))
        .expect("canonical fixture must parse")
}

fn nodes(document: &KmmDocument) -> Vec<&KmmNode> {
    document
        .nodes
        .iter()
        .flat_map(|node| std::iter::once(node).chain(node.children.iter()))
        .collect()
}

fn assert_source_contract(document: &KmmDocument) {
    assert_eq!(document.fingerprint.algorithm, "sha256");
    for node in nodes(document) {
        assert!(!node.id.0.is_empty());
        assert!(node.source.byte_range.start < node.source.byte_range.end);
        assert!(node.source.line_column_range.start.line >= 1);
        assert!(!node.source.raw.text.is_empty());
        assert_eq!(node.source.raw.fingerprint().algorithm, "sha256");
        assert_eq!(node.source.raw.fingerprint().value.len(), 64);
    }
}

fn assert_heading_exists(document: &KmmDocument, expected: &str) {
    assert!(nodes(document).iter().any(|node| {
        matches!(&node.kind, KmmNodeKind::Heading(heading) if heading.text.contains(expected))
    }));
}

fn has_node(document: &KmmDocument, predicate: impl Fn(&KmmNodeKind) -> bool) -> bool {
    nodes(document).iter().any(|node| predicate(&node.kind))
}

fn assert_diagram_exists(document: &KmmDocument, expected: DiagramKind) {
    assert!(nodes(document).iter().any(|node| {
        matches!(&node.kind, KmmNodeKind::CodeBlock(CodeBlockRole::Diagram { kind }) if *kind == expected)
    }));
}

fn alert_labels(document: &KmmDocument) -> Vec<String> {
    nodes(document)
        .iter()
        .filter_map(|node| match &node.kind {
            KmmNodeKind::Alert { label } => Some(label.clone()),
            _ => None,
        })
        .collect()
}
