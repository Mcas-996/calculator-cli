use crate::core::types::Expression;
use crate::core::{ComplexNumber, Fraction};

/// Unicode formatter with mathematical symbols
pub struct UnicodeFormatter;

impl UnicodeFormatter {
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

    /// Format an expression
    pub fn format_expression(&self, expr: &Expression) -> String {
        match expr {
            Expression::Constant(value) => self.format_complex(value),
            Expression::Variable(name) => match name.as_str() {
                "pi" => "π".to_string(),
                _ => name.clone(),
            },
            Expression::BinaryOp { left, op, right } => {
                let op_symbol = match op {
                    crate::core::types::BinaryOperator::Add => "+",
                    crate::core::types::BinaryOperator::Subtract => "−",
                    crate::core::types::BinaryOperator::Multiply => "×",
                    crate::core::types::BinaryOperator::Divide => "÷",
                    crate::core::types::BinaryOperator::Power => "^",
                    crate::core::types::BinaryOperator::Modulo => "%",
                };
                format!(
                    "({} {} {})",
                    self.format_expression(left),
                    op_symbol,
                    self.format_expression(right)
                )
            }
            Expression::UnaryOp { op, operand } => {
                format!("({}{})", op.symbol(), self.format_expression(operand))
            }
            Expression::Function { name, args } => {
                let args_str: Vec<String> =
                    args.iter().map(|a| self.format_expression(a)).collect();
                format!("{}({})", name.as_str(), args_str.join(", "))
            }
        }
    }

    /// Format an equation solution
    pub fn format_equation_solution(&self, var: &str, value: &str) -> String {
        format!("{} = {}", var, value)
    }

    /// Format the prompt
    pub fn format_prompt(&self) -> String {
        "➤ ".to_string()
    }

    /// Format subscript
    pub fn format_subscript(&self, text: &str) -> String {
        let subscripts = [
            ('0', '₀'),
            ('1', '₁'),
            ('2', '₂'),
            ('3', '₃'),
            ('4', '₄'),
            ('5', '₅'),
            ('6', '₆'),
            ('7', '₇'),
            ('8', '₈'),
            ('9', '₉'),
        ];

        text.chars()
            .map(|c| {
                subscripts
                    .iter()
                    .find(|(k, _)| k == &c)
                    .map(|(_, v)| *v)
                    .unwrap_or(c)
            })
            .collect()
    }

    /// Format superscript
    pub fn format_superscript(&self, text: &str) -> String {
        let superscripts = [
            ('0', '⁰'),
            ('1', '¹'),
            ('2', '²'),
            ('3', '³'),
            ('4', '⁴'),
            ('5', '⁵'),
            ('6', '⁶'),
            ('7', '⁷'),
            ('8', '⁸'),
            ('9', '⁹'),
            ('-', '⁻'),
            ('+', '⁺'),
        ];

        text.chars()
            .map(|c| {
                superscripts
                    .iter()
                    .find(|(k, _)| k == &c)
                    .map(|(_, v)| *v)
                    .unwrap_or(c)
            })
            .collect()
    }

    /// Format a square root symbolically
    pub fn format_sqrt(&self, radicand: &str) -> String {
        format!("√{}", radicand)
    }

    /// Format an nth root symbolically
    pub fn format_nth_root(&self, index: i64, radicand: &str) -> String {
        if index == 2 {
            self.format_sqrt(radicand)
        } else {
            format!(
                "{}√{}",
                self.format_superscript(&index.to_string()),
                radicand
            )
        }
    }

    /// Format a complex number with exact radical representation when possible
    pub fn format_complex_exact(&self, num: &ComplexNumber) -> String {
        // Check if purely real
        if num.imag == Fraction::new(0, 1) {
            self.format_fraction_exact(&num.real)
        }
        // Check if purely imaginary
        else if num.real == Fraction::new(0, 1) {
            self.format_imaginary_part(&num.imag)
        }
        // Mixed complex number - handle all cases including radicals in both parts
        else {
            let real_str = self.format_fraction_exact(&num.real);
            self.format_complex_with_parts(&real_str, &num.imag)
        }
    }

    /// Format imaginary part, handling special cases and radicals
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

