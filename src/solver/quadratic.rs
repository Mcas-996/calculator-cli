use crate::core::{ComplexNumber, Fraction};

/// Solve quadratic equation: ax² + bx + c = 0
pub fn solve_quadratic(coeffs: &[ComplexNumber]) -> Result<Vec<ComplexNumber>, String> {
    if coeffs.len() < 3 {
        return Err("Quadratic equation requires 3 coefficients (a, b, c)".to_string());
    }

    let a = &coeffs[0];
    let b = &coeffs[1];
    let c = &coeffs[2];

    if a == &ComplexNumber::from_real(Fraction::new(0, 1)) {
        // This is actually a linear equation
        return Err("Coefficient a cannot be zero for quadratic equation".to_string());
    }

    // Quadratic formula: x = (-b ± √(b²-4ac)) / 2a
    let discriminant = b.clone() * b.clone()
        - ComplexNumber::from_real(Fraction::new(4, 1)) * a.clone() * c.clone();
    let sqrt_discriminant = discriminant.sqrt();

    let x1 = (-b.clone() + sqrt_discriminant.clone())
        / (ComplexNumber::from_real(Fraction::new(2, 1)) * a.clone());
    let x2 = (-b.clone() - sqrt_discriminant)
        / (ComplexNumber::from_real(Fraction::new(2, 1)) * a.clone());

    // Check if solutions are the same
    if x1 == x2 {
        Ok(vec![x1])
    } else {
        Ok(vec![x1, x2])
    }
}

/// Parse and solve quadratic equation from string
pub fn solve_quadratic_equation(equation: &str) -> Result<Vec<ComplexNumber>, String> {
    let coeffs = parse_quadratic_coefficients(equation)?;
    solve_quadratic(&coeffs)
}

/// Parse coefficients from quadratic equation string
fn parse_quadratic_coefficients(equation: &str) -> Result<Vec<ComplexNumber>, String> {
    // Normalize equation to LHS = 0
    let normalized = crate::solver::normalize_equation(equation)?;
    let lhs = normalized.replace("²", "^2");

    let mut a = Fraction::new(0, 1);
    let mut b = Fraction::new(0, 1);
    let mut c = Fraction::new(0, 1);

    let mut current_sign = 1i64;
    let mut i = 0;

    while i < lhs.len() {
        let c_char = lhs.chars().nth(i).unwrap();

        if c_char == '+' {
            current_sign = 1;
            i += 1;
        } else if c_char == '-' {
            current_sign = -1;
            i += 1;
        } else {
            let mut term_end = i;
            while term_end < lhs.len() {
                let tc = lhs.chars().nth(term_end).unwrap();
                if tc == '+' || tc == '-' {
                    break;
                }
                term_end += 1;
            }

            let term = &lhs[i..term_end];

            if term.contains("^2") || term.contains("²") {
                let coef_str = term.replace("^2", "").replace("²", "").replace("*", "");
                let coef = if coef_str.is_empty() {
                    Fraction::new(1, 1)
                } else {
                    Fraction::from_double(coef_str.parse::<f64>().unwrap_or(1.0))
                };
                a = a + coef * Fraction::new(current_sign, 1);
            } else if term.contains('x') {
                let coef_str = term.replace('x', "").replace("*", "");
                let coef = if coef_str.is_empty() {
                    Fraction::new(1, 1)
                } else {
                    Fraction::from_double(coef_str.parse::<f64>().unwrap_or(1.0))
                };
                b = b + coef * Fraction::new(current_sign, 1);
            } else {
                let coef = Fraction::from_double(term.parse::<f64>().unwrap_or(0.0));
                c = c + coef * Fraction::new(current_sign, 1);
            }

            i = term_end;
        }
    }

    Ok(vec![
        ComplexNumber::from_real(a),
        ComplexNumber::from_real(b),
        ComplexNumber::from_real(c),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_quadratic_real_roots() {
        let coeffs = vec![
            ComplexNumber::from_real(Fraction::new(1, 1)),
            ComplexNumber::from_real(Fraction::new(-5, 1)),
            ComplexNumber::from_real(Fraction::new(6, 1)),
        ];
        let solutions = solve_quadratic(&coeffs).unwrap();
        assert_eq!(solutions.len(), 2);
    }

    #[test]
    fn test_solve_quadratic_complex_roots() {
        let coeffs = vec![
            ComplexNumber::from_real(Fraction::new(1, 1)),
            ComplexNumber::from_real(Fraction::new(0, 1)),
            ComplexNumber::from_real(Fraction::new(1, 1)),
        ];
        let solutions = solve_quadratic(&coeffs).unwrap();
        assert_eq!(solutions.len(), 2);
        assert!(!solutions[0].is_approximately_real());
    }

    #[test]
    fn test_solve_quadratic_equation() {
        let solutions = solve_quadratic_equation("x^2 - 5x + 6 = 0").unwrap();
        assert_eq!(solutions.len(), 2);
    }

    #[test]
    fn test_solve_quadratic_flexible_format() {
        let solutions = solve_quadratic_equation("x^2 = 5x - 6").unwrap();
        assert_eq!(solutions.len(), 2);

        let solutions2 = solve_quadratic_equation("2x^2 + 3x = 5").unwrap();
        assert_eq!(solutions2.len(), 2);
    }
}
