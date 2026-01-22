use calculator::core::ComplexNumber;
use calculator::core::fraction::Fraction;

fn main() {
    println!("Testing quadratic formula step by step:");
    
    // For x^2 + 1 = 0, we have a=1, b=0, c=1
    let a = ComplexNumber::from_real(Fraction::new(1, 1));
    let b = ComplexNumber::from_real(Fraction::new(0, 1));
    let c = ComplexNumber::from_real(Fraction::new(1, 1));
    
    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("c = {:?}", c);
    
    // Calculate discriminant: b² - 4ac
    let discriminant = b.clone() * b.clone()
        - ComplexNumber::from_real(Fraction::new(4, 1)) * a.clone() * c.clone();
    println!("discriminant = {:?}", discriminant);
    
    // Calculate sqrt of discriminant
    let sqrt_discriminant = discriminant.sqrt();
    println!("sqrt(discriminant) = {:?}", sqrt_discriminant);
    
    // Calculate the two solutions
    let x1 = (-b.clone() + sqrt_discriminant.clone())
        / (ComplexNumber::from_real(Fraction::new(2, 1)) * a.clone());
    let x2 = (-b.clone() - sqrt_discriminant)
        / (ComplexNumber::from_real(Fraction::new(2, 1)) * a.clone());
    
    println!("x1 = {:?}", x1);
    println!("x2 = {:?}", x2);
    println!("x1 == x2: {}", x1 == x2);
    
    // Expected: x1 = i, x2 = -i
    let expected_i = ComplexNumber::new(Fraction::new(0, 1), Fraction::new(1, 1));
    let expected_minus_i = ComplexNumber::new(Fraction::new(0, 1), Fraction::new(-1, 1));
    
    println!("expected i = {:?}", expected_i);
    println!("expected -i = {:?}", expected_minus_i);
}
