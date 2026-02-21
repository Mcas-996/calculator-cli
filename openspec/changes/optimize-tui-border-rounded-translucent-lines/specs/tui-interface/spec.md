## ADDED Requirements

### Requirement: TUI Border Visual Style
The system SHALL render bordered TUI containers with rounded corners and thin, semi-transparent border lines.

#### Scenario: Rounded border corners for result cards
- **WHEN** one or more result cards are displayed in the TUI
- **THEN** each card border uses rounded corner styling instead of square corner styling

#### Scenario: Semi-transparent thin border lines
- **WHEN** bordered containers are rendered in the TUI
- **THEN** border lines use thin-line styling with reduced visual intensity to appear semi-transparent while remaining readable

#### Scenario: Consistent border style across containers
- **WHEN** the TUI shows result cards and the input container together
- **THEN** both use the same border corner and line-style contract
