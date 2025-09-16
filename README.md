# Calculator CLI

A command-line calculator application that evaluates mathematical expressions with support for basic operations, parentheses, exponents, percentages, square roots, and mathematical constants.

## Features

- Basic arithmetic operations: `+`, `-`, `*`, `/`, `^` (exponent)
- Parentheses for grouping expressions
- Negative numbers and decimal numbers
- Percentage calculations (e.g., `50%` converts to `0.5`)
- Square root function: `sqrt(x)`
- Mathematical constants: `pi` (π) and `e`
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