# Calculator CLI

A lightweight Rust command-line calculator capable of evaluating real/complex expressions, solving equations (with exact symbolic output up to quintics), and processing small systems of linear equations.

## Quick Start

1. Clone the project: `git clone https://github.com/allen/calculator-cli && cd calculator-cli`.
2. Build a Release binary: `cargo build --release`.
3. Run an expression straight from your shell: `./target/release/calculator "x^2-5x+6=0"`.

## 安装和使用

### npm全局安装（推荐）
```bash
npm install -g mathcalc-cli
mathcalc "2 + 2"
mathcalc "x^2-5x+6=0"
```

### npx方式（无需全局安装）
```bash
npx mathcalc-cli "2 + 2"
npx mathcalc-cli "x^2-5x+6=0"
```

### 本地项目使用
```bash
npm install mathcalc-cli
npx mathcalc-cli "2 + 2"
```

### 从GitHub Release下载
```bash
https://github.com/Mcas-996/calculator-cli/releases
```

下载对应平台的二进制文件
```


### For ARM Systems (Apple Silicon, ARM64 Linux)
The npm package includes precompiled binaries only for x64 systems. For ARM systems:

```bash
# 1. Install Rust if you haven't already:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 2. Install calculator-cli from source:
cargo install calculator
```

### Platform Support
| Platform | Architecture | Installation Method |
|----------|-------------|--------------------|
| Windows | x64 | npm install |
| macOS | x64 (Intel) | npm install |
| macOS | ARM (Apple Silicon) | cargo install |
| Linux | x64 | npm install |
| Linux | ARM64 | cargo install |

## Features

### Expression Engine
- Addition, subtraction, multiplication, division, exponentiation.
- Percentages (`50% * 200`), parentheses, unary minus.
- Constants `pi`, `e`, imaginary unit `i`, and decimal or fractional inputs (fractions auto-simplify in output).
- Square root `sqrt()`, absolute value `abs()`, trigonometric functions in radians `sin()/cos()` and degree variants `sind()/cosd()`.
- Full complex-number arithmetic, e.g. `sqrt(-4)`, `(3+2i)*(1-i)`, `cosd(60+i)`.

### Equation Solving
- Linear equations: `2x+5=0`
- Quadratic equations: `x^2-5x+6=0` (real or complex roots)
- Cubic equations: `x^3-6x^2+11x-6=0`
- Quartic equations: `x^4-2=0` (symbolic roots via `sqrt`/`cbrt`, numeric Durand–Kerner fallback)
- Quintic equations: `x^5+2x^4+...=0` → numeric approximation via Durand-Kerner method
- Systems of linear equations (up to 3 variables): `x+y=5, x-y=1`

### Output Formatting
- Results favor exact fractions when possible (e.g. `1/3` stays rational) and fall back to decimals only when necessary.
- Complex numbers print as `a + bi`, with simplified `i`/`-i`.
- Multiple output formats: ASCII, Unicode, and LaTeX.

## Usage

```bash
# Basic usage (expression as CLI argument)
./target/release/calculator "3 + 5 * (2 - 8)^2"

# Complex numbers
./target/release/calculator "(3+2i) * (1 - i)"
./target/release/calculator "sqrt(-9)"        # -> 3i

# Trigonometry
./target/release/calculator "sin(pi / 6)"     # radians
./target/release/calculator "sind(30)"        # degrees

# Equation solving
./target/release/calculator "x^2-5x+6=0"
./target/release/calculator "x+y=5, x-y=1"

# Output formatting
./target/release/calculator --unicode "sqrt(16)"
./target/release/calculator --latex "pi"
./target/release/calculator --ascii "3 + 4"
```

Passing `--help` or `--version` prints CLI info. Without an argument, the program enters interactive mode.

## Building

The project uses Rust and Cargo.

```bash
# Build in release mode
cargo build --release

# Run tests
cargo test

# Run with clippy for additional checks
cargo clippy
```

- Rust 1.75.0 or later is required
- Cargo handles all dependencies automatically
- The binary will be available at `target/release/calculator`

## Interactive Mode

Run the calculator without arguments to enter interactive mode:

```bash
./target/release/calculator
```

Type expressions and press Enter to evaluate. Type `exit` or `quit` to exit, or press Ctrl+D.

## Project Structure

- `src/core/` - Core data types (ComplexNumber, Fraction, Expression)
- `src/parser/` - Expression parser and tokenizer
- `src/solver/` - Equation solvers (linear, quadratic, cubic, quartic, quintic)
- `src/output/` - Output formatters (ASCII, Unicode, LaTeX)
- `src/main.rs` - CLI entry point

## Troubleshooting

- **Cargo not found**: Install Rust from https://rustup.rs/
- **Build fails**: Ensure you have Rust 1.75.0 or later
- **Locale-dependent parsing issues**: force the C locale before running (`LC_ALL=C ./calculator`)

## License

MIT License - See LICENSE file for details

## Migration from C++ Version

This calculator was originally implemented in C++20 with SymEngine. The Rust version provides:
- Faster build times (seconds instead of minutes)
- No external dependencies (no 374MB SymEngine vendored code)
- Memory safety without garbage collection
- Smaller binary size
- Better cross-platform support