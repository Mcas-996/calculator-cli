# quadratic-solution-correctness Specification

## Purpose
TBD - created by archiving change fix-quadratic-complex-output. Update Purpose after archive.
## Requirements
### Requirement: Quadratic coefficient normalization preserves mathematical equivalence
The system SHALL normalize user-entered quadratic equations to canonical `ax^2 + bx + c = 0` form without introducing or mutating coefficients beyond mathematically valid term movement and simplification.

#### Scenario: Standard-form quadratic remains unchanged
- **WHEN** the user solves `x^2 + 2x + 1 = 0`
- **THEN** the canonical coefficients used by the quadratic solver are `a=1`, `b=2`, `c=1`

#### Scenario: Non-zero right-hand side is moved correctly
- **WHEN** the user solves `x^2 + 2x = -1`
- **THEN** the canonical coefficients used by the quadratic solver are `a=1`, `b=2`, `c=1`

### Requirement: Quadratic roots are computed from canonical coefficients
The system SHALL compute quadratic roots directly from normalized coefficients and return mathematically correct repeated-real or complex-conjugate solutions.

#### Scenario: Repeated real root is correct
- **WHEN** the user solves `x^2 + 2x + 1 = 0`
- **THEN** the reported solution is `x = -1` (or equivalent repeated root representation)

#### Scenario: Complex conjugate roots are correct
- **WHEN** the user solves a quadratic with negative discriminant such as `x^2 + 2x + 10 = 0`
- **THEN** the reported solutions are the complex conjugate pair `x = -1 + 3i` and `x = -1 - 3i` (or equivalent exact form)

### Requirement: Complex quadratic results render consistently across result surfaces
The system SHALL render complex quadratic roots in valid human-readable form for both the results panel and the last-result summary, with no malformed stacked fragments or duplicated operators.

#### Scenario: Results panel shows stable complex form
- **WHEN** a quadratic solve yields complex roots
- **THEN** each root is displayed as a complete single expression in `a +/- bi` style or equivalent exact fractional form

#### Scenario: Last-result summary matches selected root expression
- **WHEN** the system stores and displays the last result for a complex quadratic solution
- **THEN** the summary contains a syntactically valid complex value consistent with the computed root

