# Spec: Linear Systems Solving

## ADDED Requirements

### Requirement: 2x2 Linear System Solving
The system SHALL solve systems of 2 linear equations with 2 variables.

#### Scenario: Simple 2x2 system
- **WHEN** solving `"equation2(x+y=5,x-y=1)"`
- **THEN** result is `"x = 3, y = 2"`

#### Scenario: 2x2 with fractional solutions
- **WHEN** solving `"equation2(2x+3y=12,4x-y=5)"`
- **THEN** result shows fractions if applicable

#### Scenario: 2x2 with negative solutions
- **WHEN** solving a system with negative solutions
- **THEN** negative values are displayed correctly

#### Scenario: 2x2 with decimal coefficients
- **WHEN** solving `"equation2(1.5x+2y=10, x-y=3)"`
- **THEN** solutions handle decimal coefficients

#### Scenario: 2x2 zero coefficient
- **WHEN** solving `"equation2(x=3, y=5)"`
- **THEN** result is `"x = 3, y = 5"`

### Requirement: 3x3 Linear System Solving
The system SHALL solve systems of 3 linear equations with 3 variables.

#### Scenario: Simple 3x3 system
- **WHEN** solving `"equation2(x+y+z=6,x-y+z=2,2x+y-z=3)"`
- **THEN** result is `"x = 1, y = 2, z = 3"`

#### Scenario: 3x3 with fractional solutions
- **WHEN** solving a 3x3 system with fractional answers
- **THEN** fractions are displayed where exact

#### Scenario: 3x3 with zero variables
- **WHEN** solving `"equation2(x=1,y=2,z=3)"`
- **THEN** result is `"x = 1, y = 2, z = 3"`

#### Scenario: 3x3 with mixed coefficients
- **WHEN** solving a 3x3 with various coefficient values
- **THEN** solutions are computed correctly

### Requirement: Linear System Parsing
The system SHALL parse linear system input strings correctly.

#### Scenario: Parse 2x2 format
- **WHEN** parsing `"x+y=5,x-y=1"`
- **THEN** two equations are extracted

#### Scenario: Parse 3x3 format
- **WHEN** parsing `"x+y+z=6,x-y+z=2,2x+y-z=3"`
- **THEN** three equations are extracted

#### Scenario: Parse multiple variables
- **WHEN** parsing system with different variables
- **THEN** all variables are recognized

#### Scenario: Extract coefficients
- **WHEN** parsing `"2x+3y=12,4x-y=5"`
- **THEN** coefficient matrix `[[2,3],[4,-1]]` is extracted

#### Scenario: Extract constant terms
- **WHEN** parsing `"x+y=5,x-y=1"`
- **THEN** constant vector `[5,1]` is extracted

#### Scenario: Parse with spaces
- **WHEN** parsing `"x + y = 5, x - y = 1"`
- **THEN** spaces are ignored and equations parsed correctly

#### Scenario: Parse without spaces
- **WHEN** parsing `"x+y=5,x-y=1"`
- **THEN** equations are parsed correctly

#### Scenario: Invalid system format
- **WHEN** parsing `"x+y+z=6,x-y+z=2"` (only 2 equations for 3 variables)
- **THEN** error message indicates mismatch in equation count

### Requirement: Gaussian Elimination
The system SHALL use Gaussian elimination to solve linear systems.

#### Scenario: Forward elimination
- **WHEN** solving a 3x3 system
- **THEN** matrix is transformed to upper triangular form

#### Scenario: Back substitution
- **WHEN** upper triangular matrix is obtained
- **THEN** variables are solved from bottom to top

#### Scenario: Row operations
- **WHEN** performing elimination
- **THEN** row operations preserve solution set

#### Scenario: Partial pivoting
- **WHEN** a diagonal element is zero
- **THEN** rows are swapped to avoid division by zero

### Requirement: Singular Matrix Detection
The system SHALL detect singular matrices and handle them appropriately.

#### Scenario: No unique solution (dependent equations)
- **WHEN** solving `"x+y=1, 2x+2y=2"`
- **THEN** error message indicates infinite solutions

#### Scenario: No solution (inconsistent equations)
- **WHEN** solving `"x+y=1, x+y=2"`
- **THEN** error message indicates no solution

#### Scenario: Singular 2x2 matrix
- **WHEN** solving a 2x2 system with determinant zero
- **THEN** appropriate error message is returned

#### Scenario: Singular 3x3 matrix
- **WHEN** solving a 3x3 system with determinant zero
- **THEN** appropriate error message is returned

### Requirement: System Solution Formatting
The system SHALL format linear system solutions in a clear format.

#### Scenario: 2x2 solution format
- **WHEN** displaying 2-variable solution
- **THEN** format is `"x = value1, y = value2"`

#### Scenario: 3x3 solution format
- **WHEN** displaying 3-variable solution
- **THEN** format is `"x = value1, y = value2, z = value3"`

#### Scenario: Subscript notation
- **WHEN** using Unicode output mode
- **THEN** variables with indices use subscripts

#### Scenario: Fraction preservation
- **WHEN** solution is rational
- **THEN** exact fraction is displayed

#### Scenario: Zero values
- **WHEN** a variable solution is zero
- **THEN** it is displayed as `"0"`

#### Scenario: Negative values
- **WHEN** a variable solution is negative
- **THEN** minus sign is displayed correctly

### Requirement: Variable Detection
The system SHALL detect and extract variables from linear system equations.

#### Scenario: Detect variable x
- **WHEN** parsing equation with `x`
- **THEN** variable `x` is recognized

#### Scenario: Detect variable y
- **WHEN** parsing equation with `y`
- **THEN** variable `y` is recognized

#### Scenario: Detect variable z
- **WHEN** parsing equation with `z`
- **THEN** variable `z` is recognized

#### Scenario: Detect variable w
- **WHEN** parsing equation with `w`
- **THEN** variable `w` is recognized

#### Scenario: Order variables alphabetically
- **WHEN** solving system with variables in any order
- **THEN** solutions are displayed in alphabetical order

### Requirement: System Size Validation
The system SHALL validate that the system is properly sized.

#### Scenario: Validate 2 equations, 2 variables
- **WHEN** solving `"equation2(x+y=5,x-y=1)"`
- **THEN** system is recognized as 2x2

#### Scenario: Validate 3 equations, 3 variables
- **WHEN** solving `"equation2(x+y+z=6,x-y+z=2,2x+y-z=3)"`
- **THEN** system is recognized as 3x3

#### Scenario: Reject over-determined system
- **WHEN** solving system with more equations than variables
- **THEN** error message indicates over-determined system

#### Scenario: Reject under-determined system
- **WHEN** solving system with fewer equations than variables
- **THEN** error message indicates under-determined system

#### Scenario: Reject 4+ variable system
- **WHEN** solving system with 4 or more variables
- **THEN** error message indicates system size not supported

### Requirement: Linear System Error Handling
The system SHALL provide clear error messages for invalid linear systems.

#### Scenario: Invalid syntax
- **WHEN** parsing `"x+y, x-y=1"` (missing equals in first)
- **THEN** error message describes invalid syntax

#### Scenario: Too few equations
- **WHEN** solving `"equation2(x=1)"`
- **THEN** error message indicates insufficient equations

#### Scenario: Too many variables
- **WHEN** solving 4-variable system
- **THEN** error message indicates variable limit exceeded

#### Scenario: Singular matrix
- **WHEN** solving singular system
- **THEN** error message indicates no unique solution

#### Scenario: Inconsistent system
- **WHEN** solving contradictory equations
- **THEN** error message indicates no solution exists
