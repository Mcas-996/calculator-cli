## ADDED Requirements

### Requirement: TUI Math Rendering Fallback Formatting
The system SHALL preserve mathematically equivalent textual output when `tui-math` rendering is unavailable or fails for a given value.

#### Scenario: Fallback for unsupported symbolic layout
- **WHEN** TUI rendering receives a value that `tui-math` cannot represent
- **THEN** the system outputs a normalized textual representation in the result card

#### Scenario: Fallback preserves sign and operator semantics
- **WHEN** a complex or radical expression is rendered via fallback
- **THEN** the textual output preserves sign, multiplication, and root semantics

#### Scenario: Fallback output remains deterministic
- **WHEN** the same value is rendered repeatedly in fallback mode
- **THEN** the textual output is stable across runs for identical inputs

### Requirement: TUI Math Rendering Priority
The system SHALL prioritize `tui-math` output in TUI result cards before plain text formatting.

#### Scenario: Prefer tui-math in TUI mode
- **WHEN** rendering a supported fraction, radical, or equation term in TUI mode
- **THEN** `tui-math` output is used as the primary display representation

#### Scenario: Non-TUI output unchanged
- **WHEN** output is generated through non-TUI formatters
- **THEN** existing ASCII, Unicode, and LaTeX formatter behavior remains unchanged
