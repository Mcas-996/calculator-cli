use calculator::core::ComplexNumber;
use calculator::core::fraction::Fraction;

fn main() {
    println!("Testing sqrt of -4:");
    let z = ComplexNumber::from_real(Fraction::new(-4, 1));
    println!("z = {:?}", z);
    
    let r = (z.real * z.real + z.imag * z.imag).sqrt();
    println!("r = |z| = sqrt((-4)² + 0²) = sqrt(16) = {:?}", r);
    
    let sqrt_r = r.sqrt();
    println!("sqrt(r) = sqrt(4) = {:?}", sqrt_r);
    
    println!("self.real >= 0: {}", z.real >= Fraction::new(0, 1));
    
    if z.real >= Fraction::new(0, 1) {
        let result = ComplexNumber::new(sqrt_r, Fraction::new(1, 2) * z.imag / sqrt_r);
        println!("Using positive real case: {:?}", result);
    } else {
        let result = ComplexNumber::new(
            z.imag.abs() / (Fraction::new(2, 1) * r.sqrt()),
            z.imag.signum() * sqrt_r,
        );
        println!("Using negative real case: {:?}", result);
    }
    
    // The correct formula should give us 2i or -2i
    println!("Expected: 2i or -2i");
    println!("2i = {:?}", ComplexNumber::new(Fraction::new(0, 1), Fraction::new(2, 1)));
}
