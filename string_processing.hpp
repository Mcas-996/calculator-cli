#ifndef STRING_PROCESSING_HPP
#define STRING_PROCESSING_HPP

#include <string>
#include "fractions.hpp" // Include the Fraction class

namespace sp {
    // Function to evaluate middle school math expressions
    // Now returns a Fraction to support fractional results
    Fraction evaluateExpression(const std::string& expression);
    
    // Function to solve simple linear equations
    // Supports format: equation(x+1=0) or equation(2x-3=7)
    // Returns the solution for x
    // Throws std::runtime_error for invalid equations or non-linear equations
    double solveEquation(const std::string& equation);
    
    // Function to solve systems of linear equations with multiple variables
    // Supports format: equation2(x+y=5,x-y=1) for 2x2 system
// or equation2(x+y+z=6,x-y+z=2,x+y-z=0) for 3x3 system
    // Returns a string containing all solutions in format: "x = value1, y = value2, z = value3"
    // Supports variables: x, y, z, w, etc.
    // Throws std::runtime_error for invalid systems, singular matrices, or inconsistent systems
    std::string solveLinearSystem(const std::string& system);
    
    // Function to solve quadratic equations
    // Supports format: equation(x^2+2x+1=0) or equation(2x^2-3x+1=0)
    // Returns a string containing all solutions (real or complex)
    // For equations with two real solutions, returns format: "x1 = value1, x2 = value2"
    // For equations with one solution, returns format: "x = value"
    // For equations with complex solutions, returns format: "x1 = a+bi, x2 = a-bi"
    // Throws std::runtime_error for invalid equations
    std::string solveQuadraticEquation(const std::string& equation);
    
    // Unified function to process any input string
    // Automatically detects the type of input and calls appropriate function
    // Supports:
    // - Regular expressions: "3 + 5 * 2"
    // - Linear equations: "equation(x+1=0)"
    // - Quadratic equations: "equation(x^2+2x+1=0)"
    // Returns the result as a string that can be directly displayed
    std::string processInput(const std::string& input);
}

#endif // STRING_PROCESSING_HPP