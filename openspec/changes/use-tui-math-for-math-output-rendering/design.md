## Context

The current TUI path uses custom formatting logic in `src/tui/latex.rs` and `src/tui/result_card.rs` to render fractions, radicals, superscripts, and equation lines. This works for common cases but mixes rendering concerns with formatting logic and creates maintenance risk as more symbolic forms are added.  
This change standardizes TUI math rendering on the `tui-math` crate while preserving current CLI formatting behavior outside TUI mode.

## Goals / Non-Goals

**Goals:**
- Route TUI result-card math rendering through a `tui-math` based renderer.
- Keep current visible behavior stable for existing supported expressions where possible.
- Define fallback behavior when `tui-math` cannot represent a result cleanly.
- Keep non-TUI formatter behavior (`src/output/*`) unchanged.

**Non-Goals:**
- Rewriting parser or solver internals.
- Changing CLI flags or output mode selection semantics.
- Introducing image-based rendering in TUI.

## Decisions

### Decision 1: Add a TUI math rendering adapter around `tui-math`
- Choice: Introduce a small adapter module in the TUI layer that maps calculator result types into `tui-math` input structures and returns renderable lines.
- Rationale: Encapsulates crate-specific details and keeps `ResultCard` construction readable.
- Alternative considered: Replacing all existing formatting calls inline in `result_card.rs`. Rejected because it spreads crate coupling across UI code.

### Decision 2: Keep deterministic text fallback for unsupported forms
- Choice: If `tui-math` rendering fails or lacks a representation, fallback to existing text formatting (`format_complex_root`-style output).
- Rationale: Prevents rendering failures from breaking result display and keeps error handling predictable.
- Alternative considered: Hard fail and show renderer errors. Rejected because it degrades UX for otherwise solvable expressions.

### Decision 3: Preserve card layout contract in `ResultCard::render`
- Choice: Only replace content generation (`result_lines`), not card border/layout behavior.
- Rationale: Avoids unrelated UI regressions in scroll, sizing, and visual framing.
- Alternative considered: Full card rendering rewrite with richer spans. Rejected for scope and risk.

### Decision 4: Verify behavior with rendering-focused tests
- Choice: Add tests for representative outputs (fraction, radical, mixed complex, multi-solution equation, fallback path).
- Rationale: Rendering regressions are easy to introduce and hard to detect manually.
- Alternative considered: Manual smoke checks only. Rejected due to low reliability.

## Risks / Trade-offs

- [Risk] `tui-math` output may differ from current custom glyph spacing.  
  Mitigation: Treat semantic readability as primary; update specs/examples to accept crate-consistent formatting.

- [Risk] Conversion from internal math types to `tui-math` inputs may lose special-case formatting.  
  Mitigation: Keep fallback path and add targeted tests for edge cases (`i`, `-i`, symbolic roots, conjugate pairs).

- [Risk] New dependency can impact build size or compile time.  
  Mitigation: Restrict crate usage to TUI modules and avoid enabling unnecessary features.

## Migration Plan

1. Add `tui-math` crate dependency and adapter module in TUI rendering path.
2. Integrate adapter into `ResultCard` creation flows for expressions/equations.
3. Keep existing formatter as fallback during transition.
4. Update/expand tests for rendering behavior in `src/tui/result_card.rs` and related modules.
5. Remove dead custom rendering branches only after parity is validated.

## Open Questions

- Which exact `tui-math` APIs and feature flags are required for current symbolic formats?
- Should equation variable labels (`x₁`, `x₂`) be handled by `tui-math` output or remain as outer text composition?
- Do we need a user-facing toggle to disable enhanced rendering in constrained terminals, or is automatic fallback sufficient?
