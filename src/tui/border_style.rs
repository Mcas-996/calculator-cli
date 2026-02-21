use ratatui::style::{Style, Stylize};

#[derive(Clone, Copy, Debug)]
pub struct BorderStyle {
    pub top_left: char,
    pub top_right: char,
    pub bottom_left: char,
    pub bottom_right: char,
    pub horizontal: char,
    pub vertical: char,
    pub style: Style,
}

impl BorderStyle {
    pub fn rounded() -> Self {
        Self {
            top_left: '╭',
            top_right: '╮',
            bottom_left: '╰',
            bottom_right: '╯',
            horizontal: '─',
            vertical: '│',
            style: Style::default().dim(),
        }
    }

    pub fn fallback() -> Self {
        Self {
            top_left: '┌',
            top_right: '┐',
            bottom_left: '└',
            bottom_right: '┘',
            horizontal: '─',
            vertical: '│',
            style: Style::default(),
        }
    }
}

impl Default for BorderStyle {
    fn default() -> Self {
        Self::rounded()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rounded_border_style_has_rounded_corners() {
        let style = BorderStyle::rounded();
        assert_eq!(style.top_left, '╭');
        assert_eq!(style.top_right, '╮');
        assert_eq!(style.bottom_left, '╰');
        assert_eq!(style.bottom_right, '╯');
    }

    #[test]
    fn test_fallback_border_style_has_square_corners() {
        let style = BorderStyle::fallback();
        assert_eq!(style.top_left, '┌');
        assert_eq!(style.top_right, '┐');
        assert_eq!(style.bottom_left, '└');
        assert_eq!(style.bottom_right, '┘');
    }

    #[test]
    fn test_default_is_rounded() {
        let default_style = BorderStyle::default();
        assert_eq!(default_style.top_left, '╭');
    }
}
