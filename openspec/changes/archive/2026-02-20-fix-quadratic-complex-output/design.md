## Context

The calculator currently accepts equation-like input and routes quadratic expressions to solver/output layers that mix parser normalization, symbolic/fraction math, and TUI formatting. The reported behavior indicates at least two failures in this path: (1) coefficients are altered during normalization and (2) complex roots are rendered as malformed multi-line mixed text. The fix should preserve existing architecture while tightening boundaries between normalization, solving, and display formatting.

## Goals / Non-Goals

**Goals:**
- Preserve user-provided quadratic coefficients when canonicalizing to `ax^2 + bx + c = 0`.
- Produce mathematically correct roots from canonical coefficients for both repeated-real and complex-conjugate cases.
- Render complex roots in a stable single-expression form in result cards and last-result summaries.
- Add focused regression tests that lock expected behavior for the failing scenarios.

**Non-Goals:**
- Reworking all polynomial solvers or introducing a new symbolic engine.
- Redesigning TUI layout or changing global output style conventions.
- Changing unrelated parser grammar behavior beyond what is required for quadratic correctness.

## Decisions

1. Canonicalize equation terms once at parser/normalization boundary and pass explicit `(a, b, c)` to the quadratic solver.
Rationale: A single canonical source removes duplicate transformations that can inject constants or drift coefficients.
Alternative considered: Re-normalize inside solver; rejected because it duplicates logic and hides parser-originated defects.

2. Keep solving math in solver/core layers and treat presentation as a formatting concern only.
Rationale: Separating math from rendering prevents formatting code from influencing computed values.
Alternative considered: Build display strings directly during solve; rejected because it couples algorithm correctness to UI output.

3. Standardize complex root formatting through one output helper used by TUI result surfaces.
Rationale: The screenshot suggests divergent rendering paths; a shared formatter ensures consistent and valid `a +/- bi` output.
Alternative considered: Patch each output call site independently; rejected due to regression risk and duplicated formatting rules.

4. Add regression tests at two levels: solver-level root correctness and output-level representation checks.
Rationale: The issue spans math and display, so both layers need protection.
Alternative considered: Only end-to-end tests; rejected because failures would be harder to localize.

## Risks / Trade-offs

- [Risk] Tightening canonicalization may change behavior for undocumented edge-case input forms. -> Mitigation: keep normalization changes minimal and add tests for currently supported equation forms.
- [Risk] Converting to a single formatter may alter formatting details users are used to. -> Mitigation: preserve existing style tokens where possible and update tests/snapshots intentionally.
- [Risk] Fraction-to-decimal fallback for complex components could still produce noisy output for pathological values. -> Mitigation: define deterministic formatting rules and clamp fallback precision.
