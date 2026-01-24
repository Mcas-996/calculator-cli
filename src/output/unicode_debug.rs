/// Debug version of Unicode formatter to test radical recognition
/// This file is for testing purposes only
use crate::core::types::Expression;
use crate::core::{ComplexNumber, Fraction};

/// Unicode formatter with mathematical symbols - Debug version
pub struct UnicodeFormatterDebug;

impl UnicodeFormatterDebug {
    /// Format a complex number
    pub fn format_complex(&self, num: &ComplexNumber) -> String {
        self.format_complex_exact(num)
    }

    /// Format a fraction
    pub fn format_fraction(&self, frac: &Fraction) -> String {
        if frac.denom() == 1 {
            frac.numer().to_string()
        } else {
            format!("{}/{}", frac.numer(), frac.denom())
        }
    }

    /// Format a complex number with exact radical representation - debug version
    pub fn format_complex_exact(&self, num: &ComplexNumber) -> String {
        let decimal_real = num.real.to_f64();
        let decimal_imag = num.imag.to_f64();

        println!("DEBUG: ComplexNumber");
        println!(
            "  real = {} = {:.20}",
            self.format_fraction(&num.real),
            decimal_real
        );
        println!(
            "  imag = {} = {:.20}",
            self.format_fraction(&num.imag),
            decimal_imag
        );

        // Check if purely real
        if num.imag == Fraction::new(0, 1) {
            let result = self.format_fraction_exact(&num.real);
            println!("  -> Pure real: {}", result);
            result
        }
        // Check if purely imaginary
        else if num.real == Fraction::new(0, 1) {
            let result = self.format_imaginary_part(&num.imag);
            println!("  -> Pure imaginary: {}", result);
            result
        }
        // Mixed complex number
        else {
            let real_str = self.format_fraction_exact(&num.real);
            let result = self.format_complex_with_parts(&real_str, &num.imag);
            println!("  -> Mixed complex: {}", result);
            result
        }
    }

    /// Format imaginary part
    fn format_imaginary_part(&self, imag: &Fraction) -> String {
        if imag == &Fraction::new(1, 1) {
            "i".to_string()
        } else if imag == &Fraction::new(-1, 1) {
            "-i".to_string()
        } else {
            let imag_str = self.format_fraction_exact(imag);
            self.wrap_imaginary(&imag_str)
        }
    }

    /// Wrap imaginary string
    fn wrap_imaginary(&self, imag_str: &str) -> String {
        let needs_parens =
            imag_str.contains('+') || imag_str.contains('-') || imag_str.contains('/');
        if needs_parens {
            format!("({})i", imag_str)
        } else {
            format!("{}i", imag_str)
        }
    }

    /// Format complex with parts
    fn format_complex_with_parts(&self, real_str: &str, imag: &Fraction) -> String {
        if imag == &Fraction::new(1, 1) {
            format!("{} + i", real_str)
        } else if imag == &Fraction::new(-1, 1) {
            format!("{} - i", real_str)
        } else if imag > &Fraction::new(0, 1) {
            let imag_str = self.format_fraction_exact(imag);
            format!("{} + {}", real_str, self.wrap_imaginary(&imag_str))
        } else {
            let neg_imag = -imag.clone();
            let imag_str = self.format_fraction_exact(&neg_imag);
            format!("{} - {}", real_str, self.wrap_imaginary(&imag_str))
        }
    }

