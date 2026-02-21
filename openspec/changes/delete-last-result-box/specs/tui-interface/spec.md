## MODIFIED Requirements

### Requirement: TUI layout structure
The system SHALL divide the TUI screen into exactly two areas.

#### Scenario: TUI layout structure
- **WHEN** TUI is running
- **THEN** the screen is divided into:
  - Upper area: scrollable list of result cards
  - Lower area: fixed input box
- **AND** there is no persistent "Last Result" or "ans" display area

## ADDED Requirements

### Requirement: No persistent last result display
The system SHALL NOT display the last calculation result in a persistent UI box.

#### Scenario: No last result box
- **WHEN** TUI is running
- **THEN** there is no dedicated area showing the last result
- **AND** users can access the last result via the `ans` or `ans()` commands
