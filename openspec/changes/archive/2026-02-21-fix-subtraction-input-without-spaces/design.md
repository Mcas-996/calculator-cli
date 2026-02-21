## Context

The calculator currently rejects contiguous subtraction forms like `44-55` with `Invalid number format`, indicating the tokenizer or numeric scanner is treating the minus as part of a malformed numeric literal instead of an operator boundary. This impacts common calculator entry behavior in both CLI and TUI interfaces.

## Goals / Non-Goals

**Goals:**
- Accept binary subtraction without mandatory surrounding whitespace.
- Keep unary-minus semantics intact in expression starts and after operators/left parentheses.
- Prevent regression via parser/evaluator test coverage for contiguous minus patterns.

**Non-Goals:**
- No redesign of the full expression grammar.
- No changes to operator precedence rules.
- No UI-level feature additions beyond corrected parsing behavior.

## Decisions

- Normalize minus handling in tokenization by using parse context (previous token class) to distinguish unary and binary `-`.
Rationale: This resolves contiguous subtraction while preserving existing unary cases without requiring users to add spaces.
Alternative considered: Preprocessing input to inject spaces around operators. Rejected because it is brittle for functions, decimals, and signed literals.

- Treat parser-acceptable forms consistently across input surfaces (CLI and TUI) by reusing shared expression evaluation path.
Rationale: Prevents divergence where one interface accepts an expression and the other rejects it.
Alternative considered: Interface-specific input normalization. Rejected due to duplicated logic and higher regression risk.

- Add focused regression tests for contiguous subtraction and unary-minus adjacency cases.
Rationale: These are the edge conditions most likely to break when adjusting minus tokenization.
Alternative considered: Relying on existing arithmetic tests. Rejected because they currently miss no-whitespace subtraction cases.

## Risks / Trade-offs

- [Risk] Unary/binary disambiguation bugs in edge expressions (for example, `3--2`, `(-2)-(-3)`) -> Mitigation: add explicit parser/evaluator test cases for adjacency patterns.
- [Risk] Parser change may affect existing error messages -> Mitigation: preserve current error path for truly malformed numbers and validate representative invalid inputs.
- [Trade-off] Slightly more complex tokenizer state handling -> Mitigation: confine logic to minus classification and document token-context rules.

## Migration Plan

- Implement parser/tokenizer adjustment behind existing evaluation path.
- Run unit tests for expression parsing and evaluation, then add/validate new subtraction edge-case tests.
- Rollback strategy: revert parser change and associated tests if regression appears in unrelated expression forms.

## Open Questions

- Should contiguous subtraction acceptance be expanded to adjacent plus (`44+55`) and other operators in the same parser pass, or kept narrowly scoped to minus for this fix?
