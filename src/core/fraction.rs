use num_rational::Rational64;

/// Wrapper for Fraction using num-rational's Rational64
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord)]
pub struct Fraction(pub Rational64);

impl Fraction {
    /// Create a new fraction
    pub fn new(numerator: i64, denominator: i64) -> Self {
        Fraction(Rational64::new(numerator, denominator))
    }

    /// Create a Fraction from a double value with rational approximation
    pub fn from_double(value: f64) -> Self {
        // Handle special cases
        if value == 0.0 {
            return Fraction(Rational64::new(0, 1));
        }

        // Try simple fractions first
        let simple_fractions = vec![
            (0.5, 1, 2),
            (0.25, 1, 4),
            (0.75, 3, 4),
            (0.125, 1, 8),
            (0.375, 3, 8),
            (0.625, 5, 8),
            (0.875, 7, 8),
            (0.3333333333333333, 1, 3),
            (0.6666666666666666, 2, 3),
            (0.2, 1, 5),
            (0.4, 2, 5),
            (0.6, 3, 5),
            (0.8, 4, 5),
        ];

        let abs_value = value.abs();
        for (frac_val, num, den) in simple_fractions {
            if (abs_value - frac_val).abs() < 1e-10 {
                let mut result = Fraction(Rational64::new(num, den));
                if value < 0.0 {
                    result = -result;
                }
                return result;
            }
        }

        // Use continued fraction approximation for other values
        // Use a simple rational approximation with a limited denominator

        // Use a simple rational approximation with a limited denominator
        let mut best_num = 0;
        let mut best_den = 1;
        let mut best_error = abs_value;

        for den in 1..10000 {
            let num = (abs_value * den as f64).round() as i64;
            let error = (abs_value - num as f64 / den as f64).abs();
            if error < best_error {
                best_error = error;
                best_num = num;
                best_den = den;
            }
        }

        let mut result = Fraction(Rational64::new(best_num, best_den));
        if value < 0.0 {
            result = -result;
        }
        result
    }

    /// Convert to string representation
    pub fn to_string(&self) -> String {
        if *self.0.denom() == 1 {
            self.0.numer().to_string()
        } else {
            format!("{}/{}", self.0.numer(), self.0.denom())
        }
    }

    /// Convert to f64
    pub fn to_f64(&self) -> f64 {
        num_traits::ToPrimitive::to_f64(&self.0).unwrap_or(0.0)
    }

    /// Get numerator
    pub fn numer(&self) -> i64 {
        *self.0.numer()
    }

    /// Get denominator
    pub fn denom(&self) -> i64 {
        *self.0.denom()
    }

    /// Square root (returns approximation)
    pub fn sqrt(&self) -> Self {
        Fraction::from_double(self.to_f64().sqrt())
    }

    /// Sine (returns approximation)
    pub fn sin(&self) -> Self {
        Fraction::from_double(self.to_f64().sin())
    }

    /// Cosine (returns approximation)
    pub fn cos(&self) -> Self {
        Fraction::from_double(self.to_f64().cos())
    }

    /// Hyperbolic sine (returns approximation)
    pub fn sinh(&self) -> Self {
        Fraction::from_double(self.to_f64().sinh())
    }

    /// Hyperbolic cosine (returns approximation)
    pub fn cosh(&self) -> Self {
        Fraction::from_double(self.to_f64().cosh())
    }

    /// Signum
    pub fn signum(&self) -> Self {
        if *self > Fraction::new(0, 1) {
            Fraction::new(1, 1)
        } else if *self < Fraction::new(0, 1) {
            Fraction::new(-1, 1)
        } else {
            Fraction::new(0, 1)
        }
    }

    /// Absolute value
    pub fn abs(&self) -> Self {
        if *self < Fraction::new(0, 1) {
            -*self
        } else {
            *self
        }
    }

    /// Check if this fraction is a perfect square (i.e., can be written as (a/b)² for integers a, b)
    pub fn is_perfect_square(&self) -> bool {
        if self.0.numer() < &0 {
            return false; // Negative numbers can't be perfect squares in real domain
        }
        let numer = self.0.numer().abs();
        let denom = self.0.denom().abs();

        let sqrt_numer = (numer as f64).sqrt().round() as i64;
        let sqrt_denom = (denom as f64).sqrt().round() as i64;

        sqrt_numer * sqrt_numer == numer && sqrt_denom * sqrt_denom == denom
    }

    /// Get the square root as a fraction if it's a perfect square, or approximate it
    pub fn sqrt_exact(&self) -> (bool, Self) {
        if self.is_perfect_square() {
            let numer = self.0.numer().abs();
            let denom = self.0.denom().abs();

            let sqrt_numer = (numer as f64).sqrt().round() as i64;
            let sqrt_denom = (denom as f64).sqrt().round() as i64;

            let result = Fraction::new(sqrt_numer, sqrt_denom);
            if *self < Fraction::new(0, 1) {
                (true, -result) // This case shouldn't occur if is_perfect_square returned true for negative number
            } else {
                (true, result)
            }
        } else {
            // Not a perfect square, return approximate value
            (false, self.sqrt())
        }
    }

