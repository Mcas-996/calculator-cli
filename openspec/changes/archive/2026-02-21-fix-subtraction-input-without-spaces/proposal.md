## Why

Users entering basic subtraction without spaces (for example, `44-55`) currently receive `Invalid number format` instead of a computed result. This breaks a common calculator input pattern and causes valid arithmetic to fail in both CLI and TUI workflows.

## What Changes

- Update expression tokenization/parsing behavior so binary subtraction is recognized when operands and `-` are contiguous (for example, `44-55`, `3.5-1.2`, and `2*(8-3)`).
- Preserve existing unary minus behavior (for example, `-5+2`, `3*-2`) while removing false numeric-format failures for valid subtraction expressions.
- Add coverage for contiguous subtraction inputs to prevent regression.

## Capabilities

### New Capabilities
- None.

### Modified Capabilities
- `expression-evaluation`: Expand expression parsing requirements to accept subtraction expressions without whitespace around the minus operator.

## Impact

- Affected specs: `openspec/specs/expression-evaluation/spec.md` (delta in this change).
- Affected code areas: expression tokenizer/parser and related evaluation error handling.
- Affected tests: expression parsing/evaluation tests for subtraction and unary-minus edge cases.
