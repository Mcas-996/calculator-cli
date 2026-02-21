## 1. Parser Minus Classification

- [x] 1.1 Update expression tokenization/parsing logic to classify `-` as binary subtraction when preceded by a value or right parenthesis, even without surrounding whitespace.
- [x] 1.2 Preserve unary-minus handling for expression start and post-operator/left-parenthesis contexts (for example, `-5+2`, `3*-2`, `(-2)`).

## 2. Evaluation and Error Behavior

- [x] 2.1 Ensure contiguous subtraction expressions (for example, `44-55`, `3.5-1.2`, `2*(8-3)`) evaluate successfully through the shared evaluation path used by CLI and TUI.
- [x] 2.2 Keep malformed numeric input behavior intact so only genuinely invalid numbers return `Invalid number format` (or equivalent parser error).

## 3. Regression Test Coverage

- [x] 3.1 Add parser tests for contiguous subtraction and adjacency edge cases (`44-55`, `2*(8-3)`, `3--2`, `(-2)-(-3)`).
- [x] 3.2 Add/adjust evaluation tests to verify correct numeric results for contiguous subtraction inputs and no regressions for unary-minus scenarios.
- [x] 3.3 Run test suite sections covering expression parsing/evaluation and resolve failures before implementation completion.
