use crate::core::{ComplexNumber, Fraction};
use crate::output::UnicodeFormatter;
use crate::tui::latex::TuiLatexRenderer;
use crate::tui::math_renderer::render_math_text;

pub(crate) fn format_complex_root(num: &ComplexNumber) -> String {
    let zero = Fraction::new(0, 1);
    let one = Fraction::new(1, 1);
    let formatter = UnicodeFormatter;

    if num.imag == zero {
        return format_fraction_component(&formatter, &num.real);
    }

    if num.real == zero {
        if num.imag == one {
            return "i".to_string();
        }
        if num.imag == -one {
            return "-i".to_string();
        }

        let imag_abs = if num.imag > zero { num.imag } else { -num.imag };
        let imag_coeff = format_fraction_component(&formatter, &imag_abs);

        if imag_coeff.contains("sqrt(") {
            if num.imag > zero {
                format!("i*{}", imag_coeff)
            } else {
                format!("-i*{}", imag_coeff)
            }
        } else if num.imag > zero {
            format!("{}i", imag_coeff)
        } else {
            format!("-{}i", imag_coeff)
        }
    } else {
        let real_str = format_fraction_component(&formatter, &num.real);

        if num.imag == one {
            return format!("{} + i", real_str);
        }
        if num.imag == -one {
            return format!("{} - i", real_str);
        }

        let imag_abs = if num.imag > zero { num.imag } else { -num.imag };
        let mut imag_coeff = format_fraction_component(&formatter, &imag_abs);
        if needs_imaginary_parentheses(&imag_coeff) {
            imag_coeff = format!("({})", imag_coeff);
        }

        if num.imag > zero {
            format!("{} + {}i", real_str, imag_coeff)
        } else {
            format!("{} - {}i", real_str, imag_coeff)
        }
    }
}

fn render_with_tui_math_or_fallback(value_text: &str) -> Vec<String> {
    render_math_text(value_text).unwrap_or_else(|_| vec![value_text.to_string()])
}

fn prefix_value_lines(var: &str, value_lines: &[String]) -> Vec<String> {
    if value_lines.is_empty() {
        return vec![format!("{var} = ")];
    }

    let mut result = Vec::with_capacity(value_lines.len());
    result.push(format!("{var} = {}", value_lines[0]));

    let indent = " ".repeat(var.chars().count() + 3);
    for line in value_lines.iter().skip(1) {
        result.push(format!("{indent}{line}"));
    }

    result
}

fn format_fraction_component(formatter: &UnicodeFormatter, frac: &Fraction) -> String {
    if frac.denom() == 1 || frac.denom().abs() <= 128 {
        return frac.to_string();
    }

    let rendered = formatter.format_complex(&ComplexNumber::from_real(*frac));
    normalize_unicode_math(&rendered)
}

fn needs_imaginary_parentheses(value: &str) -> bool {
    value.contains('+') || value.contains('-') || (value.contains('/') && value.contains("sqrt("))
}

