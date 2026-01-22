use calculator::solver::normalize_equation;

fn main() {
    println!("Testing linear coefficient parsing:");
    
    // Test what happens with equation(x=2)
    let equation_str = "equation(x=2)";
    println!("Input: {}", equation_str);
    
    match normalize_equation(equation_str) {
        Ok(normalized) => {
            println!("Normalized: {}", normalized);
            
            // Parse LHS to extract coefficients
            let lhs = normalized;
            let mut a = 0.0;
            let mut b = 0.0;
            
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
                    println!("  Term: '{}' with sign: {}", term, current_sign);
                    
                    if term.contains('x') {
                        let coef_str = term.replace('x', "").replace("*", "");
                        let coef = if coef_str.is_empty() {
                            1.0
                        } else {
                            coef_str.parse::<f64>().unwrap_or(1.0)
                        };
                        println!("    x coefficient: {}", coef);
                        a += coef * current_sign as f64;
                    } else {
                        let coef = term.parse::<f64>().unwrap_or(0.0);
                        println!("    constant coefficient: {}", coef);
                        b += coef * current_sign as f64;
                    }
                    
                    i = term_end;
                }
            }
            
            println!("  Final coefficients: a={}, b={}", a, b);
            println!("  Solution: x = -b/a = {}", if a != 0.0 { -b/a } else { 0.0 });
        }
        Err(e) => println!("Error: {}", e),
    }
}