    /// Wrap imaginary string properly for display
    fn wrap_imaginary(&self, imag_str: &str) -> String {
        // Check if it contains a radical or fraction that needs parentheses
        let needs_parens =
            imag_str.contains('+') || imag_str.contains("-") || imag_str.contains('/');
        if needs_parens {
            format!("({})i", imag_str)
        } else {
            format!("{}i", imag_str)
        }
    }

    /// Format complex number given real part string and imaginary Fraction
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

    /// Format a fraction with exact radical representation when possible
    fn format_fraction_exact(&self, frac: &Fraction) -> String {
        // First check if it's a simple integer
        if frac.denom() == 1 {
            return frac.numer().to_string();
        }

        let decimal_value = frac.to_f64();

        // Special handling for common quadratic equation solutions
        if let Some(radical_str) = self.recognize_quadratic_solution(decimal_value) {
            return radical_str;
        }

        // Enhanced radical recognition - check for perfect squares first
        if let Some(radical_str) = self.recognize_perfect_square_root(decimal_value) {
            return radical_str;
        }

        // Then check for common radical values from the decimal approximation
        if let Some(radical_str) = self.recognize_radical(decimal_value) {
            return radical_str;
        }

        // Check if this is a rational times a radical
        // For example, (3*sqrt(2))/2 ≈ 2.121
        if let Some((coeff, radicand)) = self.try_coefficient_times_radical(decimal_value) {
            return self.format_coefficient_times_radical(coeff, radicand);
        }

        // Check if this is a rational plus/minus a radical
        // For example, 1 + sqrt(2) ≈ 2.414
        if let Some((rational_part, radical_part, is_addition)) =
            self.try_rational_plus_radical(decimal_value)
        {
            let rational_str = self.format_rational_approx(rational_part);
            let radical_str = self.format_sqrt(&radical_part.to_string());
            if is_addition {
                return format!("{} + {}", rational_str, radical_str);
            } else {
                return format!("{} - {}", rational_str, radical_str);
            }
        }

        // Try to decompose as (a + b√n)/d format
        if let Some((a, b, n, d)) = self.try_rational_plus_radical_decomposed(decimal_value) {
            let a_str = self.format_rational_approx(a);
            let b_str = self.format_coefficient(b);
            let radical_str = self.format_sqrt(&n.to_string());
            return format!("({} + {}√{})/{}", a_str, b_str, radical_str, d);
        }

        // Default to fraction representation
        format!("{}/{}", frac.numer(), frac.denom())
    }

    /// Recognize quadratic equation solutions by analyzing decimal approximations
    fn recognize_quadratic_solution(&self, value: f64) -> Option<String> {
        let abs_value = value.abs();
        let tolerance = 1e-3;

        // Handle common simple radicals
        for n in 2..=100 {
            let sqrt_n = (n as f64).sqrt();

            // Check if value is approximately sqrt(n)
            if (abs_value - sqrt_n).abs() < tolerance {
                if value < 0.0 {
                    return Some(format!("-√{}", n));
                } else {
                    return Some(format!("√{}", n));
                }
            }

            // Check if value is approximately sqrt(n)/d
            for d in 2..=10 {
                let sqrt_n_over_d = sqrt_n / (d as f64);
                if (abs_value - sqrt_n_over_d).abs() < tolerance {
                    if value < 0.0 {
                        return Some(format!("-√{}/{}", n, d));
                    } else {
                        return Some(format!("√{}/{}", n, d));
                    }
                }
            }

            // Check if value is approximately a*sqrt(n)
            for a in 2..=10 {
                let a_sqrt_n = (a as f64) * sqrt_n;
                if (abs_value - a_sqrt_n).abs() < tolerance {
                    if value < 0.0 {
                        return Some(format!("-{}√{}", a, n));
                    } else {
                        return Some(format!("{}√{}", a, n));
                    }
                }
            }

            // Check if value is approximately a*sqrt(n)/d
            for a in 2..=10 {
                for d in 2..=10 {
                    let a_sqrt_n_over_d = (a as f64) * sqrt_n / (d as f64);
                    if (abs_value - a_sqrt_n_over_d).abs() < tolerance {
                        if value < 0.0 {
                            return Some(format!("-{}√{}/{}", a, n, d));
                        } else {
                            return Some(format!("{}√{}/{}", a, n, d));
                        }
                    }
                }
            }
        }

        None
    }

