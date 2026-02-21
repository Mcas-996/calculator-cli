use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Stylize},
    widgets::{Block, Widget},
};
use std::collections::VecDeque;

#[derive(Clone)]
pub struct InputArea {
    pub lines: Vec<String>,
    pub cursor: (usize, usize),
    pub scroll: (usize, usize),
    max_width: usize,
    pub history: VecDeque<String>,
    pub history_index: Option<usize>,
    prompt: String,
}

impl InputArea {
    pub fn new(prompt: String, max_width: usize) -> Self {
        Self {
            lines: vec![String::new()],
            cursor: (0, 0),
            scroll: (0, 0),
            max_width,
            history: VecDeque::new(),
            history_index: None,
            prompt,
        }
    }

    pub fn get_text(&self) -> String {
        self.lines.join("\n")
    }

    pub fn clear(&mut self) {
        self.lines = vec![String::new()];
        self.cursor = (0, 0);
        self.scroll = (0, 0);
        self.history_index = None;
    }

    pub fn push_to_history(&mut self) {
        let text = self.get_text();
        if !text.is_empty() {
            if self.history.front() != Some(&text) {
                self.history.push_front(text);
                if self.history.len() > 100 {
                    self.history.pop_back();
                }
            }
        }
        self.history_index = None;
    }

    pub fn history_up(&mut self) {
        if self.history.is_empty() {
            return;
        }

        let new_index = match self.history_index {
            None => 0,
            Some(i) if i + 1 < self.history.len() => i + 1,
            _ => return,
        };

        self.history_index = Some(new_index);
        if let Some(text) = self.history.get(new_index) {
            self.lines = text.lines().map(|s| s.to_string()).collect();
            if self.lines.is_empty() {
                self.lines = vec![String::new()];
            }
            self.cursor = (
                self.lines.len().saturating_sub(1),
                self.lines.last().map(|l| l.len()).unwrap_or(0),
            );
        }
    }

    pub fn history_down(&mut self) {
        let new_index = match self.history_index {
            None => return,
            Some(0) => None,
            Some(i) => Some(i - 1),
        };

        self.history_index = new_index;
        match new_index {
            None => {
                self.clear();
            }
            Some(i) => {
                if let Some(text) = self.history.get(i) {
                    self.lines = text.lines().map(|s| s.to_string()).collect();
                    if self.lines.is_empty() {
                        self.lines = vec![String::new()];
                    }
                    self.cursor = (
                        self.lines.len().saturating_sub(1),
                        self.lines.last().map(|l| l.len()).unwrap_or(0),
                    );
                }
            }
        }
    }

    fn move_cursor_left(&mut self) {
        if self.cursor.1 > 0 {
            self.cursor.1 -= 1;
        } else if self.cursor.0 > 0 {
            self.cursor.0 -= 1;
            self.cursor.1 = self.lines[self.cursor.0].len();
        }
    }

    fn move_cursor_right(&mut self) {
        let current_line_len = self.lines[self.cursor.0].len();
        if self.cursor.1 < current_line_len {
            self.cursor.1 += 1;
        } else if self.cursor.0 < self.lines.len() - 1 {
            self.cursor.0 += 1;
            self.cursor.1 = 0;
        }
    }

    fn move_cursor_up(&mut self) {
        if self.cursor.0 > 0 {
            self.cursor.0 -= 1;
            let line_len = self.lines[self.cursor.0].len();
            self.cursor.1 = self.cursor.1.min(line_len);
        }
    }

    fn move_cursor_down(&mut self) {
        if self.cursor.0 < self.lines.len() - 1 {
            self.cursor.0 += 1;
            let line_len = self.lines[self.cursor.0].len();
            self.cursor.1 = self.cursor.1.min(line_len);
        }
    }

    fn insert_char(&mut self, c: char) {
        let line = &mut self.lines[self.cursor.0];
        line.insert(self.cursor.1, c);
        self.cursor.1 += 1;
    }

