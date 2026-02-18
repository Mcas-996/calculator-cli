use crate::core::{ComplexNumber, Fraction};

pub struct TuiLatexRenderer;

impl TuiLatexRenderer {
    pub fn new() -> Self {
        Self
    }

    pub fn format_complex(&self, num: &ComplexNumber) -> Vec<String> {
        if num.real == Fraction::new(0, 1) && num.imag == Fraction::new(0, 1) {
            vec!["0".to_string()]
        } else if num.imag == Fraction::new(0, 1) {
            self.format_fraction(&num.real)
        } else if num.real == Fraction::new(0, 1) {
            if num.imag == Fraction::new(1, 1) {
                vec!["i".to_string()]
            } else if num.imag == Fraction::new(-1, 1) {
                vec!["-i".to_string()]
            } else {
                let mut lines = self.format_fraction(&num.imag);
                for line in lines.iter_mut() {
                    line.push('i');
                }
                lines
            }
        } else {
            let real_lines = self.format_fraction(&num.real);
            if num.imag == Fraction::new(1, 1) {
                self.combine_lines(&real_lines, &vec!["i".to_string()], " + ")
            } else if num.imag == Fraction::new(-1, 1) {
                self.combine_lines(&real_lines, &vec!["i".to_string()], " - ")
            } else if num.imag > Fraction::new(0, 1) {
                let mut imag_lines = self.format_fraction(&num.imag);
                for line in imag_lines.iter_mut() {
                    line.push('i');
                }
                self.combine_lines(&real_lines, &imag_lines, " + ")
            } else {
                let mut imag_lines = self.format_fraction(&(-num.imag));
                for line in imag_lines.iter_mut() {
                    line.push('i');
                }
                self.combine_lines(&real_lines, &imag_lines, " - ")
            }
        }
    }

    fn format_fraction(&self, frac: &Fraction) -> Vec<String> {
        if frac.denom() == 1 {
            vec![frac.numer().to_string()]
        } else {
            let num_str = frac.numer().to_string();
            let den_str = frac.denom().to_string();
            let width = num_str.len().max(den_str.len());

            let num_padded = format!("{:^width$}", num_str, width = width);
            let den_padded = format!("{:^width$}", den_str, width = width);

            let line = "─".repeat(width);

            vec![
                format!("{:^width$}", num_padded, width = width + 2),
                format!("{}{}", " ", line),
                format!("{:^width$}", den_padded, width = width + 2),
            ]
        }
    }

    fn combine_lines(&self, left: &[String], right: &[String], op: &str) -> Vec<String> {
        let max_len = left.len().max(right.len());
        let mut result = Vec::with_capacity(max_len);

        for i in 0..max_len {
            let l = left.get(i).map(|s| s.as_str()).unwrap_or("");
            let r = right.get(i).map(|s| s.as_str()).unwrap_or("");
            let padding_left = " ".repeat(left[0].len().saturating_sub(l.len()));
            let padding_right = " ".repeat(right[0].len().saturating_sub(r.len()));

            result.push(format!("{}{}{}{}{}", padding_left, l, op, padding_right, r));
        }

        result
    }

    pub fn format_radical(&self, radicand: &str, index: Option<i32>) -> Vec<String> {
        let index_str = index.map(|i| i.to_string()).unwrap_or_default();
        let width = radicand.len() + 2;

        if index.is_some() {
            vec![
                format!("{:>width$}", format!("{}√", index_str), width = width + 1),
                format!(
                    "{:>width$}",
                    format!(" {}", "─".repeat(radicand.len())),
                    width = width + 1
                ),
                format!("{:>width$}", format!(" {}", radicand), width = width + 1),
            ]
        } else {
            vec![
                format!("{:>width$}", format!("√"), width = width + 1),
                format!(
                    "{:>width$}",
                    format!(" {}", "─".repeat(radicand.len())),
                    width = width + 1
                ),
                format!("{:>width$}", format!(" {}", radicand), width = width + 1),
            ]
        }
    }

    pub fn format_superscript(&self, base: &str, exp: &str) -> String {
        let superscripts = [
            ('0', '⁰'),
            ('1', '¹'),
            ('2', '²'),
            ('3', '³'),
            ('4', '⁴'),
            ('5', '⁵'),
            ('6', '⁶'),
            ('7', '⁷'),
            ('8', '⁸'),
            ('9', '⁹'),
            ('+', '⁺'),
            ('-', '⁻'),
            ('=', '⁼'),
            ('(', '⁽'),
            (')', '⁾'),
            ('n', 'ⁿ'),
            ('i', 'ⁱ'),
            ('x', 'ˣ'),
            ('y', 'ʸ'),
        ];

        let exp_converted: String = exp
            .chars()
            .map(|c| {
                superscripts
                    .iter()
                    .find(|(orig, _)| *orig == c)
                    .map(|(_, sup)| *sup)
                    .unwrap_or(c)
            })
            .collect();

        format!("{}{}", base, exp_converted)
    }