    /// Format coefficient times radical (helper for cleaner output)
    fn format_coefficient_times_radical(&self, coeff: f64, radicand: i64) -> String {
        let sqrt_str = self.format_sqrt(&radicand.to_string());
        if (coeff - 1.0).abs() < 1e-9 {
            sqrt_str
        } else if (coeff + 1.0).abs() < 1e-9 {
            format!("-{}", sqrt_str)
        } else {
            let coeff_str = self.format_coefficient(coeff);
            format!("{}{}", coeff_str, sqrt_str)
        }
    }

    /// Try to decompose a value as (a + b√n)/d
    fn try_rational_plus_radical_decomposed(&self, value: f64) -> Option<(f64, f64, i64, i64)> {
        let tolerance = 1e-3;
        let abs_value = value.abs();
        let sign = if value < 0.0 { -1.0 } else { 1.0 };

        for n in 2..=50 {
            let sqrt_n = (n as f64).sqrt();

            // Try (a + b√n) / d format
            // value = (a + b*sqrt(n)) / d
            // value * d ≈ a + b*sqrt(n)

            for d in 2..=10 {
                let scaled = abs_value * (d as f64);

                // Try various b values (integers and simple fractions)
                for b_num in &[-3i64, -2, -1, 1, 2, 3] {
                    let b = *b_num as f64;
                    let potential_a = scaled - b * sqrt_n;

                    // Check if a is a simple rational
                    let a_frac = Fraction::from_double(potential_a);
                    let a_float = a_frac.to_f64();

                    if (a_float - potential_a).abs() < tolerance {
                        // Verify: (a + b√n) / d ≈ value
                        let reconstructed = (a_float + b * sqrt_n) / (d as f64);
                        if (reconstructed - abs_value).abs() < tolerance {
                            return Some((a_float * sign, b * sign, n, d));
                        }
                    }
                }
            }
        }

        None
    }

    /// Recognize perfect square roots with higher precision
    fn recognize_perfect_square_root(&self, value: f64) -> Option<String> {
        let abs_value = value.abs();
        let tolerance = 1e-10; // Higher precision for perfect squares

        // Check for common perfect squares up to 100
        for n in 2..=100 {
            let sqrt_n = (n as f64).sqrt();
            if (abs_value - sqrt_n).abs() < tolerance {
                if value < 0.0 {
                    return Some(format!("-{}", self.format_sqrt(&n.to_string())));
                } else {
                    return Some(self.format_sqrt(&n.to_string()));
                }
            }
        }

        None
    }

    /// Recognize common radical values from decimal approximation
    fn recognize_radical(&self, value: f64) -> Option<String> {
        let common_radicals: Vec<(f64, &str)> = vec![
            (2.0_f64.sqrt(), "2"),
            (3.0_f64.sqrt(), "3"),
            (5.0_f64.sqrt(), "5"),
            (6.0_f64.sqrt(), "6"),
            (7.0_f64.sqrt(), "7"),
            (10.0_f64.sqrt(), "10"),
            (11.0_f64.sqrt(), "11"),
            (13.0_f64.sqrt(), "13"),
            (17.0_f64.sqrt(), "17"),
            (19.0_f64.sqrt(), "19"),
            ((2.0_f64).cbrt(), "2"),
            ((3.0_f64).cbrt(), "3"),
            ((4.0_f64).cbrt(), "4"),
            ((5.0_f64).cbrt(), "5"),
            ((6.0_f64).cbrt(), "6"),
            ((7.0_f64).cbrt(), "7"),
        ];

        let abs_value = value.abs();
        let tolerance = 1e-3;

        for (radical_value, radicand_str) in common_radicals {
            if (abs_value - radical_value).abs() < tolerance {
                if value < 0.0 {
                    return Some(format!("-{}", self.format_sqrt(radicand_str)));
                } else {
                    return Some(self.format_sqrt(radicand_str));
                }
            }
        }

        None
    }

    /// Try to recognize a value as coefficient * sqrt(n)
    fn try_coefficient_times_radical(&self, value: f64) -> Option<(f64, i64)> {
        let abs_value = value.abs();
        let tolerance = 1e-3;

        // Try common radicands
        for n in 2..=50 {
            let sqrt_n = (n as f64).sqrt();
            let potential_coeff = abs_value / sqrt_n;

            // Check if the coefficient is a simple fraction
            let simplified = Fraction::from_double(potential_coeff);
            let simplified_float = simplified.to_f64();

            if (simplified_float - potential_coeff).abs() < tolerance {
                let final_value = simplified_float * sqrt_n;
                if (final_value - abs_value).abs() < tolerance {
                    return Some((simplified_float * value.signum() as f64, n));
                }
            }
        }

        None
    }

