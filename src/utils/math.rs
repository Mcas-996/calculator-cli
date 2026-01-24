use crate::core::Fraction;

/// Calculate greatest common divisor
pub fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a.abs();
    let mut b = b.abs();

    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }

    a
}

/// Approximate a double value to a fraction
pub fn approx_double_to_fraction(value: f64, max_denominator: i64) -> Fraction {
    // Handle special cases
    if value == 0.0 {
        return Fraction::new(0, 1);
    }

    let mut x = value.abs();

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

    let mut result = Fraction::new(h, k);
    if value < 0.0 {
        result = -result;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(17, 5), 1);
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(-12, 8), 4);
    }

    #[test]
    fn test_approx_double_to_fraction() {
        assert_eq!(approx_double_to_fraction(0.5, 100), Fraction::new(1, 2));
        assert_eq!(approx_double_to_fraction(0.75, 100), Fraction::new(3, 4));
        assert_eq!(
            approx_double_to_fraction(0.333333, 1000),
            Fraction::new(1, 3)
        );
    }
}
