# Change: Improve Equation Parsing to Support Flexible Formats

## Why
The current equation solvers have multiple issues that create a poor user experience:

1. **Rigid format requirements** - Equations that don't explicitly end with "= 0" are rejected, even when mathematically equivalent
2. **Complex number bugs** - Quadratic equations with complex solutions return incorrect results (e.g., `x^2 + 1 = 0` returns `x = 0` instead of complex solutions)
3. **Wrapper syntax issues** - The `equation(x=2)` function syntax isn't properly supported and produces incorrect results

## What Changes

### Phase 1: Fix Complex Number Math
- Fix the complex square root implementation in `src/core/complex.rs`
- Ensure quadratic equations with complex solutions return correct results
- Add comprehensive tests for complex number operations

### Phase 2: Fix Equation Wrapper Syntax
- Add `equation` function support in the parser or modify equation detection to strip `equation()` wrappers
- Ensure `equation(x=2)` works correctly by properly handling the function syntax
- Update equation detection logic in `src/main.rs`

### Phase 3: Complete Flexible Format Support
- Update equation parsing logic to automatically rearrange equations to standard form `... = 0`
- Support equations like `x = 0`, `x = 5`, `2x + 3 = 7` by moving all terms to the left side
- Maintain backward compatibility with existing `... = 0` format
- Update all solvers (linear, quadratic, cubic, quartic, quintic) to handle flexible equation formats

## Impact
- Affected specs: `equation-solving`
- Affected code: `src/core/complex.rs`, `src/main.rs`, `src/core/types.rs`, `src/solver/linear.rs`, `src/solver/quadratic.rs`, `src/solver/cubic.rs`, `src/solver/quartic.rs`, `src/solver/quintic.rs`, `src/parser/expression.rs`

## Test Cases to Validate
1. `equation(x=2)` → `x = 2`
2. `equation(x^2+1=0)` → `x = i` or `x = -i`
3. `equation(2x+3=7)` → `x = 2`
4. `x=2` (direct equation) → `x = 2`
5. `x^2+1=0` (direct equation) → complex solutions