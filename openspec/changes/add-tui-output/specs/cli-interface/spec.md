# Spec: CLI Interface

## MODIFIED Requirements

### Requirement: Interactive Mode
The system SHALL provide an interactive REPL for evaluating expressions.

#### Scenario: Enter interactive mode
- **WHEN** no expression argument is provided
- **THEN** TUI application starts and displays the main interface

#### Scenario: Display prompt
- **WHEN** in interactive mode
- **THEN** input box is displayed at the bottom of the screen

#### Scenario: Evaluate expression
- **WHEN** user enters expression in input box and presses Ctrl+Enter
- **THEN** expression is evaluated and result is displayed as a result card

#### Scenario: Empty input handling
- **WHEN** user presses Ctrl+Enter with empty input
- **THEN** nothing happens, input remains empty

#### Scenario: Continue after evaluation
- **WHEN** expression is evaluated in interactive mode
- **THEN** input box is cleared and ready for next input

#### Scenario: Exit interactive mode
- **WHEN** user presses Ctrl+C
- **THEN** interactive mode exits and program terminates
