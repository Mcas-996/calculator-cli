# Design: Rust Refactor Architecture

## Context
The calculator CLI is currently implemented in C++20 with SymEngine for symbolic math. The goal is to refactor the core library to Rust while maintaining all existing functionality and CLI interface. The architecture must support future 100% Rust migration.

## Goals / Non-Goals

### Goals
- Maintain 100% feature parity with C++ implementation
- Eliminate 374MB vendored SymEngine dependency
- Reduce build time from minutes to seconds
- Improve memory safety and reduce undefined behavior risks
- Keep identical CLI interface and behavior
- Support cross-platform builds (Linux, macOS, Windows)
- Enable future complete Rust migration

### Non-Goals
- Adding new mathematical features beyond current scope
- Changing CLI interface or argument structure
- Implementing new output formats beyond ASCII/Unicode/LaTeX
- Removing any existing functionality
- Performance optimization beyond what Rust provides naturally

## Decisions

### 1. Symbolic Math Library Selection
**Decision**: Use `symbolica` crate as SymEngine replacement

**Rationale**:
- Pure Rust implementation (no FFI overhead)
- Comprehensive CAS features matching SymEngine capabilities
- Active development and maintenance
- Polynomial operations, equation solving, symbolic differentiation
- Better integration with Rust ecosystem

**Alternatives considered**:
- `symbolic_polynomials`: Limited scope, only polynomial operations
- `rusymbols`: Less mature, smaller feature set
- Keep SymEngine via FFI: Would retain build complexity and memory overhead

### 2. Project Structure
**Decision**: Hybrid Cargo + existing repo structure during transition

```
calculator-cli/
├── Cargo.toml                 # New Rust project root
├── src/                       # Rust source code
│   ├── main.rs                # CLI entry point (replaces main_cli.cpp)
│   ├── lib.rs                 # Library interface
│   ├── core/
│   │   ├── mod.rs
│   │   ├── complex.rs         # Complex number arithmetic
│   │   ├── fraction.rs        # Fraction operations
│   │   └── types.rs           # Shared types
│   ├── parser/
│   │   ├── mod.rs
│   │   ├── expression.rs      # Expression parsing
│   │   └── tokenizer.rs       # Lexical analysis
│   ├── solver/
│   │   ├── mod.rs
│   │   ├── linear.rs          # Linear equations
│   │   ├── quadratic.rs       # Quadratic equations
│   │   ├── cubic.rs           # Cubic equations
│   │   ├── quartic.rs         # Quartic equations
│   │   ├── quintic.rs         # Quintic equations (via symbolica)
│   │   └── linear_system.rs   # 2x2/3x3 systems
│   ├── output/
│   │   ├── mod.rs
│   │   ├── ascii.rs           # ASCII formatting
│   │   ├── unicode.rs         # Unicode formatting
│   │   ├── latex.rs           # LaTeX generation
│   │   └── pretty.rs          # Unified output interface
│   └── utils/
│       ├── mod.rs
│       └── math.rs            # Math helpers (gcd, approximations)
├── tests/                     # Integration tests
│   ├── integration_test.rs
│   └── examples/
├── benches/                   # Benchmarks (optional)
└── examples/                   # Example usage
```

**Rationale**:
- Clear separation of concerns (core, parser, solver, output)
- Easy to add/remove features
- Follows Rust community conventions
- Supports future 100% Rust migration path

### 3. Core Data Types
**Decision**: Use native Rust types with custom implementations

```rust
// Complex number with exact fraction support
pub struct ComplexNumber {
    pub real: Rational64,      // Use num_rational for exact fractions
    pub imag: Rational64,
}

// Fraction using num-rational crate
pub type Fraction = Rational64;

// Expression AST
pub enum Expression {
    Constant(ComplexNumber),
    Variable(String),
    BinaryOp {
        left: Box<Expression>,
        op: BinaryOperator,
        right: Box<Expression>,
    },
    UnaryOp {
        op: UnaryOperator,
        operand: Box<Expression>,
    },
    Function {
        name: String,
        args: Vec<Expression>,
    },
}
```

