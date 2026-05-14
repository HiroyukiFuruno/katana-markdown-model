use katana_markdown_model::{
    CodeBlockRole, DiagramKind, HtmlBlockRole, KatanaMarkdownModel, KmmNode, KmmNodeKind,
    MarkdownInput,
};

#[test]
fn parses_sample_fixture_into_renderer_neutral_nodes() {
    let document = KatanaMarkdownModel::parse(MarkdownInput::from_content(
        "tests/fixtures/sample.md",
        include_str!("fixtures/sample.md"),
    ))
    .unwrap();

    assert!(document.nodes.iter().all(|node| !node.id.0.is_empty()));
    assert!(
        document
            .nodes
            .iter()
            .all(|node| !node.source.raw.text.is_empty())
    );
    assert!(
        document
            .nodes_by_kind(|kind| matches!(kind, KmmNodeKind::Heading(_)))
            .len()
            >= 3
    );
    assert!(
        document
            .nodes_by_kind(|kind| matches!(kind, KmmNodeKind::Table(_)))
            .len()
            == 1
    );
    assert!(
        document
            .nodes
            .iter()
            .any(|node| { matches!(node.kind, KmmNodeKind::HtmlBlock(HtmlBlockRole::BadgeRow)) })
    );
    assert!(document.nodes.iter().any(|node| {
        matches!(
            &node.kind,
            KmmNodeKind::CodeBlock(CodeBlockRole::Diagram {
                kind: DiagramKind::Mermaid
            })
        )
    }));
}

#[test]
fn parses_alert_blocks_without_erasing_labels() {
    let document = KatanaMarkdownModel::parse(MarkdownInput::from_content(
        "tests/fixtures/alerts.md",
        include_str!("fixtures/alerts.md"),
    ))
    .unwrap();

    let labels: Vec<String> = document
        .nodes_by_kind(|kind| matches!(kind, KmmNodeKind::Alert { .. }))
        .iter()
        .filter_map(|node| match &node.kind {
            KmmNodeKind::Alert { label } => Some(label.clone()),
            _ => None,
        })
        .collect();

    assert_eq!(labels, vec!["NOTE", "WARNING"]);
}

#[test]
fn parses_description_list_as_owned_node() {
    let document = KatanaMarkdownModel::parse(MarkdownInput::from_content(
        "tests/fixtures/description_list.md",
        include_str!("fixtures/description_list.md"),
    ))
    .unwrap();

    let lists = document.nodes_by_kind(|kind| matches!(kind, KmmNodeKind::DescriptionList { .. }));
    assert_eq!(lists.len(), 1);
    assert!(matches!(
        &lists[0].kind,
        KmmNodeKind::DescriptionList { items } if items.len() == 2
    ));
}

#[test]
fn keeps_table_cell_source_ranges() {
    let document = KatanaMarkdownModel::parse(MarkdownInput::from_content(
        "inline-table.md",
        "| Name | Value |\n| :--- | ---: |\n| Alpha | 123 |\n",
    ))
    .unwrap();
    let table = document
        .nodes_by_kind(|kind| matches!(kind, KmmNodeKind::Table(_)))
        .remove(0);

    let KmmNodeKind::Table(table) = &table.kind else {
        panic!("expected table node");
    };
    let value_cell = &table.rows[2].cells[1];

    assert_eq!(value_cell.text, "123");
    assert_eq!(value_cell.source.raw.text, " 123 ");
    assert_eq!(value_cell.source.line_column_range.start.line, 3);
    assert_eq!(value_cell.source.line_column_range.start.column, 10);
}

#[test]
fn keeps_shortcode_and_unicode_emoji_as_child_nodes() {
    let document = KatanaMarkdownModel::parse(MarkdownInput::from_content(
        "emoji.md",
        "# Title :sparkles:\n\nHello 🚀\n",
    ))
    .unwrap();
    let emoji_nodes = document
        .nodes
        .iter()
        .flat_map(|node| node.children.iter())
        .filter(|node| matches!(node.kind, KmmNodeKind::Emoji(_)))
        .collect::<Vec<&KmmNode>>();

    assert_eq!(emoji_nodes.len(), 2);
    assert!(matches!(
        &emoji_nodes[0].kind,
        KmmNodeKind::Emoji(emoji) if emoji.value == ":sparkles:" && emoji.shortcode.as_deref() == Some("sparkles")
    ));
    assert!(matches!(
        &emoji_nodes[1].kind,
        KmmNodeKind::Emoji(emoji) if emoji.value == "🚀" && emoji.shortcode.is_none()
    ));
}
