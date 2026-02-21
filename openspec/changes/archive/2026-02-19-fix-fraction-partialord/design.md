## Context
The Fraction struct in calculator-cli's core module currently has a trait implementation conflict. The struct derives PartialOrd while also providing a manual implementation of the same trait. This violates Rust's trait implementation rules and prevents compilation, blocking all development work.

The Fraction type wraps Rational64 from the num-rational crate, which already provides proper comparison operations. Our manual implementation delegates to the underlying Rational64, but the derived version conflicts with it.

## Goals / Non-Goals
- Goals: 
  - Resolve the compilation error from trait implementation conflicts
  - Maintain existing comparison behavior for Fraction instances
  - Ensure no breaking changes to the public API
  - Preserve all existing functionality in the calculator

- Non-Goals:
  - Changing the Fraction comparison semantics
  - Adding new comparison methods
  - Modifying the underlying rational number representation

## Decisions
- Decision: Remove PartialOrd from the derive macro and keep only the manual implementation
  - Reason: The manual implementation is more explicit about delegating to Rational64
  - The derived version would duplicate this logic and cause conflicts
  - Manual implementation provides clearer intent for future maintenance

- Decision: Keep the manual PartialOrd implementation that delegates to Rational64
  - Reason: This ensures consistency with the underlying rational library
  - Provides correct handling of edge cases like different denominators
  - Guarantees proper ordering semantics

- Decision: Not implement custom comparison logic
  - Reason: Rational64 already has proven, battle-tested comparison operations
  - No need to reinvent complex fraction comparison algorithms
  - Leveraging existing tested code reduces maintenance burden

## Risks / Trade-offs
- Risk: Manual implementation might not cover all edge cases that derive would
  - Mitigation: The implementation delegates to Rational64 which handles complex cases
  - Test: Comprehensive comparison tests in tasks.md

- Risk: Future maintainers might accidentally add PartialOrd to derive again
  - Mitigation: Add code comment explaining why PartialOrd is manually implemented
  - Documentation: Clearly annotate the struct with reason for manual impl

- Trade-off: Slightly more verbose code vs. clear intent
  - Decision: Verbose code with clear intent is better for long-term maintenance

## Migration Plan
1. Remove PartialOrd from the derive macro
2. Verify manual implementation exists and is correct
3. Add explanatory comment to prevent future conflicts
4. Run full test suite to ensure no regression
5. Test compilation with `cargo build --release`

Rollback plan:
- If issues arise, restore working version from git
- The change is minimal and isolated to trait implementations

## Open Questions
- None - this is a straightforward trait conflict resolution