## Context

The calculator currently computes correct results for many cases but often degrades output presentation by forcing irrational or complex values into approximate rational text. This is most visible in TUI result cards and the last-result summary, where values like `sqrt(2)` or `i*sqrt(2)` appear as large fractional approximations that reduce readability and user trust.

## Goals / Non-Goals

**Goals:**
- Preserve exact mathematical structure for irrational and complex results until final formatting.
- Render stable, canonical math text for results panel entries and last-result summaries.
- Keep existing exact integer/fraction output behavior unchanged for already-rational values.
- Add regression tests that lock expected formatting for known failing inputs.

**Non-Goals:**
- Building a general-purpose symbolic algebra engine.
- Redesigning the TUI layout or introducing new interaction modes.
- Changing solver algorithms beyond what is required to preserve exact display intent.

## Decisions

1. Introduce a structured display-expression model at the output boundary.
Rationale: Formatting from a typed expression tree (rational, radical, complex) avoids lossy conversion into arbitrary fractions before rendering.
Alternative considered: Continue formatting from generic numeric values; rejected because approximation already causes the reported failures.

2. Define canonical formatting rules for radicals and complex values.
Rationale: A single canonical rule set (`sqrt(n)`, simplified coefficients, normalized signs, `a + bi`) prevents inconsistent strings across code paths.
Alternative considered: Keep formatter behavior per call site; rejected due to drift and repeated bugs.

3. Route both results panel rendering and last-result summary through the same formatter helper.
Rationale: The same value should render identically wherever it appears.
Alternative considered: Separate formatters per surface; rejected because it increases divergence risk.

4. Keep approximate numeric evaluation available only as an internal or optional auxiliary value, not as the primary display form.
Rationale: Exact math text must be the default display contract for affected cases.
Alternative considered: Show approximations by default and exact form second; rejected because it preserves current confusion.

## Risks / Trade-offs

- [Risk] New exact-format rules may expose previously hidden edge cases for nested radicals or mixed rational/radical complex parts. -> Mitigation: constrain first pass to supported expression families and add targeted tests for boundaries.
- [Risk] Canonical text choices may differ from individual user preference (for example `i*sqrt(2)` vs `sqrt(2)i`). -> Mitigation: choose one documented style and enforce consistency.
- [Risk] Shared formatter refactor can affect unrelated output paths. -> Mitigation: add regression tests for unchanged rational/integer cases and isolate formatter entry points.

## Migration Plan

- No data migration is required.
- Roll out by updating formatting pathways behind existing solve/evaluate commands.
- If regressions appear, fallback is to temporarily disable exact-format path for affected expression types while keeping tests to drive fixes.

## Open Questions

- Should optional approximate values (for example decimal hints) be displayed in the UI now or deferred to a separate UX change?
- Should multiplication between coefficient and `i`/`sqrt` be rendered with explicit `*` everywhere for parser round-tripping, or omitted for readability?
