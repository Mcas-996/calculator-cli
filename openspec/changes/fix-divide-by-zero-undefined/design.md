## Context

The calculator currently crashes when evaluating a division whose denominator is zero. This behavior breaks CLI usability for otherwise validly parsed expressions and prevents downstream formatting logic from handling the result gracefully.

The proposal defines a new runtime capability: division by zero must evaluate to `undefined` instead of terminating execution.

## Goals / Non-Goals

**Goals:**
- Ensure division by zero no longer crashes evaluation.
- Return `undefined` as the canonical result for divide-by-zero cases.
- Keep all existing non-divide-by-zero evaluation behavior unchanged.
- Preserve predictable CLI output flow after evaluation.

**Non-Goals:**
- Changing arithmetic precedence or parser behavior.
- Introducing IEEE infinity/NaN semantics.
- Altering behavior for other runtime errors unrelated to zero divisors.

## Decisions

- Decision: Represent divide-by-zero as the existing `undefined` result path
  - Rationale: Reuses current output concepts and avoids introducing a new error/result type.
  - Alternative considered: Throwing a recoverable runtime error; rejected because users requested value semantics (`undefined`) instead of failure semantics.

- Decision: Detect zero denominator at the division operation boundary
  - Rationale: Keeps behavior local to division logic and avoids broad cross-module changes.
  - Alternative considered: Global post-evaluation sanitization; rejected because it is harder to reason about and may mask unrelated issues.

- Decision: Preserve evaluation continuation where possible
  - Rationale: Prevents process crashes and keeps CLI interactions stable.
  - Alternative considered: Early process exit with message; rejected because it still interrupts normal workflow.

## Risks / Trade-offs

- Risk: `undefined` propagation may expose latent assumptions in formatting or equation paths
  - Mitigation: Add focused tests for direct division and expression-level evaluation output.

- Risk: Ambiguity between parse errors and runtime undefined results
  - Mitigation: Keep parse error handling unchanged and limit this change to division-by-zero runtime cases.

- Trade-off: Returning `undefined` hides explicit error details
  - Mitigation: This is acceptable for this CLI's user-facing behavior and matches requested semantics.

## Migration Plan

1. Update division evaluation logic to guard denominator zero and produce `undefined`.
2. Verify direct input (`1/0`) and nested expressions with zero divisors return `undefined`.
3. Run existing tests and add/update cases for division-by-zero behavior.
4. Confirm CLI no longer crashes on divide-by-zero inputs.

Rollback:
- Revert evaluation guard changes and restore previous division behavior if regressions appear.

## Open Questions

- Should future work standardize `undefined` behavior for other invalid arithmetic operations (for example `0/0`) if currently handled differently?
