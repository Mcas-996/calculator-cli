#include <iostream>
#include "fractions.hpp"

int main() {
    Fraction frac = Fraction::fromDouble(-0.5);
    std::cout << "-0.5 = " << frac.numerator << "/" << frac.denominator << std::endl;
    std::cout << "-0.5 = " << frac.toString() << std::endl;
    return 0;
}
