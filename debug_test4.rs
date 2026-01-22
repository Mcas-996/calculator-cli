use calculator::solver::solve_quadratic_equation;

fn main() {
    println!("Testing quadratic solver directly:");
    match solve_quadratic_equation("x^2+1=0") {
        Ok(solutions) => {
            println!("  Solutions: {}", solutions.len());
            for (i, sol) in solutions.iter().enumerate() {
                println!("    x{} = {:?}", i+1, sol);
            }
        }
        Err(e) => println!("  Error: {}", e),
    }
    
    println!("Testing linear solver directly:");
    match calculator::solver::solve_linear_equation("x=2") {
        Ok(solutions) => {
            println!("  Solutions: {}", solutions.len());
            for (i, sol) in solutions.iter().enumerate() {
                println!("    x{} = {:?}", i+1, sol);
            }
        }
        Err(e) => println!("  Error: {}", e),
    }
}