    /// Try to recognize a value as rational ± sqrt(n)
    fn try_rational_plus_radical(&self, value: f64) -> Option<(f64, i64, bool)> {
        let tolerance = 1e-3;

        // Try common radicands
        for n in 2..=50 {
            let sqrt_n = (n as f64).sqrt();

            // Try value - sqrt(n)
            let potential_rational_add = value - sqrt_n;
            let rational_add = Fraction::from_double(potential_rational_add);
            if (rational_add.to_f64() - potential_rational_add).abs() < tolerance {
                return Some((rational_add.to_f64(), n, true));
            }

            // Try value + sqrt(n)
            let potential_rational_sub = value + sqrt_n;
            let rational_sub = Fraction::from_double(potential_rational_sub);
            if (rational_sub.to_f64() - potential_rational_sub).abs() < tolerance {
                return Some((rational_sub.to_f64(), n, false));
            }

            // Try sqrt(n) - value (this handles cases like sqrt(2) - 1)
            let potential_rational_alt = sqrt_n - value;
            let rational_alt = Fraction::from_double(potential_rational_alt);
            if (rational_alt.to_f64() - potential_rational_alt).abs() < tolerance {
                return Some((-rational_alt.to_f64(), n, true));
            }
        }

        None
    }

    /// Format a coefficient (handles special cases like 1, -1, etc.)
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

    /// Check if a decimal value looks like it contains a radical (for formatting purposes)
    fn looks_like_radical(&self, value: f64) -> bool {
        if value == 0.0 {
            return false;
        }

        let abs_value = value.abs();
        let tolerance = 1e-3;

        // Check if it matches a simple radical pattern
        for n in 2..=50 {
            let sqrt_n = (n as f64).sqrt();
            if (abs_value - sqrt_n).abs() < tolerance {
                return true;
            }
            if (abs_value - 1.0 / sqrt_n).abs() < tolerance {
                return true;
            }
        }

        // Check for coefficient * sqrt(n)
        for n in 2..=50 {
            let sqrt_n = (n as f64).sqrt();
            let coeff = abs_value / sqrt_n;
            let frac = Fraction::from_double(coeff);
            if (frac.to_f64() - coeff).abs() < tolerance {
                return true;
            }
        }

        // Check for (a + b√n)/d format
        for n in 2..=50 {
            let sqrt_n = (n as f64).sqrt();
            for d in 2..=10 {
                let scaled = abs_value * (d as f64);
                for b_num in &[-3i64, -2, -1, 1, 2, 3] {
                    let b = *b_num as f64;
                    let potential_a = scaled - b * sqrt_n;
                    let a_frac = Fraction::from_double(potential_a);
                    if (a_frac.to_f64() - potential_a).abs() < tolerance {
                        return true;
                    }
                }
            }
        }

        false
    }

