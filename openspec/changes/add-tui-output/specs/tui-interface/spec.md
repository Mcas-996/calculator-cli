# Spec: TUI Interface

## ADDED Requirements

### Requirement: TUI Application Startup
The system SHALL start a terminal user interface when entering interactive mode.

#### Scenario: Start TUI without arguments
- **WHEN** no expression is provided and interactive mode is entered
- **THEN** TUI application starts and displays the main interface

#### Scenario: TUI layout structure
- **WHEN** TUI is running
- **THEN** the screen is divided into:
  - Upper area: scrollable list of result cards
  - Lower area: fixed input box

### Requirement: Result Card Display
The system SHALL display calculation results as styled "LaTeX cards".

#### Scenario: Display expression result
- **WHEN** user enters expression `2 + 2` and executes
- **THEN** a result card is added to the upper area showing `2 + 2 = 4`

#### Scenario: Display equation solution
- **WHEN** user enters equation `x^2 - 4 = 0` and executes
- **THEN** a result card is added showing LaTeX-style solution with proper fraction rendering

#### Scenario: LaTeX-style fraction rendering
- **WHEN** displaying a fraction like `-5/2`
- **THEN** it is rendered as:
  ```
      -5
  ─────
      2
  ```

#### Scenario: LaTeX-style radical rendering
- **WHEN** displaying a radical like `√2`
- **THEN** it uses the proper radical symbol

#### Scenario: Multiple solutions display
- **WHEN** equation has multiple solutions
- **THEN** solutions are displayed in set notation, e.g., `{-2, 2}`

### Requirement: Input Handling
The system SHALL handle multi-line input in the TUI.

#### Scenario: Single-line execution
- **WHEN** user enters expression and presses Ctrl+Enter
- **THEN** the expression is evaluated and result is displayed

#### Scenario: Multi-line input with Shift+Enter
- **WHEN** user presses Shift+Enter in input box
- **THEN** a newline is inserted and cursor moves to new line

#### Scenario: Multi-line execution
- **WHEN** user has entered multiple lines and presses Ctrl+Enter
- **THEN** all lines are concatenated and evaluated as single expression

#### Scenario: Empty input
- **WHEN** user presses Ctrl+Enter with empty input
- **THEN** nothing happens, input remains empty

### Requirement: ans Variable
The system SHALL save the result of the last calculation as `ans`.

#### Scenario: Save expression result
- **WHEN** user evaluates expression `3 + 5`
- **THEN** the variable `ans` is set to `8`

#### Scenario: Save linear equation solution
- **WHEN** user solves linear equation `x + 5 = 10`
- **THEN** the variable `ans` is set to the value of `x` (e.g., `5`)

#### Scenario: Save polynomial equation solution
- **WHEN** user solves polynomial equation like `x^2 - 4 = 0`
- **THEN** the variable `ans` is set to the first solution `x1` (e.g., `-2`)

#### Scenario: Use ans in expression
- **WHEN** user enters expression containing `ans`
- **THEN** `ans` is replaced with its stored value before evaluation

### Requirement: Result History
The system SHALL maintain a scrollable history of results.

#### Scenario: Scroll through results
- **WHEN** there are more than screen can display
- **THEN** user can scroll up/down to view older results

#### Scenario: New result at top
- **WHEN** new result is calculated
- **THEN** it is added to the top of the result list

### Requirement: Exit TUI
The system SHALL exit gracefully when requested.

#### Scenario: Exit with Ctrl+C
- **WHEN** user presses Ctrl+C
- **THEN** TUI exits and program terminates

### Requirement: Error Display
The system SHALL display errors appropriately in the TUI.

#### Scenario: Display evaluation error
- **WHEN** expression evaluation fails
- **THEN** error message is displayed in the result area

#### Scenario: Keep input on error
- **WHEN** evaluation produces an error
- **THEN** the input box retains its content for correction
