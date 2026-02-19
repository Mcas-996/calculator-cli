## 1. Reproduce and Isolate the Quadratic Regression

- [ ] 1.1 Add/confirm a failing regression test for `x^2 + 2x + 1 = 0` that currently produces incorrect coefficients or roots.
- [ ] 1.2 Add/confirm a failing regression test for a negative-discriminant quadratic that currently renders malformed complex output.

## 2. Fix Canonical Coefficient Flow

- [ ] 2.1 Trace parser/normalization path and ensure canonical `(a, b, c)` extraction preserves user-entered equation equivalence.
- [ ] 2.2 Update quadratic solver inputs so root computation uses only canonical coefficients from normalization.
- [ ] 2.3 Add unit tests for canonicalization cases including standard form and non-zero RHS form.

## 3. Fix Complex Root Rendering

- [ ] 3.1 Introduce or consolidate a single complex-root formatting helper used by quadratic result output paths.
- [ ] 3.2 Update results panel rendering to display each complex root as one valid expression without stacked fragments.
- [ ] 3.3 Update last-result summary formatting to match the computed complex root syntax.

## 4. Validate End-to-End Behavior

- [ ] 4.1 Add/refresh solver-level tests for repeated real roots and complex conjugate roots.
- [ ] 4.2 Add/refresh output-level tests (or snapshots) covering complex result display in TUI surfaces.
- [ ] 4.3 Run full test suite and verify no regressions in other equation-solving and output-formatting flows.
