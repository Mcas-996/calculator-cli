## Why

Quadratic solve output is currently unreliable for some user inputs: coefficients can be mis-normalized and complex roots render as malformed mixed fraction text, producing incorrect or confusing results in the TUI. This needs to be fixed now because it breaks trust in core calculator behavior.

## What Changes

- Correct quadratic equation normalization so entered coefficients are preserved when converting user input to `ax^2 + bx + c = 0`.
- Ensure quadratic roots are computed from normalized coefficients without introducing unintended constants.
- Fix complex-root result rendering so conjugate roots display as valid, human-readable expressions in the results panel and last-result line.
- Add regression coverage for equations like `x^2 + 2x + 1 = 0` and representative negative-discriminant cases.

## Capabilities

### New Capabilities
- `quadratic-solution-correctness`: Define required behavior for quadratic coefficient normalization, root correctness, and stable complex-number presentation.

### Modified Capabilities
- (none)

## Impact

- Affected code likely includes quadratic solver flow, equation parsing/normalization, complex/fraction display formatting, and TUI result presentation.
- Affected files are expected in `src/parser/*`, `src/solver/quadratic.rs`, `src/output/*`, and `src/tui/*`.
- No new external dependencies are expected.