fn normalize_unicode_math(value: &str) -> String {
    let chars: Vec<char> = value.chars().collect();
    let mut out = String::new();
    let mut i = 0;

    while i < chars.len() {
        let ch = chars[i];

        if ch == '−' {
            out.push('-');
            i += 1;
            continue;
        }

        if ch == '√' {
            if out
                .chars()
                .last()
                .is_some_and(|last| last.is_ascii_digit() || last == ')')
            {
                out.push('*');
            }

            i += 1;
            let mut radicand = String::new();
            while i < chars.len() && chars[i].is_ascii_alphanumeric() {
                radicand.push(chars[i]);
                i += 1;
            }

            if radicand.is_empty() {
                out.push_str("sqrt(?)");
            } else {
                out.push_str("sqrt(");
                out.push_str(&radicand);
                out.push(')');
            }
            continue;
        }

        out.push(ch);
        i += 1;
    }

    out
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
        let value_text = format_complex_root(num);
        let result_lines = render_with_tui_math_or_fallback(&value_text);
        Self::new(expression, result_lines)
    }

    pub fn from_equation(
        expression: Option<String>,
        var: &str,
        solutions: &[ComplexNumber],
    ) -> Self {
        if solutions.is_empty() {
            return Self::new(expression, vec![format!("{var} = ∅")]);
        }

        let renderer = TuiLatexRenderer::new();
        let mut result_lines = Vec::new();
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

            let value_text = format_complex_root(sol);
            let rendered_lines = render_with_tui_math_or_fallback(&value_text);
            result_lines.extend(prefix_value_lines(&sol_var, &rendered_lines));
        }

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

            let value_text = format_complex_root(sol);
            let rendered_lines = render_with_tui_math_or_fallback(&value_text);
            all_lines.extend(prefix_value_lines(&sol_var, &rendered_lines));
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
    use crate::solver::solve_quadratic_equation;

    #[test]
    fn test_from_equation_solution_renders_multiline_fraction_layouts() {
        let solutions = vec![
            ComplexNumber::new(Fraction::new(-1, 2), Fraction::new(3, 2)),
            ComplexNumber::new(Fraction::new(-1, 2), Fraction::new(-3, 2)),
        ];
        let card = ResultCard::from_equation_solution(None, "x", &solutions);

        assert!(card.result_lines.len() >= 2);
        assert!(card.result_lines.iter().any(|line| line.contains("x₁ =")));
        assert!(card.result_lines.iter().any(|line| line.contains("x₂ =")));
        assert!(card.result_lines.iter().any(|line| line.contains('─')));
    }

    #[test]
    fn test_from_complex_uses_root_formatter() {
        let value = ComplexNumber::new(Fraction::new(0, 1), Fraction::new(-1, 1));
        let card = ResultCard::from_complex(Some("ans".to_string()), &value);
        assert_eq!(card.result_lines, vec!["-i".to_string()]);
    }

    #[test]
    fn test_from_complex_formats_symbolic_sqrt() {
        let value = ComplexNumber::from_double(2.0_f64.sqrt());
        let card = ResultCard::from_complex(Some("sqrt(2)".to_string()), &value);
        assert!(card.result_lines.iter().any(|line| line.contains("√2")));
    }

    #[test]
    fn test_from_complex_simplifies_sqrt8_to_coefficient_times_sqrt2() {
        let value = ComplexNumber::from_double(8.0_f64.sqrt());
        let card = ResultCard::from_complex(Some("sqrt(8)".to_string()), &value);
        assert!(card.result_lines.iter().any(|line| line.contains('2')));
        assert!(card.result_lines.iter().any(|line| line.contains("√2")));
    }

    #[test]
    fn test_from_equation_solution_formats_pure_imaginary_radicals() {
        let solutions = solve_quadratic_equation("x^2 = -2").unwrap();
        let card = ResultCard::from_equation_solution(None, "x", &solutions);

        assert!(card.result_lines.len() >= 2);
        assert!(card.result_lines.iter().any(|line| line.contains('i')));
        assert!(card.result_lines.iter().any(|line| line.contains("√2")));
    }

    #[test]
    fn test_from_equation_solution_formats_mixed_complex_conjugates() {
        let solutions = solve_quadratic_equation("x^2 + 2x + 10 = 0").unwrap();
        let card = ResultCard::from_equation_solution(None, "x", &solutions);

        assert!(card.result_lines.len() >= 2);
        assert!(card.result_lines.iter().any(|line| line.contains("-1")));
        assert!(card.result_lines.iter().any(|line| line.contains("3i")));
    }

    #[test]
    fn test_fallback_keeps_plain_text_when_tui_math_render_fails() {
        assert_eq!(
            render_with_tui_math_or_fallback("sqrt("),
            vec!["sqrt(".to_string()]
        );
    }
}
