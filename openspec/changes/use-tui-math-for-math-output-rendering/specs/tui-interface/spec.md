## MODIFIED Requirements

### Requirement: Result Card Display
The system SHALL display calculation results as styled math cards, using `tui-math` rendering for supported mathematical structures.

#### Scenario: Display expression result
- **WHEN** user enters expression `2 + 2` and executes
- **THEN** a result card is added to the upper area showing `2 + 2 = 4` with `tui-math` rendering when applicable

#### Scenario: Display equation solution
- **WHEN** user enters equation `x^2 - 4 = 0` and executes
- **THEN** a result card is added showing equation solutions using `tui-math` layout for mathematical terms

#### Scenario: Fraction rendering via tui-math
- **WHEN** displaying a fraction like `-5/2`
- **THEN** the fraction is rendered using `tui-math` fraction layout instead of ad-hoc string assembly

#### Scenario: Radical rendering via tui-math
- **WHEN** displaying a radical like `sqrt(2)`
- **THEN** the radical is rendered using `tui-math` radical formatting

#### Scenario: Multiple solutions display
- **WHEN** equation has multiple solutions
- **THEN** solutions are displayed with stable ordering and readable math formatting in a single result card

#### Scenario: Fallback when tui-math cannot render expression
- **WHEN** a computed result cannot be represented by `tui-math`
- **THEN** the result card falls back to deterministic text formatting without losing mathematical meaning
