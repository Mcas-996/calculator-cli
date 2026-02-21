use std::error::Error;
use std::fmt;

use tui_math::{render_latex, RenderError};

#[derive(Debug)]
pub enum TuiMathAdapterError {
    UnsupportedInput(&'static str),
    Render(RenderError),
}

impl fmt::Display for TuiMathAdapterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedInput(msg) => write!(f, "unsupported input: {msg}"),
            Self::Render(err) => write!(f, "render error: {err}"),
        }
    }
}

impl Error for TuiMathAdapterError {}

pub fn render_math_text(input: &str) -> Result<Vec<String>, TuiMathAdapterError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(TuiMathAdapterError::UnsupportedInput("empty math text"));
    }

    let latex = to_latex(trimmed)?;
    let rendered = render_latex(&latex).map_err(TuiMathAdapterError::Render)?;
    let lines = rendered
        .lines()
        .map(|line| line.trim_end().to_string())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    if lines.is_empty() {
        return Err(TuiMathAdapterError::UnsupportedInput(
            "renderer returned no visible output",
        ));
    }

    Ok(lines)
}

fn to_latex(input: &str) -> Result<String, TuiMathAdapterError> {
    let sqrt_rewritten = rewrite_sqrt_calls(input)?;
    let fractions_rewritten = rewrite_simple_numeric_fractions(&sqrt_rewritten);
    let product_rewritten = fractions_rewritten.replace('*', r" \cdot ");
    Ok(product_rewritten)
}

fn rewrite_sqrt_calls(input: &str) -> Result<String, TuiMathAdapterError> {
    let chars = input.chars().collect::<Vec<_>>();
    let mut i = 0;
    let mut out = String::new();

    while i < chars.len() {
        if starts_with_sqrt_call(&chars, i) {
            i += 5; // skip "sqrt("
            let mut depth = 1;
            let inner_start = i;
            while i < chars.len() && depth > 0 {
                if chars[i] == '(' {
                    depth += 1;
                } else if chars[i] == ')' {
                    depth -= 1;
                }
                i += 1;
            }

            if depth != 0 {
                return Err(TuiMathAdapterError::UnsupportedInput(
                    "unclosed sqrt() expression",
                ));
            }

            let inner_end = i - 1;
            let inner = chars[inner_start..inner_end].iter().collect::<String>();
            let rewritten_inner = rewrite_sqrt_calls(&inner)?;
            out.push_str(r"\sqrt{");
            out.push_str(&rewritten_inner);
            out.push('}');
            continue;
        }

        out.push(chars[i]);
        i += 1;
    }

    Ok(out)
}

fn starts_with_sqrt_call(chars: &[char], idx: usize) -> bool {
    idx + 5 <= chars.len()
        && chars[idx] == 's'
        && chars[idx + 1] == 'q'
        && chars[idx + 2] == 'r'
        && chars[idx + 3] == 't'
        && chars[idx + 4] == '('
}

fn rewrite_simple_numeric_fractions(input: &str) -> String {
    let chars = input.chars().collect::<Vec<_>>();
    let mut i = 0;
    let mut out = String::new();

    while i < chars.len() {
        let start = i;
        let mut has_sign = false;

        if chars[i] == '-' {
            has_sign = true;
            i += 1;
            if i >= chars.len() || !chars[i].is_ascii_digit() {
                out.push('-');
                continue;
            }
        }

        if !chars[i].is_ascii_digit() {
            out.push(chars[i]);
            i += 1;
            continue;
        }

        let numer_start = if has_sign { start + 1 } else { start };
        while i < chars.len() && chars[i].is_ascii_digit() {
            i += 1;
        }
        let numer_end = i;

        if i < chars.len() && chars[i] == '/' {
            i += 1;
            let denom_start = i;
            while i < chars.len() && chars[i].is_ascii_digit() {
                i += 1;
            }
            if denom_start < i {
                let numer = chars[numer_start..numer_end].iter().collect::<String>();
                let denom = chars[denom_start..i].iter().collect::<String>();
                if has_sign {
                    out.push('-');
                }
                out.push_str(r"\frac{");
                out.push_str(&numer);
                out.push_str("}{");
                out.push_str(&denom);
                out.push('}');
                continue;
            }
            i = start;
        } else {
            i = start;
        }

        out.push(chars[i]);
        i += 1;
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rewrites_sqrt_and_fraction_to_latex() {
        let latex = to_latex("-i*sqrt(2) + 3/4").unwrap();
        assert!(latex.contains(r"\sqrt{2}"));
        assert!(latex.contains(r"\frac{3}{4}"));
        assert!(latex.contains(r"\cdot"));
    }

    #[test]
    fn render_math_text_renders_fraction_multiline() {
        let rendered = render_math_text("-5/2").unwrap();
        assert!(rendered.iter().any(|line| line.contains('─')));
    }
}
