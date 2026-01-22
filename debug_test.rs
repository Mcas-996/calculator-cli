use calculator::solver::normalize_equation;

fn main() {
    println!("Testing normalize_equation:");
    println!("Input: x^2+1=0");
    match normalize_equation("x^2+1=0") {
        Ok(normalized) => println!("Normalized: {}", normalized),
        Err(e) => println!("Error: {}", e),
    }
    
    println!("Input: x=2");
    match normalize_equation("x=2") {
        Ok(normalized) => println!("Normalized: {}", normalized),
        Err(e) => println!("Error: {}", e),
    }
}
