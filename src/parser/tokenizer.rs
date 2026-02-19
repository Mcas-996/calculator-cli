use crate::core::types::{BinaryOperator, FunctionName, UnaryOperator};
use std::iter::Peekable;
use std::str::Chars;

/// Token types
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Fraction(i64, i64), // numerator, denominator
    Variable(String),
    Function(FunctionName),
    BinaryOp(BinaryOperator),
    UnaryOp(UnaryOperator),
    LeftParen,
    RightParen,
    Comma,
    Percent, // % sign
    Equal,   // = sign
    EOF,
}

/// Tokenizer for mathematical expressions
pub struct Tokenizer<'a> {
    chars: Peekable<Chars<'a>>,
    position: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Tokenizer {
            chars: input.chars().peekable(),
            position: 0,
        }
    }

    /// Get current position
    pub fn position(&self) -> usize {
        self.position
    }

    /// Peek at next character
    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    /// Consume next character
    fn next(&mut self) -> Option<char> {
        self.position += 1;
        self.chars.next()
    }

    /// Skip whitespace
    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.peek() {
            if c.is_whitespace() {
                self.next();
            } else {
                break;
            }
        }
    }

    /// Tokenize a number (integer, decimal, or fraction)
    fn tokenize_number(&mut self) -> Result<Token, String> {
        let mut num_str = String::new();
        let mut has_decimal = false;

        // Parse integer part
        while let Some(&c) = self.peek() {
            if c.is_ascii_digit() || c == '-' {
                num_str.push(c);
                self.next();
            } else if c == '.' {
                if has_decimal {
                    return Err("Multiple decimal points in number".to_string());
                }
                has_decimal = true;
                num_str.push(c);
                self.next();
            } else {
                break;
            }
        }

        // Check for fraction (e.g., "1/2" or "3/4").
        // If denominator is zero, keep this as a normal division expression (1 / 0)
        // so evaluation can return `undefined`.
        if let Some(&'/') = self.peek() {
            let mut lookahead = self.chars.clone();
            lookahead.next(); // consume '/'
            let mut denom_str = String::new();

            while let Some(c) = lookahead.peek().copied() {
                if c.is_ascii_digit() {
                    denom_str.push(c);
                    lookahead.next();
                } else {
                    break;
                }
            }

            if !denom_str.is_empty() {
                let denom: i64 = denom_str
                    .parse()
                    .map_err(|_| "Invalid fraction denominator".to_string())?;

                if denom != 0 {
                    self.next(); // consume '/'
                    for _ in 0..denom_str.len() {
                        self.next();
                    }

                    let num: i64 = num_str
                        .parse()
                        .map_err(|_| "Invalid fraction numerator".to_string())?;
                    return Ok(Token::Fraction(num, denom));
                }
            }
        }

        // Parse as float
        let value: f64 = num_str
            .parse()
            .map_err(|_| "Invalid number format".to_string())?;
        Ok(Token::Number(value))
    }

    /// Tokenize a variable or function name
    fn tokenize_identifier(&mut self) -> Result<Token, String> {
        let mut name = String::new();

        while let Some(&c) = self.peek() {
            if c.is_alphabetic() || c == '_' {
                name.push(c);
                self.next();
            } else {
                break;
            }
        }

        if name.is_empty() {
            return Err("Empty identifier".to_string());
        }

        // Check if it's a function name
        if let Some(func_name) = FunctionName::from_str(&name) {
            Ok(Token::Function(func_name))
        } else {
            Ok(Token::Variable(name))
        }
    }

    /// Get next token
    pub fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace();

        match self.peek() {
            None => Ok(Token::EOF),
            Some(&c) => {
                match c {
                    '0'..='9' | '-' => {
                        // Check if '-' is a unary operator or part of a number
                        if c == '-' {
                            // Look ahead to see if this is a negative number
                            let next_char = self.chars.clone().nth(1);
                            if next_char
                                .map(|n| n.is_ascii_digit() || n == '.')
                                .unwrap_or(false)
                            {
                                self.tokenize_number()
                            } else {
                                self.next();
                                Ok(Token::UnaryOp(UnaryOperator::Negate))
                            }
                        } else {
                            self.tokenize_number()
                        }
                    }
                    '+' => {
                        self.next();
                        Ok(Token::BinaryOp(BinaryOperator::Add))
                    }
                    '*' => {
                        self.next();
                        Ok(Token::BinaryOp(BinaryOperator::Multiply))
                    }
                    '/' => {
                        self.next();
                        Ok(Token::BinaryOp(BinaryOperator::Divide))
                    }
                    '^' => {
                        self.next();
                        Ok(Token::BinaryOp(BinaryOperator::Power))
                    }
                    '%' => {
                        self.next();
                        Ok(Token::Percent)
                    }
                    '(' => {
                        self.next();
                        Ok(Token::LeftParen)
                    }
                    ')' => {
                        self.next();
                        Ok(Token::RightParen)
                    }
                    ',' => {
                        self.next();
                        Ok(Token::Comma)
                    }
                    '=' => {
                        self.next();
                        Ok(Token::Equal)
                    }
                    'a'..='z' | 'A'..='Z' | '_' => self.tokenize_identifier(),
                    _ => Err(format!("Unexpected character: '{}'", c)),
                }
            }
        }
    }

    /// Tokenize entire input into a vector of tokens
    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token()?;
            if token == Token::EOF {
                break;
            }
            tokens.push(token);
        }

        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_number() {
        let mut tokenizer = Tokenizer::new("123");
        assert_eq!(tokenizer.next_token().unwrap(), Token::Number(123.0));
    }

    #[test]
    fn test_tokenize_decimal() {
        let mut tokenizer = Tokenizer::new("3.14");
        assert_eq!(tokenizer.next_token().unwrap(), Token::Number(3.14));
    }

    #[test]
    fn test_tokenize_fraction() {
        let mut tokenizer = Tokenizer::new("1/2");
        assert_eq!(tokenizer.next_token().unwrap(), Token::Fraction(1, 2));
    }

    #[test]
    fn test_tokenize_negative_number() {
        let mut tokenizer = Tokenizer::new("-5");
        assert_eq!(tokenizer.next_token().unwrap(), Token::Number(-5.0));
    }

    #[test]
    fn test_tokenize_variable() {
        let mut tokenizer = Tokenizer::new("x");
        assert_eq!(
            tokenizer.next_token().unwrap(),
            Token::Variable("x".to_string())
        );
    }

    #[test]
    fn test_tokenize_function() {
        let mut tokenizer = Tokenizer::new("sin");
        assert_eq!(
            tokenizer.next_token().unwrap(),
            Token::Function(FunctionName::Sin)
        );
    }

    #[test]
    fn test_tokenize_operators() {
        let mut tokenizer = Tokenizer::new("+ - * / ^");
        assert_eq!(
            tokenizer.next_token().unwrap(),
            Token::BinaryOp(BinaryOperator::Add)
        );
        assert_eq!(
            tokenizer.next_token().unwrap(),
            Token::BinaryOp(BinaryOperator::Subtract)
        );
        assert_eq!(
            tokenizer.next_token().unwrap(),
            Token::BinaryOp(BinaryOperator::Multiply)
        );
        assert_eq!(
            tokenizer.next_token().unwrap(),
            Token::BinaryOp(BinaryOperator::Divide)
        );
        assert_eq!(
            tokenizer.next_token().unwrap(),
            Token::BinaryOp(BinaryOperator::Power)
        );
    }

    #[test]
    fn test_tokenize_parentheses() {
        let mut tokenizer = Tokenizer::new("( )");
        assert_eq!(tokenizer.next_token().unwrap(), Token::LeftParen);
        assert_eq!(tokenizer.next_token().unwrap(), Token::RightParen);
    }

    #[test]
    fn test_tokenize_complex_expression() {
        let mut tokenizer = Tokenizer::new("3 + 4 * x");
        let tokens = tokenizer.tokenize().unwrap();
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0], Token::Number(3.0));
        assert_eq!(tokens[1], Token::BinaryOp(BinaryOperator::Add));
        assert_eq!(tokens[2], Token::Number(4.0));
        assert_eq!(tokens[3], Token::BinaryOp(BinaryOperator::Multiply));
        assert_eq!(tokens[4], Token::Variable("x".to_string()));
    }

    #[test]
    fn test_tokenize_function_call() {
        let mut tokenizer = Tokenizer::new("sqrt(16)");
        let tokens = tokenizer.tokenize().unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], Token::Function(FunctionName::Sqrt));
        assert_eq!(tokens[1], Token::LeftParen);
        assert_eq!(tokens[2], Token::Number(16.0));
    }

    #[test]
    fn test_tokenize_division_by_zero_as_operator_expression() {
        let mut tokenizer = Tokenizer::new("1/0");
        let tokens = tokenizer.tokenize().unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], Token::Number(1.0));
        assert_eq!(tokens[1], Token::BinaryOp(BinaryOperator::Divide));
        assert_eq!(tokens[2], Token::Number(0.0));
    }
}
