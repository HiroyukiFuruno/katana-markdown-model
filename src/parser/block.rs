use crate::{CodeBlockRole, DescriptionItem, DiagramKind, HeadingNode, HtmlBlockRole, KmmNodeKind};

const MAX_HEADING_LEVEL: usize = 6;

pub(crate) struct BlockParser;

impl BlockParser {
    pub(crate) fn heading(line: &str) -> Option<KmmNodeKind> {
        let hashes = line.chars().take_while(|it| *it == '#').count();
        if !(1..=MAX_HEADING_LEVEL).contains(&hashes) || !line[hashes..].starts_with(' ') {
            return None;
        }
        Some(KmmNodeKind::Heading(HeadingNode {
            level: hashes as u8,
            text: line[hashes + 1..].trim().to_string(),
        }))
    }

    pub(crate) fn code_block_role(fence: &str) -> CodeBlockRole {
        let language = fence.trim_start_matches("```").trim();
        match language.to_ascii_lowercase().as_str() {
            "mermaid" => CodeBlockRole::Diagram {
                kind: DiagramKind::Mermaid,
            },
            "drawio" | "draw.io" => CodeBlockRole::Diagram {
                kind: DiagramKind::DrawIo,
            },
            "plantuml" | "puml" => CodeBlockRole::Diagram {
                kind: DiagramKind::PlantUml,
            },
            "math" => CodeBlockRole::Math,
            "" => CodeBlockRole::Plain { language: None },
            value => CodeBlockRole::Plain {
                language: Some(value.to_string()),
            },
        }
    }

    pub(crate) fn html_role(raw: &str) -> HtmlBlockRole {
        if raw.contains("<img") && raw.contains("<a ") {
            return HtmlBlockRole::BadgeRow;
        }
        if raw.contains("align=\"center\"") || raw.contains("<center") {
            return HtmlBlockRole::Centered;
        }
        HtmlBlockRole::Generic
    }

    pub(crate) fn alert_label(lines: &[String]) -> Option<String> {
        let first = lines.first()?.trim_start_matches('>').trim();
        if let Some(label) = first.strip_prefix("[!").and_then(|it| it.strip_suffix(']')) {
            return Some(label.to_ascii_uppercase());
        }
        let legacy = first.trim_matches('*').to_ascii_uppercase();
        matches!(
            legacy.as_str(),
            "NOTE" | "TIP" | "IMPORTANT" | "WARNING" | "CAUTION"
        )
        .then_some(legacy)
    }

    pub(crate) fn description_items(lines: &[String]) -> Vec<DescriptionItem> {
        lines
            .chunks(2)
            .filter_map(|chunk| {
                let term = chunk.first()?.trim().to_string();
                let description = chunk
                    .get(1)?
                    .trim()
                    .trim_start_matches(':')
                    .trim()
                    .to_string();
                Some(DescriptionItem { term, description })
            })
            .collect()
    }

    pub(crate) fn unordered_list_line(line: &str) -> bool {
        line.trim_start().starts_with("- ")
    }

    pub(crate) fn ordered_list_line(line: &str) -> bool {
        let trimmed = line.trim_start();
        let Some((number, rest)) = trimmed.split_once('.') else {
            return false;
        };
        !number.is_empty() && number.chars().all(|it| it.is_ascii_digit()) && rest.starts_with(' ')
    }
}
