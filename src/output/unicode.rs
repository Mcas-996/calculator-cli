use crate::core::{ComplexNumber, Fraction};
use crate::core::types::Expression;

/// Unicode formatter with mathematical symbols
pub struct UnicodeFormatter;

impl UnicodeFormatter {
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
                format!("{} − i", self.format_fraction(&num.real))
            } else if num.imag > Fraction::new(0, 1) {
                format!("{} + {}i", self.format_fraction(&num.real), self.format_fraction(&num.imag))
            } else {
                format!("{} − {}i", self.format_fraction(&num.real), self.format_fraction(&-num.imag))
            }
        }
    }

    /// Format a fraction
    pub fn format_fraction(&self, frac: &Fraction) -> String {
        if frac.denom() == 1 {
            frac.numer().to_string()
        } else {
            format!("{}/{}", frac.numer(), frac.denom())
        }
    }

    /// Format an expression
    pub fn format_expression(&self, expr: &Expression) -> String {
        match expr {
            Expression::Constant(value) => self.format_complex(value),
            Expression::Variable(name) => {
                match name.as_str() {
                    "pi" => "π".to_string(),
                    _ => name.clone(),
                }
            }
            Expression::BinaryOp { left, op, right } => {
                let op_symbol = match op {
                    crate::core::types::BinaryOperator::Add => "+",
                    crate::core::types::BinaryOperator::Subtract => "−",
                    crate::core::types::BinaryOperator::Multiply => "×",
                    crate::core::types::BinaryOperator::Divide => "÷",
                    crate::core::types::BinaryOperator::Power => "^",
                    crate::core::types::BinaryOperator::Modulo => "%",
                };
                format!("({} {} {})", 
                    self.format_expression(left),
                    op_symbol,
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
        "➤ ".to_string()
    }

    /// Format subscript
    pub fn format_subscript(&self, text: &str) -> String {
        let subscripts = [
            ('0', '₀'), ('1', '₁'), ('2', '₂'), ('3', '₃'), ('4', '₄'),
            ('5', '₅'), ('6', '₆'), ('7', '₇'), ('8', '₈'), ('9', '₉'),
        ];
        
        text.chars()
            .map(|c| subscripts.iter().find(|(k, _)| k == &c).map(|(_, v)| *v).unwrap_or(c))
            .collect()
    }

    /// Format superscript
    pub fn format_superscript(&self, text: &str) -> String {
        let superscripts = [
            ('0', '⁰'), ('1', '¹'), ('2', '²'), ('3', '³'), ('4', '⁴'),
            ('5', '⁵'), ('6', '⁶'), ('7', '⁷'), ('8', '⁸'), ('9', '⁹'),
            ('-', '⁻'), ('+', '⁺'),
        ];
        
        text.chars()
            .map(|c| superscripts.iter().find(|(k, _)| k == &c).map(|(_, v)| *v).unwrap_or(c))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_complex_real() {
        let formatter = UnicodeFormatter;
        let num = ComplexNumber::from_real(Fraction::new(5, 1));
        assert_eq!(formatter.format_complex(&num), "5");
    }

    #[test]
    fn test_format_complex_mixed() {
        let formatter = UnicodeFormatter;
        let num = ComplexNumber::new(Fraction::new(3, 1), Fraction::new(4, 1));
        assert_eq!(formatter.format_complex(&num), "3 + 4i");
    }

    #[test]
    fn test_format_subscript() {
        let formatter = UnicodeFormatter;
        assert_eq!(formatter.format_subscript("123"), "₁₂₃");
    }

    #[test]
    fn test_format_superscript() {
        let formatter = UnicodeFormatter;
        assert_eq!(formatter.format_superscript("23"), "²³");
    }

    #[test]
    fn test_format_prompt() {
        let formatter = UnicodeFormatter;
        assert_eq!(formatter.format_prompt(), "➤ ");
    }
}