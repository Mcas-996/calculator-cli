## Context

The TUI currently renders panel and card borders with a heavier visual weight and square corners. While functionally correct, this makes card stacks look dense and reduces separation between content and chrome. This change introduces a consistent border style contract for TUI containers so the interface remains readable while preserving clear boundaries.

## Goals / Non-Goals

**Goals:**
- Introduce rounded-corner border styling for bordered TUI components.
- Use thin, semi-transparent border lines to reduce visual noise.
- Apply one shared style policy across result cards and input container.
- Keep current interaction behavior and layout structure unchanged.

**Non-Goals:**
- Redesigning overall TUI layout hierarchy.
- Changing result content rendering logic or math formatting semantics.
- Introducing new rendering backends or external UI dependencies.

## Decisions

### Decision 1: Centralize border styling in a shared TUI style helper
- Choice: Define border-related styling once (corner glyph set, line style, alpha-like color intensity) and reuse it for all bordered widgets.
- Rationale: Prevents drift between result cards and input box, and simplifies future theme changes.
- Alternative considered: Per-widget styling constants. Rejected due to duplication and inconsistency risk.

### Decision 2: Use rounded border glyphs and thin-line style as default visual contract
- Choice: Switch bordered containers to rounded corners with thin lines.
- Rationale: Rounded thin borders reduce visual density while preserving structure.
- Alternative considered: Keep square borders and only change color opacity. Rejected because corner geometry is a key part of the requested visual polish.

### Decision 3: Implement semi-transparent effect via reduced foreground intensity compatible with terminal limitations
- Choice: Represent translucency using lower-contrast border color (theme token) instead of true alpha blending.
- Rationale: Most TUI backends/terminals do not support real alpha compositing; intensity-based styling is stable and portable.
- Alternative considered: Terminal-specific true-color blending tricks. Rejected due to portability and maintenance cost.

## Risks / Trade-offs

- [Risk] Border contrast may become too low on some terminal themes.
  Mitigation: Keep configurable style token and validate readability on common dark/light terminal profiles.

- [Risk] Rounded glyph rendering may vary by font/terminal.
  Mitigation: Use broadly supported rounded border symbols from the TUI framework and keep fallback to standard line set if needed.

- [Risk] Visual changes could affect snapshot/golden UI tests.
  Mitigation: Update affected fixtures and add explicit style assertions for border theme usage.

## Migration Plan

1. Add shared border style constants/helper in TUI styling module.
2. Apply helper to result card border rendering.
3. Apply helper to input container border rendering.
4. Update/add tests or snapshots for border style expectations.
5. Smoke-test in interactive mode to confirm readability and consistency.

## Open Questions

- Should border contrast vary between focused and unfocused input states?
- Do we need a user-facing theme toggle for accessibility in very low-contrast terminals?
