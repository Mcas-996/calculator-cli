use crate::core::ComplexNumber;
use crate::tui::latex::TuiLatexRenderer;

pub(crate) fn format_complex_root(num: &ComplexNumber) -> String {
    num.to_string()
}

#[derive(Clone, Debug)]
pub struct ResultCard {
    pub expression: Option<String>,
    pub result_lines: Vec<String>,
    pub is_error: bool,
}

impl ResultCard {
    pub fn new(expression: Option<String>, result: Vec<String>) -> Self {
        Self {
            expression,
            result_lines: result,
            is_error: false,
        }
    }

    pub fn error(expression: Option<String>, error_msg: String) -> Self {
        Self {
            expression,
            result_lines: vec![error_msg],
            is_error: true,
        }
    }

    pub fn from_complex(expression: Option<String>, num: &ComplexNumber) -> Self {
        Self::new(expression, vec![format_complex_root(num)])
    }

    pub fn from_equation(
        expression: Option<String>,
        var: &str,
        solutions: &[ComplexNumber],
    ) -> Self {
        let renderer = TuiLatexRenderer::new();
        let result_lines = renderer.format_equation(var, solutions);
        Self::new(expression, result_lines)
    }

    pub fn from_equation_solution(
        expression: Option<String>,
        var: &str,
        solutions: &[ComplexNumber],
    ) -> Self {
        let renderer = TuiLatexRenderer::new();
        if solutions.is_empty() {
            return Self::error(expression, format!("No solution for {}", var));
        }

        let mut all_lines: Vec<String> = Vec::new();

        for (i, sol) in solutions.iter().enumerate() {
            let sol_var = if solutions.len() == 1 {
                var.to_string()
            } else {
                format!(
                    "{}{}",
                    var,
                    renderer.format_subscript_str("", &(i + 1).to_string())
                )
            };

            all_lines.push(format!("{} = {}", sol_var, format_complex_root(sol)));
        }

        Self::new(expression, all_lines)
    }

    pub fn height(&self) -> usize {
        let expr_height = self
            .expression
            .as_ref()
            .map(|e| e.lines().count())
            .unwrap_or(0);

        expr_height + self.result_lines.len() + 1
    }

    pub fn render(&self, width: usize) -> Vec<String> {
        let mut lines = Vec::new();

        let border = "┌".to_string() + &"─".repeat(width.saturating_sub(2)) + "┐";
        lines.push(border);

        if let Some(expr) = &self.expression {
            for expr_line in expr.lines() {
                let padded = format!("│ {} ", expr_line);
                let padded = format!(
                    "{}{}",
                    padded,
                    " ".repeat(width.saturating_sub(padded.len() + 1))
                );
                lines.push(format!("{}{}", &padded[..width.saturating_sub(1)], "│"));
            }

            lines.push(format!("│{}│", " ".repeat(width - 2)));
        }

        let style = if self.is_error { "│ error: " } else { "│ " };

        for (i, result_line) in self.result_lines.iter().enumerate() {
            let prefix = if i == 0 { style } else { "│ " };
            let padded = format!("{}{}", prefix, result_line);
            let padded = format!(
                "{}{}",
                padded,
                " ".repeat(width.saturating_sub(padded.len() + 1))
            );
            lines.push(format!("{}{}", &padded[..width.saturating_sub(1)], "│"));
        }

        let border = "└".to_string() + &"─".repeat(width.saturating_sub(2)) + "┘";
        lines.push(border);

        lines
    }
}

impl Default for ResultCard {
    fn default() -> Self {
        Self::new(None, Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Fraction;

    #[test]
    fn test_from_equation_solution_uses_single_line_complex_roots() {
        let solutions = vec![
            ComplexNumber::new(Fraction::new(-1, 2), Fraction::new(3, 2)),
            ComplexNumber::new(Fraction::new(-1, 2), Fraction::new(-3, 2)),
        ];
        let card = ResultCard::from_equation_solution(None, "x", &solutions);

        assert_eq!(card.result_lines.len(), 2);
        assert_eq!(card.result_lines[0], "x₁ = -1/2 + 3/2i");
        assert_eq!(card.result_lines[1], "x₂ = -1/2 - 3/2i");
    }

    #[test]
    fn test_from_complex_uses_root_formatter() {
        let value = ComplexNumber::new(Fraction::new(0, 1), Fraction::new(-1, 1));
        let card = ResultCard::from_complex(Some("ans".to_string()), &value);
        assert_eq!(card.result_lines, vec!["-i".to_string()]);
    }
}
