# Calculator CLI

A command-line calculator application that evaluates mathematical expressions with support for basic operations, parentheses, exponents, percentages, square roots, mathematical constants, equation solving, and systems of linear equations.

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