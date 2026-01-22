## 1. Complex Number Math Fixes
- [ ] 1.1 Fix complex square root implementation in `src/core/complex.rs`
- [ ] 1.2 Test quadratic equations with complex solutions
- [ ] 1.3 Verify complex number arithmetic works correctly

## 2. Equation Wrapper Syntax
- [ ] 2.1 Add `equation` function support to parser OR modify equation detection
- [ ] 2.2 Strip `equation()` wrappers before processing equations
- [ ] 2.3 Test `equation(x=2)` and `equation(x^2+1=0)` syntax

## 3. Flexible Format Implementation
- [ ] 3.1 Create utility function to normalize equations to standard form
- [ ] 3.2 Update linear equation solver to accept flexible formats
- [ ] 3.3 Update quadratic equation solver to accept flexible formats
- [ ] 3.4 Update cubic equation solver to accept flexible formats
- [ ] 3.5 Update quartic equation solver to accept flexible formats
- [ ] 3.6 Update quintic equation solver to accept flexible formats

## 4. Testing and Validation
- [ ] 4.1 Add comprehensive tests for complex number solutions
- [ ] 4.2 Add tests for equation wrapper syntax
- [ ] 4.3 Add tests for flexible equation formats
- [ ] 4.4 Run existing tests to ensure backward compatibility
- [ ] 4.5 Test edge cases (e.g., `x = 0`, `2x = 4`, `x^2 = 4`)

## 5. Integration Testing
- [ ] 5.1 Test the reported failing cases:
  - `equation(x=2)` should return `x = 2`
  - `equation(x^2+1=0)` should return complex solutions
- [ ] 5.2 Verify CLI behavior with both wrapper and direct equation syntax