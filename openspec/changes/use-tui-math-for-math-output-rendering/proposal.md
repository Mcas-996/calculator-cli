## Why

Current TUI math rendering behavior is described as LaTeX-style output but does not define a consistent rendering backend. Standardizing rendering on the `tui-math` crate improves output consistency for fractions, radicals, superscripts, and future math layouts.

## What Changes

- Define requirement-level behavior for rendering math output in TUI using `tui-math` as the rendering mechanism.
- Update TUI result card expectations to align with `tui-math`-based rendering output.
- Clarify fallback behavior when an expression cannot be rendered in enhanced math layout.

## Capabilities

### New Capabilities
- None.

### Modified Capabilities
- `tui-interface`: Update result card rendering requirements to use `tui-math` output conventions.
- `output-formatting`: Clarify formatting expectations for TUI-targeted math presentation and fallback text formatting.

## Impact

- Affected specs: `openspec/specs/tui-interface/spec.md`, `openspec/specs/output-formatting/spec.md`.
- Affected code areas (expected): TUI result rendering pipeline and formatting adapter code.
- Dependency impact: Introduces or standardizes usage of the `tui-math` crate in rendering-related modules.
