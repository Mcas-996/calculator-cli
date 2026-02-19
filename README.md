# Calculator CLI

A lightweight Rust command-line calculator for evaluating real/complex expressions, solving equations, and handling small linear systems.

## Quick Start

1. Clone the project:
   `git clone https://github.com/Mcas-996/calculator-cli && cd calculator-cli`
2. Run a calculation directly:
   `cargo run -- "x^2-5x+6=0"`
3. Build a release binary:
   `cargo build --release`
4. Run the release binary:
   `./target/release/calctui "2 + 2"`

## Installation

### Install from crates.io (recommended)

```bash
cargo install calculator-tui
calctui "2 + 2"
calctui "x^2-5x+6=0"
```

### Download from GitHub Releases

Download the prebuilt binary for your platform from:
https://github.com/Mcas-996/calculator-cli/releases

After downloading, run the executable with an expression.

### Build from source

```bash
git clone https://github.com/Mcas-996/calculator-cli && cd calculator-cli
cargo build --release
./target/release/calctui "2 + 2"
```

### Platform Support

| Platform | Architecture | Installation Method |
|----------|-------------|--------------------|
| Windows | x64 | cargo install / GitHub Releases |
| macOS | x64 (Intel) | cargo install / GitHub Releases |
| macOS | ARM (Apple Silicon) | cargo install / GitHub Releases |
| Linux | x64 | cargo install / GitHub Releases |
| Linux | ARM64 | cargo install / GitHub Releases |

## Features

### Expression Engine

- Addition, subtraction, multiplication, division, exponentiation.
- Percentages (`50% * 200`), parentheses, unary minus.
- Constants `pi`, `e`, imaginary unit `i`, and decimal or fractional inputs.
- Functions: `sqrt()`, `abs()`, `sin()`, `cos()`, `sind()`, `cosd()`.
- Complex-number arithmetic, for example `sqrt(-4)` and `(3+2i)*(1-i)`.

### Equation Solving

- Linear equations: `2x+5=0`
- Quadratic equations: `x^2-5x+6=0`
- Cubic equations: `x^3-6x^2+11x-6=0`
- Quartic equations: `x^4-2=0`
- Quintic and higher: numeric approximation via Durand-Kerner
- Systems of linear equations (up to 3 variables): `x+y=5, x-y=1`

### Output Modes

- Exact-style output with fractions when possible
- Decimal output with `--decimal`
- Multiple output formats: ASCII, Unicode, LaTeX (`--ascii`, `--unicode`, `--latex`)

### Interactive Modes

- Default: TUI mode (run without an expression)
- Legacy interactive CLI mode: `--v1`

## Usage

```bash
# Basic expression
calctui "3 + 5 * (2 - 8)^2"

# Complex numbers
calctui "(3+2i) * (1 - i)"
calctui "sqrt(-9)"                # -> 3i

# Trigonometry
calctui "sin(pi / 6)"             # radians
calctui "sind(30)"                # degrees

# Equation solving
calctui "x^2-5x+6=0"
calctui "x+y=5, x-y=1"

# Output formatting
calctui --unicode "sqrt(16)"
calctui --latex "pi"
calctui --ascii "3 + 4"
calctui --decimal "1/3"

# Interactive modes
calctui
calctui --v1
```

Use `--help` or `--version` to print CLI information.

## Building

```bash
# Build in release mode
cargo build --release

# Run tests
cargo test

# Run clippy checks
cargo clippy
```

- Rust 1.70.0 or later is required
- Cargo handles dependencies automatically
- Release binary path: `target/release/calctui`

## Project Structure

- `src/core/` - Core data types (`ComplexNumber`, `Fraction`, `Expression`)
- `src/parser/` - Expression parser and tokenizer
- `src/solver/` - Equation solvers (linear through quintic/system)
- `src/output/` - Output formatters (ASCII, Unicode, LaTeX)
- `src/tui/` - Terminal UI
- `src/main.rs` - CLI entry point

## Troubleshooting

- **Cargo not found**: Install Rust from https://rustup.rs/
- **Build fails**: Verify Rust is 1.70.0 or newer

## License

MIT License. See `LICENSE` for details.