    /// Format fraction with radical recognition - debug version
    fn format_fraction_exact(&self, frac: &Fraction) -> String {
        let decimal_value = frac.to_f64();

        println!("DEBUG: format_fraction_exact");
        println!("  fraction = {}", self.format_fraction(frac));
        println!("  decimal = {:.20}", decimal_value);

        // First check if it's a simple integer
        if frac.denom() == 1 {
            let result = frac.numer().to_string();
            println!("  -> Simple integer: {}", result);
            return result;
        }

        // Test various recognition methods
        println!("  Testing recognize_perfect_square_root...");
        if let Some(radical_str) = self.recognize_perfect_square_root(decimal_value) {
            println!("  -> Found perfect square root: {}", radical_str);
            return radical_str;
        }

        println!("  Testing recognize_radical...");
        if let Some(radical_str) = self.recognize_radical(decimal_value) {
            println!("  -> Found common radical: {}", radical_str);
            return radical_str;
        }

        println!("  Testing try_coefficient_times_radical...");
        if let Some((coeff, radicand)) = self.try_coefficient_times_radical(decimal_value) {
            let result = self.format_coefficient_times_radical(coeff, radicand);
            println!("  -> Found coefficient*radical: {}", result);
            return result;
        }

        println!("  Testing try_rational_plus_radical...");
        if let Some((rational_part, radical_part, is_addition)) =
            self.try_rational_plus_radical(decimal_value)
        {
            let rational_str = self.format_rational_approx(rational_part);
            let radical_str = self.format_sqrt(&radical_part.to_string());
            let result = if is_addition {
                format!("{} + {}", rational_str, radical_str)
            } else {
                format!("{} - {}", rational_str, radical_str)
            };
            println!("  -> Found rational + radical: {}", result);
            return result;
        }

        // Fallback to fraction
        let result = format!("{}/{}", frac.numer(), frac.denom());
        println!("  -> Fallback to fraction: {}", result);
        result
    }

    /// Recognize perfect square roots with higher precision
    fn recognize_perfect_square_root(&self, value: f64) -> Option<String> {
        let abs_value = value.abs();
        let tolerance = 1e-6;

        println!(
            "    recognize_perfect_square_root({:.20}, tol={})",
            abs_value, tolerance
        );

        // Check for common perfect squares up to 100
        for n in 2..=100 {
            let sqrt_n = (n as f64).sqrt();
            let diff = (abs_value - sqrt_n).abs();
            println!(
                "    Checking sqrt({}) = {:.20}, diff = {:.20}",
                n, sqrt_n, diff
            );
            if diff < tolerance {
                println!("    -> MATCH: sqrt({})", n);
                if value < 0.0 {
                    return Some(format!("-√{}", n));
                } else {
                    return Some(format!("√{}", n));
                }
            }
        }

        None
    }

    /// Recognize common radical values
    fn recognize_radical(&self, value: f64) -> Option<String> {
        let abs_value = value.abs();
        let tolerance = 1e-6;

        let common_radicals: Vec<(f64, &str)> = vec![
            (2.0_f64.sqrt(), "2"),
            (3.0_f64.sqrt(), "3"),
            (5.0_f64.sqrt(), "5"),
            (6.0_f64.sqrt(), "6"),
            (7.0_f64.sqrt(), "7"),
            (10.0_f64.sqrt(), "10"),
        ];

        for (radical_value, radicand_str) in &common_radicals {
            let diff = (abs_value - radical_value).abs();
            println!(
                "    Checking {} vs radical {:.20}, diff = {:.20}",
                radicand_str, radical_value, diff
            );
            if diff < tolerance {
                if value < 0.0 {
                    return Some(format!("-√{}", radicand_str));
                } else {
                    return Some(format!("√{}", radicand_str));
                }
            }
        }

        None
    }

    /// Try to recognize a value as coefficient * sqrt(n)
    fn try_coefficient_times_radical(&self, value: f64) -> Option<(f64, i64)> {
        let abs_value = value.abs();
        let tolerance = 1e-6;

        println!("    try_coefficient_times_radical({:.20})", abs_value);

        for n in 2..=50 {
            let sqrt_n = (n as f64).sqrt();
            let potential_coeff = abs_value / sqrt_n;
            let simplified = Fraction::from_double(potential_coeff);
            let simplified_float = simplified.to_f64();

            println!(
                "    Checking {} * sqrt({}) = {:.20}, coeff = {:.20}",
                n, n, sqrt_n, potential_coeff
            );
            println!(
                "      simplified = {}/{} = {:.20}",
                simplified.numer(),
                simplified.denom(),
                simplified_float
            );

            if (simplified_float - potential_coeff).abs() < tolerance {
                let final_value = simplified_float * sqrt_n;
                let diff = (final_value - abs_value).abs();
                if diff < tolerance {
                    println!("    -> MATCH: {} * sqrt({})", simplified_float, n);
                    return Some((simplified_float * value.signum() as f64, n));
                }
            }
        }

        None
    }

