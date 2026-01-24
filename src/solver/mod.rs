pub mod cubic;
pub mod linear;
pub mod linear_system;
pub mod quadratic;
pub mod quartic;
pub mod quintic;

pub use cubic::{solve_cubic, solve_cubic_equation};
pub use linear::{solve_linear, solve_linear_equation};
pub use linear_system::{solve_2x2_system, solve_3x3_system, solve_linear_system};
pub use quadratic::{solve_quadratic, solve_quadratic_equation};
pub use quartic::{solve_quartic, solve_quartic_equation};
pub use quintic::{solve_quintic, solve_quintic_equation};

/// Normalize an equation string to standard form (LHS = 0)
/// Support equations like "x = 5", "2x + 3 = 7" by moving all terms to the left side
pub fn normalize_equation(equation: &str) -> Result<String, String> {
    let eq = equation.to_lowercase().replace(" ", "");

    let parts: Vec<&str> = eq.split('=').collect();
    if parts.len() != 2 {
        return Err("Invalid equation format: must contain exactly one '='".to_string());
    }

    let lhs = parts[0];
    let rhs = parts[1];

    if rhs == "0" {
        return Ok(lhs.to_string());
    }

    // Normalize by negating terms on RHS and appending to LHS
    let mut normalized = lhs.to_string();
    let mut i = 0;
    let rhs_chars: Vec<char> = rhs.chars().collect();

    // If RHS starts with a term without sign, it needs a '-'
    if i < rhs_chars.len() && rhs_chars[i] != '+' && rhs_chars[i] != '-' {
        normalized.push('-');
    }

    while i < rhs_chars.len() {
        let c = rhs_chars[i];
        if c == '+' {
            normalized.push('-');
        } else if c == '-' {
            normalized.push('+');
        } else {
            normalized.push(c);
        }
        i += 1;
    }

    Ok(normalized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_equation() {
        assert_eq!(normalize_equation("x = 5").unwrap(), "x-5");
        assert_eq!(normalize_equation("2x = 6").unwrap(), "2x-6");
        assert_eq!(normalize_equation("x + 5 = 10").unwrap(), "x+5-10");
        assert_eq!(normalize_equation("2x + 3 = 7").unwrap(), "2x+3-7");
        assert_eq!(normalize_equation("x - 5 = 0").unwrap(), "x-5");
        assert_eq!(normalize_equation("x^2 = 4").unwrap(), "x^2-4");
        assert_eq!(normalize_equation("x = x").unwrap(), "x-x");
    }
}
