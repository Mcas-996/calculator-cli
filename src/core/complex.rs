use crate::core::fraction::Fraction;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Complex number with exact fraction support
#[derive(Debug, Clone, PartialEq)]
pub struct ComplexNumber {
    pub real: Fraction,
    pub imag: Fraction,
}

impl ComplexNumber {
    /// Create a new complex number
    pub fn new(real: Fraction, imag: Fraction) -> Self {
        Self { real, imag }
    }

    /// Create a real number (imaginary part is zero)
    pub fn from_real(value: Fraction) -> Self {
        Self::new(value, Fraction::new(0, 1))
    }

    /// Create from double value
    pub fn from_double(value: f64) -> Self {
        Self::from_real(Fraction::from_double(value))
    }

    /// Create from real and imaginary doubles
    pub fn from_doubles(real: f64, imag: f64) -> Self {
        Self::new(Fraction::from_double(real), Fraction::from_double(imag))
    }

    /// Check if approximately real (imaginary part is very small)
    pub fn is_approximately_real(&self) -> bool {
        self.imag == Fraction::new(0, 1)
    }

    /// Square root (principal value)
    pub fn sqrt(&self) -> Self {
        // Special case for pure real numbers
        if self.imag == Fraction::new(0, 1) {
            let real_value = self.real.to_f64();
            if real_value >= 0.0 {
                // For positive real numbers, return the square root as a pure real number
                return ComplexNumber::from_double(real_value.sqrt());
            } else {
                // For negative real numbers, return as pure imaginary
                let sqrt_abs = (-real_value).sqrt();
                return ComplexNumber::new(Fraction::new(0, 1), Fraction::from_double(sqrt_abs));
            }
        }

        // Standard complex square root formula for general case
        let magnitude_squared = self.real * self.real + self.imag * self.imag;
        let magnitude = magnitude_squared.sqrt();

        // For the special case of zero
        if magnitude == Fraction::new(0, 1) {
            return ComplexNumber::from_real(Fraction::new(0, 1));
        }

        let real_part = ((magnitude + self.real) / Fraction::new(2, 1)).sqrt();
        let imag_part = ((magnitude - self.real) / Fraction::new(2, 1)).sqrt();

        // The sign of the imaginary part follows the sign of the original imaginary part
        let imag_sign = if self.imag >= Fraction::new(0, 1) {
            Fraction::new(1, 1)
        } else {
            Fraction::new(-1, 1)
        };

        ComplexNumber::new(real_part, imag_sign * imag_part)
    }

    /// Power operation
    pub fn pow(&self, n: i32) -> Self {
        if n == 0 {
            return ComplexNumber::from_real(Fraction::new(1, 1));
        }
        if n < 0 {
            return self.pow(-n).inverse();
        }

        let mut result = ComplexNumber::from_real(Fraction::new(1, 1));
        let mut base = self.clone();
        let mut exp = n;

        while exp > 0 {
            if exp % 2 == 1 {
                result = &result * &base;
            }
            base = &base * &base;
            exp /= 2;
        }

        result
    }

    /// Inverse (1 / z)
    pub fn inverse(&self) -> Self {
        let denom = self.real * self.real + self.imag * self.imag;
        if denom == Fraction::new(0, 1) {
            panic!("Cannot invert zero");
        }
        ComplexNumber::new(self.real / denom, -self.imag / denom)
    }

    /// Sine (using Taylor series approximation)
    pub fn sin(&self) -> Self {
        // sin(z) = sin(a+bi) = sin(a)cosh(b) + i*cos(a)sinh(b)
        let a = self.real;
        let b = self.imag;

        let sin_a = a.sin();
        let cos_a = a.cos();
        let sinh_b = b.sinh();
        let cosh_b = b.cosh();

        ComplexNumber::new(sin_a * cosh_b, cos_a * sinh_b)
    }

    /// Cosine (using Taylor series approximation)
    pub fn cos(&self) -> Self {
        // cos(z) = cos(a+bi) = cos(a)cosh(b) - i*sin(a)sinh(b)
        let a = self.real;
        let b = self.imag;

        let sin_a = a.sin();
        let cos_a = a.cos();
        let sinh_b = b.sinh();
        let cosh_b = b.cosh();

        ComplexNumber::new(cos_a * cosh_b, -sin_a * sinh_b)
    }

    /// Convert to string representation
    pub fn to_string(&self) -> String {
        if self.imag == Fraction::new(0, 1) {
            self.real.to_string()
        } else if self.real == Fraction::new(0, 1) {
            if self.imag == Fraction::new(1, 1) {
                "i".to_string()
            } else if self.imag == Fraction::new(-1, 1) {
                "-i".to_string()
            } else {
                format!("{}i", self.imag)
            }
        } else {
            if self.imag == Fraction::new(1, 1) {
                format!("{} + i", self.real)
            } else if self.imag == Fraction::new(-1, 1) {
                format!("{} - i", self.real)
            } else if self.imag > Fraction::new(0, 1) {
                format!("{} + {}i", self.real, self.imag)
            } else {
                format!("{} - {}i", self.real, -self.imag)
            }
        }
    }
}

// Implement arithmetic operators
impl Add for ComplexNumber {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        ComplexNumber::new(self.real + other.real, self.imag + other.imag)
    }
}

impl Sub for ComplexNumber {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        ComplexNumber::new(self.real - other.real, self.imag - other.imag)
    }
}

