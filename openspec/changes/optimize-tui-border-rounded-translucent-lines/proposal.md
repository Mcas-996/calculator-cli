## Why

The current TUI borders are visually heavy and sharp-cornered, which reduces readability and makes result cards feel dense. Defining lighter border styling with rounded corners improves visual hierarchy and makes the interface easier to scan.

## What Changes

- Define requirement-level border styling for TUI result cards and input container using rounded corners.
- Define requirement-level use of thin, semi-transparent border lines to reduce visual noise while preserving panel boundaries.
- Clarify consistency requirements so all TUI bordered components follow the same corner and line style.

## Capabilities

### New Capabilities
- None.

### Modified Capabilities
- `tui-interface`: Update visual styling requirements for bordered components to use rounded corners and semi-transparent thin borders.

## Impact

- Affected specs: `openspec/specs/tui-interface/spec.md`.
- Affected code areas (expected): TUI theme/style definitions and widgets that draw bordered panels/cards.
- Dependency impact: No new runtime dependency required; uses existing TUI styling primitives.
