use crate::model::{HarnessDocument, HarnessError, HarnessNode};
use std::path::Path;

pub struct HtmlRenderer;

impl HtmlRenderer {
    pub fn write(path: &Path, document: &HarnessDocument) -> Result<(), HarnessError> {
        std::fs::write(path, render(document))
            .map_err(|source| HarnessError::WriteHtml(path.to_path_buf(), source))
    }
}

fn render(document: &HarnessDocument) -> String {
    format!(
        r#"<!doctype html>
<html lang="ja">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>KMM Manual Harness</title>
<style>{}</style>
</head>
<body>
<header>
  <h1>KMM Manual Harness</h1>
  <p>{}</p>
</header>
<main>
  <section class="pane source"><h2>Markdown</h2><pre>{}</pre></section>
  <section class="pane nodes"><h2>KMM Nodes</h2>{}</section>
  <section class="pane detail"><h2>Selected Node</h2><div id="node-detail"></div></section>
  <section class="pane metadata"><h2>Metadata</h2>{}</section>
</main>
<script>{}</script>
</body>
</html>
"#,
        style(),
        escape(&document.path),
        escape(&document.source),
        render_nodes(&document.nodes),
        render_metadata(&document.metadata),
        script(&document.nodes)
    )
}

fn render_nodes(nodes: &[HarnessNode]) -> String {
    nodes
        .iter()
        .enumerate()
        .map(|(index, node)| {
            format!(
                r#"<button class="node" data-index="{index}">
<span>#{index:03}</span><strong>{}</strong><small>{}:{}..{}:{}</small>
</button>"#,
                escape(&node.kind),
                node.start_line,
                node.start_column,
                node.end_line,
                node.end_column
            )
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn render_metadata(items: &[String]) -> String {
    items
        .iter()
        .map(|item| format!("<div class=\"metadata-item\">{}</div>", escape(item)))
        .collect::<Vec<String>>()
        .join("\n")
}

fn script(nodes: &[HarnessNode]) -> String {
    let json = serde_json::to_string(nodes).expect("harness nodes must serialize");
    format!(
        r##"
const nodes = {json};
const detail = document.querySelector("#node-detail");
const buttons = Array.from(document.querySelectorAll(".node"));
function selectNode(index) {{
  const node = nodes[index];
  buttons.forEach((button) => button.classList.toggle("selected", button.dataset.index == index));
  detail.innerHTML = `
    <dl>
      <dt>id</dt><dd>${{node.id}}</dd>
      <dt>kind</dt><dd>${{node.kind}}</dd>
      <dt>source range</dt><dd>${{node.byte_start}}..${{node.byte_end}}</dd>
      <dt>line-column</dt><dd>${{node.start_line}}:${{node.start_column}}..${{node.end_line}}:${{node.end_column}}</dd>
      <dt>fingerprint</dt><dd>${{node.fingerprint}}</dd>
    </dl>
    <h3>raw snippet</h3>
    <pre>${{escapeHtml(node.raw)}}</pre>
  `;
}}
function escapeHtml(value) {{
  return value.replace(/[&<>"']/g, (char) => ({{
    "&": "&amp;", "<": "&lt;", ">": "&gt;", "\"": "&quot;", "'": "&#39;"
  }}[char]));
}}
buttons.forEach((button) => button.addEventListener("click", () => selectNode(button.dataset.index)));
selectNode(0);
"##
    )
}

fn style() -> &'static str {
    r#"
body { margin: 0; font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif; color: #17202a; background: #f5f7fa; }
header { padding: 16px 20px; border-bottom: 1px solid #d9e1ea; background: #ffffff; }
h1 { margin: 0 0 4px; font-size: 20px; letter-spacing: 0; }
h2 { margin: 0 0 12px; font-size: 15px; letter-spacing: 0; }
h3 { margin: 14px 0 8px; font-size: 13px; letter-spacing: 0; }
p { margin: 0; font-size: 13px; color: #5f6f7d; }
main { display: grid; grid-template-columns: minmax(280px, 1.2fr) minmax(260px, .8fr) minmax(320px, 1fr); grid-template-rows: 1fr 190px; gap: 1px; min-height: calc(100vh - 74px); background: #d9e1ea; }
.pane { min-width: 0; overflow: auto; padding: 14px; background: #ffffff; }
.source { grid-row: 1 / 3; }
.metadata { grid-column: 2 / 4; }
pre { margin: 0; white-space: pre-wrap; word-break: break-word; font: 12px/1.55 ui-monospace, SFMono-Regular, Menlo, monospace; }
.node { width: 100%; display: grid; grid-template-columns: 52px 1fr; gap: 4px 8px; padding: 8px; border: 1px solid #d8e0e8; background: #fbfcfe; text-align: left; border-radius: 6px; margin-bottom: 6px; cursor: pointer; }
.node:hover, .node.selected { border-color: #2563eb; background: #eef5ff; }
.node span, .node small { color: #617384; font-size: 12px; }
.node strong { font-size: 13px; font-weight: 650; }
dl { display: grid; grid-template-columns: 110px minmax(0, 1fr); gap: 8px 10px; margin: 0; font-size: 13px; }
dt { color: #617384; }
dd { margin: 0; overflow-wrap: anywhere; }
.metadata-item { font: 13px/1.5 ui-monospace, SFMono-Regular, Menlo, monospace; padding: 6px 8px; border: 1px solid #d8e0e8; border-radius: 6px; margin-bottom: 6px; background: #fbfcfe; }
@media (max-width: 980px) { main { display: block; } .pane { max-height: 70vh; border-bottom: 1px solid #d9e1ea; } }
"#
}

fn escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
