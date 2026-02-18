use clap::Parser;
use std::io::{self, Write};

use calculator_tui::output::{Formatter, PrettyConfig, PrettyLevel};
use calculator_tui::parser::parse_expression;
use calculator_tui::solver::{
    solve_2x2_system, solve_3x3_system, solve_cubic_equation, solve_linear_equation,
    solve_quadratic_equation, solve_quartic_equation, solve_quintic_equation,
};
use calculator_tui::tui::app::TuiApp;

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

    /// Show decimal approximations instead of exact fractions
    #[arg(
        short = 'd',
        long = "decimal",
        help = "Show decimal approximations instead of exact fractions"
    )]
    decimal: bool,

    /// Show exact radical forms (e.g., √2, ∛3) when possible
    #[arg(
        short = 'e',
        long = "exact",
        help = "Show exact radical forms when possible"
    )]
    exact: bool,

    /// Use legacy CLI interactive mode (instead of TUI)
    #[arg(long = "v1", help = "Use legacy CLI interactive mode instead of TUI")]
    v1: bool,

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
    } else if cli.exact {
        config.set_level(PrettyLevel::Unicode);
    }

    let formatter = config.get_formatter();

    // Process expression or enter interactive mode
    if let Some(expr) = cli.expression {
        // Single expression mode
        process_expression(&expr, formatter.as_ref(), cli.decimal, cli.exact);
    } else if cli.v1 {
        // CLI interactive mode (legacy)
        run_interactive_mode(formatter.as_ref(), cli.decimal, cli.exact);
    } else {
        // TUI mode (default)
        run_tui_mode();
    }
}

/// Process a single expression or equation
fn process_expression(input: &str, formatter: &dyn Formatter, show_decimal: bool, use_exact: bool) {
    let input = input.trim();

    // Check if it's an equation
    if input.contains('=') {
        process_equation(input, formatter, show_decimal, use_exact);
    } else {
        // It's an expression
        match parse_expression(input) {
            Ok(expr) => match expr.evaluate() {
                Ok(result) => {
                    if show_decimal {
                        println!("{}", format_decimal_approximation(&result));
                    } else {
                        println!("{}", formatter.format_complex(&result));
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            },
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}

/// Strip equation() wrapper from input if present
fn strip_equation_wrapper(input: &str) -> &str {
    let trimmed = input.trim();
    if trimmed.starts_with("equation(") && trimmed.ends_with(')') {
        &trimmed[9..trimmed.len() - 1] // Remove "equation(" prefix and ")" suffix
    } else {
        trimmed
    }
}

/// Process an equation
fn process_equation(input: &str, formatter: &dyn Formatter, show_decimal: bool, use_exact: bool) {
    // Strip equation wrapper if present
    let clean_input = strip_equation_wrapper(input);
    // Check if it's a system of equations
    if clean_input.contains(',') {
        let equations: Vec<String> = clean_input
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        if equations.len() == 2 {
            match solve_2x2_system(&equations) {
                Ok(solutions) => {
                    for (var, value) in solutions {
                        let formatted_value =
                            format_solution_value(&value, show_decimal, use_exact, formatter);
                        println!(
                            "{}",
                            formatter.format_equation_solution(&var, &formatted_value)
                        );
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
                        let formatted_value =
                            format_solution_value(&value, show_decimal, use_exact, formatter);
                        println!(
                            "{}",
                            formatter.format_equation_solution(&var, &formatted_value)
                        );
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
        let degree = determine_equation_degree(clean_input);

        let solutions = match degree {
            1 => solve_linear_equation(clean_input),
            2 => solve_quadratic_equation(clean_input),
            3 => solve_cubic_equation(clean_input),
            4 => solve_quartic_equation(clean_input),
            5.. => solve_quintic_equation(clean_input),
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
                    let formatted_value =
                        format_solution_value(sol, show_decimal, use_exact, formatter);
                    println!(
                        "{}",
                        formatter.format_equation_solution(&var, &formatted_value)
                    );
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}

/// Format a solution value based on the selected output mode
fn format_solution_value(
    value: &calculator_tui::core::ComplexNumber,
    show_decimal: bool,
    use_exact: bool,
    formatter: &dyn Formatter,
) -> String {
    if show_decimal {
        format_decimal_approximation(value)
    } else if use_exact {
        // Use the exact formatting from the Unicode formatter
        formatter.format_complex(value)
    } else {
        formatter.format_complex(value)
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
fn run_interactive_mode(formatter: &dyn Formatter, show_decimal: bool, use_exact: bool) {
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

                process_expression(input, formatter, show_decimal, use_exact);
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }
}

/// Run TUI mode
fn run_tui_mode() {
    let mut app = TuiApp::new();
    if let Err(e) = app.run() {
        eprintln!("Error running TUI: {}", e);
    }
}

/// Format a complex number as a decimal approximation
fn format_decimal_approximation(num: &calculator_tui::core::ComplexNumber) -> String {
    let real_part = num.real.to_f64();
    let imag_part = num.imag.to_f64();

    if imag_part == 0.0 {
        format!("{:.6}", real_part)
    } else if real_part == 0.0 {
        if imag_part == 1.0 {
            "i".to_string()
        } else if imag_part == -1.0 {
            "-i".to_string()
        } else {
            format!("{:.6}i", imag_part)
        }
    } else {
        if imag_part == 1.0 {
            format!("{:.6} + i", real_part)
        } else if imag_part == -1.0 {
            format!("{:.6} - i", real_part)
        } else if imag_part > 0.0 {
            format!("{:.6} + {:.6}i", real_part, imag_part)
        } else {
            format!("{:.6} - {:.6}i", real_part, -imag_part)
        }
    }
}
