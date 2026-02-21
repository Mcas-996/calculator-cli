## Why

The TUI displays a "Last Result" box showing the most recent calculation result at all times. This takes up screen space and duplicates information that users can already access via the `ans` or `ans()` commands. Removing this box simplifies the UI and reduces visual clutter.

## What Changes

- **Remove** the "Last Result" UI box from the TUI interface
- **Preserve** the `ans` / `ans()` commands functionality - users can still manually retrieve the last result
- **Simplify** the layout to only show results list and input area

## Capabilities

### New Capabilities
<!-- No new capabilities being introduced -->

### Modified Capabilities
- `tui-interface`: Remove the persistent "Last Result" display from the TUI layout

## Impact

- **Modified**: `src/tui/app.rs` - Remove ans_area layout constraint and rendering code
