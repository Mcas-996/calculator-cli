# Change: Refactor Core Calculator to Rust

## Why
The current C++20 implementation has several limitations:
- Heavy dependency on vendored SymEngine (requires 32GB+ RAM to build, multi-minute compilation)
- Complex build system with CMake and clang-specific requirements
- Limited safety guarantees (manual memory management, undefined behavior risks)
- Large third-party codebase in-tree (~374MB vendored SymEngine)
- Slower development iteration due to long build times

Refactoring to Rust provides:
- Memory safety without garbage collection
- Modern tooling (Cargo, rustfmt, clippy)
- Faster compilation and smaller binaries
- Elimination of heavy SymEngine dependency
- Better cross-platform support
- Future-proof architecture for 100% Rust migration

## What Changes
- **BREAKING**: Replace C++ core library with Rust implementation
- Replace vendored SymEngine with `symbolica` crate (Rust CAS library)
- Port core modules to Rust:
  - Complex number arithmetic
  - Fraction operations and rational approximation
  - Expression parsing and evaluation
  - Equation solving (linear, quadratic, cubic, quartic, quintic)
  - Linear systems solver (2x2, 3x3)
  - Pretty output formatting (ASCII/Unicode/LaTeX)
  - Unicode formatter
  - LaTeX renderer (via external tool invocation)
- Remove CMake build system, replace with Cargo
- Remove CI/CD workflows as requested
- Maintain identical CLI interface and all features
- Keep interactive mode and command-line arguments
- Preserve all mathematical functionality and output formats

## Impact
- **Affected specs**: All capabilities (will be defined in this proposal)
- **Affected code**:
  - Core library: All `.hpp` and `.cpp` files except `main_cli.cpp`
  - Build system: `CMakeLists.txt` replaced by `Cargo.toml`
  - CI/CD: `.github/workflows/c-cpp.yml` removed
  - Third-party: `third-part/symengine/` removed (~374MB saved)
- **Compatibility**: External API unchanged (CLI arguments, input format, output format)
- **Performance**: Expected improvement due to Rust's optimization and reduced overhead
- **Build time**: Drastically reduced (from minutes to seconds, no SymEngine compilation)
- **Binary size**: Smaller due to static linking and Rust's optimization