    /// Check if this fraction is a perfect nth power (i.e., can be written as (a/b)^n for integers a, b)
    pub fn is_perfect_power(&self, n: u32) -> bool {
        if self.0.numer() < &0 && n % 2 == 0 {
            return false; // Even roots of negative numbers are not real
        }

        let numer = self.0.numer().abs();
        let denom = self.0.denom().abs();

        let n_root_numer = (numer as f64).powf(1.0 / n as f64).round() as i64;
        let n_root_denom = (denom as f64).powf(1.0 / n as f64).round() as i64;

        n_root_numer.pow(n) == numer && n_root_denom.pow(n) == denom
    }

    /// Get the nth root as a fraction if it's a perfect nth power, or approximate it
    pub fn nth_root_exact(&self, n: u32) -> (bool, Self) {
        if self.is_perfect_power(n) {
            let numer = self.0.numer().abs();
            let denom = self.0.denom().abs();

            let n_root_numer = (numer as f64).powf(1.0 / n as f64).round() as i64;
            let n_root_denom = (denom as f64).powf(1.0 / n as f64).round() as i64;

            let result = Fraction::new(n_root_numer, n_root_denom);
            if *self < Fraction::new(0, 1) && n % 2 == 1 {
                // Odd root of negative number is negative
                (true, -result)
            } else if *self < Fraction::new(0, 1) {
                // Even root of negative number is complex (return as positive for real part)
                (true, result)
            } else {
                (true, result)
            }
        } else {
            // Not a perfect nth power, return approximate value
            (
                false,
                Fraction::from_double(self.to_f64().powf(1.0 / n as f64)),
            )
        }
    }
}

// Implement arithmetic operators
impl std::ops::Add for Fraction {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Fraction(self.0 + other.0)
    }
}

impl std::ops::Sub for Fraction {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Fraction(self.0 - other.0)
    }
}

impl std::ops::Mul for Fraction {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Fraction(self.0 * other.0)
    }
}

impl std::ops::Div for Fraction {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Fraction(self.0 / other.0)
    }
}

impl std::ops::Neg for Fraction {
    type Output = Self;

    fn neg(self) -> Self {
        Fraction(-self.0)
    }
}

impl std::cmp::PartialEq<f64> for Fraction {
    fn eq(&self, other: &f64) -> bool {
        (self.to_f64() - other).abs() < 1e-10
    }
}

impl std::cmp::PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl std::fmt::Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_double_integer() {
        let frac = Fraction::from_double(5.0);
        assert_eq!(frac, Fraction::new(5, 1));
    }

    #[test]
    fn test_from_double_simple() {
        let frac = Fraction::from_double(0.5);
        assert_eq!(frac, Fraction::new(1, 2));
    }

    #[test]
    fn test_from_double_negative() {
        let frac = Fraction::from_double(-0.75);
        assert_eq!(frac, Fraction::new(-3, 4));
    }

    #[test]
    fn test_to_string_integer() {
        let frac = Fraction::new(5, 1);
        assert_eq!(frac.to_string(), "5");
    }

    #[test]
    fn test_to_string_fraction() {
        let frac = Fraction::new(3, 4);
        assert_eq!(frac.to_string(), "3/4");
    }

    #[test]
    fn test_to_string_negative() {
        let frac = Fraction::new(-3, 4);
        assert_eq!(frac.to_string(), "-3/4");
    }

    #[test]
    fn test_arithmetic() {
        let a = Fraction::new(1, 2);
        let b = Fraction::new(1, 3);

        assert_eq!(a + b, Fraction::new(5, 6));
        assert_eq!(a - b, Fraction::new(1, 6));
        assert_eq!(a * b, Fraction::new(1, 6));
        assert_eq!(a / b, Fraction::new(3, 2));
    }

    #[test]
    fn test_partial_ord() {
        let a = Fraction::new(1, 2);
        let b = Fraction::new(2, 3);
        let c = Fraction::new(3, 4);

        assert!(a < b);
        assert!(b < c);
        assert!(a < c);
        assert_eq!(a.partial_cmp(&a), Some(std::cmp::Ordering::Equal));

        // Test negative fractions
        let neg_a = Fraction::new(-1, 2);
        let neg_b = Fraction::new(-2, 3);
        assert!(neg_b < neg_a);
        assert!(neg_a < a);

        // Test same numerator different denominator
        let d1 = Fraction::new(1, 2);
        let d2 = Fraction::new(1, 4);
        assert!(d2 < d1);
    }
}
