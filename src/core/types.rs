use crate::core::complex::ComplexNumber;
use crate::core::fraction::Fraction;
use std::fmt;

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    Add,      // +
    Subtract, // -
    Multiply, // *
    Divide,   // /
    Power,    // ^
    Modulo,   // %
}

impl BinaryOperator {
    /// Get operator precedence (higher number = higher precedence)
    pub fn precedence(&self) -> u8 {
        match self {
            BinaryOperator::Power => 3,
            BinaryOperator::Multiply | BinaryOperator::Divide | BinaryOperator::Modulo => 2,
            BinaryOperator::Add | BinaryOperator::Subtract => 1,
        }
    }

    /// Get operator symbol
    pub fn symbol(&self) -> &str {
        match self {
            BinaryOperator::Add => "+",
            BinaryOperator::Subtract => "-",
            BinaryOperator::Multiply => "*",
            BinaryOperator::Divide => "/",
            BinaryOperator::Power => "^",
            BinaryOperator::Modulo => "%",
        }
    }
}

/// Unary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    Negate, // -
}

impl UnaryOperator {
    /// Get operator symbol
    pub fn symbol(&self) -> &str {
        match self {
            UnaryOperator::Negate => "-",
        }
    }
}

/// Function names
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionName {
    Sqrt,
    Abs,
    Sin,
    Cos,
    Sind, // sin with degrees
    Cosd, // cos with degrees
}

impl FunctionName {
    /// Parse function name from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "sqrt" => Some(FunctionName::Sqrt),
            "abs" => Some(FunctionName::Abs),
            "sin" => Some(FunctionName::Sin),
            "cos" => Some(FunctionName::Cos),
            "sind" => Some(FunctionName::Sind),
            "cosd" => Some(FunctionName::Cosd),
            _ => None,
        }
    }

    /// Get function name as string
    pub fn as_str(&self) -> &str {
        match self {
            FunctionName::Sqrt => "sqrt",
            FunctionName::Abs => "abs",
            FunctionName::Sin => "sin",
            FunctionName::Cos => "cos",
            FunctionName::Sind => "sind",
            FunctionName::Cosd => "cosd",
        }
    }
}

/// Expression AST
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Constant(ComplexNumber),
    Variable(String),
    BinaryOp {
        left: Box<Expression>,
        op: BinaryOperator,
        right: Box<Expression>,
    },
    UnaryOp {
        op: UnaryOperator,
        operand: Box<Expression>,
    },
    Function {
        name: FunctionName,
        args: Vec<Expression>,
    },
}

impl Expression {
    /// Create a constant expression
    pub fn constant(value: ComplexNumber) -> Self {
        Expression::Constant(value)
    }

    /// Create a variable expression
    pub fn variable(name: String) -> Self {
        Expression::Variable(name)
    }

    /// Create a binary operation
    pub fn binary_op(left: Expression, op: BinaryOperator, right: Expression) -> Self {
        Expression::BinaryOp {
            left: Box::new(left),
            op,
            right: Box::new(right),
        }
    }

    /// Create a unary operation
    pub fn unary_op(op: UnaryOperator, operand: Expression) -> Self {
        Expression::UnaryOp {
            op,
            operand: Box::new(operand),
        }
    }

    /// Create a function call
    pub fn function(name: FunctionName, args: Vec<Expression>) -> Self {
        Expression::Function { name, args }
    }

