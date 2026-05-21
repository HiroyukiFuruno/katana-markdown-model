use katana_markdown_model::{
    DollarMathBlockNode, InlineSpan, KatanaMarkdownModel, KmmDocument, KmmNode, KmmNodeKind,
    MarkdownInput,
};

#[test]
fn copied_katana_sample_ja_exposes_kdv_export_ast_regressions() {
    let document = parse(
        "tests/fixtures/canonical/katana_sample_ja.md",
        include_str!("fixtures/canonical/katana_sample_ja.md"),
    );

    assert_source_contract(&document);
    assert!(has_node(&document, |kind| {
        matches!(
            kind,
            KmmNodeKind::DollarMathBlock(DollarMathBlockNode { expression })
                if expression == r"\sum_{k=1}^{n} k = \frac{n(n+1)}{2}"
        )
    }));
    assert!(has_node(&document, |kind| {
        matches!(
            kind,
            KmmNodeKind::Strong(InlineSpan { text }) if text == "太字と*イタリック*の混在"
        )
    }));
    let nodes = document_nodes(&document);
    let nested_strong = nodes
        .iter()
        .copied()
        .find(|node| {
            matches!(
                &node.kind,
                KmmNodeKind::Strong(InlineSpan { text }) if text == "太字と*イタリック*の混在"
            )
        })
        .expect("copied sample must expose nested strong span");
    assert!(has_text_child(nested_strong, "太字と"));
    assert!(nested_strong.children.iter().any(|child| {
        matches!(
            &child.kind,
            KmmNodeKind::Emphasis(InlineSpan { text }) if text == "イタリック"
        )
    }));
    assert!(has_text_child(nested_strong, "の混在"));
    assert!(list_item_body_contains(&document, "リスト項目 1"));
    assert!(list_item_body_contains(&document, "ネストされた項目 2-1"));
}

#[test]
fn github_alert_with_japanese_list_item_does_not_panic() {
    let document = parse(
        "alert-japanese-list.md",
        r#"> [!NOTE]
> **重要** な補足
>
> - 箇条書き
"#,
    );

    assert!(has_node(&document, |kind| {
        matches!(kind, KmmNodeKind::Alert { label } if label == "NOTE")
    }));
    assert!(list_item_body_contains(&document, "箇条書き"));
}

fn parse(path: &str, source: &str) -> KmmDocument {
    KatanaMarkdownModel::parse(MarkdownInput::from_content(path, source))
        .expect("fixture must parse")
}

fn assert_source_contract(document: &KmmDocument) {
    for node in document_nodes(document) {
        assert!(node.source.byte_range.start < node.source.byte_range.end);
        assert!(!node.source.raw.text.is_empty());
        assert!(node.source.raw.text.is_char_boundary(0));
        assert!(
            node.source
                .raw
                .text
                .is_char_boundary(node.source.raw.text.len())
        );
    }
}

fn has_node(document: &KmmDocument, predicate: impl Fn(&KmmNodeKind) -> bool) -> bool {
    document_nodes(document)
        .iter()
        .any(|node| predicate(&node.kind))
}

fn list_item_body_contains(document: &KmmDocument, expected: &str) -> bool {
    document_nodes(document).iter().any(|node| {
        let KmmNodeKind::List(list) = &node.kind else {
            return false;
        };
        list.items.iter().any(|item| {
            item.body.iter().any(|body| {
                matches!(&body.kind, KmmNodeKind::Text(text) if text.text.contains(expected))
            })
        })
    })
}

fn has_text_child(node: &KmmNode, expected: &str) -> bool {
    node.children.iter().any(|child| {
        matches!(
            &child.kind,
            KmmNodeKind::Text(text) if text.text == expected
        )
    })
}

fn document_nodes(document: &KmmDocument) -> Vec<&KmmNode> {
    let mut nodes = Vec::new();
    for node in &document.nodes {
        collect_node(node, &mut nodes);
    }
    nodes
}

fn collect_node<'a>(node: &'a KmmNode, nodes: &mut Vec<&'a KmmNode>) {
    nodes.push(node);
    for child in &node.children {
        collect_node(child, nodes);
    }
    if let KmmNodeKind::List(list) = &node.kind {
        for item in &list.items {
            for body in &item.body {
                collect_node(body, nodes);
            }
            for child in &item.children {
                collect_node(child, nodes);
            }
        }
    }
}
