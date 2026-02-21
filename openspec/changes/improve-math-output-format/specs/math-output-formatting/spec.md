## ADDED Requirements

### Requirement: Irrational results SHALL render as exact radical expressions
The system SHALL render square-root-based irrational values in simplified exact radical form instead of high-denominator rational approximations.

#### Scenario: Direct square root stays symbolic
- **WHEN** the user evaluates `sqrt(2)`
- **THEN** the displayed result is `sqrt(2)` as the primary output
- **THEN** the display does not replace `sqrt(2)` with a large rational approximation

#### Scenario: Simplifiable radical coefficients are normalized
- **WHEN** the user evaluates an expression equivalent to `sqrt(8)`
- **THEN** the displayed result is the simplified exact form `2*sqrt(2)` (or equivalent canonical radical form)

### Requirement: Complex results SHALL render in canonical `a + bi` style with exact components
The system SHALL render complex values as syntactically valid single expressions with normalized signs and exact rational/radical parts.

#### Scenario: Pure imaginary quadratic roots keep exact radical part
- **WHEN** the user solves `x^2 = -2`
- **THEN** the two roots are displayed as exact conjugates equivalent to `i*sqrt(2)` and `-i*sqrt(2)`
- **THEN** no malformed stacked or fragmented text is shown

#### Scenario: Mixed real and imaginary roots are formatted consistently
- **WHEN** the user solves `x^2 + 2x + 10 = 0`
- **THEN** the roots are displayed as `-1 + 3i` and `-1 - 3i` (or equivalent exact form with canonical sign placement)

### Requirement: Exact math formatting SHALL be consistent across output surfaces
The system SHALL use the same canonical formatter for result cards and last-result summaries for the same computed value.

#### Scenario: Last result matches selected displayed root
- **WHEN** a complex root is produced and stored as the last result
- **THEN** the last-result summary text is mathematically equivalent to the selected root shown in the results panel
- **THEN** formatting conventions for signs and exact components are consistent between both surfaces

### Requirement: Existing exact rational and integer formatting SHALL remain stable
The system SHALL preserve current integer and rational rendering behavior for values that are already exact without radicals or imaginary parts.

#### Scenario: Integer arithmetic output remains unchanged
- **WHEN** the user evaluates `1+1`
- **THEN** the displayed result remains `2`

#### Scenario: Rational output remains exact rational
- **WHEN** the user evaluates an expression equivalent to one-third
- **THEN** the displayed result remains an exact rational representation