**Rationale**:
- `num-rational` provides exact fraction arithmetic
- Native Rust enums for expression AST (type-safe, pattern matching)
- No need for manual memory management

### 4. Error Handling
**Decision**: Use `anyhow` for ergonomic error handling with custom error types

```rust
#[derive(Debug, thiserror::Error)]
pub enum CalculatorError {
    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Evaluation error: {0}")]
    EvaluationError(String),

    #[error("Equation solving error: {0}")]
    SolveError(String),

    #[error("Division by zero")]
    DivisionByZero,

    #[error("Invalid equation format: {0}")]
    InvalidEquation(String),
}

pub type Result<T> = std::result::Result<T, CalculatorError>;
```

**Rationale**:
- `anyhow` provides context for errors
- `thiserror` for error type derivation
- Clear error messages for CLI output

### 5. Output Formatting Strategy
**Decision**: Trait-based polymorphism for output formatters

```rust
pub trait Formatter {
    fn format_complex(&self, num: &ComplexNumber) -> String;
    fn format_fraction(&self, frac: &Fraction) -> String;
    fn format_expression(&self, expr: &Expression) -> String;
    fn format_equation_solution(&self, var: &str, value: &str) -> String;
    fn format_prompt(&self) -> String;
}

pub struct AsciiFormatter;
pub struct UnicodeFormatter;
pub struct LatexFormatter;

// Global configuration
pub struct PrettyConfig {
    level: PrettyLevel,
}
```

**Rationale**:
- Extensible design (add new formatters easily)
- Compile-time dispatch (zero overhead)
- Matches C++ polymorphism pattern

### 6. Equation Solving Strategy
**Decision**: Analytic solutions for low-degree polynomials, symbolic for high-degree

```rust
// Linear: ax + b = 0 → x = -b/a
fn solve_linear(coeffs: &[ComplexNumber]) -> Vec<ComplexNumber>

// Quadratic: ax² + bx + c = 0 → quadratic formula
fn solve_quadratic(coeffs: &[ComplexNumber]) -> Vec<ComplexNumber>

// Cubic: Cardano's formula
fn solve_cubic(coeffs: &[ComplexNumber]) -> Vec<ComplexNumber>

// Quartic: Ferrari's formula (or symbolic)
fn solve_quartic(coeffs: &[ComplexNumber]) -> Vec<ComplexNumber>

// Quintic+: Symbolic via symbolica
fn solve_quintic_symbolic(expr: &Expression) -> Vec<Expression>
```

**Rationale**:
- Exact solutions for solvable polynomials (degree ≤ 4)
- Symbolic for unsolvable (degree ≥ 5, per Abel-Ruffini theorem)
- Matches current C++ behavior

### 7. CLI Interface
**Decision**: Use `clap` crate for argument parsing

```rust
#[derive(Parser, Debug)]
#[command(name = "calculator")]
#[command(about = "A command-line calculator with symbolic math support")]
struct Cli {
    #[arg(short = 'p', long = "pretty")]
    pretty: bool,

    #[arg(short = 'u', long = "unicode")]
    unicode: bool,

    #[arg(short = 'l', long = "latex")]
    latex: bool,

    #[arg(short = 'a', long = "ascii")]
    ascii: bool,

    #[arg(short = 'v', long = "version")]
    version: bool,

    expression: Option<String>,
}
```

**Rationale**:
- Declarative argument parsing
- Automatic help generation
- Type-safe argument handling

### 8. Testing Strategy
**Decision**: Three-tier testing approach

1. **Unit tests**: Inline in source files (`#[cfg(test)]`)
2. **Integration tests**: `tests/` directory
3. **Property-based tests**: `proptest` for numeric edge cases

