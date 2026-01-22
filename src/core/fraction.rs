use num_rational::Rational64;

/// Wrapper for Fraction using num-rational's Rational64
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
            (0.5, 1, 2), (0.25, 1, 4), (0.75, 3, 4),
            (0.125, 1, 8), (0.375, 3, 8), (0.625, 5, 8), (0.875, 7, 8),
            (0.3333333333333333, 1, 3), (0.6666666666666666, 2, 3),
            (0.2, 1, 5), (0.4, 2, 5), (0.6, 3, 5), (0.8, 4, 5),
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
        let mut x = abs_value;
        let max_denominator = 10000i64;
        
        // Continued fraction approximation
        let mut a = x.floor() as i64;
        let mut h1 = 1i64;
        let mut k2 = 1i64;
        let mut h = a;
        let mut k = 1i64;
        
        while x.fract() != 0.0 && k.abs() <= max_denominator {
            x = 1.0 / (x.fract());
            a = x.floor() as i64;
            let temp = h;
            h = a * h + h1;
            h1 = temp;
            let temp = k;
            k = a * k + k2;
            k2 = temp;
        }
        
        let mut result = Fraction(Rational64::new(h, k));
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
}