# Change: Fix Fraction PartialOrd Conflict

## Why
The current Fraction struct has conflicting implementations of the PartialOrd trait - one being derived and another manually implemented. This causes a compilation error when building the project, preventing any further development or testing.

## What Changes
- Remove PartialOrd from the derive macro in Fraction struct definition
- Keep the manual implementation of PartialOrd for Fraction
- Ensure the implementation properly wraps the underlying Rational64's comparison logic

## Impact
- Affected specs: core
- Affected code: src/core/fraction.rs
- Breaking changes: None (preserves existing behavior)