## MODIFIED Requirements

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
