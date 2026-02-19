use crate::core::types::{Expression, FunctionName, UnaryOperator};
use crate::core::{ComplexNumber, Fraction};
use crate::parser::tokenizer::{Token, Tokenizer};
use std::f64::consts::PI;

/// Expression parser
pub struct ExpressionParser {
    tokens: Vec<Token>,
    current: usize,
}

impl ExpressionParser {
    pub fn new(tokens: Vec<Token>) -> Self {
        ExpressionParser { tokens, current: 0 }
    }

    /// Parse tokens into an expression
    pub fn parse(&mut self) -> Result<Expression, String> {
        self.parse_expression(0)
    }

    /// Parse expression with given minimum precedence
    fn parse_expression(&mut self, min_precedence: u8) -> Result<Expression, String> {
        let mut left = self.parse_atom()?;

        while self.current < self.tokens.len() {
            if let Token::BinaryOp(op) = self.tokens[self.current].clone() {
                let precedence = op.precedence();
                if precedence < min_precedence {
                    break;
                }

                self.current += 1;
                let right = self.parse_expression(precedence + 1)?;
                left = Expression::BinaryOp {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        Ok(left)
    }

    /// Parse atomic expression (number, variable, function, parenthesized expression, or unary op)
    fn parse_atom(&mut self) -> Result<Expression, String> {
        if self.current >= self.tokens.len() {
            return Err("Unexpected end of input".to_string());
        }

        let token = self.tokens[self.current].clone();
        self.current += 1;

        match token {
            Token::Number(value) => {
                // Handle percentage: "50%" -> 0.5
                if self.current < self.tokens.len() && self.tokens[self.current] == Token::Percent {
                    self.current += 1;
                    Ok(Expression::Constant(ComplexNumber::from_double(
                        value / 100.0,
                    )))
                } else {
                    Ok(Expression::Constant(ComplexNumber::from_double(value)))
                }
            }
            Token::Fraction(num, denom) => {
                let frac = Fraction::new(num, denom);
                Ok(Expression::Constant(ComplexNumber::from_real(frac)))
            }
            Token::Variable(name) => {
                // Handle special constants
                match name.as_str() {
                    "pi" | "π" => Ok(Expression::Constant(ComplexNumber::from_double(PI))),
                    "e" => Ok(Expression::Constant(ComplexNumber::from_double(
                        std::f64::consts::E,
                    ))),
                    "i" => Ok(Expression::Constant(ComplexNumber::new(
                        Fraction::new(0, 1),
                        Fraction::new(1, 1),
                    ))),
                    _ => Ok(Expression::Variable(name)),
                }
            }
            Token::Function(func_name) => self.parse_function_call(func_name),
            Token::LeftParen => {
                let expr = self.parse_expression(0)?;
                if self.current >= self.tokens.len()
                    || self.tokens[self.current] != Token::RightParen
                {
                    return Err("Missing closing parenthesis".to_string());
                }
                self.current += 1;
                Ok(expr)
            }
            Token::UnaryOp(UnaryOperator::Negate) => {
                let operand = self.parse_atom()?;
                Ok(Expression::UnaryOp {
                    op: UnaryOperator::Negate,
                    operand: Box::new(operand),
                })
            }
            _ => Err(format!("Unexpected token: {:?}", token)),
        }
    }

    /// Parse function call
    fn parse_function_call(&mut self, func_name: FunctionName) -> Result<Expression, String> {
        if self.current >= self.tokens.len() || self.tokens[self.current] != Token::LeftParen {
            return Err(format!(
                "Expected '(' after function {}",
                func_name.as_str()
            ));
        }
        self.current += 1;

        let mut args = Vec::new();

        // Parse arguments
        if self.current < self.tokens.len() && self.tokens[self.current] != Token::RightParen {
            args.push(self.parse_expression(0)?);

            // Parse additional arguments separated by commas
            while self.current < self.tokens.len() && self.tokens[self.current] == Token::Comma {
                self.current += 1;
                args.push(self.parse_expression(0)?);
            }
        }

        if self.current >= self.tokens.len() || self.tokens[self.current] != Token::RightParen {
            return Err(format!(
                "Missing closing parenthesis for function {}",
                func_name.as_str()
            ));
        }
        self.current += 1;

        Ok(Expression::Function {
            name: func_name,
            args,
        })
    }

    /// Check if there are more tokens to parse
    pub fn has_more(&self) -> bool {
        self.current < self.tokens.len()
    }
}

/// Parse a string into an expression
pub fn parse_expression(input: &str) -> Result<Expression, String> {
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize()?;
    let mut parser = ExpressionParser::new(tokens);
    let expr = parser.parse()?;

    // Ensure we consumed all tokens
    if parser.has_more() {
        return Err("Unexpected extra tokens at end of input".to_string());
    }

    Ok(expr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_number() {
        let expr = parse_expression("42").unwrap();
        assert_eq!(expr.evaluate().unwrap().real, Fraction::new(42, 1));
    }

    #[test]
    fn test_parse_addition() {
        let expr = parse_expression("3 + 5").unwrap();
        assert_eq!(expr.evaluate().unwrap().real, Fraction::new(8, 1));
    }

    #[test]
    fn test_parse_subtraction() {
        let expr = parse_expression("10 - 4").unwrap();
        assert_eq!(expr.evaluate().unwrap().real, Fraction::new(6, 1));
    }

    #[test]
    fn test_parse_multiplication() {
        let expr = parse_expression("3 * 4").unwrap();
        assert_eq!(expr.evaluate().unwrap().real, Fraction::new(12, 1));
    }

    #[test]
    fn test_parse_division() {
        let expr = parse_expression("10 / 2").unwrap();
        assert_eq!(expr.evaluate().unwrap().real, Fraction::new(5, 1));
    }

    #[test]
    fn test_parse_precedence() {
        let expr = parse_expression("3 + 4 * 2").unwrap();
        assert_eq!(expr.evaluate().unwrap().real, Fraction::new(11, 1));
    }

    #[test]
    fn test_parse_parentheses() {
        let expr = parse_expression("(3 + 4) * 2").unwrap();
        assert_eq!(expr.evaluate().unwrap().real, Fraction::new(14, 1));
    }

    #[test]
    fn test_parse_power() {
        let expr = parse_expression("2 ^ 3").unwrap();
        assert_eq!(expr.evaluate().unwrap().real, Fraction::new(8, 1));
    }

    #[test]
    fn test_parse_power_precedence() {
        let expr = parse_expression("2 ^ 3 + 1").unwrap();
        assert_eq!(expr.evaluate().unwrap().real, Fraction::new(9, 1));
    }

    #[test]
    fn test_parse_unary_negation() {
        let expr = parse_expression("-5").unwrap();
        assert_eq!(expr.evaluate().unwrap().real, Fraction::new(-5, 1));
    }

    #[test]
    fn test_parse_function_call() {
        let expr = parse_expression("sqrt(16)").unwrap();
        assert_eq!(expr.evaluate().unwrap().real, Fraction::new(4, 1));
    }

    #[test]
    fn test_parse_variable_pi() {
        let expr = parse_expression("pi").unwrap();
        let result = expr.evaluate().unwrap();
        assert!((result.real.to_f64() - PI).abs() < 1e-10);
    }

    #[test]
    fn test_parse_percentage() {
        let expr = parse_expression("50%").unwrap();
        assert_eq!(expr.evaluate().unwrap().real, Fraction::new(1, 2));
    }

    #[test]
    fn test_parse_fraction() {
        let expr = parse_expression("1/2").unwrap();
        assert_eq!(expr.evaluate().unwrap().real, Fraction::new(1, 2));
    }

    #[test]
    fn test_parse_complex_expression() {
        let expr = parse_expression("3 + 4 * (2 - 1)").unwrap();
        assert_eq!(expr.evaluate().unwrap().real, Fraction::new(7, 1));
    }

    #[test]
    fn test_parse_mismatched_parentheses() {
        let result = parse_expression("(3 + 4");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_token() {
        let result = parse_expression("3 @ 4");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_direct_division_by_zero_returns_undefined() {
        let expr = parse_expression("1/0").unwrap();
        assert_eq!(expr.evaluate(), Err("undefined".to_string()));
    }

    #[test]
    fn test_parse_nested_division_by_zero_returns_undefined() {
        let expr = parse_expression("1/(2*0)").unwrap();
        assert_eq!(expr.evaluate(), Err("undefined".to_string()));
    }
}
