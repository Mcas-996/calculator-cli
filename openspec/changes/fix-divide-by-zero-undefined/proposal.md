## Why

Current calculator behavior crashes when an expression attempts division by zero. This is a user-facing reliability issue and should be normalized to a defined runtime result instead of terminating execution.

## What Changes

- Change division-by-zero evaluation behavior from crash/error termination to returning `undefined`.
- Ensure this behavior applies consistently to direct division expressions (e.g., `1/0`) and equivalent evaluated forms.
- Keep all non-division-by-zero arithmetic behavior unchanged.

## Capabilities

### New Capabilities
- `division-by-zero-result`: Define calculator runtime semantics for division by zero to produce `undefined` rather than crashing.

### Modified Capabilities
- None.

## Impact

- Affected code: expression evaluation logic for division operations and related error propagation paths.
- Affected behavior: CLI output for expressions with zero divisors.
- APIs/dependencies: no external dependency changes expected.