    /// Try to recognize a value as rational ± sqrt(n)
    fn try_rational_plus_radical(&self, value: f64) -> Option<(f64, i64, bool)> {
        let tolerance = 1e-6;

        println!("    try_rational_plus_radical({:.20})", value);

        for n in 2..=50 {
            let sqrt_n = (n as f64).sqrt();

            // Try value - sqrt(n)
            let potential_rational_add = value - sqrt_n;
            let rational_add = Fraction::from_double(potential_rational_add);
            let diff = (rational_add.to_f64() - potential_rational_add).abs();
            println!(
                "    Checking {} - sqrt({}) = {:.20}, rational_part = {:.20}",
                value, n, sqrt_n, potential_rational_add
            );
            if diff < tolerance {
                println!("    -> MATCH: {} - sqrt({})", rational_add.to_f64(), n);
                return Some((rational_add.to_f64(), n, false));
            }

            // Try value + sqrt(n)
            let potential_rational_sub = value + sqrt_n;
            let rational_sub = Fraction::from_double(potential_rational_sub);
            let diff = (rational_sub.to_f64() - potential_rational_sub).abs();
            if diff < tolerance {
                println!("    -> MATCH: {} + sqrt({})", rational_sub.to_f64(), n);
                return Some((rational_sub.to_f64(), n, true));
            }
        }

        None
    }

    /// Format coefficient
    fn format_coefficient(&self, coeff: f64) -> String {
        if (coeff - 1.0).abs() < 1e-9 {
            String::new()
        } else if (coeff + 1.0).abs() < 1e-9 {
            "-".to_string()
        } else {
            let frac = Fraction::from_double(coeff);
            if frac.denom() == 1 {
                frac.numer().to_string()
            } else {
                format!("{}/{}", frac.numer(), frac.denom())
            }
        }
    }

    /// Format coefficient times radical
    fn format_coefficient_times_radical(&self, coeff: f64, radicand: i64) -> String {
        let sqrt_str = format!("√{}", radicand);
        if (coeff - 1.0).abs() < 1e-9 {
            sqrt_str
        } else if (coeff + 1.0).abs() < 1e-9 {
            format!("-{}", sqrt_str)
        } else {
            let coeff_str = self.format_coefficient(coeff);
            format!("{}{}", coeff_str, sqrt_str)
        }
    }

    /// Format rational approximation
    fn format_rational_approx(&self, value: f64) -> String {
        let frac = Fraction::from_double(value);
        if frac.denom() == 1 {
            frac.numer().to_string()
        } else {
            format!("{}/{}", frac.numer(), frac.denom())
        }
    }

    /// Format sqrt
    pub fn format_sqrt(&self, radicand: &str) -> String {
        format!("√{}", radicand)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_sqrt2() {
        let formatter = UnicodeFormatterDebug;

        let sqrt2 = 2.0_f64.sqrt();
        println!("\n=== Testing sqrt(2) ===");
        println!("Expected: √2");
        println!("Actual decimal: {:.20}", sqrt2);
        println!("Actual fraction: {}\n", Fraction::from_double(sqrt2));

        let frac = Fraction::from_double(sqrt2);
        let result = formatter.format_fraction_exact(&frac);
        println!("Result: {}", result);
    }

    #[test]
    fn test_debug_complex_sqrt() {
        let formatter = UnicodeFormatterDebug;

        let sqrt2 = 2.0_f64.sqrt();
        let num = ComplexNumber::from_double(sqrt2);

        println!("\n=== Testing ComplexNumber for sqrt(2) ===");
        let result = formatter.format_complex(&num);
        println!("\nFinal result: {}", result);
    }
}

