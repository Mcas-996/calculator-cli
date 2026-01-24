fn main() {
    println!("=== Analyzing 8119/10497 ===\n");

    let frac = 8119.0 / 10497.0;
    let sqrt2 = 2.0_f64.sqrt();

    println!("8119/10497 = {:.20}", frac);
    println!("sqrt(2)     = {:.20}", sqrt2);
    println!("Difference  = {:.20}", (frac - sqrt2).abs());
    println!();

    // Check what sqrt2 is as a fraction with this denominator
    println!("=== Checking if it's related to sqrt(2) ===");

    // For x^2 - 2 = 0, solutions should be ±sqrt(2)
    let a = 1.0;
    let b = 0.0;
    let c = -2.0;
    let discriminant = b * b - 4.0 * a * c;

    println!("For equation x^2 - 2 = 0:");
    println!("a = {}, b = {}, c = {}", a, b, c);
    println!("Discriminant = b^2 - 4ac = {}", discriminant);
    println!();

    let sqrt_disc = discriminant.sqrt();
    println!(
        "sqrt(discriminant) = sqrt({}) = {}",
        discriminant, sqrt_disc
    );
    println!("                   = {:.20}", sqrt_disc);
    println!();

    // Check common patterns
    let patterns = vec![
        ("sqrt(2)", sqrt2),
        ("sqrt(8)", sqrt_disc),
        ("sqrt(2)/2", sqrt2 / 2.0),
        ("2*sqrt(2)", 2.0 * sqrt2),
    ];

    for (name, value) in patterns {
        let diff = (frac - value).abs();
        println!("{} = {:.20}", name, value);
        println!("  vs 8119/10497 = {:.20}", frac);
        println!("  Difference = {:.20}", diff);
        if diff < 1e-3 {
            println!("  ✓ CLOSE MATCH!");
        }
        println!();
    }

    // Check if 8119/10497 is a root of something
    println!("=== Checking if 8119/10497 is a root ===");
    let x = frac;
    println!("If x = 8119/10497 = {:.20}", x);
    println!("Then x^2 = {:.20}", x * x);
    println!("So x^2 - 2 = {:.20}", x * x - 2.0);
    println!();

    if (x * x - 2.0).abs() < 1e-6 {
        println!("✓ This IS approximately sqrt(2)");
    } else {
        println!("✗ This is NOT sqrt(2)");
    }
}
