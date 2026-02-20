## MODIFIED Requirements

### Requirement: Equation Format Parsing
The system SHALL parse equation strings and extract polynomial coefficients correctly, supporting flexible equation formats by automatically rearranging to standard form.

#### Scenario: Parse simple quadratic (standard form)
- **WHEN** parsing `"x^2-5x+6=0"`
- **THEN** coefficients are extracted as `[1, -5, 6]`

#### Scenario: Parse quadratic with non-zero RHS
- **WHEN** parsing `"x^2-5x=-6"`
- **THEN** equation is normalized to `"x^2-5x+6=0"` and coefficients are `[1, -5, 6]`

#### Scenario: Parse equation with variable equals value
- **WHEN** parsing `"x=0"`
- **THEN** equation is normalized to `"x=0"` and coefficients are `[1, 0]`

#### Scenario: Parse equation with variable equals non-zero
- **WHEN** parsing `"x=5"`
- **THEN** equation is normalized to `"x-5=0"` and coefficients are `[1, -5]`

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

#### Scenario: Equation with multiple equals signs
- **WHEN** parsing `"x=0=0"`
- **THEN** an error message describes the invalid syntax

#### Scenario: Equation with complex RHS
- **WHEN** parsing `"2x+3=7"`
- **THEN** equation is normalized to `"2x-4=0"` and solved correctly

#### Scenario: Equation wrapper syntax with simple assignment
- **WHEN** parsing `"equation(x=2)"`
- **THEN** wrapper is stripped, equation is normalized to `"x-2=0"` and solution is `x = 2`

#### Scenario: Equation wrapper syntax with quadratic
- **WHEN** parsing `"equation(x^2+1=0)"`
- **THEN** wrapper is stripped and equation is solved with complex solutions `x = i` and `x = -i`

#### Scenario: Complex solutions for quadratics
- **WHEN** solving `"x^2+1=0"`
- **THEN** solutions include complex numbers `x = i` and `x = -i`

#### Scenario: Complex solutions for other quadratics
- **WHEN** solving `"x^2-4x+5=0"`
- **THEN** solutions are `x = 2 + i` and `x = 2 - i`