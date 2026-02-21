## 1. Shared Border Style Foundation

- [ ] 1.1 Locate current TUI border style usage for result cards and input container, and identify the common styling hook/module
- [ ] 1.2 Add a shared border style helper/token set that defines rounded corners plus thin, reduced-intensity border lines
- [ ] 1.3 Ensure the shared helper has a clear fallback path for terminals that do not render rounded borders cleanly

## 2. Apply Border Contract to TUI Components

- [ ] 2.1 Update result card rendering to use the shared rounded/thin/semi-transparent border style
- [ ] 2.2 Update input container rendering to use the same shared border style contract
- [ ] 2.3 Verify bordered containers remain layout-compatible (no clipping/overflow regressions)

## 3. Validation and Regression Safety

- [ ] 3.1 Add or update UI/snapshot tests to assert rounded-corner and thin-border style usage
- [ ] 3.2 Validate readability on common terminal themes and adjust contrast token if needed
- [ ] 3.3 Run the test suite and interactive smoke checks, then resolve any border-style regressions
