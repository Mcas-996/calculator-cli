#ifndef STRING_PROCESSING_HPP
#define STRING_PROCESSING_HPP

#include <string>

namespace sp {
    // Function to evaluate middle school math expressions
    // Supports:
    // - Basic operations: +, -, *, /, ^ (exponents)
    // - Parentheses for grouping
    // - Negative numbers and decimals
    // - Percentages (e.g., "50%" converts to 0.5)
    // - sqrt() function for square roots
    // - Natural constants: pi (3.14159...) and e (2.71828...)
    // 
    // Examples:
    // "3 + 5 * (2 - 8)^2" => 153
    // "-2.5 * 4 + 3^2" => -1
    // "50% * 200" => 100
    // "3.14 * 2^2" => 12.56
    // "sqrt(16) + 3" => 7
    // "sqrt(2 + 2) * 5" => 10
    // "pi * 2" => 6.28319
    // "e^2" => 7.38906
    // "pi * r^2" (with r=3) => 28.2743
    // 
    // Throws std::runtime_error for invalid expressions, division by zero, or sqrt of negative number
    double evaluateExpression(const std::string& expression);
}

#endif // STRING_PROCESSING_HPP