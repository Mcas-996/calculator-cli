# equation-solving Specification

## Purpose
TBD - created by archiving change refactor-to-rust. Update Purpose after archive.
## Requirements
### Requirement: Linear Equation Solving
The system SHALL solve linear equations in the form `ax + b = 0` and return the solution for `x`.

#### Scenario: Simple linear equation
- **WHEN** solving `"equation(2x+5=0)"`
- **THEN** result is `"x = -5/2"`

#### Scenario: Linear equation with coefficient
- **WHEN** solving `"equation(3x-6=0)"`
- **THEN**- result is `"x = 2"`

#### Scenario: Linear equation negative solution
- **WHEN** solving `"equation(x+1=0)"`
- **THEN** result is `"x = -1"`

#### Scenario: Linear equation invalid format
- **WHEN** solving `"equation(x^2+2x+1=0)"`
- **THEN** it is recognized as quadratic, not linear

### Requirement: Quadratic Equation Solving
The system SHALL solve quadratic equations in the form `ax² + bx + c = 0` and return one or two real or complex solutions.

#### Scenario: Two real solutions
- **WHEN** solving `"equation(x^2-5x+6=0)"`
- **THEN** result is `"x1 = 2, x2 = 3"`

#### Scenario: One repeated real solution
- **WHEN** solving `"equation(x^2+2x+1=0)"`
- **THEN**- result is `"x = -1"` (double root)

#### Scenario: Complex solutions
- **WHEN** solving `"equation(x^2+1=0)"`
- **THEN**- result is `"x1 = i, x2 = -i"`

#### Scenario: Fractional coefficients
- **WHEN** solving `"equation(2x^2-3x+1=0)"`
- **THEN** result shows fractional solutions

#### Scenario: Negative discriminant
- **WHEN** solving `"equation(x^2+4x+5=0)"`
- **THEN** result shows complex conjugate solutions

#### Scenario: Leading coefficient not 1
- **WHEN** solving `"equation(3x^2+2x-1=0)"`
- **THEN** result correctly handles non-unity leading coefficient

### Requirement: Cubic Equation Solving
The system SHALL solve cubic equations in the form `ax³ + bx² + cx + d = 0` using Cardano's formula and return up to three solutions.

#### Scenario: Three real solutions
- **WHEN** solving `"equation(x^3-6x^2+11x-6=0)"`
- **THEN** result is `"x1 = 1, x2 = 2, x3 = 3"`

#### Scenario: One real, two complex solutions
- **WHEN** solving `"equation(x^3-1=0)"`
- **THEN** result is `"x = 1"` with complex solutions shown

#### Scenario: Fractional solutions
- **WHEN** solving a cubic with rational roots
- **THEN** results are shown as fractions where applicable

#### Scenario: Use of subscripts
- **WHEN** formatting cubic solutions
- **THEN** subscripts `x₁, x₂, x₃` are used (Unicode mode)

### Requirement: Quartic Equation Solving
The system SHALL solve quartic equations in the form `ax⁴ + bx³ + cx² + dx + e = 0` and return up to four solutions.

#### Scenario: Simple quartic
- **WHEN** solving `"equation(x^4-2=0)"`
- **THEN** result uses symbolic notation with square roots

#### Scenario: Four real solutions
- **WHEN** solving `"equation(x^4-5x^2+4=0)"`
- **THEN** result shows four real solutions

#### Scenario: Complex solutions
- **WHEN** solving `"equation(x^4+1=0)"`
- **THEN** result shows complex solutions

#### Scenario: Use of cube roots
- **WHEN** solving certain quartic equations
- **THEN** result uses `cbrt` notation

#### Scenario: Fallback to numeric solver
- **WHEN** symbolic solution is too complex
- **THEN** numeric Durand-Kerner method provides approximate solutions

### Requirement: Quintic Equation Solving
The system SHALL solve quintic and higher-degree equations symbolically and return `RootOf(polynomial, k)` notation.

