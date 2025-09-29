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

## Running on macOS

If you download the pre-compiled binary for macOS, you might encounter a security warning because the application is not signed with an Apple Developer certificate. This is normal for applications distributed outside the App Store.

To run the calculator on macOS, you can do one of the following:

### Option 1: Using Terminal
Open Terminal and run the following command to remove the quarantine attribute:
```bash
xattr -d com.apple.quarantine /path/to/calculator
```

Replace `/path/to/calculator` with the actual path to the downloaded calculator executable.

### Option 2: Using Finder
1. In Finder, navigate to the calculator application
2. Right-click (or Control-click) on the application
3. Select "Open" from the context menu
4. Confirm that you want to open the application when prompted

Note: This is a common security measure on macOS to protect users from potentially malicious software. As this is an open-source project, you can review the code and compile it yourself to ensure its safety.

## Usage

```bash
# Basic usage
./calculator
# Then enter expressions like: 2 + 3 * 4

# Or pass expression directly as argument
./calculator "2 + 3 * 4"
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