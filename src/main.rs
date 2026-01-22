use clap::Parser;
use std::io::{self, Write};

use calculator::output::{PrettyConfig, PrettyLevel, Formatter};
use calculator::parser::parse_expression;
use calculator::solver::{
    solve_linear_equation, solve_quadratic_equation, solve_cubic_equation,
    solve_quartic_equation, solve_quintic_equation, solve_2x2_system, solve_3x3_system,
};

/// A command-line calculator with symbolic math support
#[derive(Parser, Debug)]
#[command(name = "calculator")]
#[command(author = "Allen <anomalyco@github.com>")]
#[command(version = "2.0.0")]
#[command(about = "A command-line calculator with symbolic math support", long_about = None)]
struct Cli {
    /// Enable pretty output
    #[arg(short = 'p', long = "pretty")]
    pretty: bool,

    /// Force Unicode output
    #[arg(short = 'u', long = "unicode")]
    unicode: bool,

    /// Force LaTeX output
    #[arg(short = 'l', long = "latex")]
    latex: bool,

    /// Force ASCII output
    #[arg(short = 'a', long = "ascii")]
    ascii: bool,

    /// Expression or equation to evaluate
    expression: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    // Configure output format
    let mut config = PrettyConfig::instance().clone();
    if cli.pretty {
        config.set_level(PrettyLevel::Unicode);
    } else if cli.unicode {
        config.set_level(PrettyLevel::Unicode);
    } else if cli.latex {
        config.set_level(PrettyLevel::Latex);
    } else if cli.ascii {
        config.set_level(PrettyLevel::Ascii);
    }

    let formatter = config.get_formatter();

    // Process expression or enter interactive mode
    if let Some(expr) = cli.expression {
        // Single expression mode
        process_expression(&expr, formatter.as_ref());
    } else {
        // Interactive mode
        run_interactive_mode(formatter.as_ref());
    }
}

/// Process a single expression or equation
fn process_expression(input: &str, formatter: &dyn Formatter) {
    let input = input.trim();

    // Check if it's an equation
    if input.contains('=') {
        process_equation(input, formatter);
    } else {
        // It's an expression
        match parse_expression(input) {
            Ok(expr) => {
                match expr.evaluate() {
                    Ok(result) => {
                        println!("{}", formatter.format_complex(&result));
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        std::process::exit(1);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}

/// Process an equation
fn process_equation(input: &str, formatter: &dyn Formatter) {
    // Check if it's a system of equations
    if input.contains(',') {
        let equations: Vec<String> = input.split(',')
            .map(|s| s.trim().to_string())
            .collect();

        if equations.len() == 2 {
            match solve_2x2_system(&equations) {
                Ok(solutions) => {
                    for (var, value) in solutions {
                        println!("{}", formatter.format_equation_solution(&var, &formatter.format_complex(&value)));
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        } else if equations.len() == 3 {
            match solve_3x3_system(&equations) {
                Ok(solutions) => {
                    for (var, value) in solutions {
                        println!("{}", formatter.format_equation_solution(&var, &formatter.format_complex(&value)));
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        } else {
            eprintln!("Error: Only 2x2 and 3x3 systems are supported");
            std::process::exit(1);
        }
    } else {
        // Single equation - determine degree
        let degree = determine_equation_degree(input);

        let solutions = match degree {
            1 => solve_linear_equation(input),
            2 => solve_quadratic_equation(input),
            3 => solve_cubic_equation(input),
            4 => solve_quartic_equation(input),
            5.. => solve_quintic_equation(input),
            _ => Err("Could not determine equation degree".to_string()),
        };

        match solutions {
            Ok(sols) => {
                for (i, sol) in sols.iter().enumerate() {
                    let var = if sols.len() == 1 {
                        "x".to_string()
                    } else {
                        format!("x{}", i + 1)
                    };
                    println!("{}", formatter.format_equation_solution(&var, &formatter.format_complex(sol)));
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}

/// Determine the degree of an equation
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

/// Run interactive mode
fn run_interactive_mode(formatter: &dyn Formatter) {
    let prompt = formatter.format_prompt();
    
    println!("Calculator CLI v2.0.0 (Rust)");
    println!("Type 'exit' or 'quit' to exit");
    println!();

    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                // EOF (Ctrl+D)
                println!();
                break;
            }
            Ok(_) => {
                let input = input.trim();
                
                if input.is_empty() {
                    continue;
                }

                if input == "exit" || input == "quit" {
                    println!("Goodbye!");
                    break;
                }

                process_expression(input, formatter);
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }
}