    fn insert_newline(&mut self) {
        let current_line = self.lines[self.cursor.0].clone();
        let after_cursor = current_line[self.cursor.1..].to_string();

        self.lines[self.cursor.0].truncate(self.cursor.1);
        self.lines.insert(self.cursor.0 + 1, after_cursor);

        self.cursor.0 += 1;
        self.cursor.1 = 0;
    }

    fn delete_char_forward(&mut self) {
        if self.cursor.1 < self.lines[self.cursor.0].len() {
            self.lines[self.cursor.0].remove(self.cursor.1);
        } else if self.cursor.0 < self.lines.len() - 1 {
            let next_line = self.lines.remove(self.cursor.0 + 1);
            self.lines[self.cursor.0].push_str(&next_line);
        }
    }

    fn delete_char_backward(&mut self) {
        if self.cursor.1 > 0 {
            self.lines[self.cursor.0].remove(self.cursor.1 - 1);
            self.cursor.1 -= 1;
        } else if self.cursor.0 > 0 {
            let current_line = self.lines.remove(self.cursor.0);
            self.cursor.0 -= 1;
            self.cursor.1 = self.lines[self.cursor.0].len();
            self.lines[self.cursor.0].push_str(&current_line);
        }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> InputAction {
        match key_event {
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::CONTROL,
                ..
            } => InputAction::Execute,
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::SHIFT,
                ..
            } => {
                self.insert_newline();
                InputAction::None
            }
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
                ..
            } => InputAction::Execute,
            KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => InputAction::Quit,
            KeyEvent {
                code: KeyCode::Char('d'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                if self.get_text().is_empty() {
                    return InputAction::Quit;
                }
                InputAction::None
            }
            KeyEvent {
                code: KeyCode::Char('w'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                let line = &self.lines[self.cursor.0];
                let mut end = self.cursor.1;
                while end > 0
                    && line
                        .chars()
                        .nth(end - 1)
                        .map_or(false, |c| c.is_whitespace())
                {
                    end -= 1;
                }
                while end > 0
                    && !line
                        .chars()
                        .nth(end - 1)
                        .map_or(false, |c| c.is_whitespace())
                {
                    end -= 1;
                }
                if end < self.cursor.1 {
                    self.lines[self.cursor.0].drain(end..self.cursor.1);
                    self.cursor.1 = end;
                }
                InputAction::None
            }
            KeyEvent {
                code: KeyCode::Char('k'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                let line = &self.lines[self.cursor.0];
                if self.cursor.1 < line.len() {
                    self.lines[self.cursor.0].truncate(self.cursor.1);
                }
                InputAction::None
            }
            KeyEvent {
                code: KeyCode::Char('u'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                if self.cursor.1 > 0 {
                    self.lines[self.cursor.0].drain(..self.cursor.1);
                    self.cursor.1 = 0;
                }
                InputAction::None
            }
            KeyEvent {
                code: KeyCode::Char('a'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                self.cursor.1 = 0;
                InputAction::None
            }
            KeyEvent {
                code: KeyCode::Char('e'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                self.cursor.1 = self.lines[self.cursor.0].len();
                InputAction::None
            }
            KeyEvent {
                code: KeyCode::Up,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                self.move_cursor_up();
                InputAction::None
            }
            KeyEvent {
                code: KeyCode::Down,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                self.move_cursor_down();
                InputAction::None
            }
            KeyEvent {
                code: KeyCode::Left,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                self.move_cursor_left();
                InputAction::None
            }
            KeyEvent {
                code: KeyCode::Right,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                self.move_cursor_right();
                InputAction::None
            }
            KeyEvent {
                code: KeyCode::Home,
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                self.cursor.0 = 0;
                self.cursor.1 = 0;
                InputAction::None
            }
            KeyEvent {
                code: KeyCode::End,
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                self.cursor.0 = self.lines.len() - 1;
                self.cursor.1 = self.lines[self.cursor.0].len();
                InputAction::None
            }
            KeyEvent {
                code: KeyCode::Backspace,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                self.delete_char_backward();
                InputAction::None
            }
            KeyEvent {
                code: KeyCode::Delete,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                self.delete_char_forward();
                InputAction::None
            }
            KeyEvent {
                code: KeyCode::Tab,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                for _ in 0..4 {
                    self.insert_char(' ');
                }
                InputAction::None
            }
            KeyEvent {
                code: KeyCode::Char('+'),
                modifiers: KeyModifiers::SHIFT,
                ..
            } => {
                self.insert_char('+');
                InputAction::None
            }
            KeyEvent {
                code: KeyCode::Char('*'),
                modifiers: KeyModifiers::SHIFT,
                ..
            } => {
                self.insert_char('*');
                InputAction::None
            }
            KeyEvent {
                code: KeyCode::Char('('),
                modifiers: KeyModifiers::SHIFT,
                ..
            } => {
                self.insert_char('(');
                InputAction::None
            }
            KeyEvent {
                code: KeyCode::Char(')'),
                modifiers: KeyModifiers::SHIFT,
                ..
            } => {
                self.insert_char(')');
                InputAction::None
            }
            KeyEvent {
                code: KeyCode::Char('_'),
                modifiers: KeyModifiers::SHIFT,
                ..
            } => {
                self.insert_char('_');
                InputAction::None
            }
            KeyEvent {
                code: KeyCode::Char('='),
                modifiers: KeyModifiers::SHIFT,
                ..
            } => {
                self.insert_char('+');
                InputAction::None
            }
            KeyEvent {
                code: KeyCode::Char('^'),
                modifiers: KeyModifiers::SHIFT,
                ..
            } => {
                self.insert_char('^');
                InputAction::None
            }
            KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                self.insert_char(c);
                InputAction::None
            }
            _ => InputAction::None,
        }
    }

    fn get_cursor_position(&self, area: Rect) -> (u16, u16) {
        let y = area.y + self.cursor.0 as u16;
        let x = area.x + self.prompt.len() as u16 + self.cursor.1 as u16;
        (x.min(area.right() - 1), y.min(area.bottom() - 1))
    }
}

impl Widget for InputArea {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width < 5 {
            return;
        }

        let block = Block::default()
            .title(" Input ")
            .borders(ratatui::widgets::Borders::ALL)
            .border_style(if self.get_text().is_empty() {
                Style::default()
            } else {
                Color::Green.into()
            });

        let inner = block.inner(area);
        block.render(area, buf);

        for (y, line) in self.lines.iter().enumerate() {
            if y as u16 >= inner.height {
                break;
            }

            let x = inner.x;
            let y = inner.y + y as u16;

            for (i, c) in line.chars().enumerate() {
                if x + i as u16 >= inner.right() {
                    break;
                }
                buf.get_mut(x + i as u16, y).set_char(c);
            }
        }

        let (cursor_x, cursor_y) = self.get_cursor_position(inner);
        buf.get_mut(cursor_x, cursor_y)
            .set_style(Style::default().add_modifier(ratatui::style::Modifier::REVERSED));

        let help_text = " [Ctrl+Enter: execute, Shift+Enter: newline, Ctrl+C: quit] ";
        let help_x = area.right().saturating_sub(help_text.len() as u16);
        for (i, c) in help_text.chars().enumerate() {
            let cell = buf.get_mut(help_x + i as u16, area.bottom() - 1);
            cell.set_char(c);
            cell.set_style(Style::default().dim());
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputAction {
    None,
    Execute,
    Interrupt,
    Quit,
    HistoryUp,
    HistoryDown,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::KeyEvent;

    #[test]
    fn test_ctrl_c_quits() {
        let mut input = InputArea::new("> ".to_string(), 80);
        let action =
            input.handle_key_event(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL));
        assert_eq!(action, InputAction::Quit);
    }
}