impl Mul for ComplexNumber {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        ComplexNumber::new(
            self.real * other.real - self.imag * other.imag,
            self.real * other.imag + self.imag * other.real,
        )
    }
}

impl<'a, 'b> Mul<&'b ComplexNumber> for &'a ComplexNumber {
    type Output = ComplexNumber;

    fn mul(self, other: &'b ComplexNumber) -> Self::Output {
        ComplexNumber::new(
            self.real * other.real - self.imag * other.imag,
            self.real * other.imag + self.imag * other.real,
        )
    }
}

impl Div for ComplexNumber {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let denom = other.real * other.real + other.imag * other.imag;
        if denom == Fraction::new(0, 1) {
            panic!("Division by zero");
        }
        ComplexNumber::new(
            (self.real * other.real + self.imag * other.imag) / denom,
            (self.imag * other.real - self.real * other.imag) / denom,
        )
    }
}

impl Neg for ComplexNumber {
    type Output = Self;

    fn neg(self) -> Self {
        ComplexNumber::new(-self.real, -self.imag)
    }
}

impl<'a> Neg for &'a ComplexNumber {
    type Output = ComplexNumber;

    fn neg(self) -> Self::Output {
        ComplexNumber::new(-self.real, -self.imag)
    }
}

impl<'a, 'b> Sub<&'b ComplexNumber> for &'a ComplexNumber {
    type Output = ComplexNumber;

    fn sub(self, other: &'b ComplexNumber) -> Self::Output {
        ComplexNumber::new(self.real - other.real, self.imag - other.imag)
    }
}

impl<'a> Div<ComplexNumber> for &'a ComplexNumber {
    type Output = ComplexNumber;

    fn div(self, other: ComplexNumber) -> Self::Output {
        let denom = other.real * other.real + other.imag * other.imag;
        if denom == Fraction::new(0, 1) {
            panic!("Division by zero");
        }
        ComplexNumber::new(
            (self.real * other.real + self.imag * other.imag) / denom,
            (self.imag * other.real - self.real * other.imag) / denom,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_real() {
        let c = ComplexNumber::from_real(Fraction::new(5, 1));
        assert_eq!(c.real, Fraction::new(5, 1));
        assert_eq!(c.imag, Fraction::new(0, 1));
    }

    #[test]
    fn test_addition() {
        let c1 = ComplexNumber::from_real(Fraction::new(3, 1));
        let c2 = ComplexNumber::from_real(Fraction::new(2, 1));
        let result = c1 + c2;
        assert_eq!(result.real, Fraction::new(5, 1));
    }

    #[test]
    fn test_multiplication() {
        let c1 = ComplexNumber::new(Fraction::new(1, 1), Fraction::new(1, 1));
        let c2 = ComplexNumber::new(Fraction::new(1, 1), Fraction::new(-1, 1));
        let result = c1 * c2;
        assert_eq!(result.real, Fraction::new(2, 1));
        assert_eq!(result.imag, Fraction::new(0, 1));
    }

    #[test]
    fn test_inverse() {
        let c = ComplexNumber::new(Fraction::new(1, 1), Fraction::new(1, 1));
        let result = c.inverse();
        assert_eq!(result.real, Fraction::new(1, 2));
        assert_eq!(result.imag, Fraction::new(-1, 2));
    }

    #[test]
    fn test_pow() {
        let c = ComplexNumber::new(Fraction::new(2, 1), Fraction::new(0, 1));
        let result = c.pow(3);
        assert_eq!(result.real, Fraction::new(8, 1));
    }

    #[test]
    fn test_is_approximately_real() {
        let c1 = ComplexNumber::from_real(Fraction::new(5, 1));
        assert!(c1.is_approximately_real());

        let c2 = ComplexNumber::new(Fraction::new(5, 1), Fraction::new(1, 2));
        assert!(!c2.is_approximately_real());
    }

    #[test]
    fn test_sqrt() {
        // Test sqrt of positive real number
        let c1 = ComplexNumber::from_real(Fraction::new(4, 1));
        let result1 = c1.sqrt();
        assert!(result1.real > Fraction::new(1, 1) && result1.real < Fraction::new(3, 1));
        assert_eq!(result1.imag, Fraction::new(0, 1));

        // Test sqrt of i (0 + 1i) should be approximately (√2/2 + √2/2 i)
        let c2 = ComplexNumber::new(Fraction::new(0, 1), Fraction::new(1, 1));
        let result2 = c2.sqrt();
        let sqrt_half = Fraction::from_double((2.0_f64).sqrt() / 2.0);
        assert!((result2.real - sqrt_half).abs() < Fraction::from_double(0.01));
        assert!((result2.imag - sqrt_half).abs() < Fraction::from_double(0.01));

        // Test sqrt of -1 (should be i)
        let c3 = ComplexNumber::from_real(Fraction::new(-1, 1));
        let result3 = c3.sqrt();
        assert_eq!(result3.real, Fraction::new(0, 1));
        assert!(result3.imag > Fraction::new(0, 1));

        // Test sqrt of -i (0 - 1i)
        let c4 = ComplexNumber::new(Fraction::new(0, 1), Fraction::new(-1, 1));
        let result4 = c4.sqrt();
        assert!(result4.real > Fraction::new(0, 1));
        assert!(result4.imag < Fraction::new(0, 1));
    }
}
