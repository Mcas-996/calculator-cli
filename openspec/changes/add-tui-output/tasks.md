## 1. Setup

- [x] 1.1 Add ratatui dependency to Cargo.toml
- [x] 1.2 Add crossterm dependency (if not already present)
- [x] 1.3 Create src/tui/ module structure

## 2. LaTeX Renderer

- [x] 2.1 Create src/tui/latex.rs with fraction rendering
- [x] 2.2 Implement radical rendering
- [x] 2.3 Implement superscript/subscript rendering
- [x] 2.4 Implement complex number rendering with proper LaTeX style
- [x] 2.5 Add unit tests for LaTeX renderer

## 3. TUI Components

- [x] 3.1 Create src/tui/result_card.rs for result card component
- [x] 3.2 Create src/tui/input.rs for input handling
- [x] 3.3 Implement multi-line input with Shift+Enter
- [x] 3.4 Implement Ctrl+Enter execution

## 4. TUI Application

- [x] 4.1 Create src/tui/app.rs with main application logic
- [x] 4.2 Implement layout (upper result area + lower input area)
- [x] 4.3 Implement result card list with scrolling
- [x] 4.4 Implement ans variable storage and retrieval
- [x] 4.5 Handle Ctrl+C exit gracefully

## 5. Integration

- [x] 5.1 Modify src/main.rs to use TUI mode instead of run_interactive_mode()
- [x] 5.2 Wire up expression evaluation to TUI
- [x] 5.3 Wire up equation solving to TUI
- [ ] 5.4 Test the complete flow

## 6. Testing

- [ ] 6.1 Manual testing of multi-line input
- [ ] 6.2 Test ans variable in expressions
- [ ] 6.3 Test equation solving with LaTeX rendering
- [ ] 6.4 Test error handling in TUI
