use crate::core::{ComplexNumber, Fraction};

/// Solve quartic equation: ax⁴ + bx³ + cx² + dx + e = 0
/// Uses Ferrari's formula
pub fn solve_quartic(coeffs: &[ComplexNumber]) -> Result<Vec<ComplexNumber>, String> {
    if coeffs.len() < 5 {
        return Err("Quartic equation requires 5 coefficients (a, b, c, d, e)".to_string());
    }

    let a = &coeffs[0];
    let b = &coeffs[1];
    let c = &coeffs[2];
    let d = &coeffs[3];
    let e = &coeffs[4];

    if a == &ComplexNumber::from_real(Fraction::new(0, 1)) {
        return Err("Coefficient a cannot be zero for quartic equation".to_string());
    }

    // Convert to depressed quartic: y⁴ + py² + qy + r = 0
    let a_val = a.real.to_f64();
    let b_val = b.real.to_f64();
    let c_val = c.real.to_f64();
    let d_val = d.real.to_f64();
    let e_val = e.real.to_f64();

    let p = (8.0 * a_val * c_val - 3.0 * b_val * b_val) / (8.0 * a_val * a_val);
    let q = (b_val * b_val * b_val - 4.0 * a_val * b_val * c_val + 8.0 * a_val * a_val * d_val) 
            / (8.0 * a_val * a_val * a_val);
    let r = (-3.0 * b_val.powi(4) + 256.0 * a_val.powi(3) * e_val - 64.0 * a_val * a_val * b_val * d_val 
            + 16.0 * a_val * b_val * b_val * c_val - 16.0 * a_val * a_val * c_val * c_val) 
            / (256.0 * a_val.powi(4));

    // Solve the resolvent cubic
    let resolvent_coeffs = vec![
        ComplexNumber::from_real(Fraction::new(1, 1)),
        ComplexNumber::from_real(Fraction::from_double(2.0 * p)),
        ComplexNumber::from_real(Fraction::from_double(p * p - 4.0 * r)),
        ComplexNumber::from_real(Fraction::from_double(-q * q)),
    ];

    let m_roots = crate::solver::cubic::solve_cubic(&resolvent_coeffs)?;
    
    // Choose a real, non-negative root m
    let m = m_roots.iter()
        .find(|r| r.is_approximately_real() && r.real.to_f64() >= 0.0)
        .unwrap_or(&m_roots[0])
        .real.to_f64();

    let sqrt_m = m.sqrt();

    // Solve two quadratic equations
    let _sqrt_2m = (2.0 * m).sqrt();

    let quad1_coeffs = vec![
        ComplexNumber::from_real(Fraction::new(1, 1)),
        ComplexNumber::from_real(Fraction::from_double(sqrt_m)),
        ComplexNumber::from_real(Fraction::from_double((p + m) / 2.0 - q / (2.0 * sqrt_m))),
    ];

    let quad2_coeffs = vec![
        ComplexNumber::from_real(Fraction::new(1, 1)),
        ComplexNumber::from_real(Fraction::from_double(-sqrt_m)),
        ComplexNumber::from_real(Fraction::from_double((p + m) / 2.0 + q / (2.0 * sqrt_m))),
    ];

    let mut roots = Vec::new();
    roots.extend(crate::solver::quadratic::solve_quadratic(&quad1_coeffs)?);
    roots.extend(crate::solver::quadratic::solve_quadratic(&quad2_coeffs)?);

    // Adjust roots back to original variable
    let shift = b_val / (4.0 * a_val);
    for root in roots.iter_mut() {
        *root = root.clone() - ComplexNumber::from_real(Fraction::from_double(shift));
    }

    Ok(roots)
}

/// Parse and solve quartic equation from string
pub fn solve_quartic_equation(equation: &str) -> Result<Vec<ComplexNumber>, String> {
    let coeffs = parse_quartic_coefficients(equation)?;
    solve_quartic(&coeffs)
}

/// Parse coefficients from quartic equation string
fn parse_quartic_coefficients(equation: &str) -> Result<Vec<ComplexNumber>, String> {
    let eq_lower = equation.to_lowercase();
    let eq = eq_lower.replace(" ", "")
        .replace("⁴", "^4")
        .replace("³", "^3")
        .replace("²", "^2");
    
    let parts: Vec<&str> = eq.split('=').collect();
    if parts.len() != 2 {
        return Err("Invalid equation format".to_string());
    }
    
    if parts[1] != "0" {
        return Err("Only equations in the form '... = 0' are supported".to_string());
    }
    
    let lhs = parts[0];
    let mut a = Fraction::new(0, 1);
    let mut b = Fraction::new(0, 1);
    let mut c = Fraction::new(0, 1);
    let mut d = Fraction::new(0, 1);
    let mut e = Fraction::new(0, 1);
    
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
            
            if term.contains("^4") || term.contains("⁴") {
                let coef_str = term.replace("^4", "").replace("⁴", "").replace("*", "");
                let coef = if coef_str.is_empty() {
                    Fraction::new(1, 1)
                } else {
                    Fraction::from_double(coef_str.parse::<f64>().unwrap_or(1.0))
                };
                a = a + coef * Fraction::new(current_sign, 1);
            } else if term.contains("^3") || term.contains("³") {
                let coef_str = term.replace("^3", "").replace("³", "").replace("*", "");
                let coef = if coef_str.is_empty() {
                    Fraction::new(1, 1)
                } else {
                    Fraction::from_double(coef_str.parse::<f64>().unwrap_or(1.0))
                };
                b = b + coef * Fraction::new(current_sign, 1);
            } else if term.contains("^2") || term.contains("²") {
                let coef_str = term.replace("^2", "").replace("²", "").replace("*", "");
                let coef = if coef_str.is_empty() {
                    Fraction::new(1, 1)
                } else {
                    Fraction::from_double(coef_str.parse::<f64>().unwrap_or(1.0))
                };
                c = c + coef * Fraction::new(current_sign, 1);
            } else if term.contains('x') {
                let coef_str = term.replace('x', "").replace("*", "");
                let coef = if coef_str.is_empty() {
                    Fraction::new(1, 1)
                } else {
                    Fraction::from_double(coef_str.parse::<f64>().unwrap_or(1.0))
                };
                d = d + coef * Fraction::new(current_sign, 1);
            } else {
                let coef = Fraction::from_double(term.parse::<f64>().unwrap_or(0.0));
                e = e + coef * Fraction::new(current_sign, 1);
            }
            
            i = term_end;
        }
    }
    
    Ok(vec![
        ComplexNumber::from_real(a),
        ComplexNumber::from_real(b),
        ComplexNumber::from_real(c),
        ComplexNumber::from_real(d),
        ComplexNumber::from_real(e),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_quartic() {
        let coeffs = vec![
            ComplexNumber::from_real(Fraction::new(1, 1)),
            ComplexNumber::from_real(Fraction::new(-10, 1)),
            ComplexNumber::from_real(Fraction::new(35, 1)),
            ComplexNumber::from_real(Fraction::new(-50, 1)),
            ComplexNumber::from_real(Fraction::new(24, 1)),
        ];
        let solutions = solve_quartic(&coeffs).unwrap();
        assert_eq!(solutions.len(), 4);
    }
}