# Rust Refactor Proposal Summary

## Overview
This proposal outlines refactoring the calculator CLI from C++20 to Rust while maintaining 100% feature parity.

## Files Created
- **proposal.md**: Why, What Changes, and Impact
- **design.md**: Technical decisions and architectural choices
- **tasks.md**: Ordered implementation checklist with dependencies
- **specs/**: Capability-specific requirements (6 capabilities)

## Capabilities Specified
1. **core-arithmetic**: Complex numbers, fractions, mathematical constants, precision
2. **expression-evaluation**: Parsing, evaluation, functions, operators, error handling
3. **equation-solving**: Linear, quadratic, cubic, quartic, quintic equations
4. **linear-systems**: 2x2 and 3x3 systems, Gaussian elimination
5. **output-formatting**: ASCII, Unicode, LaTeX rendering, terminal detection
6. **cli-interface**: Arguments, help, version, interactive mode, exit codes

## Statistics
- Total Requirements: 50
- Total Scenarios: 282
- Implementation Phases: 7
- Estimated Timeline: 7 weeks

## Key Decisions
- Use `symbolica` crate as SymEngine replacement
- Native Rust types with num-rational for fractions
- Trait-based polymorphism for output formatters
- Cargo replaces CMake build system
- CI/CD removed as requested
- Edition 2021, MSRV 1.70.0

## Benefits
- 374MB reduction (no vendored SymEngine)
- Build time: minutes → seconds
- Memory safety without GC
- Modern tooling (Cargo, rustfmt, clippy)
- Smaller binary size
- Future-proof for 100% Rust migration

## Next Steps
1. Review proposal, design, and tasks
2. Validate spec deltas (manually or with openspec CLI if available)
3. Address any questions or concerns
4. Approve proposal
5. Begin implementation (apply phase)
