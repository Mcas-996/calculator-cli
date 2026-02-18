npm install -g calculator-cli
```

Once installed, you can run:

```bash
calculator "2 + 2"
calculator "x^2-5x+6=0"
```

### For ARM Systems (Apple Silicon, ARM64 Linux)

The npm package includes precompiled binaries only for x64 systems. For ARM systems:

```bash
# 1. Install Rust if you haven't already:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 2. Install calculator-cli from source:
cargo install calculator-tui
```

## Usage

```bash
# Basic calculations
calculator "3 + 5 * (2 - 8)^2"

# Complex numbers
calculator "(3+2i) * (1 - i)"
calculator "sqrt(-9)"        # -> 3i

# Trigonometry
calculator "sin(pi / 6)"     # radians
calculator "sind(30)"        # degrees

# Equation solving
calculator "x^2-5x+6=0"
calculator "x+y=5, x-y=1"

# Interactive mode (no arguments)
calculator
```

## Platform Support

| Platform | Architecture | Installation Method |
|----------|-------------|--------------------|
| Windows | x64 | npm install |
| macOS | x64 (Intel) | npm install |
| macOS | ARM (Apple Silicon) | cargo install |
| Linux | x64 | npm install |
| Linux | ARM64 | cargo install |

## Features

- Expression evaluation with support for complex numbers
- Equation solving from linear to quintic polynomials
- Exact fractions and symbolic output when possible
- Multiple output formats (ASCII, Unicode, LaTeX)
- Interactive mode for exploring calculations

## License

MIT License - See LICENSE file for details

## For More Information

See the full project documentation at: https://github.com/anomalyco/calculator-cli