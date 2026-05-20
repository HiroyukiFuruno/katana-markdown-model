use super::engine::ParserCursor;
use crate::{DollarMathBlockNode, KmmNodeKind};

const DOLLAR_MATH_DELIMITER: &str = "$$";

impl ParserCursor<'_> {
    pub(super) fn dollar_math_block(&mut self) -> KmmNodeKind {
        if let Some(expression) = one_line_dollar_math(self.current().text.trim()) {
            self.line += 1;
            return KmmNodeKind::DollarMathBlock(DollarMathBlockNode { expression });
        }

        self.line += 1;
        let mut expression = Vec::new();
        while self.line < self.index.lines().len() {
            let text = self.current().text.trim();
            if text == DOLLAR_MATH_DELIMITER {
                self.line += 1;
                break;
            }
            expression.push(self.current().text.clone());
            self.line += 1;
        }
        KmmNodeKind::DollarMathBlock(DollarMathBlockNode {
            expression: expression.join("\n"),
        })
    }

    pub(super) fn is_dollar_math_block(&self) -> bool {
        let trimmed = self.current().text.trim();
        trimmed == DOLLAR_MATH_DELIMITER || one_line_dollar_math(trimmed).is_some()
    }
}

fn one_line_dollar_math(trimmed: &str) -> Option<String> {
    if !trimmed.starts_with(DOLLAR_MATH_DELIMITER) || !trimmed.ends_with(DOLLAR_MATH_DELIMITER) {
        return None;
    }
    let inner_start = DOLLAR_MATH_DELIMITER.len();
    let inner_end = trimmed.len().checked_sub(DOLLAR_MATH_DELIMITER.len())?;
    if inner_start >= inner_end {
        return None;
    }
    let expression = trimmed[inner_start..inner_end].trim().to_string();
    (!expression.is_empty()).then_some(expression)
}
