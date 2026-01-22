use crate::core::{ComplexNumber, Fraction};

/// Solve linear system using Gaussian elimination
pub fn solve_linear_system(equations: &[String]) -> Result<Vec<(String, ComplexNumber)>, String> {
    if equations.is_empty() {
        return Err("No equations provided".to_string());
    }

    // Determine system size
    let n = equations.len();
    
    // Parse equations into augmented matrix
    let mut matrix = parse_system_equations(equations, n)?;
    
    // Gaussian elimination
    for i in 0..n {
        // Find pivot
        let mut pivot_row = i;
        for j in (i + 1)..n {
            if matrix[j][i].real.to_f64().abs() > matrix[pivot_row][i].real.to_f64().abs() {
                pivot_row = j;
            }
        }
        
        // Swap rows
        if pivot_row != i {
            matrix.swap(i, pivot_row);
        }
        
        // Check for singular matrix
        if matrix[i][i] == ComplexNumber::from_real(Fraction::new(0, 1)) {
            return Err("Singular matrix - no unique solution".to_string());
        }
        
        // Eliminate below
        for j in (i + 1)..n {
            let factor = matrix[j][i].clone() / matrix[i][i].clone();
            for k in i..(n + 1) {
                matrix[j][k] = matrix[j][k].clone() - factor.clone() * matrix[i][k].clone();
            }
        }
    }
    
    // Back substitution
    let mut solutions = vec![ComplexNumber::from_real(Fraction::new(0, 1)); n];
    for i in (0..n).rev() {
        let mut sum = ComplexNumber::from_real(Fraction::new(0, 1));
        for j in (i + 1)..n {
            sum = sum + matrix[i][j].clone() * solutions[j].clone();
        }
        solutions[i] = (matrix[i][n].clone() - sum) / matrix[i][i].clone();
    }
    
    // Generate variable names
    let vars: Vec<String> = (0..n)
        .map(|i| format!("x{}", i + 1))
        .collect();
    
    Ok(vars.into_iter().zip(solutions).collect())
}

/// Parse system equations into augmented matrix
fn parse_system_equations(equations: &[String], n: usize) -> Result<Vec<Vec<ComplexNumber>>, String> {
    let mut matrix = vec![vec![ComplexNumber::from_real(Fraction::new(0, 1)); n + 1]; n];
    
    for (row, eq) in equations.iter().enumerate() {
        let eq_lower = eq.to_lowercase();
        let eq = eq_lower.replace(" ", "");
        
        // Split by "="
        let parts: Vec<&str> = eq.split('=').collect();
        if parts.len() != 2 {
            return Err(format!("Invalid equation format in equation {}", row + 1));
        }
        
        let lhs = parts[0];
        let rhs = parts[1];
        
        // Parse LHS coefficients
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
                
                // Find variable index
                let var_idx = if term.contains("x3") || term.contains("z") {
                    2
                } else if term.contains("x2") || term.contains("y") {
                    1
                } else if term.contains("x1") || term.contains("x") {
                    0
                } else {
                    return Err(format!("Unknown variable in term: {}", term));
                };
                
                let coef_str = term
                    .replace("x3", "").replace("z", "")
                    .replace("x2", "").replace("y", "")
                    .replace("x1", "").replace("x", "")
                    .replace("*", "");
                
                let coef = if coef_str.is_empty() {
                    Fraction::new(1, 1)
                } else {
                    Fraction::from_double(coef_str.parse::<f64>().unwrap_or(1.0))
                };
                
                matrix[row][var_idx] = matrix[row][var_idx].clone()
                    + ComplexNumber::from_real(coef * Fraction::new(current_sign, 1));
                
                i = term_end;
            }
        }
        
        // Parse RHS
        let rhs_val = rhs.parse::<f64>().unwrap_or(0.0);
        matrix[row][n] = ComplexNumber::from_real(Fraction::from_double(rhs_val));
    }
    
    Ok(matrix)
}

/// Solve 2x2 linear system
pub fn solve_2x2_system(equations: &[String]) -> Result<Vec<(String, ComplexNumber)>, String> {
    if equations.len() != 2 {
        return Err("2x2 system requires exactly 2 equations".to_string());
    }
    solve_linear_system(equations)
}

/// Solve 3x3 linear system
pub fn solve_3x3_system(equations: &[String]) -> Result<Vec<(String, ComplexNumber)>, String> {
    if equations.len() != 3 {
        return Err("3x3 system requires exactly 3 equations".to_string());
    }
    solve_linear_system(equations)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_2x2_system() {
        let equations = vec![
            "x + y = 5".to_string(),
            "x - y = 1".to_string(),
        ];
        let solutions = solve_2x2_system(&equations).unwrap();
        assert_eq!(solutions.len(), 2);
    }

    #[test]
    fn test_solve_3x3_system() {
        let equations = vec![
            "x + y + z = 6".to_string(),
            "x - y + z = 2".to_string(),
            "2x + y - z = 3".to_string(),
        ];
        let solutions = solve_3x3_system(&equations).unwrap();
        assert_eq!(solutions.len(), 3);
    }
}