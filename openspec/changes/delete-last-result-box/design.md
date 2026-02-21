## Context

The Calculator CLI TUI currently displays three areas:
1. Results area (scrollable list of result cards)
2. Last Result area (3-line box showing the most recent calculation)
3. Input area

The "Last Result" box displays the same information that users can access via `ans` or `ans()` commands, taking up valuable screen space.

## Goals / Non-Goals

**Goals:**
- Remove the persistent "Last Result" display from the TUI layout
- Simplify the UI to only show results list and input area
- Preserve the `ans` / `ans()` command functionality

**Non-Goals:**
- No changes to calculation logic
- No changes to how results are stored or accessed via `ans`

## Decisions

**Decision: Remove Last Result UI box**
- **Rationale**: The box takes 3 lines of screen space and duplicates information available via `ans` command
- **Alternative**: Keep the box (rejected - adds visual clutter without adding value)

## Risks / Trade-offs

**Risk**: Users may miss the ability to use `ans` without the visible reminder
- **Mitigation**: The help text already mentions `ans / ans()` command, so users can discover it
