#include "unicode_formatter.hpp"
#include <iostream>
#include <iomanip>

int main() {
    double value = 2.8284271247;
    std::cout << "Testing simplifyRadical for " << value << std::endl;
    std::cout << "Result: " << pretty::UnicodeFormatter::simplifyRadical(value) << std::endl;
    
    value = 1.4142135624;
    std::cout << "\nTesting simplifyRadical for " << value << std::endl;
    std::cout << "Result: " << pretty::UnicodeFormatter::simplifyRadical(value) << std::endl;
    
    value = 3.0;
    std::cout << "\nTesting simplifyRadical for " << value << std::endl;
    std::cout << "Result: " << pretty::UnicodeFormatter::simplifyRadical(value) << std::endl;
    
    return 0;
}
