## 1. Division Evaluation Update

- [x] 1.1 Locate division operator evaluation path and add zero-denominator guard
- [x] 1.2 Return `undefined` result instead of crashing when denominator is zero
- [x] 1.3 Ensure non-zero division behavior remains unchanged

## 2. Result Propagation and CLI Output

- [x] 2.1 Verify `undefined` result flows through existing result/output pipeline without panic
- [x] 2.2 Confirm direct expression input `1/0` prints `undefined`
- [x] 2.3 Confirm nested expressions with zero denominator print `undefined`

## 3. Test Coverage and Validation

- [x] 3.1 Add or update tests for direct division-by-zero behavior
- [x] 3.2 Add or update tests for nested-expression zero-denominator behavior
- [x] 3.3 Run project tests and confirm no regressions in standard division cases
