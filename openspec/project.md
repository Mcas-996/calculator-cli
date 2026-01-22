# Project Context

## Purpose
A lightweight C++20 command-line calculator capable of evaluating real/complex mathematical expressions, solving equations (with exact symbolic output up to quintic polynomials), and processing small systems of linear equations. The project focuses on mathematical accuracy, providing exact solutions where possible (using fractions and symbolic notation), and offers optional pretty output with Unicode and LaTeX rendering.

## Tech Stack
- **Language**: C++20
- **Build System**: CMake 3.20+
- **Compiler**: clang/clang++ (enforced via CMakeLists.txt)
- **Symbolic Math**: SymEngine (vendored in `third-part/`)
- **Testing**: CTest
- **Terminal Rendering**: Kitty protocol support for LaTeX output

## Project Conventions

### Code Style
- **Class Names**: CamelCase (e.g., `ComplexNumber`, `Fraction`, `PrettyConfig`)
- **Function Names**: snake_case (e.g., `toString()`, `simplify()`, `sqrtPrincipal()`)
- **Constants**: UPPER_CASE or constexpr (e.g., `COMPLEX_EPSILON`)
- **Header Guards**: `#ifndef FILENAME_HPP` pattern
- **Error Handling**: `std::runtime_error` for exceptional conditions
- **Public Members**: Struct-like classes expose members as public (e.g., `Fraction`)
- **Comments**: Mix of English and Chinese (English preferred for code)
- **Standard Library**: Use `std::` prefix consistently (no `using namespace std` except in main_cli.cpp)
- **Numerical Precision**: `COMPLEX_EPSILON = 1e-9` for floating-point comparisons
- **Fraction Simplification**: Auto-simplify on construction using `std::gcd()`

### Architecture Patterns
- **Header-Only Utilities**: Core types (`ComplexNumber`, `Fraction`) defined in headers only
- **Core Library Pattern**: `calculator_core` library contains shared logic, linked by executables
- **Modular Components**:
  - `string_processing.*` - Expression parsing and evaluation
  - `symbolic_solver.*` - Equation solving (linear to quintic)
  - `complex_number.*` - Complex number arithmetic
  - `fractions.*` - Fraction operations and formatting
  - `pretty_output.*` - Output formatting (ASCII/Unicode/LaTeX)
- **Singleton Pattern**: `PrettyConfig::getInstance()` for global pretty output configuration
- **Visitor Pattern**: Used internally by SymEngine for expression traversal
- **Factory Pattern**: `Fraction::fromDouble()` for rational approximation

### Testing Strategy
- **Test Framework**: CTest with custom test runner (`calculator_tests_runner`)
- **Test Location**: `calculator_tests.cpp`
- **Execution**: `ctest --output-on-failure --test-dir build`
- **Coverage**: Regression tests for arithmetic, complex numbers, symbolic solving, and equation systems
- **Targeted Testing**: `ctest -R <name>` for specific test cases during debugging

### Git Workflow
- **Branch Strategy**: Feature branch workflow implied by commit history
- **Commit Messages**: Concise, but not strictly following conventional commits
- **CI/CD**: GitHub Actions workflow (`.github/workflows/c-cpp.yml`) for Linux/macOS/Windows builds and tests
- **Release**: Automated releases with CI artifacts

## Domain Context

### Mathematical Features
- **Expression Evaluation**: Supports `+`, `-`, `*`, `/`, `^`, `%`, parentheses, unary minus
- **Constants**: `pi`, `e`, imaginary unit `i`
- **Functions**: `sqrt()`, `abs()`, `sin()`, `cos()`, `sind()`, `cosd()` (degree variants support complex numbers)
- **Complex Numbers**: Full arithmetic with `a + bi` representation
- **Equation Solving**:
  - Linear: `equation(2x+5=0)`
  - Quadratic: `equation(x^2-5x+6=0)`
  - Cubic: `equation(x^3-6x^2+11x-6=0)`
  - Quartic: `equation(x^4-2=0)` (symbolic or Durand-Kerner fallback)
  - Quintic: `equation(x^5+...)` → `RootOf(polynomial, k)` notation
  - Linear Systems: `equation2(x+y=5,x-y=1)` (up to 3 variables)
- **Output Formatting**: Exact fractions preferred, decimals only when necessary, symbolic notation for unsolvable polynomials

### Output Representation
- **Exact Fractions**: `1/3` stays rational, `2/1` displays as `2`
- **Complex Format**: `a + bi`, `3i`, `-i`, simplified where possible
- **Symbolic Solutions**: Uses `sqrt`, `cbrt`, `RootOf` notation to preserve algebraic structure
- **Pretty Output**: ASCII (default), Unicode (×, ÷, π, √, ², ₀), or LaTeX (via Kitty protocol)

## Important Constraints

### Build Requirements
- **Minimum RAM**: 32GB recommended (SymEngine compilation is memory-intensive)
- **Compiler**: C++20 with clang/clang++ (enforced)
- **Build Time**: First build takes several minutes due to SymEngine compilation
- **Platform**: Cross-platform (Linux, macOS, Windows), but uses clang specifically

### Technical Constraints
- **SymEngine Dependency**: Vendored source in `third-part/symengine/`, no external package manager
- **LaTeX Rendering**: Requires `pdflatex` and terminal supporting Kitty graphics protocol
- **Locale Sensitivity**: Parsing may break with comma decimals; use `LC_ALL=C` if needed
- **Complex Number Precision**: Uses `std::complex<double>`, no arbitrary precision
- **Fraction Limits**: `MAX_DENOMINATOR = 1000000` for rational approximation

### Security and Safety
- **No Network**: No external network calls or API dependencies
- **Local Operations**: All computation happens locally
- **Error Messages**: Return "Error:" prefix and exit code 1 on failure
- **Input Validation**: Parse-time error handling, no code execution

## External Dependencies

### SymEngine (Vendored)
- **Purpose**: Symbolic mathematics library for polynomial operations and equation solving
- **Location**: `third-part/symengine/`
- **Integration**: Built from source, linked statically into `calculator_core`
- **Version**: Tracked in vendored directory

### System Libraries
- **libm**: Math library (linked via `-lm` in CMake)
- **Standard Library**: C++20 STL (std::complex, std::gcd, std::numbers, etc.)

### Optional Dependencies
- **pdflatex**: For LaTeX rendering (optional, detected at runtime)
- **Kitty Terminal Protocol**: For LaTeX graphics output (optional, detected at runtime)
