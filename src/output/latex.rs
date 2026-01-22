use crate::core::{ComplexNumber, Fraction};
use crate::core::types::Expression;

/// LaTeX formatter for generating LaTeX source code
pub struct LatexFormatter;

impl LatexFormatter {
    /// Format a complex number
    pub fn format_complex(&self, num: &ComplexNumber) -> String {
        if num.imag == Fraction::new(0, 1) {
            self.format_fraction(&num.real)
        } else if num.real == Fraction::new(0, 1) {
            if num.imag == Fraction::new(1, 1) {
                "i".to_string()
            } else if num.imag == Fraction::new(-1, 1) {
                "-i".to_string()
            } else {
                format!("{}i", self.format_fraction(&num.imag))
            }
        } else {
            if num.imag == Fraction::new(1, 1) {
                format!("{} + i", self.format_fraction(&num.real))
            } else if num.imag == Fraction::new(-1, 1) {
                format!("{} - i", self.format_fraction(&num.real))
            } else if num.imag > Fraction::new(0, 1) {
                format!("{} + {}i", self.format_fraction(&num.real), self.format_fraction(&num.imag))
            } else {
                format!("{} - {}i", self.format_fraction(&num.real), self.format_fraction(&-num.imag))
            }
        }
    }

    /// Format a fraction
    pub fn format_fraction(&self, frac: &Fraction) -> String {
        if frac.denom() == 1 {
            frac.numer().to_string()
        } else {
            format!("\\frac{{{}}}{{{}}}", frac.numer(), frac.denom())
        }
    }

    /// Format an expression
    pub fn format_expression(&self, expr: &Expression) -> String {
        match expr {
            Expression::Constant(value) => self.format_complex(value),
            Expression::Variable(name) => {
                match name.as_str() {
                    "pi" => "\\pi".to_string(),
                    _ => name.clone(),
                }
            }
            Expression::BinaryOp { left, op, right } => {
                let op_latex = match op {
                    crate::core::types::BinaryOperator::Add => "+",
                    crate::core::types::BinaryOperator::Subtract => "-",
                    crate::core::types::BinaryOperator::Multiply => "\\cdot",
                    crate::core::types::BinaryOperator::Divide => "\\div",
                    crate::core::types::BinaryOperator::Power => "^",
                    crate::core::types::BinaryOperator::Modulo => "\\bmod",
                };
                format!("({} {} {})", 
                    self.format_expression(left),
                    op_latex,
                    self.format_expression(right))
            }
            Expression::UnaryOp { op, operand } => {
                format!("({}{})", op.symbol(), self.format_expression(operand))
            }
            Expression::Function { name, args } => {
                let func_name = match name {
                    crate::core::types::FunctionName::Sqrt => "\\sqrt",
                    _ => name.as_str(),
                };
                let args_str: Vec<String> = args.iter()
                    .map(|a| self.format_expression(a))
                    .collect();
                if func_name == "\\sqrt" {
                    format!("{{{}}}", args_str.join(", "))
                } else {
                    format!("{}({})", func_name, args_str.join(", "))
                }
            }
        }
    }

    /// Format an equation solution
    pub fn format_equation_solution(&self, var: &str, value: &str) -> String {
        format!("{} = {}", var, value)
    }

    /// Format the prompt
    pub fn format_prompt(&self) -> String {
        ">>> ".to_string()
    }

    /// Format subscript
    pub fn format_subscript(&self, base: &str, sub: &str) -> String {
        format!("{}_{{{}}}", base, sub)
    }

    /// Wrap expression in LaTeX display mode
    pub fn wrap_display(&self, expr: &str) -> String {
        format!("[{}]", expr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_complex_real() {
        let formatter = LatexFormatter;
        let num = ComplexNumber::from_real(Fraction::new(5, 1));
        assert_eq!(formatter.format_complex(&num), "5");
    }

    #[test]
    fn test_format_fraction() {
        let formatter = LatexFormatter;
        let frac = Fraction::new(3, 4);
        assert_eq!(formatter.format_fraction(&frac), "\\frac{3}{4}");
    }

    #[test]
    fn test_format_subscript() {
        let formatter = LatexFormatter;
        assert_eq!(formatter.format_subscript("x", "1"), "x_{1}");
    }

    #[test]
    fn test_wrap_display() {
        let formatter = LatexFormatter;
        assert_eq!(formatter.wrap_display("x = 5"), "[x = 5]");
    }
}