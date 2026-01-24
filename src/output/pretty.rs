use crate::output::ascii::AsciiFormatter;
use crate::output::latex::LatexFormatter;
use crate::output::unicode::UnicodeFormatter;
use std::sync::OnceLock;

/// Output formatting levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrettyLevel {
    Ascii,
    Unicode,
    Latex,
}

/// Formatter trait
pub trait Formatter {
    fn format_complex(&self, num: &crate::core::ComplexNumber) -> String;
    fn format_fraction(&self, frac: &crate::core::Fraction) -> String;
    fn format_expression(&self, expr: &crate::core::types::Expression) -> String;
    fn format_equation_solution(&self, var: &str, value: &str) -> String;
    fn format_prompt(&self) -> String;
}

impl Formatter for AsciiFormatter {
    fn format_complex(&self, num: &crate::core::ComplexNumber) -> String {
        self.format_complex(num)
    }

    fn format_fraction(&self, frac: &crate::core::Fraction) -> String {
        self.format_fraction(frac)
    }

    fn format_expression(&self, expr: &crate::core::types::Expression) -> String {
        self.format_expression(expr)
    }

    fn format_equation_solution(&self, var: &str, value: &str) -> String {
        self.format_equation_solution(var, value)
    }

    fn format_prompt(&self) -> String {
        self.format_prompt()
    }
}

impl Formatter for UnicodeFormatter {
    fn format_complex(&self, num: &crate::core::ComplexNumber) -> String {
        self.format_complex(num)
    }

    fn format_fraction(&self, frac: &crate::core::Fraction) -> String {
        self.format_fraction(frac)
    }

    fn format_expression(&self, expr: &crate::core::types::Expression) -> String {
        self.format_expression(expr)
    }

    fn format_equation_solution(&self, var: &str, value: &str) -> String {
        self.format_equation_solution(var, value)
    }

    fn format_prompt(&self) -> String {
        self.format_prompt()
    }
}

impl Formatter for LatexFormatter {
    fn format_complex(&self, num: &crate::core::ComplexNumber) -> String {
        self.format_complex(num)
    }

    fn format_fraction(&self, frac: &crate::core::Fraction) -> String {
        self.format_fraction(frac)
    }

    fn format_expression(&self, expr: &crate::core::types::Expression) -> String {
        self.format_expression(expr)
    }

    fn format_equation_solution(&self, var: &str, value: &str) -> String {
        self.format_equation_solution(var, value)
    }

    fn format_prompt(&self) -> String {
        self.format_prompt()
    }
}

/// Global pretty output configuration
#[derive(Clone)]
pub struct PrettyConfig {
    level: PrettyLevel,
}

impl PrettyConfig {
    /// Get the global instance
    pub fn instance() -> &'static Self {
        static INSTANCE: OnceLock<PrettyConfig> = OnceLock::new();
        INSTANCE.get_or_init(|| PrettyConfig {
            level: Self::detect_best_format(),
        })
    }

    /// Detect the best format based on terminal capabilities
    fn detect_best_format() -> PrettyLevel {
        // Check for pdflatex availability
        if std::path::Path::new("/usr/bin/pdflatex").exists()
            || std::path::Path::new("/usr/local/bin/pdflatex").exists()
        {
            return PrettyLevel::Latex;
        }

        // Default to Unicode for modern terminals
        PrettyLevel::Unicode
    }

    /// Get the current pretty level
    pub fn get_level(&self) -> PrettyLevel {
        self.level
    }

    /// Set the pretty level
    pub fn set_level(&mut self, level: PrettyLevel) {
        self.level = level;
    }

    /// Get the appropriate formatter
    pub fn get_formatter(&self) -> Box<dyn Formatter> {
        match self.level {
            PrettyLevel::Ascii => Box::new(AsciiFormatter),
            PrettyLevel::Unicode => Box::new(UnicodeFormatter),
            PrettyLevel::Latex => Box::new(LatexFormatter),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pretty_config_instance() {
        let config = PrettyConfig::instance();
        assert!(matches!(
            config.get_level(),
            PrettyLevel::Ascii | PrettyLevel::Unicode | PrettyLevel::Latex
        ));
    }

    #[test]
    fn test_set_level() {
        let mut config = PrettyConfig {
            level: PrettyLevel::Ascii,
        };
        config.set_level(PrettyLevel::Unicode);
        assert_eq!(config.get_level(), PrettyLevel::Unicode);
    }

    #[test]
    fn test_get_formatter() {
        let config = PrettyConfig {
            level: PrettyLevel::Ascii,
        };
        let formatter = config.get_formatter();
        assert_eq!(formatter.format_prompt(), ">>> ");
    }
}
