use crate::core::{ComplexNumber, Fraction};

/// Solve linear equation: ax + b = 0
pub fn solve_linear(coeffs: &[ComplexNumber]) -> Result<Vec<ComplexNumber>, String> {
    if coeffs.len() < 2 {
        return Err("Linear equation requires at least 2 coefficients (a, b)".to_string());
    }

    let a = &coeffs[0];
    let b = &coeffs[1];

    if a == &ComplexNumber::from_real(Fraction::new(0, 1)) {
        if b == &ComplexNumber::from_real(Fraction::new(0, 1)) {
            return Err("Infinite solutions (0x = 0)".to_string());
        } else {
            return Err("No solution (0x = b where b ≠ 0)".to_string());
        }
    }

    let x = -b.clone() / a.clone();
    Ok(vec![x])
}

/// Parse and solve linear equation from string
pub fn solve_linear_equation(equation: &str) -> Result<Vec<ComplexNumber>, String> {
    // Parse the equation to extract coefficients
    let coeffs = parse_linear_coefficients(equation)?;
    solve_linear(&coeffs)
}

/// Parse coefficients from linear equation string
fn parse_linear_coefficients(equation: &str) -> Result<Vec<ComplexNumber>, String> {
    // Simplified parsing: look for pattern like "ax + b = 0"
    // This is a basic implementation - a full parser would be more sophisticated
    
    let eq_lower = equation.to_lowercase();
    
    // Remove spaces and normalize
    let eq = eq_lower.replace(" ", "");
    
    // Split by "="
    let parts: Vec<&str> = eq.split('=').collect();
    if parts.len() != 2 {
        return Err("Invalid equation format".to_string());
    }
    
    // Check if RHS is 0
    if parts[1] != "0" {
        return Err("Only equations in the form '... = 0' are supported".to_string());
    }
    
    // Parse LHS to extract coefficients
    let lhs = parts[0];
    
    // Look for terms with x and constant term
    let mut a = Fraction::new(0, 1);
    let mut b = Fraction::new(0, 1);
    
    // Split by + or -
    let mut current_sign = 1i64;
    let mut i = 0;
    
    while i < lhs.len() {
        let c = lhs.chars().nth(i).unwrap();
        
        if c == '+' {
            current_sign = 1;
            i += 1;
        } else if c == '-' {
            current_sign = -1;
            i += 1;
        } else {
            // Parse term
            let mut term_end = i;
            while term_end < lhs.len() {
                let tc = lhs.chars().nth(term_end).unwrap();
                if tc == '+' || tc == '-' {
                    break;
                }
                term_end += 1;
            }
            
            let term = &lhs[i..term_end];
            
            if term.contains('x') {
                // This is the 'a' coefficient
                let coef_str = term.replace('x', "").replace("*", "");
                let coef = if coef_str.is_empty() {
                    Fraction::new(1, 1)
                } else {
                    Fraction::from_double(coef_str.parse::<f64>().unwrap_or(1.0))
                };
                a = a + coef * Fraction::new(current_sign, 1);
            } else {
                // This is the 'b' constant
                let coef = Fraction::from_double(term.parse::<f64>().unwrap_or(0.0));
                b = b + coef * Fraction::new(current_sign, 1);
            }
            
            i = term_end;
        }
    }
    
    Ok(vec![
        ComplexNumber::from_real(a),
        ComplexNumber::from_real(b),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_linear() {
        let coeffs = vec![
            ComplexNumber::from_real(Fraction::new(2, 1)),
            ComplexNumber::from_real(Fraction::new(-6, 1)),
        ];
        let solutions = solve_linear(&coeffs).unwrap();
        assert_eq!(solutions.len(), 1);
        assert!((solutions[0].real.to_f64() - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_solve_linear_equation() {
        let solutions = solve_linear_equation("2x - 6 = 0").unwrap();
        assert_eq!(solutions.len(), 1);
        assert!((solutions[0].real.to_f64() - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_solve_linear_no_solution() {
        let coeffs = vec![
            ComplexNumber::from_real(Fraction::new(0, 1)),
            ComplexNumber::from_real(Fraction::new(5, 1)),
        ];
        let result = solve_linear(&coeffs);
        assert!(result.is_err());
    }
}