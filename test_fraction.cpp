#include <iostream>
#include <cmath>
#include "fractions.hpp"

int main() {
    double value = 0.8660254038;
    double sqrt2 = std::sqrt(2.0);
    double coeff = value / sqrt2;
    
    std::cout << "value = " << value << std::endl;
    std::cout << "sqrt2 = " << sqrt2 << std::endl;
    std::cout << "coeff = " << coeff << std::endl;
    
    Fraction frac = Fraction::fromDouble(coeff);
    std::cout << "frac = " << frac.numerator << "/" << frac.denominator << std::endl;
    std::cout << "fracValue = " << (double)frac.numerator / frac.denominator << std::endl;
    
    return 0;
}
