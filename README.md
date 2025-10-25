# Calculator CLI

A lightweight C++20 command-line calculator capable of evaluating real/complex expressions, solving equations, and processing small systems of linear equations.

## Features

### Expression Engine
- Addition, subtraction, multiplication, division, exponentiation.
- Percentages (`50% * 200`), parentheses, unary minus.
- Constants `pi`, `e`, imaginary unit `i`, and decimal or fractional inputs (fractions auto-simplify in output).
- Square root `sqrt()`, absolute value `abs()`, trigonometric functions in radians `sin()/cos()` and degree variants `sind()/cosd()`.
- Full complex-number arithmetic, e.g. `sqrt(-4)`, `(3+2i)*(1-i)`, `cosd(60+i)`.

### Equation Solving
- Linear equations: `equation(2x+5=0)`
- Quadratic equations: `equation(x^2-5x+6=0)` (real or complex roots)
- Cubic equations: `equation(x^3-6x^2+11x-6=0)`
- Systems of linear equations (up to 3 variables): `equation2(x+y=5,x-y=1)`

### Output Formatting
- Results favor exact fractions when possible (e.g. `1/3` stays rational) and fall back to decimals for irrational/complex parts.
- Complex numbers print as `a + bi`, with simplified `i`/`-i` where appropriate.

## Usage

```bash
# Basic usage (expression as CLI argument)
./calculator "3 + 5 * (2 - 8)^2"

# Complex numbers
./calculator "(3+2i) * (1 - i)"
./calculator "sqrt(-9)"        # -> 3i

# Trigonometry
./calculator "sin(pi / 6)"     # radians
./calculator "sind(30)"        # degrees

# Equation solving
./calculator "equation(x^2-5x+6=0)"
./calculator "equation2(x+y=5,x-y=1)"
```

Passing `--help` or `--version` prints CLI info. Without an argument the program exits with usage guidance.

## Building

The preferred workflow uses CMake (required >= 3.10) and a C++20 compiler.

```bash
cmake -B build -S . -DCMAKE_BUILD_TYPE=Release
cmake --build build --parallel
ctest --output-on-failure --test-dir build   # optional, runs calculator_tests
```

- Windows (MSVC, VS Build Tools, or clang-cl) should specify the desired architecture: `cmake -B build -S . -A x64`.
- macOS/Linux follow the same commands; install `cmake`/`g++`/`clang++` via your package manager.
- Legacy helper scripts (`build_windows.bat`, `build_linux.sh`, `build_macos.sh`) remain available but the CMake flow above is authoritative.

## macOS Gatekeeper

Unsigned binaries downloaded from CI may trigger Gatekeeper warnings. To run them:

```bash
xattr -d com.apple.quarantine /path/to/calculator
```

or right-click the app in Finder, choose "Open," and confirm.

## Project Structure

- `complex_number.hpp`, `fractions.hpp`, `string_processing.*` - expression/core logic.
- `main_cli.cpp` - CLI entry point.
- `calculator_tests.cpp` - unit/regression tests invoked via CTest.
- `.github/workflows/c-cpp.yml` - GitHub Actions pipeline for Linux/macOS/Windows builds, tests, and releases.

## License

MIT
