use calculator::solver::normalize_equation;

fn parse_quadratic_debug(equation: &str) {
    println!("Parsing equation: {}", equation);
    let normalized = match normalize_equation(equation) {
        Ok(n) => {
            println!("  Normalized: {}", n);
            n
        }
        Err(e) => {
            println!("  Error normalizing: {}", e);
            return;
        }
    };
    
    let lhs = normalized.replace("²", "^2");
    println!("  LHS for parsing: {}", lhs);
    
    let mut a = 0.0;
    let mut b = 0.0;
    let mut c = 0.0;
    
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
            
            if term.contains("^2") || term.contains("²") {
                let coef_str = term.replace("^2", "").replace("²", "").replace("*", "");
                let coef = if coef_str.is_empty() {
                    1.0
                } else {
                    coef_str.parse::<f64>().unwrap_or(1.0)
                };
                println!("    x^2 coefficient: {}", coef);
                a += coef * current_sign as f64;
            } else if term.contains('x') {
                let coef_str = term.replace('x', "").replace("*", "");
                let coef = if coef_str.is_empty() {
                    1.0
                } else {
                    coef_str.parse::<f64>().unwrap_or(1.0)
                };
                println!("    x coefficient: {}", coef);
                b += coef * current_sign as f64;
            } else {
                let coef = term.parse::<f64>().unwrap_or(0.0);
                println!("    constant coefficient: {}", coef);
                c += coef * current_sign as f64;
            }
            
            i = term_end;
        }
    }
    
    println!("  Final coefficients: a={}, b={}, c={}", a, b, c);
}

fn main() {
    parse_quadratic_debug("x^2+1=0");
    parse_quadratic_debug("x=2");
}
