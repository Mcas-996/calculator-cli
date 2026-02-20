## 1. Reproduce and Prepare the Formatting Refactor

- [x] 1.1 Add/confirm regression tests that reproduce current output issues for `sqrt(2)` and `x^2 = -2`.
- [x] 1.2 Trace the current solver-to-output flow and identify the shared boundary where exact display expressions should be introduced.
- [ ] 1.3 Add or adapt internal display expression types to represent exact rational, radical, and complex values.

## 2. Implement Canonical Exact Formatter

- [x] 2.1 Implement radical formatting rules that preserve exact form and simplify equivalent values (for example, `sqrt(8)` -> `2*sqrt(2)`).
- [x] 2.2 Implement complex formatting rules with canonical sign placement and stable `a + bi` style output.
- [x] 2.3 Ensure pure-imaginary outputs are rendered as valid exact expressions (for example, `i*sqrt(2)` and `-i*sqrt(2)`).

## 3. Wire Formatter into User-Facing Output

- [x] 3.1 Replace divergent per-surface rendering paths with a shared formatter helper for result cards and last-result summaries.
- [x] 3.2 Update quadratic result presentation to emit exact conjugate forms for negative-discriminant cases.
- [x] 3.3 Keep existing integer and rational rendering behavior unchanged for non-radical, non-complex results.

## 4. Validate End-to-End Behavior

- [x] 4.1 Add unit tests for radical simplification and complex-format canonicalization.
- [x] 4.2 Add integration or TUI-level regression tests covering `sqrt(2)`, `x^2 = -2`, `x^2 + 2x + 10 = 0`, and `1+1`.
- [ ] 4.3 Run the full test suite and verify no regressions in equation solving and output formatting paths.
