use crate::core::{ComplexNumber, Fraction};

/// Solve cubic equation: ax³ + bx² + cx + d = 0
/// Uses Cardano's formula
pub fn solve_cubic(coeffs: &[ComplexNumber]) -> Result<Vec<ComplexNumber>, String> {
    if coeffs.len() < 4 {
        return Err("Cubic equation requires 4 coefficients (a, b, c, d)".to_string());
    }

    let a = &coeffs[0];
    let b = &coeffs[1];
    let c = &coeffs[2];
    let d = &coeffs[3];

    if a == &ComplexNumber::from_real(Fraction::new(0, 1)) {
        return Err("Coefficient a cannot be zero for cubic equation".to_string());
    }

    // Convert to depressed cubic: t³ + pt + q = 0
    // Using substitution x = t - b/(3a)

    let a_val = a.real.to_f64();
    let b_val = b.real.to_f64();
    let c_val = c.real.to_f64();
    let d_val = d.real.to_f64();

    let p = (3.0 * a_val * c_val - b_val * b_val) / (3.0 * a_val * a_val);
    let q = (2.0 * b_val * b_val * b_val - 9.0 * a_val * b_val * c_val
        + 27.0 * a_val * a_val * d_val)
        / (27.0 * a_val * a_val * a_val);

    let discriminant = (q * q / 4.0) + (p * p * p / 27.0);

    let mut roots = Vec::new();

    if discriminant.abs() < 1e-10 {
        // One real root (triple) or two real roots (one double)
        let u = -q / 2.0;
        let t = cbrt(u);

        roots.push(ComplexNumber::from_double(t - b_val / (3.0 * a_val)));

        if p.abs() < 1e-10 {
            // Triple root
            roots.push(ComplexNumber::from_double(t - b_val / (3.0 * a_val)));
            roots.push(ComplexNumber::from_double(t - b_val / (3.0 * a_val)));
        } else {
            // Double root
            roots.push(ComplexNumber::from_double(t - b_val / (3.0 * a_val)));
            roots.push(ComplexNumber::from_double(-2.0 * t - b_val / (3.0 * a_val)));
        }
    } else if discriminant > 0.0 {
        // One real root, two complex conjugate roots
        let sqrt_disc = discriminant.sqrt();
        let u = cbrt(-q / 2.0 + sqrt_disc);
        let v = cbrt(-q / 2.0 - sqrt_disc);

        let t1 = u + v;
        let real_part = -t1 / 2.0;
        let imag_part = (3.0_f64).sqrt() * (u - v) / 2.0;

        roots.push(ComplexNumber::from_double(t1 - b_val / (3.0 * a_val)));
        roots.push(ComplexNumber::from_doubles(
            real_part - b_val / (3.0 * a_val),
            imag_part,
        ));
        roots.push(ComplexNumber::from_doubles(
            real_part - b_val / (3.0 * a_val),
            -imag_part,
        ));
    } else {
        // Three real roots
        let phi = (-q / 2.0).atan2((-discriminant).sqrt());
        let r = (-p / 3.0).sqrt();

        for k in 0..3 {
            let theta = (phi + 2.0 * std::f64::consts::PI * k as f64) / 3.0;
            let t = 2.0 * r * theta.cos();
            roots.push(ComplexNumber::from_double(t - b_val / (3.0 * a_val)));
        }
    }

    Ok(roots)
}

/// Cube root function that handles negative numbers
fn cbrt(x: f64) -> f64 {
    if x >= 0.0 {
        x.powf(1.0 / 3.0)
    } else {
        -(-x).powf(1.0 / 3.0)
    }
}

/// Parse and solve cubic equation from string
pub fn solve_cubic_equation(equation: &str) -> Result<Vec<ComplexNumber>, String> {
    let coeffs = parse_cubic_coefficients(equation)?;
    solve_cubic(&coeffs)
}

/// Parse coefficients from cubic equation string
fn parse_cubic_coefficients(equation: &str) -> Result<Vec<ComplexNumber>, String> {
    // Normalize equation to LHS = 0
    let normalized = crate::solver::normalize_equation(equation)?;
    let lhs = normalized.replace("³", "^3").replace("²", "^2");

    let mut a = Fraction::new(0, 1);
    let mut b = Fraction::new(0, 1);
    let mut c = Fraction::new(0, 1);
    let mut d = Fraction::new(0, 1);

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

            if term.contains("^3") || term.contains("³") {
                let coef_str = term.replace("^3", "").replace("³", "").replace("*", "");
                let coef = if coef_str.is_empty() {
                    Fraction::new(1, 1)
                } else {
                    Fraction::from_double(coef_str.parse::<f64>().unwrap_or(1.0))
                };
                a = a + coef * Fraction::new(current_sign, 1);
            } else if term.contains("^2") || term.contains("²") {
                let coef_str = term.replace("^2", "").replace("²", "").replace("*", "");
                let coef = if coef_str.is_empty() {
                    Fraction::new(1, 1)
                } else {
                    Fraction::from_double(coef_str.parse::<f64>().unwrap_or(1.0))
                };
                b = b + coef * Fraction::new(current_sign, 1);
            } else if term.contains('x') {
                let coef_str = term.replace('x', "").replace("*", "");
                let coef = if coef_str.is_empty() {
                    Fraction::new(1, 1)
                } else {
                    Fraction::from_double(coef_str.parse::<f64>().unwrap_or(1.0))
                };
                c = c + coef * Fraction::new(current_sign, 1);
            } else {
                let coef = Fraction::from_double(term.parse::<f64>().unwrap_or(0.0));
                d = d + coef * Fraction::new(current_sign, 1);
            }

            i = term_end;
        }
    }

    Ok(vec![
        ComplexNumber::from_real(a),
        ComplexNumber::from_real(b),
        ComplexNumber::from_real(c),
        ComplexNumber::from_real(d),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_cubic() {
        let coeffs = vec![
            ComplexNumber::from_real(Fraction::new(1, 1)),
            ComplexNumber::from_real(Fraction::new(-6, 1)),
            ComplexNumber::from_real(Fraction::new(11, 1)),
            ComplexNumber::from_real(Fraction::new(-6, 1)),
        ];
        let solutions = solve_cubic(&coeffs).unwrap();
        assert_eq!(solutions.len(), 3);
    }

    #[test]
    fn test_solve_cubic_flexible_format() {
        let solutions = solve_cubic_equation("x^3 = 6x^2 - 11x + 6").unwrap();
        assert_eq!(solutions.len(), 3);

        let solutions2 = solve_cubic_equation("x^3 + 2x^2 = 3x + 2").unwrap();
        assert_eq!(solutions2.len(), 3);
    }
}
