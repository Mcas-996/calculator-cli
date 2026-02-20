## Why

Current result rendering degrades mathematical expressions into confusing approximations. For example, `sqrt(2)` is shown as a large fraction and pure imaginary roots are shown as noisy fractional `i` terms, which makes correct results look wrong to users.

## What Changes

- Add a math-aware output path that preserves exact symbolic forms for irrational and complex results in TUI result cards and last-result summaries.
- Display square-root-based irrational values in canonical forms such as `sqrt(2)` or `3/2*sqrt(5)` instead of high-denominator rational approximations.
- Display complex values in stable `a + bi` form, including pure-imaginary results like `i*sqrt(2)` and conjugate pairs with consistent signs.
- Keep integer and rational rendering behavior unchanged for cases that are already exact.
- Add regression tests for known failing examples (`sqrt(2)`, `x^2 = -2`) and representative mixed real/imaginary quadratic roots.

## Capabilities

### New Capabilities
- `math-output-formatting`: Define exact-expression formatting requirements for irrational and complex results across calculator output surfaces.

### Modified Capabilities
- (none)

## Impact

- Affected code is expected in output/formatting layers, quadratic result rendering, and last-result summary composition.
- May require extending numeric/result value representation to preserve exact symbolic structure until final formatting.
- Tests will be added or updated for solver/output integration and TUI display regression coverage.