#### Scenario: Quintic equation
- **WHEN** solving `"equation(x^5+2x^4+3x^3+4x^2-1=0)"`
- **THEN** result is `"x = RootOf(x^5+2x^4+3x^3+4x^2-1, k)"`

#### Scenario: Root index specification
- **WHEN** displaying quintic solutions
- **THEN** each root is indexed as `RootOf(polynomial, 0)`, `RootOf(polynomial, 1)`, etc.

#### Scenario: Numeric approximation
- **WHEN** solving quintic equation
- **THEN** symbolic output includes numeric approximations for reference

#### Scenario: Abel-Ruffini theorem respect
- **WHEN** attempting to solve quintic equation
- **THEN** symbolic notation is used (no attempt at radical solution)

### Requirement: Equation Format Parsing
The system SHALL parse equation strings and extract polynomial coefficients correctly.

#### Scenario: Parse simple quadratic
- **WHEN** parsing `"x^2-5x+6=0"`
- **THEN** coefficients are extracted as `[1, -5, 6]`

#### Scenario: Parse with missing terms
- **WHEN** parsing `"x^2+1=0"`
- **THEN** missing coefficient is zero `[1, 0, 1]`

#### Scenario: Parse negative coefficients
- **WHEN** parsing `"x^2-2x-3=0"`
- **THEN** coefficients are `[1, -2, -3]`

#### Scenario: Parse fractional coefficients
- **WHEN** parsing `"(1/2)x^2-(1/3)x+1=0"`
- **THEN** coefficients handle fractions correctly

#### Scenario: Parse non-standard variable
- **WHEN** parsing equation with variable other than `x`
- **THEN** it is recognized and solved for that variable

#### Scenario: Invalid equation format
- **WHEN** parsing `"x^2++1=0"`
- **THEN** an error message describes the invalid syntax

### Requirement: Solution Formatting
The system SHALL format equation solutions in a clear, user-friendly manner.

#### Scenario: Single solution format
- **WHEN** displaying one solution
- **THEN** format is `"x = value"`

#### Scenario: Multiple solutions format
- **WHEN** displaying multiple solutions
- **THEN** format is `"x1 = value1, x2 = value2, ..."`

#### Scenario: Subscript notation
- **WHEN** using Unicode output mode
- **THEN** variables use subscripts: `x₁, x₂, x₃`

#### Scenario: Exact fractions
- **WHEN** solution is rational
- **THEN** exact fraction is displayed: `"x = 3/2"`

#### Scenario: Complex number format
- **WHEN** solution is complex
- **THEN** format is `"a + bi"` or `"a - bi"`

#### Scenario: Simplified imaginary
- **WHEN** imaginary part coefficient is 1
- **THEN** format is `"a + i"` or `"a - i"`

#### Scenario: Pure imaginary
- **WHEN** real part is zero
- **THEN** format is `"bi"` or `"-bi"` or `"i"` or `"-i"`

### Requirement: Equation Detection
The system SHALL automatically detect equation type and route to appropriate solver.

#### Scenario: Detect linear equation
- **WHEN** input is `"equation(2x+5=0)"`
- **THEN** linear solver is invoked

#### Scenario: Detect quadratic equation
- **WHEN** input is `"equation(x^2+2x+1=0)"`
- **THEN** quadratic solver is invoked

#### Scenario: Detect cubic equation
- **WHEN** input is `"equation(x^3-6x^2+11x-6=0)"`
- **THEN** cubic solver is invoked

#### Scenario: Detect quartic equation
- **WHEN** input is `"equation(x^4-2=0)"`
- **THEN** quartic solver is invoked

#### Scenario: Detect quintic equation
- **WHEN** input is `"equation(x^5+...)"`
- **THEN** symbolic quintic solver is invoked

#### Scenario: Distinguish from expression
- **WHEN** input is `"x^2+2x+1"` (no equals sign)
- **THEN** it is treated as expression evaluation, not equation solving

