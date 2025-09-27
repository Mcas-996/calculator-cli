# Calculator CLI

A command-line calculator that supports basic arithmetic operations, equation solving, and more.

## Features

### Basic Operations
- Addition (+)
- Subtraction (-)
- Multiplication (*)
- Division (/)
- Exponentiation (^)
- Parentheses for grouping
- Percentage calculations (%)
- Square root function (sqrt)
- Absolute value function (abs)
- Constants: pi, e

### Equation Solving
- Linear equations (e.g., `equation(2x+5=0)`)
- Quadratic equations (e.g., `equation(x^2+5x+6=0)`)
- Cubic equations (e.g., `equation(x^3-6x^2+11x-6=0)`)
- System of linear equations (e.g., `equation2(x+y=5,x-y=1)`)

### Number Format
- Integer and decimal numbers
- Fractions (automatic conversion)
- Mixed numbers

## Usage

Run the calculator executable and enter expressions or equations to solve.

### Examples

Basic arithmetic:
> 2 + 3 * 4
> (1 + 2) * (3 + 4)
> sqrt(16) + 2^3
> 50%

Equation solving:
> equation(2x+5=0)
> equation(x^2+5x+6=0)
> equation(x^3-6x^2+11x-6=0)
> equation2(x+y=5,x-y=1)

## Implementation Details

The calculator uses a stack-based algorithm to evaluate expressions with proper operator precedence. Fractions are used internally for precise calculations.

For equation solving:
- Linear equations are solved using basic algebra
- Quadratic equations use the quadratic formula
- Cubic equations are solved using Cardano's method
- Systems of linear equations use Gaussian elimination

## Building

To build the calculator, compile the C++ source files with a C++11 compatible compiler.

## License

MIT

[中文版说明文档 (Chinese README)](README_zh.md) | [WSL使用说明 (WSL Instructions)](README_WSL.md)

## Features

- Basic arithmetic operations: `+`, `-`, `*`, `/`, `^` (exponent)
- Parentheses for grouping expressions
- Negative numbers and decimal numbers
- Percentage calculations (e.g., `50%` converts to `0.5`)
- Square root function: `sqrt(x)`
- Mathematical constants: `pi` (π) and `e`
- Linear equation solving: `equation(x+1=0)`
- Quadratic equation solving: `equation(x^2+2x+1=0)`
- Systems of linear equations: `equation2(x+y=5,x-y=1)`
- Error handling for invalid expressions

## Supported Operations

```bash
# Basic arithmetic
3 + 5 * (2 - 8)^2
-2.5 * 4 + 3^2

# Percentages
50% * 200

# Square roots
sqrt(16) + 3

# Constants
pi * 2
e^2

# Linear equations
equation(x+1=0)
equation(2x-3=7)

# Quadratic equations
equation(x^2+2x+1=0)
equation(x^2-5x+6=0)

# Systems of linear equations
equation2(x+y=5,x-y=1)
equation2(2x+3y=12,4x-y=5)
equation2(x+y+z=6,x-y+z=2,2x+y-z=3)
```

## Building

### Windows
```bash
build_windows.bat
```

### Linux
```bash
chmod +x build_linux.sh
./build_linux.sh
```

### macOS
```bash
chmod +x build_macos.sh
./build_macos.sh
```

## Usage

```bash
# Basic usage
calculator "3 + 4 * 2"

# Show help
calculator --help
# or
calculator -h
```

## Requirements

- C++11 compatible compiler (g++)
- Standard C++ library
- Math library

## Files

- `main_cli.cpp` - Main application entry point
- `string_processing.cpp` - Expression evaluation logic
- `string_processing.hpp` - Header file for the calculator functions
- `build_*.sh/bat` - Platform-specific build scripts