# Project Context

## Purpose
A lightweight Rust command-line calculator capable of evaluating real/complex mathematical expressions, solving equations (with exact symbolic output up to quintic polynomials), and processing small systems of linear equations. The project focuses on mathematical accuracy, providing exact solutions where possible (using fractions and symbolic notation), and offers optional pretty output with Unicode and LaTeX rendering.

## Tech Stack
- **Language**: Rust (Edition 2021, MSRV 1.70.0)
- **Build System**: Cargo
- **Compiler**: rustc 1.75.0+
- **Symbolic Math**: Pure Rust implementation (no external symbolic math library)
- **Testing**: Rust's built-in test framework
- **Terminal Rendering**: LaTeX source generation (external pdflatex optional)

## Project Conventions

### Code Style
- **Struct/Enum Names**: CamelCase (e.g., `ComplexNumber`, `Fraction`, `PrettyConfig`)
- **Function Names**: snake_case (e.g., `to_string()`, `sqrt()`, `pow()`)
- **Constants**: UPPER_CASE or const (e.g., `MAX_DENOMINATOR`)
- **Error Handling**: `anyhow::Error` with custom `thiserror` types
- **Public Members**: Structs expose fields as public (e.g., `ComplexNumber { real, imag }`)
- **Comments**: English preferred for code
- **Standard Library**: Use `std::` prefix consistently
- **Numerical Precision**: Fraction-based exact arithmetic where possible
- **Fraction Simplification**: Auto-simplify using `num-rational`

### Architecture Patterns
- **Module System**: Organized into `core/`, `parser/`, `solver/`, `output/`, `utils/`
- **Trait-Based Polymorphism**: `Formatter` trait for output formatters
- **Library Pattern**: `src/lib.rs` exports public API, `src/main.rs` is CLI entry point
- **Modular Components**:
  - `core/` - Core types (`ComplexNumber`, `Fraction`, `Expression`)
  - `parser/` - Expression parsing and tokenization (`tokenizer.rs`, `expression.rs`)
  - `solver/` - Equation solving (linear to quintic)
  - `output/` - Output formatting (ASCII/Unicode/LaTeX)
  - `utils/` - Utility functions (`math.rs`)
- **Singleton Pattern**: `PrettyConfig::instance()` for global pretty output configuration
- **Factory Pattern**: `Fraction::from_double()` for rational approximation
- **Type Aliases**: `type Fraction = Rational64` for clarity

### Testing Strategy
- **Test Framework**: Rust's built-in `#[test]` attribute
- **Test Location**: Inline tests in source files (`#[cfg(test)]` modules)
- **Execution**: `cargo test`
- **Coverage**: Unit tests for all modules
- **Targeted Testing**: `cargo test <module_name>` for specific modules

### Git Workflow
- **Branch Strategy**: Feature branch workflow
- **Commit Messages**: Concise, descriptive
- **CI/CD**: None (removed as requested)
- **Release**: Manual tagging and publishing

## Domain Context

### Mathematical Features
- **Expression Evaluation**: Supports `+`, `-`, `*`, `/`, `^`, `%`, parentheses, unary minus
- **Constants**: `pi`, `e`, imaginary unit `i`
- **Functions**: `sqrt()`, `abs()`, `sin()`, `cos()`, `sind()`, `cosd()` (degree variants support complex numbers)
- **Complex Numbers**: Full arithmetic with `a + bi` representation
- **Equation Solving**:
  - Linear: `2x+5=0`
  - Quadratic: `x^2-5x+6=0`
  - Cubic: `x^3-6x^2+11x-6=0` (Cardano's formula)
  - Quartic: `x^4-2=0` (Ferrari's formula)
  - Quintic: `x^5+...` (Durand-Kerner numeric approximation)
  - Linear Systems: `x+y=5, x-y=1` (up to 3 variables, Gaussian elimination)
- **Output Formatting**: Exact fractions preferred, decimals only when necessary

### Output Representation
- **Exact Fractions**: `1/3` stays rational, `2/1` displays as `2`
- **Complex Format**: `a + bi`, `3i`, `-i`, simplified where possible
- **Numeric Solutions**: Quintic+ equations use numeric approximation
- **Pretty Output**: ASCII (default), Unicode (×, ÷, π, √, ², ₀), or LaTeX

## Important Constraints

### Build Requirements
- **Minimum RAM**: 4GB (no heavy dependency compilation)
- **Compiler**: Rust 1.75.0 or later
- **Build Time**: Seconds (no SymEngine compilation)
- **Platform**: Cross-platform (Linux, macOS, Windows)

### Technical Constraints
- **No Symbolic Math Library**: Pure Rust implementation
- **LaTeX Rendering**: Optional `pdflatex` for LaTeX source generation
- **Locale Sensitivity**: Parsing may break with comma decimals; use `LC_ALL=C` if needed
- **Complex Number Precision**: Uses `f64` with fraction wrapper for exact arithmetic
- **Fraction Limits**: `MAX_DENOMINATOR = 10000` for rational approximation

### Security and Safety
- **No Network**: No external network calls or API dependencies
- **Local Operations**: All computation happens locally
- **Error Messages**: Return "Error:" prefix and exit code 1 on failure
- **Input Validation**: Parse-time error handling, no code execution
- **Memory Safety**: Rust's ownership system prevents memory errors

## External Dependencies

### Cargo Dependencies
- **clap**: Command-line argument parsing (derive feature)
- **anyhow**: Ergonomic error handling
- **thiserror**: Custom error type derivation
- **num-rational**: Exact fraction arithmetic
- **num-traits**: Numeric traits for generic programming
- **regex**: Regular expression support

### System Libraries
- **Standard Library**: Rust std library

### Optional Dependencies
- **pdflatex**: For LaTeX rendering (optional, detected at runtime)