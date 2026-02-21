# expression-evaluation Specification

## Purpose
TBD - created by archiving change refactor-to-rust. Update Purpose after archive.
## Requirements
### Requirement: Expression Parsing
The system SHALL parse mathematical expressions from strings into an abstract syntax tree, including binary subtraction when the `-` operator is contiguous with adjacent operands.

#### Scenario: Simple arithmetic expression
- **WHEN** parsing `"3 + 5 * 2"`
- **THEN** the AST respects operator precedence (multiplication before addition)

#### Scenario: Parentheses grouping
- **WHEN** parsing `"3 + 5 * (2 - 8)^2"`
- **THEN** parentheses are evaluated first, then exponent, then multiplication

#### Scenario: Unary minus
- **WHEN** parsing `"-2.5 * 4 + 3^2"`
- **THEN** the unary minus is applied to `2.5`

#### Scenario: Percentage handling
- **WHEN** parsing `"50% * 200"`
- **THEN** the percentage is converted to decimal `0.5` before multiplication

#### Scenario: Function call with parentheses
- **WHEN** parsing `"sqrt(16) + 3"`
- **THEN** the function `sqrt` is recognized and called with argument `16`

#### Scenario: Complex number literal
- **WHEN** parsing `"(3+2i) * (1-i)"`
- **THEN** both complex numbers are parsed correctly

#### Scenario: Constant substitution
- **WHEN** parsing `"pi * 2"`
- **THEN** `pi` is replaced with its numeric value

#### Scenario: Exponentiation operator
- **WHEN** parsing `"2^3"`
- **THEN** it is evaluated as `8`

#### Scenario: Modulo operation
- **WHEN** parsing `"10 % 3"`
- **THEN** it is evaluated as `1`

#### Scenario: Contiguous subtraction without spaces
- **WHEN** parsing `"44-55"`
- **THEN** `-` is parsed as a binary subtraction operator between two numeric operands

#### Scenario: Contiguous subtraction in grouped expression
- **WHEN** parsing `"2*(8-3)"`
- **THEN** subtraction inside parentheses is parsed correctly without requiring surrounding whitespace

### Requirement: Expression Evaluation
The system SHALL evaluate parsed expressions and return results as strings.

#### Scenario: Evaluate simple arithmetic
- **WHEN** evaluating `"3 + 5 * 2"`
- **THEN** the result is `"13"`

#### Scenario: Evaluate with parentheses
- **WHEN** evaluating `"3 + 5 * (2 - 8)^2"`
- **THEN** the result is `"183"`

#### Scenario: Evaluate square root
- **WHEN** evaluating `"sqrt(16) + 3"`
- **THEN** the result is `"7"`

#### Scenario: Evaluate sine in radians
- **WHEN** evaluating `"sin(pi / 6)"`
- **THEN** the result is approximately `"1/2"` (exact fraction)

#### Scenario: Evaluate sine in degrees
- **WHEN** evaluating `"sind(30)"`
- **THEN** the result is `"1/2"`

#### Scenario: Evaluate cosine in radians
- **WHEN** evaluating `"cos(pi)"`
- **THEN** the result is `"-1"`

#### Scenario: Evaluate cosine in degrees
- **WHEN** evaluating `"cosd(180)"`
- **THEN** the result is `"-1"`

#### Scenario: Evaluate exponential
- **WHEN** evaluating `"e^2"`
- **THEN** the result is approximately `7.389`

#### Scenario: Evaluate percentage
- **WHEN** evaluating `"50% * 200"`
- **THEN** the result is `"100"`

#### Scenario: Evaluate complex expression
- **WHEN** evaluating `"(3+2i) * (1-i)"`
- **THEN** the result is `"5 - i"`

#### Scenario: Evaluate absolute value
- **WHEN** evaluating `"abs(-5)"`
- **THEN** the result is `"5"`

### Requirement: Mathematical Functions
The system SHALL support a set of mathematical functions for expression evaluation.

#### Scenario: Square root function
- **WHEN** evaluating `"sqrt(9)"`
- **THEN** the result is `"3"`

#### Scenario: Square root of negative
- **WHEN** evaluating `"sqrt(-9)"`
- **THEN** the result is `"3i"`

#### Scenario: Absolute value function
- **WHEN** evaluating `"abs(-3.14)"`
- **THEN** the result is `"3.14"`

#### Scenario: Sine function (radians)
- **WHEN** evaluating `"sin(0)"`
- **THEN** the result is `"0"`

#### Scenario: Cosine function (radians)
- **WHEN** evaluating `"cos(0)"`
- **THEN** the result is `"1"`

#### Scenario: Sine function (degrees)
- **WHEN** evaluating `"sind(90)"`
- **THEN** the result is `"1"`

#### Scenario: Cosine function (degrees)
- **WHEN** evaluating `"cosd(0)"`
- **THEN** the result is `"1"`

#### Scenario: Complex sine
- **WHEN** evaluating `"sin(i)"`
- **THEN** the result is a complex number

### Requirement: Operator Precedence
The system SHALL follow standard mathematical operator precedence rules.

#### Scenario: Exponentiation before multiplication
- **WHEN** evaluating `"2 * 3^2"`
- **THEN** the result is `"18"` (not `36`)

#### Scenario: Multiplication before addition
- **WHEN** evaluating `"3 + 5 * 2"`
- **THEN** the result is `"13"` (not `16`)

#### Scenario: Parentheses override precedence
- **WHEN** evaluating `"(3 + 5) * 2"`
- **THEN** the result is `"16"`

#### Scenario: Left-to-right for equal precedence
- **WHEN** evaluating `"10 - 3 - 2"`
- **THEN** the result is `"5"` (not `9`)

### Requirement: Error Handling
The system SHALL provide clear error messages for invalid expressions.

#### Scenario: Mismatched parentheses
- **WHEN** evaluating `"(3 + 5 * 2"`
- **THEN** the result starts with `"Error:"` and describes the syntax error

#### Scenario: Invalid operator
- **WHEN** evaluating `"3 @ 5"`
- **THEN** the result starts with `"Error:"` and describes the invalid operator

#### Scenario: Unknown function
- **WHEN** evaluating `"unknown(5)"`
- **THEN** the result starts with `"Error:"` and indicates the unknown function

#### Scenario: Division by zero
- **WHEN** evaluating `"1 / 0"`
- **THEN** the result starts with `"Error:"` and indicates division by zero

### Requirement: Fraction Preservation
The system SHALL prefer exact fraction representation over decimals when possible.

#### Scenario: Simple fraction result
- **WHEN** evaluating `"1/3"`
- **THEN** the result is `"1/3"` (not `"0.333333"`)

#### Scenario: Fraction arithmetic result
- **WHEN** evaluating `"1/2 + 1/3"`
- **THEN** the result is `"5/6"`

#### Scenario: Integer fraction result
- **WHEN** evaluating `"4/2"`
- **THEN** the result is `"2"` (not `"2/1"`)

#### Scenario: Complex number with fractions
- **WHEN** evaluating `"1/3 + (1/2)i"`
- **THEN** the result is `"1/3 + 1/2i"`
