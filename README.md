# Calculator CLI

A lightweight Rust command-line calculator capable of evaluating real/complex expressions, solving equations (with exact symbolic output up to quintics), and processing small systems of linear equations.

## Quick Start

1. Clone the project: `git clone https://github.com/allen/calculator-cli && cd calculator-cli`.
2. Build a Release binary: `cargo build --release`.
3. Run an expression straight from your shell: `./target/release/calculator "x^2-5x+6=0"`.

## 安装和使用

### cargo install (推荐)
```bash
cargo install calculator
calculator "2 + 2"
calculator "x^2-5x+6=0"
```

### 从 GitHub Release 下载
从以下地址下载对应平台的预编译二进制:
https://github.com/anomalyco/calculator-cli/releases

```bash
# Linux
curl -L https://github.com/anomalyco/calculator-cli/releases/latest/download/calculator-linux-x64 -o calculator
chmod +x calculator
./calculator "2 + 2"

# Windows
# 从 Releases 页面下载 calculator_windows-x86-64.exe
```

### 从源码构建
```bash
git clone https://github.com/anomalyco/calculator-cli && cd calculator-cli
cargo build --release
./target/release/calculator "2 + 2"
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

### TUI Mode (Interactive)
Run without arguments to enter TUI mode:
```bash
./target/release/calculator
```

Use `--v1` flag for legacy CLI interactive mode:
```bash
./target/release/calculator --v1
```

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

Passing `--help` or `--version` prints CLI info. Without an argument, the program enters TUI mode. Use `--v1` for legacy CLI interactive mode.

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

- Rust 1.70.0 or later is required
- Cargo handles all dependencies automatically
- The binary will be available at `target/release/calculator`

## Project Structure

- `src/core/` - Core data types (ComplexNumber, Fraction, Expression)
- `src/parser/` - Expression parser and tokenizer
- `src/solver/` - Equation solvers (linear, quadratic, cubic, quartic, quintic)
- `src/output/` - Output formatters (ASCII, Unicode, LaTeX)
- `src/tui/` - Terminal UI components
- `src/main.rs` - CLI entry point

## Troubleshooting

- **Cargo not found**: Install Rust from https://rustup.rs/
- **Build fails**: Ensure you have Rust 1.70.0 or later

## License

MIT License - See LICENSE file for details