    pub fn format_subscript_str(&self, base: &str, sub: &str) -> String {
        let subscripts = [
            ('0', '₀'),
            ('1', '₁'),
            ('2', '₂'),
            ('3', '₃'),
            ('4', '₄'),
            ('5', '₅'),
            ('6', '₆'),
            ('7', '₇'),
            ('8', '₈'),
            ('9', '₉'),
            ('+', '₊'),
            ('-', '₋'),
            ('=', '₌'),
            ('(', '₍'),
            (')', '₎'),
            ('a', 'ₐ'),
            ('e', 'ₑ'),
            ('o', 'ₒ'),
            ('x', 'ₓ'),
            ('h', 'ₕ'),
            ('k', 'ₖ'),
            ('l', 'ₗ'),
            ('m', 'ₘ'),
            ('n', 'ₙ'),
            ('p', 'ₚ'),
            ('s', 'ₛ'),
            ('t', 'ₜ'),
        ];

        let sub_converted: String = sub
            .chars()
            .map(|c| {
                subscripts
                    .iter()
                    .find(|(orig, _)| *orig == c)
                    .map(|(_, sub)| *sub)
                    .unwrap_or(c)
            })
            .collect();

        format!("{}{}", base, sub_converted)
    }

    pub fn format_equation(&self, var: &str, solutions: &[ComplexNumber]) -> Vec<String> {
        if solutions.is_empty() {
            return vec![format!("{} = ∅", var)];
        }

        if solutions.len() == 1 {
            let result = self.format_complex(&solutions[0]);
            return self.prepend_equation_line(var, &result);
        }

        let mut all_lines: Vec<Vec<String>> = Vec::new();

        for (i, sol) in solutions.iter().enumerate() {
            let sol_lines = self.format_complex(sol);
            if i == 0 {
                all_lines.push(sol_lines);
            } else {
                for (j, line) in sol_lines.iter().enumerate() {
                    if j < all_lines.len() {
                        all_lines[j].push(format!(", {}", line));
                    } else {
                        all_lines.push(vec![format!("  {}", line)]);
                    }
                }
            }
        }

        let vars: Vec<String> = (1..=solutions.len())
            .map(|i| format!("{}{}", var, self.format_subscript_str("", &i.to_string())))
            .collect();

        let mut result = Vec::new();
        for (i, line) in all_lines.iter().enumerate() {
            if i == 0 {
                result.push(format!("{} = {}", vars.join(", "), line.join("")));
            } else {
                result.push(line.join(""));
            }
        }

        result
    }

    fn prepend_equation_line(&self, var: &str, value_lines: &[String]) -> Vec<String> {
        let mut result = Vec::new();

        if value_lines.is_empty() {
            return result;
        }

        result.push(format!("{} = {}", var, value_lines[0]));

        for line in value_lines.iter().skip(1) {
            result.push(line.clone());
        }

        result
    }

    pub fn format_equation_solution(&self, var: &str, value: &[String]) -> Vec<String> {
        self.prepend_equation_line(var, value)
    }
}

impl Default for TuiLatexRenderer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_complex_real() {
        let renderer = TuiLatexRenderer::new();
        let num = ComplexNumber::from_real(Fraction::new(5, 1));
        assert_eq!(renderer.format_complex(&num), vec!["5"]);
    }

    #[test]
    fn test_format_fraction() {
        let renderer = TuiLatexRenderer::new();
        let frac = Fraction::new(3, 4);
        let result = renderer.format_fraction(&frac);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_format_superscript() {
        let renderer = TuiLatexRenderer::new();
        assert_eq!(renderer.format_superscript("x", "2"), "x²");
    }

    #[test]
    fn test_format_subscript() {
        let renderer = TuiLatexRenderer::new();
        assert_eq!(renderer.format_subscript_str("x", "1"), "x₁");
    }

    #[test]
    fn test_format_equation_single() {
        let renderer = TuiLatexRenderer::new();
        let num = ComplexNumber::from_real(Fraction::new(5, 1));
        let result = renderer.format_equation("x", &[num]);
        assert!(!result.is_empty());
    }
}
