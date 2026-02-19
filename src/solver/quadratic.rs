use crate::core::{ComplexNumber, Fraction};

/// Solve quadratic equation: ax² + bx + c = 0, with option for exact radical representation
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
    let (a, b, c) = parse_canonical_quadratic_coefficients(equation)?;

    Ok(vec![
        ComplexNumber::from_real(a),
        ComplexNumber::from_real(b),
        ComplexNumber::from_real(c),
    ])
}

fn parse_canonical_quadratic_coefficients(
    equation: &str,
) -> Result<(Fraction, Fraction, Fraction), String> {
    let normalized = crate::solver::normalize_equation(equation)?;
    let canonical = normalized
        .replace("²", "^2")
        .replace('−', "-")
        .replace('–', "-")
        .replace('—', "-")
        .replace('*', "")
        .replace(' ', "");

    let mut a = Fraction::new(0, 1);
    let mut b = Fraction::new(0, 1);
    let mut c = Fraction::new(0, 1);

    let mut terms = canonical.replace('-', "+-");
    if terms.starts_with("+-") {
        terms.remove(0);
    } else if terms.starts_with('+') {
        terms.remove(0);
    }

    for term in terms.split('+').filter(|term| !term.is_empty()) {
        if term.contains("x^2") {
            let coef = parse_term_coefficient(&term.replace("x^2", ""), true)?;
            a = a + coef;
        } else if term.contains('x') {
            let coef = parse_term_coefficient(&term.replace('x', ""), true)?;
            b = b + coef;
        } else {
            let coef = parse_term_coefficient(term, false)?;
            c = c + coef;
        }
    }

    Ok((a, b, c))
}

fn parse_term_coefficient(raw: &str, implicit_one_allowed: bool) -> Result<Fraction, String> {
    if implicit_one_allowed {
        if raw.is_empty() || raw == "+" {
            return Ok(Fraction::new(1, 1));
        }
        if raw == "-" {
            return Ok(Fraction::new(-1, 1));
        }
    }

    raw.parse::<f64>()
        .map(Fraction::from_double)
        .map_err(|_| format!("Invalid coefficient: {}", raw))
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

    #[test]
    fn test_quadratic_regression_repeated_root() {
        let solutions = solve_quadratic_equation("x^2 + 2x + 1 = 0").unwrap();
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions[0].real, Fraction::new(-1, 1));
        assert_eq!(solutions[0].imag, Fraction::new(0, 1));
    }

    #[test]
    fn test_quadratic_regression_complex_conjugates() {
        let solutions = solve_quadratic_equation("x^2 + 2x + 10 = 0").unwrap();
        assert_eq!(solutions.len(), 2);
        assert!(solutions
            .iter()
            .any(|sol| { sol.real == Fraction::new(-1, 1) && sol.imag == Fraction::new(3, 1) }));
        assert!(solutions
            .iter()
            .any(|sol| { sol.real == Fraction::new(-1, 1) && sol.imag == Fraction::new(-3, 1) }));
    }

    #[test]
    fn test_quadratic_canonical_coefficients_standard_form() {
        let (a, b, c) = parse_canonical_quadratic_coefficients("x^2 + 2x + 1 = 0").unwrap();
        assert_eq!(a, Fraction::new(1, 1));
        assert_eq!(b, Fraction::new(2, 1));
        assert_eq!(c, Fraction::new(1, 1));
    }

    #[test]
    fn test_quadratic_canonical_coefficients_non_zero_rhs() {
        let (a, b, c) = parse_canonical_quadratic_coefficients("x^2 + 2x = -1").unwrap();
        assert_eq!(a, Fraction::new(1, 1));
        assert_eq!(b, Fraction::new(2, 1));
        assert_eq!(c, Fraction::new(1, 1));
    }
}
