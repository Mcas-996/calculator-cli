fn determine_equation_degree(input: &str) -> usize {
    let input_lower = input.to_lowercase();

    if input_lower.contains("^5") || input_lower.contains("⁵") {
        5
    } else if input_lower.contains("^4") || input_lower.contains("⁴") {
        4
    } else if input_lower.contains("^3") || input_lower.contains("³") {
        3
    } else if input_lower.contains("^2") || input_lower.contains("²") {
        2
    } else if input_lower.contains('x') {
        1
    } else {
        0
    }
}

fn main() {
    println!("Degree determination:");
    println!("x^2+1=0 -> {}", determine_equation_degree("x^2+1=0"));
    println!("x=2 -> {}", determine_equation_degree("x=2"));
    println!("equation(x=2) -> {}", determine_equation_degree("equation(x=2)"));
}
