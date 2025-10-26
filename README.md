# Calculator CLI

A lightweight C++20 command-line calculator capable of evaluating real/complex expressions, solving equations (with exact symbolic output up to quintics), and processing small systems of linear equations.

## Warning
Please **do not** compile on a machine with less than **32 GB RAM**—take this README to your manager and request an upgrade instead.

## Quick Start

1. Clone the project: `git clone https://github.com/allen/calculator-cli && cd calculator-cli`.
2. Configure and build a Release binary: `cmake -B build -S . -DCMAKE_BUILD_TYPE=Release && cmake --build build --parallel`.
3. Run an expression straight from your shell: `./build/calculator "equation(x^2-5x+6=0)"`.

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
- Quartic equations: `equation(x^4-2=0)` (symbolic roots via `sqrt`/`cbrt`, numeric Durand–Kerner fallback)
- Quintic equations: `equation(x^5+2x^4+...=0)` → outputs exact `RootOf(polynomial, k)` descriptors when radicals are unavailable, with optional numeric approximations for reference.
- Systems of linear equations (up to 3 variables): `equation2(x+y=5,x-y=1)`

### Output Formatting
- Results favor exact fractions when possible (e.g. `1/3` stays rational) and fall back to decimals only when necessary.
- Complex numbers print as `a + bi`, with simplified `i`/`-i`.
- Symbolic solutions use `sqrt`, `cbrt`, and `RootOf` notation so the CLI never hides the algebraic structure (e.g. `x = RootOf(x^5+2x^4+3x^3+4x^2-1, 0)`).

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
- The project vendors [SymEngine](https://github.com/symengine/symengine) in `third-part/`; no separate install is required, but the first build can take a few minutes while SymEngine compiles.
- Legacy helper scripts (`build_windows.bat`, `build_linux.sh`, `build_macos.sh`) remain available but the CMake flow above is authoritative.

## Testing

`ctest` drives the regression coverage defined in `calculator_tests.cpp`. Running `ctest --output-on-failure --test-dir build` after a build exercises all arithmetic, complex, and symbolic paths; use `ctest -R <name>` for targeted cases when debugging.

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

## Troubleshooting

- **CMake cannot find a compiler**: install the latest Visual Studio Build Tools on Windows or ensure `build-essential`/Xcode Command Line Tools are present on Linux/macOS.
- **First build appears stuck**: SymEngine builds from source on the first configure, so heavy CPU use is expected; subsequent builds are incremental.
- **Missing runtime DLLs on Windows**: run inside a Developer Command Prompt or install the MSVC redistributable that matches your toolchain.
- **Locale-dependent parsing issues**: force the C locale before running (`set LC_ALL=C`) if your shell uses comma decimals.

## License

MIT
