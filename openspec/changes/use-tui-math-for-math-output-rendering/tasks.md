## 1. Dependency and Adapter Setup

- [ ] 1.1 Add the `tui-math` crate dependency with minimal required features in `Cargo.toml`
- [ ] 1.2 Create a TUI math adapter module that converts internal result types into `tui-math` render inputs
- [ ] 1.3 Define adapter error handling so unsupported render cases return a structured fallback signal

## 2. TUI Rendering Integration

- [ ] 2.1 Update `ResultCard::from_complex` to prefer adapter-based `tui-math` rendering
- [ ] 2.2 Update equation result-card builders to prefer adapter-based `tui-math` rendering
- [ ] 2.3 Keep existing card layout and border rendering unchanged while swapping line-content generation

## 3. Fallback and Output Consistency

- [ ] 3.1 Implement deterministic text fallback when adapter rendering is unavailable
- [ ] 3.2 Ensure fallback output preserves sign/operator/root semantics for complex and radical values
- [ ] 3.3 Verify non-TUI formatters remain unchanged by the TUI renderer integration

## 4. Validation and Regression Coverage

- [ ] 4.1 Add/adjust unit tests for fraction, radical, mixed complex, and multi-solution equation rendering
- [ ] 4.2 Add tests that assert fallback behavior for unsupported or failed `tui-math` render attempts
- [ ] 4.3 Run project test suite and fix rendering regressions before implementation handoff
