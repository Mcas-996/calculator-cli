use crate::core::ComplexNumber;
use crate::parser::parse_expression;
use crate::solver::{
    solve_2x2_system, solve_3x3_system, solve_cubic_equation, solve_linear_equation,
    solve_quadratic_equation, solve_quartic_equation, solve_quintic_equation,
};
use crate::tui::input::{InputAction, InputArea};
use crate::tui::result_card::ResultCard;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use std::io;

pub struct TuiApp {
    results: Vec<ResultCard>,
    input: InputArea,
    ans: Option<ComplexNumber>,
    scroll_offset: usize,
}

impl TuiApp {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
            input: InputArea::new("> ".to_string(), 80),
            ans: None,
            scroll_offset: 0,
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        self.show_welcome();

        loop {
            terminal.draw(|f| self.render(f))?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    let action = self.input.handle_key_event(key);
                    match action {
                        InputAction::Execute => {
                            if self.execute_input() {
                                break;
                            }
                        }
                        InputAction::Quit => {
                            break;
                        }
                        InputAction::Interrupt => {
                            self.input.clear();
                        }
                        InputAction::HistoryUp => {
                            self.input.history_up();
                        }
                        InputAction::HistoryDown => {
                            self.input.history_down();
                        }
                        InputAction::None => {}
                    }
                }
            }
        }

        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        Ok(())
    }

    fn show_welcome(&mut self) {
        let welcome = vec![
            " Calculator CLI v2.0.0 (TUI Mode) ",
            "==================================",
            "Commands:",
            "  Enter       - Calculate expression",
            "  Shift+Enter - New line",
            "  exit/quit   - Exit the calculator",
            "  clear       - Clear results",
            "  help        - Show this help",
            "  ans / ans() - Show last result",
            "",
            "Examples:",
            "  2 + 2           - Basic arithmetic",
            "  sqrt(2)         - Square root",
            "  x^2 + 2x + 1=0  - Solve quadratic",
            "",
        ];

        for line in welcome {
            self.results
                .push(ResultCard::new(None, vec![line.to_string()]));
        }
    }

    fn execute_input(&mut self) -> bool {
        let text = self.input.get_text().trim().to_string();

        if text.is_empty() {
            return false;
        }

        self.input.push_to_history();

        if text == "exit" || text == "quit" {
            return true;
        }

        if text == "clear" {
            self.results.clear();
            self.input.clear();
            return false;
        }

        if text == "help" {
            self.show_help();
            self.input.clear();
            return false;
        }

        if text == "ans" {
            if let Some(ans) = &self.ans {
                self.results
                    .push(ResultCard::from_complex(Some("ans".to_string()), ans));
            } else {
                self.results.push(ResultCard::error(
                    Some("ans".to_string()),
                    "No previous result".to_string(),
                ));
            }
            self.input.clear();
            return false;
        }

        if text == "ans()" {
            if let Some(ans) = &self.ans {
                self.results
                    .push(ResultCard::from_complex(Some("ans()".to_string()), ans));
            } else {
                self.results.push(ResultCard::error(
                    Some("ans()".to_string()),
                    "No previous result".to_string(),
                ));
            }
            self.input.clear();
            return false;
        }

        let input_for_display = text.clone();

        if text.contains('=') {
            self.process_equation(&text, Some(input_for_display));
        } else {
            self.process_expression(&text, Some(input_for_display));
        }

        self.input.clear();
        self.scroll_offset = self.results.len().saturating_sub(10);

        false
    }

    fn show_help(&mut self) {
        let help = vec![
            "Commands:".to_string(),
            "  exit/quit   - Exit the calculator".to_string(),
            "  clear       - Clear results".to_string(),
            "  help        - Show this help".to_string(),
            "  ans / ans() - Show last result".to_string(),
            "".to_string(),
            "Examples:".to_string(),
            "  2 + 2*3         - Basic arithmetic".to_string(),
            "  sqrt(2)         - Square root".to_string(),
            "  (1+2i)*(3+4i)   - Complex numbers".to_string(),
            "  x^2=4           - Solve equation".to_string(),
            "  x^2+2x+1=0      - Quadratic equation".to_string(),
            "  equation(x^2-4)  - Alternative syntax".to_string(),
        ];

        self.results
            .push(ResultCard::new(Some("help".to_string()), help));
    }

    fn process_expression(&mut self, input: &str, display: Option<String>) {
        match parse_expression(input) {
            Ok(expr) => match expr.evaluate() {
                Ok(result) => {
                    self.ans = Some(result.clone());
                    self.results
                        .push(ResultCard::from_complex(display, &result));
                }
                Err(e) => {
                    self.results.push(ResultCard::error(display, e));
                }
            },
            Err(e) => {
                self.results.push(ResultCard::error(display, e));
            }
        }
    }

    fn process_equation(&mut self, input: &str, display: Option<String>) {
        let clean_input = input.trim();

        if clean_input.contains(',') {
            let equations: Vec<String> = clean_input
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();

            if equations.len() == 2 {
                match solve_2x2_system(&equations) {
                    Ok(solutions) => {
                        for (var, value) in solutions {
                            let value_clone = value.clone();
                            let card = ResultCard::from_equation_solution(
                                display.clone(),
                                &var,
                                &[value_clone],
                            );
                            self.results.push(card);
                            self.ans = Some(value);
                        }
                    }
                    Err(e) => {
                        self.results.push(ResultCard::error(display, e));
                    }
                }
            } else if equations.len() == 3 {
                match solve_3x3_system(&equations) {
                    Ok(solutions) => {
                        for (var, value) in solutions {
                            let value_clone = value.clone();
                            let card = ResultCard::from_equation_solution(
                                display.clone(),
                                &var,
                                &[value_clone],
                            );
                            self.results.push(card);
                            self.ans = Some(value);
                        }
                    }
                    Err(e) => {
                        self.results.push(ResultCard::error(display, e));
                    }
                }
            } else {
                self.results.push(ResultCard::error(
                    display,
                    "Only 2x2 and 3x3 systems are supported".to_string(),
                ));
            }
        } else {
            let degree = self.determine_equation_degree(clean_input);

            let solutions = match degree {
                1 => solve_linear_equation(clean_input),
                2 => solve_quadratic_equation(clean_input),
                3 => solve_cubic_equation(clean_input),
                4 => solve_quartic_equation(clean_input),
                5.. => solve_quintic_equation(clean_input),
                _ => Err("Could not determine equation degree".to_string()),
            };

            match solutions {
                Ok(sols) => {
                    if sols.is_empty() {
                        self.results
                            .push(ResultCard::error(display, "No solutions found".to_string()));
                    } else {
                        let var = if sols.len() == 1 {
                            "x".to_string()
                        } else {
                            "x".to_string()
                        };

                        self.results
                            .push(ResultCard::from_equation_solution(display, &var, &sols));

                        self.ans = Some(sols[0].clone());
                    }
                }
                Err(e) => {
                    self.results.push(ResultCard::error(display, e));
                }
            }
        }
    }

    fn determine_equation_degree(&self, input: &str) -> usize {
        let input_lower = input.to_lowercase();

        if input_lower.contains("^5") || input_lower.contains("⁵") {
            5
        } else if input_lower.contains("^4") || input_lower.contains("⁴") {
            4
        } else if input_lower.contains("^3") || input_lower.contains("³") {
            3
        } else if input_lower.contains("^2") || input_lower.contains("²") {
            2
        } else if input_lower.contains('x') {
            1
        } else {
            0
        }
    }

    fn render(&self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(3),
                Constraint::Length(3),
                Constraint::Length(6),
            ])
            .split(f.size());

        let results_area = chunks[0];
        let ans_area = chunks[1];
        let input_area = chunks[2];

        if self.results.is_empty() {
            let empty = Paragraph::new("No results yet. Enter an expression to calculate.")
                .block(Block::default().title(" Results ").borders(Borders::ALL))
                .style(Style::default().fg(Color::DarkGray));
            f.render_widget(empty, results_area);
        } else {
            let display_results = if self.results.len() > results_area.height as usize {
                let start = self
                    .results
                    .len()
                    .saturating_sub(results_area.height as usize);
                &self.results[start..]
            } else {
                &self.results
            };

            let mut all_lines: Vec<String> = Vec::new();

            for result in display_results {
                let _card_height = (result.height() as u16).min(results_area.width);
                let card_lines = result.render(results_area.width as usize);

                for line in card_lines {
                    if all_lines.len() < results_area.height as usize {
                        all_lines.push(line);
                    }
                }

                if all_lines.len() < results_area.height as usize {
                    if !all_lines.is_empty() && !all_lines.last().map_or(false, |s| s.is_empty()) {
                        all_lines.push(String::new());
                    }
                }
            }

            let content: Vec<ListItem> = all_lines
                .iter()
                .map(|line| ListItem::new(line.as_str()))
                .collect();

            let list = List::new(content)
                .block(Block::default().title(" Results ").borders(Borders::ALL))
                .style(Style::default().fg(Color::White));

            f.render_widget(list, results_area);
        }

        let ans_text = if let Some(ans) = &self.ans {
            format!(" ans = {}", ans.to_string())
        } else {
            " ans = (none)".to_string()
        };

        let ans_widget = Paragraph::new(ans_text)
            .block(
                Block::default()
                    .title(" Last Result ")
                    .borders(Borders::ALL),
            )
            .style(Style::default().fg(Color::Yellow));

        f.render_widget(ans_widget, ans_area);

        let _input_widget = InputArea::new("> ".to_string(), input_area.width as usize);
        let mut input_clone = InputArea::new("> ".to_string(), input_area.width as usize);
        input_clone.lines = self.input.lines.clone();
        input_clone.cursor = self.input.cursor;
        input_clone.history = self.input.history.clone();
        input_clone.history_index = self.input.history_index;

        f.render_widget(input_clone, input_area);
    }
}

impl Default for TuiApp {
    fn default() -> Self {
        Self::new()
    }
}