    /// Evaluate expression to a complex number
    pub fn evaluate(&self) -> Result<ComplexNumber, String> {
        match self {
            Expression::Constant(value) => Ok(value.clone()),
            Expression::Variable(name) => match name.as_str() {
                "pi" => Ok(ComplexNumber::from_double(std::f64::consts::PI)),
                "e" => Ok(ComplexNumber::from_double(std::f64::consts::E)),
                "i" => Ok(ComplexNumber::new(Fraction::new(0, 1), Fraction::new(1, 1))),
                _ => Err(format!("Unknown variable: {}", name)),
            },
            Expression::BinaryOp { left, op, right } => {
                let left_val = left.evaluate()?;
                let right_val = right.evaluate()?;
                match op {
                    BinaryOperator::Add => Ok(left_val + right_val),
                    BinaryOperator::Subtract => Ok(left_val - right_val),
                    BinaryOperator::Multiply => Ok(left_val * right_val),
                    BinaryOperator::Divide => Ok(left_val / right_val),
                    BinaryOperator::Power => Ok(left_val.pow(right_val.real.to_f64() as i32)),
                    BinaryOperator::Modulo => {
                        if left_val.imag != Fraction::new(0, 1)
                            || right_val.imag != Fraction::new(0, 1)
                        {
                            return Err(
                                "Modulo operation not supported for complex numbers".to_string()
                            );
                        }
                        let left_f = left_val.real.to_f64();
                        let right_f = right_val.real.to_f64();
                        if right_f == 0.0 {
                            return Err("Division by zero in modulo".to_string());
                        }
                        Ok(ComplexNumber::from_double(left_f % right_f))
                    }
                }
            }
            Expression::UnaryOp { op, operand } => {
                let val = operand.evaluate()?;
                match op {
                    UnaryOperator::Negate => Ok(-val),
                }
            }
            Expression::Function { name, args } => {
                if args.len() != 1 {
                    return Err(format!(
                        "Function {} expects 1 argument, got {}",
                        name.as_str(),
                        args.len()
                    ));
                }
                let arg = args[0].evaluate()?;
                match name {
                    FunctionName::Sqrt => Ok(arg.sqrt()),
                    FunctionName::Abs => {
                        let mag = (arg.real * arg.real + arg.imag * arg.imag).sqrt();
                        Ok(ComplexNumber::from_real(mag))
                    }
                    FunctionName::Sin => Ok(arg.sin()),
                    FunctionName::Cos => Ok(arg.cos()),
                    FunctionName::Sind => {
                        // Convert degrees to radians
                        let rad = arg * ComplexNumber::from_double(std::f64::consts::PI / 180.0);
                        Ok(rad.sin())
                    }
                    FunctionName::Cosd => {
                        // Convert degrees to radians
                        let rad = arg * ComplexNumber::from_double(std::f64::consts::PI / 180.0);
                        Ok(rad.cos())
                    }
                }
            }
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Constant(value) => write!(f, "{}", value.to_string()),
            Expression::Variable(name) => write!(f, "{}", name),
            Expression::BinaryOp { left, op, right } => {
                write!(f, "({} {} {})", left, op.symbol(), right)
            }
            Expression::UnaryOp { op, operand } => {
                write!(f, "({}{})", op.symbol(), operand)
            }
            Expression::Function { name, args } => {
                write!(
                    f,
                    "{}({})",
                    name.as_str(),
                    args.iter()
                        .map(|a| a.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operator_precedence() {
        assert!(BinaryOperator::Power.precedence() > BinaryOperator::Multiply.precedence());
        assert!(BinaryOperator::Multiply.precedence() > BinaryOperator::Add.precedence());
    }

    #[test]
    fn test_constant_evaluation() {
        let expr = Expression::constant(ComplexNumber::from_double(5.0));
        assert_eq!(expr.evaluate().unwrap().real, Fraction::new(5, 1));
    }

    #[test]
    fn test_variable_evaluation() {
        let expr = Expression::variable("pi".to_string());
        let result = expr.evaluate().unwrap();
        assert!((result.real.to_f64() - std::f64::consts::PI).abs() < 1e-10);
    }

    #[test]
    fn test_binary_operation() {
        let left = Expression::constant(ComplexNumber::from_double(3.0));
        let right = Expression::constant(ComplexNumber::from_double(2.0));
        let expr = Expression::binary_op(left, BinaryOperator::Add, right);
        assert_eq!(expr.evaluate().unwrap().real, Fraction::new(5, 1));
    }

    #[test]
    fn test_function_evaluation() {
        let arg = Expression::constant(ComplexNumber::from_double(0.0));
        let expr = Expression::function(FunctionName::Sin, vec![arg]);
        let result = expr.evaluate().unwrap();
        assert!((result.real.to_f64() - 0.0).abs() < 1e-10);
    }
}