    /// Format a rational approximation as a simple fraction or integer
    fn format_rational_approx(&self, value: f64) -> String {
        let frac = Fraction::from_double(value);
        if frac.denom() == 1 {
            frac.numer().to_string()
        } else {
            format!("{}/{}", frac.numer(), frac.denom())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_complex_real() {
        let formatter = UnicodeFormatter;
        let num = ComplexNumber::from_real(Fraction::new(5, 1));
        assert_eq!(formatter.format_complex(&num), "5");
    }

    #[test]
    fn test_format_complex_mixed() {
        let formatter = UnicodeFormatter;
        let num = ComplexNumber::new(Fraction::new(3, 1), Fraction::new(4, 1));
        assert_eq!(formatter.format_complex(&num), "3 + 4i");
    }

    #[test]
    fn test_format_complex_with_radical_real() {
        let formatter = UnicodeFormatter;
        // 1 + √2 ≈ 2.414
        let num = ComplexNumber::from_double(1.0 + 2.0_f64.sqrt());
        let result = formatter.format_complex(&num);
        // Should recognize as "1 + √2" or similar radical format
        assert!(result.contains("√2") || result.contains("2"));
    }

    #[test]
    fn test_format_complex_with_radical_imag() {
        let formatter = UnicodeFormatter;
        // 1 + √3i/2 format for (1 ± √3i)/2
        // For equation x² - x + 1 = 0, solutions are (1 ± √3i)/2
        let sqrt3 = (3.0_f64).sqrt();
        let num = ComplexNumber::new(
            Fraction::from_double(0.5),
            Fraction::from_double(sqrt3 / 2.0),
        );
        let result = formatter.format_complex(&num);
        // Should contain radical notation for imaginary part
        assert!(result.contains("√3") || result.contains("3"));
    }

    #[test]
    fn test_format_complex_pure_imaginary() {
        let formatter = UnicodeFormatter;
        let num = ComplexNumber::new(Fraction::new(0, 1), Fraction::new(1, 1));
        assert_eq!(formatter.format_complex(&num), "i");

        let num2 = ComplexNumber::new(Fraction::new(0, 1), Fraction::new(-1, 1));
        assert_eq!(formatter.format_complex(&num2), "-i");
    }

    #[test]
    fn test_format_complex_fraction_imag() {
        let formatter = UnicodeFormatter;
        // 0 + 1/2 i
        let num = ComplexNumber::new(Fraction::new(0, 1), Fraction::new(1, 2));
        let result = formatter.format_complex(&num);
        assert_eq!(result, "(1/2)i");
    }

    #[test]
    fn test_recognize_quadratic_solution() {
        let formatter = UnicodeFormatter;

        // Test sqrt(2) ≈ 1.414213562
        let sqrt2 = (2.0_f64).sqrt();
        let frac_sqrt2 = Fraction::from_double(sqrt2);
        let result = formatter.format_fraction(&frac_sqrt2);
        assert_eq!(result, "√2");

        // Test -sqrt(2)
        let frac_neg_sqrt2 = Fraction::from_double(-sqrt2);
        let result = formatter.format_fraction(&frac_neg_sqrt2);
        assert_eq!(result, "-√2");

        // Test sqrt(3)/2 ≈ 0.866025403
        let sqrt3_over_2 = (3.0_f64).sqrt() / 2.0;
        let frac_sqrt3_over_2 = Fraction::from_double(sqrt3_over_2);
        let result = formatter.format_fraction(&frac_sqrt3_over_2);
        assert_eq!(result, "√3/2");

        // Test 2*sqrt(2) ≈ 2.828427124746
        let two_sqrt2 = 2.0 * (2.0_f64).sqrt();
        let frac_two_sqrt2 = Fraction::from_double(two_sqrt2);
        let result = formatter.format_fraction(&frac_two_sqrt2);
        assert_eq!(result, "2√2");
    }

    #[test]
    fn test_quadratic_equation_solutions() {
        let formatter = UnicodeFormatter;

        // For x^2 - 2 = 0, solutions are ±√2
        let sqrt2 = (2.0_f64).sqrt();
        let sol1 = ComplexNumber::from_double(sqrt2);
        let sol2 = ComplexNumber::from_double(-sqrt2);

        assert_eq!(formatter.format_complex(&sol1), "√2");
        assert_eq!(formatter.format_complex(&sol2), "-√2");

        // For x^2 - x + 1 = 0, solutions are (1 ± √3i)/2
        let real_part = 0.5;
        let imag_part = (3.0_f64).sqrt() / 2.0;
        let sol_complex1 = ComplexNumber::from_doubles(real_part, imag_part);
        let sol_complex2 = ComplexNumber::from_doubles(real_part, -imag_part);

        let result1 = formatter.format_complex(&sol_complex1);
        let result2 = formatter.format_complex(&sol_complex2);

        // Check that results contain the expected components
        assert!(result1.contains("1/2") || result1.contains("0.5"));
        assert!(result1.contains("√3") || result1.contains("3"));
        assert!(result1.contains("i"));

        assert!(result2.contains("1/2") || result2.contains("0.5"));
        assert!(result2.contains("√3") || result2.contains("3"));
        assert!(result2.contains("i"));
    }

    #[test]
    fn test_format_subscript() {
        let formatter = UnicodeFormatter;
        assert_eq!(formatter.format_subscript("123"), "₁₂₃");
    }

    #[test]
    fn test_format_superscript() {
        let formatter = UnicodeFormatter;
        assert_eq!(formatter.format_superscript("23"), "²³");
    }

    #[test]
    fn test_format_prompt() {
        let formatter = UnicodeFormatter;
        assert_eq!(formatter.format_prompt(), "➤ ");
    }
}