**Rationale**:
- Rust's built-in test framework is excellent
- Property tests catch edge cases better than manual tests
- Integration tests ensure CLI behavior matches expectations

## Risks / Trade-offs

### Risks

1. **Symbolica API stability**: New crate, potential breaking changes
   - **Mitigation**: Pin to specific version in Cargo.toml, monitor releases

2. **Performance regression**: Initial Rust implementation may be slower
   - **Mitigation**: Benchmark against C++ version, optimize hot paths

3. **Feature gaps**: Symbolica may lack some SymEngine features
   - **Mitigation**: Implement missing features manually or use multiple crates

4. **Complex number precision**: Different precision models between C++ and Rust
   - **Mitigation**: Test edge cases thoroughly, use consistent precision (f64)

### Trade-offs

1. **Build time vs. feature completeness**: Using symbolica vs. implementing manually
   - **Decision**: Use symbolica to maintain feature parity quickly

2. **Binary size vs. dependencies**: More crates = larger binary
   - **Decision**: Accept larger binary for better functionality and maintainability

3. **Zero-copy vs. ergonomic API**: Passing strings by reference vs. owned
   - **Decision**: Prioritize ergonomics (owned strings) where performance impact is minimal

## Migration Plan

### Phase 1: Foundation (Week 1)
1. Create `Cargo.toml` with dependencies
2. Set up project structure
3. Implement core types (ComplexNumber, Fraction, Expression)
4. Implement basic arithmetic operations

### Phase 2: Parser (Week 2)
1. Implement tokenizer
2. Implement expression parser
3. Add operator precedence handling
4. Add function parsing (sin, cos, sqrt, etc.)

### Phase 3: Solvers (Week 3)
1. Implement linear equation solver
2. Implement quadratic solver
3. Implement cubic solver
4. Implement quartic solver
5. Integrate symbolica for quintic+ equations

### Phase 4: Output Formatting (Week 4)
1. Implement ASCII formatter
2. Implement Unicode formatter
3. Implement LaTeX generator
4. Implement pretty output configuration
5. Add LaTeX external tool support

### Phase 5: CLI Integration (Week 5)
1. Implement CLI argument parsing with clap
2. Port main_cli.cpp logic to main.rs
3. Add interactive mode
4. Add error handling and user feedback

### Phase 6: Testing & Refinement (Week 6)
1. Port all C++ tests to Rust
2. Add integration tests
3. Property tests for edge cases
4. Performance benchmarks
5. Bug fixes and optimization

### Phase 7: Cleanup (Week 7)
1. Remove C++ code
2. Remove CMakeLists.txt
3. Remove third-part/symengine/
4. Update documentation
5. Final testing and validation

### Rollback Plan
If critical bugs are discovered post-migration:
1. Keep C++ code in a git branch for reference
2. Can revert to C++ version by switching branches
3. Document migration decision points for future review

## Open Questions

1. **LaTeX rendering**: Continue using external pdflatex or switch to Rust library?
   - **Option A**: External tool (current approach, simpler)
   - **Option B**: Use `tectonic` crate (pure Rust LaTeX)
   - **Recommendation**: Start with external, evaluate `tectonic` later

2. **Number of crates to use**:
   - Keep dependencies minimal (better maintainability)
   - Use best-in-class crates (better functionality)
   - **Recommendation**: Use well-established crates (clap, anyhow, thiserror, num-rational, symbolica)

3. **Interactive mode implementation**:
   - Use `crossterm` or `termion` for terminal handling?
   - Or stick with `std::io` for simplicity?
   - **Recommendation**: Use `std::io` initially, add terminal libs if needed

4. **Rust edition**: 2021 (current stable) or wait for 2024?
   - **Recommendation**: Use 2021 edition (stable, well-tested)

5. **MSRV (Minimum Supported Rust Version)**:
   - **Recommendation**: 1.70.0 (released July 2023, good balance of features and compatibility)
