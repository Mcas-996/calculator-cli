# Calculator CLI

A lightweight Rust command-line calculator for evaluating real/complex expressions, solving equations, and handling small linear systems.

## Quick Start

Install `calctui` first (pick one):

```bash
# macOS/Linux (curl installer)
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/Mcas-996/calculator-cli/releases/download/latest/calculator-tui-installer.sh | sh

# Windows PowerShell (irm installer)
powershell -ExecutionPolicy Bypass -c "irm https://github.com/Mcas-996/calculator-cli/releases/download/latest/calculator-tui-installer.ps1 | iex"

# Homebrew
brew install Mcas-996/tap/calculator-tui
```

Then run:

```bash
calctui "2 + 2"
calctui "x^2-5x+6=0"
```

From source (optional):

```bash
git clone https://github.com/Mcas-996/calculator-cli && cd calculator-cli
cargo build --release
./target/release/calctui "2 + 2"
```

## Installation

### Install from crates.io (recommended)

```bash
cargo install calculator-tui
calctui "2 + 2"
calctui "x^2-5x+6=0"
```

### Install prebuilt binaries (v2.1.2)

Install prebuilt binaries via shell script:

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/Mcas-996/calculator-cli/releases/download/v2.1.2/calculator-tui-installer.sh | sh
```

Install prebuilt binaries via PowerShell script:

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/Mcas-996/calculator-cli/releases/download/v2.1.2/calculator-tui-installer.ps1 | iex"
```

Install prebuilt binaries via Homebrew:

```bash
brew install Mcas-996/tap/calculator-tui
```

Download prebuilt archives directly:

| File | Platform | Checksum |
|----------|----------|----------|
| [calculator-tui-aarch64-apple-darwin.tar.xz](https://github.com/Mcas-996/calculator-cli/releases/download/v2.1.2/calculator-tui-aarch64-apple-darwin.tar.xz) | Apple Silicon macOS | [checksum](https://github.com/Mcas-996/calculator-cli/releases/download/v2.1.2/calculator-tui-aarch64-apple-darwin.tar.xz.sha256) |
| [calculator-tui-x86_64-apple-darwin.tar.xz](https://github.com/Mcas-996/calculator-cli/releases/download/v2.1.2/calculator-tui-x86_64-apple-darwin.tar.xz) | Intel macOS | [checksum](https://github.com/Mcas-996/calculator-cli/releases/download/v2.1.2/calculator-tui-x86_64-apple-darwin.tar.xz.sha256) |
| [calculator-tui-x86_64-pc-windows-msvc.zip](https://github.com/Mcas-996/calculator-cli/releases/download/v2.1.2/calculator-tui-x86_64-pc-windows-msvc.zip) | x64 Windows | [checksum](https://github.com/Mcas-996/calculator-cli/releases/download/v2.1.2/calculator-tui-x86_64-pc-windows-msvc.zip.sha256) |
| [calculator-tui-aarch64-unknown-linux-gnu.tar.xz](https://github.com/Mcas-996/calculator-cli/releases/download/v2.1.2/calculator-tui-aarch64-unknown-linux-gnu.tar.xz) | ARM64 Linux | [checksum](https://github.com/Mcas-996/calculator-cli/releases/download/v2.1.2/calculator-tui-aarch64-unknown-linux-gnu.tar.xz.sha256) |
| [calculator-tui-x86_64-unknown-linux-gnu.tar.xz](https://github.com/Mcas-996/calculator-cli/releases/download/v2.1.2/calculator-tui-x86_64-unknown-linux-gnu.tar.xz) | x64 Linux | [checksum](https://github.com/Mcas-996/calculator-cli/releases/download/v2.1.2/calculator-tui-x86_64-unknown-linux-gnu.tar.xz.sha256) |

All release files are available at:
https://github.com/Mcas-996/calculator-cli/releases/tag/v2.1.2

After downloading, run the executable with an expression.

### Build from source

```bash
git clone https://github.com/Mcas-996/calculator-cli && cd calculator-cli
cargo build --release
./target/release/calctui "2 + 2"
```

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
