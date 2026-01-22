use crate::core::{ComplexNumber, Fraction};

/// Solve quintic or higher degree equation
/// Uses Durand-Kerner method for numeric approximation
pub fn solve_quintic(coeffs: &[ComplexNumber]) -> Result<Vec<ComplexNumber>, String> {
    if coeffs.len() < 6 {
        return Err("Quintic equation requires at least 6 coefficients".to_string());
    }

    let degree = coeffs.len() - 1;
    
    // Check leading coefficient
    if coeffs[0] == ComplexNumber::from_real(Fraction::new(0, 1)) {
        return Err("Leading coefficient cannot be zero".to_string());
    }

    // Use Durand-Kerner method (Weierstrass method)
    durand_kerner(coeffs, degree, 100, 1e-10)
}

/// Durand-Kerner method for finding polynomial roots
fn durand_kerner(
    coeffs: &[ComplexNumber],
    degree: usize,
    max_iterations: usize,
    tolerance: f64,
) -> Result<Vec<ComplexNumber>, String> {
    // Normalize coefficients
    let a0 = coeffs[0].clone();
    let normalized_coeffs: Vec<ComplexNumber> = coeffs.iter()
        .map(|c| c.clone() / a0.clone())
        .collect();

    // Initial guesses: roots of unity scaled
    let radius = 1.0 + normalized_coeffs[1..].iter()
        .map(|c| c.real.to_f64().abs())
        .fold(0.0_f64, |acc, x| acc.max(x));

    let mut roots: Vec<ComplexNumber> = (0..degree)
        .map(|k| {
            let angle = 2.0 * std::f64::consts::PI * k as f64 / degree as f64;
            ComplexNumber::from_doubles(
                radius * angle.cos(),
                radius * angle.sin(),
            )
        })
        .collect();

    // Iterate
    for _ in 0..max_iterations {
        let mut max_diff = 0.0_f64;
        let mut new_roots = roots.clone();

        for i in 0..degree {
            let mut numerator = ComplexNumber::from_real(Fraction::new(0, 1));
            
            // Evaluate polynomial at root[i]
            for (_j, coeff) in normalized_coeffs.iter().enumerate().rev() {
                numerator = numerator * roots[i].clone() + coeff.clone();
            }

            // Calculate denominator: product of (root[i] - root[j]) for j != i
            let mut denominator = ComplexNumber::from_real(Fraction::new(1, 1));
            for j in 0..degree {
                if i != j {
                    denominator = denominator * (roots[i].clone() - roots[j].clone());
                }
            }

            let correction = numerator / denominator;
            new_roots[i] = roots[i].clone() - correction.clone();

            let diff = (new_roots[i].real.to_f64() - roots[i].real.to_f64()).abs()
                + (new_roots[i].imag.to_f64() - roots[i].imag.to_f64()).abs();
            max_diff = max_diff.max(diff);
        }

        roots = new_roots;

        if max_diff < tolerance {
            break;
        }
    }

    Ok(roots)
}

/// Parse and solve quintic equation from string
pub fn solve_quintic_equation(equation: &str) -> Result<Vec<ComplexNumber>, String> {
    let coeffs = parse_quintic_coefficients(equation)?;
    solve_quintic(&coeffs)
}

/// Parse coefficients from quintic equation string
fn parse_quintic_coefficients(equation: &str) -> Result<Vec<ComplexNumber>, String> {
    let eq_lower = equation.to_lowercase();
    let eq = eq_lower.replace(" ", "")
        .replace("⁵", "^5")
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
    let mut coeffs = vec![Fraction::new(0, 1); 6];
    
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
            let degree = if term.contains("^5") || term.contains("⁵") {
                5
            } else if term.contains("^4") || term.contains("⁴") {
                4
            } else if term.contains("^3") || term.contains("³") {
                3
            } else if term.contains("^2") || term.contains("²") {
                2
            } else if term.contains('x') {
                1
            } else {
                0
            };
            
            let coef_str = term
                .replace("^5", "").replace("⁵", "")
                .replace("^4", "").replace("⁴", "")
                .replace("^3", "").replace("³", "")
                .replace("^2", "").replace("²", "")
                .replace('x', "").replace("*", "");
            
            let coef = if coef_str.is_empty() {
                Fraction::new(1, 1)
            } else {
                Fraction::from_double(coef_str.parse::<f64>().unwrap_or(1.0))
            };
            
            coeffs[degree] = coeffs[degree] + coef * Fraction::new(current_sign, 1);
            
            i = term_end;
        }
    }
    
    // Convert to ComplexNumber
    Ok(coeffs.iter().map(|f| ComplexNumber::from_real(*f)).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_quintic() {
        let coeffs = vec![
            ComplexNumber::from_real(Fraction::new(1, 1)),
            ComplexNumber::from_real(Fraction::new(-15, 1)),
            ComplexNumber::from_real(Fraction::new(85, 1)),
            ComplexNumber::from_real(Fraction::new(-225, 1)),
            ComplexNumber::from_real(Fraction::new(274, 1)),
            ComplexNumber::from_real(Fraction::new(-120, 1)),
        ];
        let solutions = solve_quintic(&coeffs).unwrap();
        assert_eq!(solutions.len(), 5);
    }
}