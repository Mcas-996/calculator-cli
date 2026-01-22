fn main() {
    let input1 = "x=2";
    let input2 = "equation(x=2)";
    
    println!("Testing equation detection:");
    println!("{} contains '=': {}", input1, input1.contains('='));
    println!("{} contains '=': {}", input2, input2.contains('='));
    
    println!("\nDegree determination:");
    println!("{} -> degree: {}", input1, determine_degree(input1));
    println!("{} -> degree: {}", input2, determine_degree(input2));
}

fn determine_degree(input: &str) -> usize {
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
