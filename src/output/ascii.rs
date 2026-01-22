use crate::core::{ComplexNumber, Fraction};
use crate::core::types::Expression;

/// ASCII formatter for basic text output
pub struct AsciiFormatter;

impl AsciiFormatter {
    /// Format a complex number
    pub fn format_complex(&self, num: &ComplexNumber) -> String {
        num.to_string()
    }

    /// Format a fraction
    pub fn format_fraction(&self, frac: &Fraction) -> String {
        frac.to_string()
    }

    /// Format an expression
    pub fn format_expression(&self, expr: &Expression) -> String {
        match expr {
            Expression::Constant(value) => self.format_complex(value),
            Expression::Variable(name) => name.clone(),
            Expression::BinaryOp { left, op, right } => {
                format!("({} {} {})", 
                    self.format_expression(left),
                    op.symbol(),
                    self.format_expression(right))
            }
            Expression::UnaryOp { op, operand } => {
                format!("({}{})", op.symbol(), self.format_expression(operand))
            }
            Expression::Function { name, args } => {
                let args_str: Vec<String> = args.iter()
                    .map(|a| self.format_expression(a))
                    .collect();
                format!("{}({})", name.as_str(), args_str.join(", "))
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_complex_real() {
        let formatter = AsciiFormatter;
        let num = ComplexNumber::from_real(Fraction::new(5, 1));
        assert_eq!(formatter.format_complex(&num), "5");
    }

    #[test]
    fn test_format_complex_imaginary() {
        let formatter = AsciiFormatter;
        let num = ComplexNumber::new(Fraction::new(0, 1), Fraction::new(1, 1));
        assert_eq!(formatter.format_complex(&num), "i");
    }

    #[test]
    fn test_format_complex_mixed() {
        let formatter = AsciiFormatter;
        let num = ComplexNumber::new(Fraction::new(3, 1), Fraction::new(4, 1));
        assert_eq!(formatter.format_complex(&num), "3 + 4i");
    }

    #[test]
    fn test_format_fraction() {
        let formatter = AsciiFormatter;
        let frac = Fraction::new(3, 4);
        assert_eq!(formatter.format_fraction(&frac), "3/4");
    }

    #[test]
    fn test_format_equation_solution() {
        let formatter = AsciiFormatter;
        assert_eq!(formatter.format_equation_solution("x", "5"), "x = 5");
    }

    #[test]
    fn test_format_prompt() {
        let formatter = AsciiFormatter;
        assert_eq!(formatter.format_prompt(), ">>> ");
    }
}