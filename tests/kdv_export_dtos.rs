use katana_markdown_model::{
    KatanaMarkdownModel, KmmDocument, KmmNode, KmmNodeKind, MarkdownInput,
};

#[test]
fn exposes_inline_markdown_as_renderer_neutral_dtos() {
    let document = parse(
        "kdv-inline.md",
        r#"**太字** *斜体* ~~取り消し線~~ `code` <span data-katana="1">raw</span>
[通常のリンク](https://github.com "GitHub") <https://example.com>
![代替テキスト](image.png "Image title") footnote[^1] $E = mc^2$ :sparkles:

[^1]: 脚注本文

$$
a^2 + b^2 = c^2
$$
"#,
    );

    let paragraph = first_node(&document, |kind| matches!(kind, KmmNodeKind::Paragraph));
    assert!(has_child(paragraph, |kind| {
        matches!(kind, KmmNodeKind::Strong(span) if span.text == "太字")
    }));
    assert!(has_child(paragraph, |kind| {
        matches!(kind, KmmNodeKind::Emphasis(span) if span.text == "斜体")
    }));
    assert!(has_child(paragraph, |kind| {
        matches!(kind, KmmNodeKind::Strikethrough(span) if span.text == "取り消し線")
    }));
    assert!(has_child(paragraph, |kind| {
        matches!(kind, KmmNodeKind::InlineCode(code) if code.code == "code")
    }));
    assert!(has_child(paragraph, |kind| {
        matches!(kind, KmmNodeKind::InlineHtml(html) if html.html.contains("data-katana"))
    }));
    assert!(has_child(paragraph, |kind| {
        matches!(kind, KmmNodeKind::Text(text) if text.text.contains(' '))
    }));

    let link_paragraph = document
        .nodes
        .iter()
        .find(|node| has_child(node, |kind| matches!(kind, KmmNodeKind::Link(_))))
        .expect("link paragraph must exist");
    assert!(has_child(link_paragraph, |kind| {
        matches!(
            kind,
            KmmNodeKind::Link(link)
                if link.label == "通常のリンク"
                    && link.destination == "https://github.com"
                    && link.title.as_deref() == Some("GitHub")
                    && !link.autolink
        )
    }));
    assert!(has_child(link_paragraph, |kind| {
        matches!(
            kind,
            KmmNodeKind::Link(link)
                if link.label == "https://example.com"
                    && link.destination == "https://example.com"
                    && link.title.is_none()
                    && link.autolink
        )
    }));

    let media_paragraph = document
        .nodes
        .iter()
        .find(|node| has_child(node, |kind| matches!(kind, KmmNodeKind::Image(_))))
        .expect("media paragraph must exist");
    assert!(has_child(media_paragraph, |kind| {
        matches!(
            kind,
            KmmNodeKind::Image(image)
                if image.alt == "代替テキスト"
                    && image.src == "image.png"
                    && image.title.as_deref() == Some("Image title")
        )
    }));
    assert!(has_child(media_paragraph, |kind| {
        matches!(kind, KmmNodeKind::FootnoteReference(reference) if reference.label == "1")
    }));
    assert!(has_child(media_paragraph, |kind| {
        matches!(kind, KmmNodeKind::InlineMath(math) if math.expression == "E = mc^2")
    }));

    assert!(document.nodes.iter().any(|node| {
        matches!(
            &node.kind,
            KmmNodeKind::FootnoteDefinition(definition) if definition.label == "1"
        )
    }));
    assert!(document.nodes.iter().any(|node| {
        matches!(
            &node.kind,
            KmmNodeKind::DollarMathBlock(block) if block.expression.trim() == "a^2 + b^2 = c^2"
        )
    }));
}

#[test]
fn exposes_list_item_bodies_and_nested_blocks() {
    let document = parse(
        "kdv-list.md",
        r#"- [x] 親 **太字**
  - 子 [リンク](https://example.com)

  ```rust
  fn main() {}
  ```
"#,
    );
    let list = first_node(&document, |kind| matches!(kind, KmmNodeKind::List(_)));
    let KmmNodeKind::List(list_node) = &list.kind else {
        panic!("expected list node");
    };

    assert_eq!(list_node.items.len(), 1);
    assert_eq!(list_node.items[0].marker, "-");
    assert_eq!(list_node.items[0].task_marker.as_deref(), Some("[x]"));
    assert!(
        list_node.items[0]
            .body
            .iter()
            .any(|node| matches!(&node.kind, KmmNodeKind::Strong(span) if span.text == "太字"))
    );
    assert!(
        list_node.items[0]
            .children
            .iter()
            .any(|node| matches!(&node.kind, KmmNodeKind::List(_)))
    );
    assert!(
        list_node.items[0]
            .children
            .iter()
            .any(|node| matches!(&node.kind, KmmNodeKind::CodeBlock(_)))
    );
}

#[test]
fn exposes_blockquote_children_without_downstream_reparse() {
    let document = parse(
        "kdv-blockquote.md",
        r#"> **重要**
> - item
>
> ```rust
> let value = 1;
> ```
"#,
    );
    let blockquote = first_node(&document, |kind| matches!(kind, KmmNodeKind::BlockQuote));

    assert!(
        blockquote
            .children
            .iter()
            .any(|node| matches!(&node.kind, KmmNodeKind::Paragraph))
    );
    assert!(
        blockquote
            .children
            .iter()
            .any(|node| matches!(&node.kind, KmmNodeKind::List(_)))
    );
    assert!(
        blockquote
            .children
            .iter()
            .any(|node| matches!(&node.kind, KmmNodeKind::CodeBlock(_)))
    );
    assert!(blockquote.children.iter().any(|node| {
        node.children
            .iter()
            .any(|child| matches!(&child.kind, KmmNodeKind::Strong(span) if span.text == "重要"))
    }));
}

fn parse(path: &str, source: &str) -> KmmDocument {
    KatanaMarkdownModel::parse(MarkdownInput::from_content(path, source)).unwrap()
}

fn first_node(document: &KmmDocument, predicate: impl Fn(&KmmNodeKind) -> bool) -> &KmmNode {
    document
        .nodes
        .iter()
        .find(|node| predicate(&node.kind))
        .expect("node must exist")
}

fn has_child(node: &KmmNode, predicate: impl Fn(&KmmNodeKind) -> bool) -> bool {
    node.children.iter().any(|child| predicate(&child.kind))
}
