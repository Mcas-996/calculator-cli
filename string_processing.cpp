#include "string_processing.hpp"
#include <stack>
#include <cctype>
#include <stdexcept>
#include <cmath>
#include <string>
#include <vector>
#include <map>
#include <set>
#include <algorithm>
#include <iostream>
#include <iomanip> // For std::fixed, std::setprecision

using std::string;
using std::vector;
using std::stack;
using std::map;
using std::set;
using std::runtime_error;
using std::to_string;
using std::pow;
using std::sqrt;
using std::abs;
using std::swap;

namespace sp {
    // Helper function to perform arithmetic operations for Fractions
    Fraction applyOp(Fraction a, Fraction b, char op) {
        switch(op) {
            case '+': return a + b;
            case '-': return a - b;
            case '*': return a * b;
            case '/': 
                if (b.numerator == 0) throw runtime_error("Division by zero");
                return a / b;
            case '^': 
                // Exponentiation for fractions, converting to double for now, then back to fraction.
                // For integer exponents with Fraction objects, a more robust implementation would involve
                // repeated multiplication or a custom power function without converting to double.
                // Assuming base is not zero for now during conversion.
                if (a.denominator == 0) throw runtime_error("Invalid base for exponentiation: 0/0");
                // Handle negative exponents for fractions
                if (b.numerator < 0) {
                    // (a/b)^-n = (b/a)^n
                    Fraction base = a;
                    int exponent = -b.numerator; // Assuming integer exponent for now
                    Fraction result(1);
                    for (int i = 0; i < exponent; ++i) {
                        result = result * base;
                    }
                    return result.inverse(); // (b/a)^n
                }
                return Fraction::fromDouble(pow(static_cast<double>(a.numerator) / a.denominator, 
                                                     static_cast<double>(b.numerator) / b.denominator));
        }
        return Fraction(0); // Should not reach here
    }

    // Original applyOp for double (used by equation solvers for double calculations)
    double applyOp(double a, double b, char op) {
        switch(op) {
            case '+': return a + b;
            case '-': return a - b;
            case '*': return a * b;
            case '/': 
                if (b == 0) throw runtime_error("Division by zero");
                return a / b;
            case '^': return pow(a, b);
        }
        return 0; // Should not reach here
    }

    // Helper function to handle unary operations (like square root)
    // Still uses double for internal sqrt calculation, then converts to Fraction if needed
    double applyUnaryOp(double a, const string& op) {
        if(op == "sqrt") {
            if(a < 0) throw runtime_error("Square root of negative number");
            return sqrt(a);
        } else if(op == "abs") {
            return abs(a);
        }
        return a;
    }

    // Helper function to determine operator precedence
    int precedence(char op) {
        if(op == '+' || op == '-') return 1;
        if(op == '*' || op == '/') return 2;
        if(op == '^') return 3;  // Exponents have highest precedence
        return 0;
    }

    // Helper function to parse numbers (including decimals)
    // Now returns a Fraction
    Fraction parseNumber(const string& expression, size_t& i) {
        double result = 0; // Temporarily parse as double
        double decimalMultiplier = 0.1;
        
        // Parse integer part
        while (i < expression.length() && isdigit(expression[i])) {
            result = result * 10 + (expression[i] - '0');
            i++;
        }
        
        // Parse decimal part
        if (i < expression.length() && expression[i] == '.') {
            i++;
            while (i < expression.length() && isdigit(expression[i])) {
                result += (expression[i] - '0') * decimalMultiplier;
                decimalMultiplier *= 0.1;
                i++;
            }
        }
        
        i--; // Adjust for extra increment in the main loop
        return Fraction::fromDouble(result);
    }

    // Function to evaluate middle school math expressions
    // Now returns a Fraction
    Fraction evaluateExpression(const string& expression) {
        stack<Fraction> values;  // Stack for Fraction numbers
        stack<char> ops;    // Stack for operators
        bool expecting_operand = true; // Flag to distinguish unary minus
        
        for(size_t i = 0; i < expression.length(); i++) {
            // Skip whitespace
            if(expression[i] == ' ') continue;
            
            // Handle percentages
            if(expression[i] == '%') {
                if(values.empty()) throw runtime_error("Invalid percentage syntax");
                Fraction val = values.top();
                values.pop();
                values.push(val * Fraction(1, 100)); // Divide by 100
                expecting_operand = false; // After a number/percentage, expect an operator
                continue;
            }
            
            // Handle natural constants pi and e
            else if(i + 2 <= expression.length() && expression.substr(i, 2) == "pi") {
                Fraction pi_val = Fraction::fromDouble(3.14159265359);
                values.push(pi_val);
                i += 1; // Move past "pi"
                expecting_operand = false; // After a constant, expect an operator
                continue;
            }
            else if(expression[i] == 'e' && (i == 0 || !isalpha(expression[i-1]))) {
                // Check if it's 'e' constant (not part of another word)
                if(i + 1 >= expression.length() || !isalpha(expression[i+1])) {
                    values.push(Fraction::fromDouble(2.71828182846));
                    expecting_operand = false; // After a constant, expect an operator
                    continue;
                }
            }
            // If number, parse it
            else if(isdigit(expression[i])) { 
                Fraction val = parseNumber(expression, i);
                values.push(val);
                expecting_operand = false; // After a number, expect an operator
                continue; // parseNumber already advanced i
            }
            // Handle unary minus (e.g., -5, -(2+3))
            else if (expression[i] == '-' && expecting_operand) {
                values.push(Fraction(0)); // Push a zero
                ops.push('-'); // Push the minus operator
                expecting_operand = false; // After pushing an operator, expect an operand
            }
            // Handle sqrt function
            else if(i + 4 <= expression.length() && expression.substr(i, 4) == "sqrt") {
                i += 3; // Move to end of "sqrt" (now 't')
                
                // Skip whitespace after sqrt
                while(i + 1 < expression.length() && expression[i + 1] == ' ') i++;
                
                // Expect opening parenthesis
                if(i + 1 >= expression.length() || expression[i + 1] != '(') {
                    throw runtime_error("sqrt requires parentheses");
                }
                // At this point, i points to 't' (or last char of sqrt). expression[i+1] is '('.
                // We need to move i to point to the char *after* '('.
                i += 2; // Move past '(' and then to the first char inside.
                        // Example: sqrt(16) -> i points to 't', i+1 is '('. So i+2 points to '1'.
                
                size_t startExpr = i; // Start of the actual expression inside parenthesis
                int parenCount = 1; // Count for the current parenthesis
                size_t endExpr = i; // Will find the matching closing parenthesis
                
                while(endExpr < expression.length() && parenCount > 0) { // Loop until matching ')' is found
                    endExpr++;
                    if(endExpr >= expression.length()) throw runtime_error("Unmatched parentheses in sqrt"); // Reached end of string
                    if(expression[endExpr] == '(') parenCount++;
                    else if(expression[endExpr] == ')') parenCount--;
                }
                
                // At this point, endExpr points to the matching ')'
                // The length of the substring is (endExpr - startExpr)
                string sqrtExpr = expression.substr(startExpr, endExpr - startExpr);
                
                // Recursively call evaluateExpression which returns a Fraction, then convert to double for std::sqrt
                Fraction innerResult = evaluateExpression(sqrtExpr);
                double sqrtVal = static_cast<double>(innerResult.numerator) / innerResult.denominator;
                
                if(sqrtVal < 0) throw runtime_error("Square root of negative number");
                values.push(Fraction::fromDouble(sqrt(sqrtVal)));
                
                i = endExpr; // Move 'i' to the matching ')'
                expecting_operand = false; // After a function call, expect an operator
            }
            // If opening parenthesis, push to ops stack
            else if(expression[i] == '(') {
                ops.push(expression[i]);
                expecting_operand = true; // After an opening parenthesis, expect an operand
            }
            // If closing parenthesis, solve entire brace
            else if(expression[i] == ')') {
                while(!ops.empty() && ops.top() != '(') {
                    if (values.size() < 2) throw runtime_error("Too few operands for operator " + string(1, ops.top()));
                    Fraction val2 = values.top();
                    values.pop();
                    
                    Fraction val1 = values.top();
                    values.pop();
                    
                    char op = ops.top();
                    ops.pop();
                    
                    values.push(applyOp(val1, val2, op));
                }
                if(!ops.empty()) ops.pop(); // Pop opening parenthesis
                else throw runtime_error("Unmatched closing parenthesis");
                expecting_operand = false; // After a closing parenthesis, expect an operator
            }
            // If operator, process according to precedence
            else if(expression[i] == '+' || expression[i] == '-' || expression[i] == '*' || expression[i] == '/' || expression[i] == '^') {
                while(!ops.empty() && precedence(ops.top()) >= precedence(expression[i])) {
                    if (values.size() < 2) throw runtime_error("Too few operands for operator " + string(1, ops.top()));
                    Fraction val2 = values.top();
                    values.pop();
                    
                    Fraction val1 = values.top();
                    values.pop();
                    
                    char op = ops.top();
                    ops.pop();
                    
                    values.push(applyOp(val1, val2, op));
                }
                ops.push(expression[i]);
                expecting_operand = true; // After an operator, expect an operand
            } else {
                throw runtime_error("Invalid character in expression: " + string(1, expression[i]));
            }
        }
        
        // Process remaining operators
        while(!ops.empty()) {
            if(ops.top() == '(') throw runtime_error("Unmatched opening parenthesis");
            if (values.size() < 2) throw runtime_error("Too few operands for operator " + string(1, ops.top()));
            Fraction val2 = values.top();
            values.pop();
            
            Fraction val1 = values.top();
            values.pop();
            
            char op = ops.top();
            ops.pop();
            
            values.push(applyOp(val1, val2, op));
        }
        
        if(values.empty()) throw runtime_error("Invalid expression");
        // Ensure only one value remains in the stack
        if (values.size() != 1) throw runtime_error("Invalid expression format resulting in multiple values");
        return values.top();
    }
    
    // Function to solve simple linear equations
    string solveEquation(const string& equation) {
        // Check if equation starts with "equation(" and ends with ")"
        if (equation.length() < 11 || equation.substr(0, 9) != "equation(") {
            throw runtime_error("Invalid equation format. Use: equation(x+1=0)");
        }
        
        size_t endPos = equation.find_last_of(')');
        if (endPos == string::npos || endPos != equation.length() - 1) {
            throw runtime_error("Invalid equation format. Use: equation(x+1=0)");
        }
        
        // Extract the equation content
        string eqContent = equation.substr(9, endPos - 9);
        
        // Find the equals sign
        size_t equalsPos = eqContent.find('=');
        if (equalsPos == string::npos) {
            throw runtime_error("Equation must contain '=' sign");
        }
        
        string leftSide = eqContent.substr(0, equalsPos);
        string rightSide = eqContent.substr(equalsPos + 1);
        
        // Parse coefficients for linear equation ax + b = 0
        double a = 0, b = 0;
        bool hasX = false;
        
        // Process left side
        size_t i = 0;
        while (i < leftSide.length()) {
            // Skip whitespace
            if (leftSide[i] == ' ') {
                i++;
                continue;
            }
            
            // Look for x
            if (leftSide[i] == 'x') {
                hasX = true;
                a += 1.0; // coefficient is 1 if just 'x'
                i++;
            }
            else if (i + 1 < leftSide.length() && leftSide.substr(i, 2) == "-x") {
                hasX = true;
                a += -1.0; // coefficient is -1 for '-x'
                i += 2;
            }
            else if (i + 1 < leftSide.length() && leftSide.substr(i, 2) == "+x") {
                hasX = true;
                a += 1.0; // coefficient is 1 for '+x'
                i += 2;
            }
            else if (isdigit(leftSide[i]) || leftSide[i] == '-' || leftSide[i] == '+') {
                // Parse number
                bool isNegative = false;
                double num = 0;
                double decimalMultiplier = 0.1;
                
                if (leftSide[i] == '-') {
                    isNegative = true;
                    i++;
                } else if (leftSide[i] == '+') {
                    i++;
                }
                
                // Parse integer part
                while (i < leftSide.length() && isdigit(leftSide[i])) {
                    num = num * 10 + (leftSide[i] - '0');
                    i++;
                }
                
                // Parse decimal part
                if (i < leftSide.length() && leftSide[i] == '.') {
                    i++;
                    while (i < leftSide.length() && isdigit(leftSide[i])) {
                        num += (leftSide[i] - '0') * decimalMultiplier;
                        decimalMultiplier *= 0.1;
                        i++;
                    }
                }
                
                // Check if this number is multiplied by x
                if (i < leftSide.length() && leftSide[i] == 'x') {
                    hasX = true;
                    a += isNegative ? -num : num;
                    i++; // Skip x
                } else {
                    b += isNegative ? -num : num;
                }
            }
            else {
                throw runtime_error("Invalid character in equation: " + string(1, leftSide[i]));
            }
        }
        
        // Process right side (treat as constant)
        if (!rightSide.empty()) {
            Fraction rightValueFrac = evaluateExpression(rightSide);
            double rightValue = static_cast<double>(rightValueFrac.numerator) / rightValueFrac.denominator;
            b -= rightValue; // Move to left side: ax + b - rightValue = 0
        }
        
        if (!hasX) {
            throw runtime_error("Equation must contain variable x");
        }
        
        if (a == 0) {
            if (b == 0) {
                throw runtime_error("Infinite solutions (equation is 0 = 0)");
            } else {
                throw runtime_error("No solution (equation is " + to_string(b) + " = 0)");
            }
        }
        
        double solution = -b / a;
        Fraction solution_frac = Fraction::fromDouble(solution);
        if (std::fabs(solution - (static_cast<double>(solution_frac.numerator) / solution_frac.denominator)) < 1e-9) {
            return "x = " + solution_frac.toString();
        } else {
            std::ostringstream oss;
            oss << std::fixed << std::setprecision(10) << solution;
            return "x = " + oss.str();
        }
    }
    
    // Function to solve quadratic equations
    string solveQuadraticEquation(const string& equation) {
        // Check if equation starts with "equation(" and ends with ")"
        if (equation.length() < 11 || equation.substr(0, 9) != "equation(") {
            throw runtime_error("Invalid equation format. Use: equation(x^2+2x+1=0)");
        }
        
        size_t endPos = equation.find_last_of(')');
        if (endPos == string::npos || endPos != equation.length() - 1) {
            throw runtime_error("Invalid equation format. Use: equation(x^2+2x+1=0)");
        }
        
        // Extract the equation content
        string eqContent = equation.substr(9, endPos - 9);
        
        // Find the equals sign
        size_t equalsPos = eqContent.find('=');
        if (equalsPos == string::npos) {
            throw runtime_error("Equation must contain '=' sign");
        }
        
        string leftSide = eqContent.substr(0, equalsPos);
        string rightSide = eqContent.substr(equalsPos + 1);
        
        // Parse coefficients for quadratic equation ax^2 + bx + c = 0
        double a = 0, b = 0, c = 0;
        bool hasX2 = false;
        
        // Process left side
        size_t i = 0;
        while (i < leftSide.length()) {
            // Skip whitespace
            if (leftSide[i] == ' ') {
                i++;
                continue;
            }
            
            // Look for x^2 terms
            if (leftSide[i] == 'x' && i + 1 < leftSide.length() && leftSide[i + 1] == '^' && i + 2 < leftSide.length() && leftSide[i + 2] == '2') {
                hasX2 = true;
                a += 1.0; // coefficient is 1 if just 'x^2'
                i += 3; // Skip x^2
            }
            else if (i + 3 < leftSide.length() && leftSide.substr(i, 3) == "x^2") {
                hasX2 = true;
                a += 1.0;
                i += 3;
            }
            else if (i + 4 < leftSide.length() && leftSide.substr(i, 4) == "-x^2") {
                hasX2 = true;
                a += -1.0;
                i += 4;
            }
            else if (i + 4 < leftSide.length() && leftSide.substr(i, 4) == "+x^2") {
                hasX2 = true;
                a += 1.0;
                i += 4;
            }
            // Look for x terms
            else if (leftSide[i] == 'x' && (i + 1 >= leftSide.length() || leftSide[i + 1] != '^')) {
                b += 1.0; // coefficient is 1 if just 'x'
                i++;
            }
            else if (i + 1 < leftSide.length() && leftSide.substr(i, 2) == "-x" && (i + 2 >= leftSide.length() || leftSide[i + 2] != '^')) {
                b += -1.0;
                i += 2;
            }
            else if (i + 1 < leftSide.length() && leftSide.substr(i, 2) == "+x" && (i + 2 >= leftSide.length() || leftSide[i + 2] != '^')) {
                b += 1.0;
                i += 2;
            }
            else if (isdigit(leftSide[i]) || leftSide[i] == '-' || leftSide[i] == '+') {
                // Parse number
                bool isNegative = false;
                double num = 0;
                double decimalMultiplier = 0.1;
                
                if (leftSide[i] == '-') {
                    isNegative = true;
                    i++;
                } else if (leftSide[i] == '+') {
                    i++;
                }
                
                // Parse integer part
                while (i < leftSide.length() && isdigit(leftSide[i])) {
                    num = num * 10 + (leftSide[i] - '0');
                    i++;
                }
                
                // Parse decimal part
                if (i < leftSide.length() && leftSide[i] == '.') {
                    i++;
                    while (i < leftSide.length() && isdigit(leftSide[i])) {
                        num += (leftSide[i] - '0') * decimalMultiplier;
                        decimalMultiplier *= 0.1;
                        i++;
                    }
                }
                
                // Check if this number is multiplied by x^2
                if (i + 2 < leftSide.length() && leftSide[i] == 'x' && leftSide[i + 1] == '^' && leftSide[i + 2] == '2') {
                    hasX2 = true;
                    a += isNegative ? -num : num;
                    i += 3; // Skip x^2
                }
                // Check if this number is multiplied by x
                else if (i < leftSide.length() && leftSide[i] == 'x' && (i + 1 >= leftSide.length() || leftSide[i + 1] != '^')) {
                    b += isNegative ? -num : num;
                    i++; // Skip x
                } else {
                    c += isNegative ? -num : num;
                }
            }
            else {
                throw runtime_error("Invalid character in equation: " + string(1, leftSide[i]));
            }
        }
        
        // Process right side (treat as constant)
        if (!rightSide.empty()) {
            Fraction rightValueFrac = evaluateExpression(rightSide);
            double rightValue = static_cast<double>(rightValueFrac.numerator) / rightValueFrac.denominator;
            c -= rightValue; // Move to left side: ax^2 + bx + c - rightValue = 0
        }
        
        if (!hasX2) {
            throw runtime_error("Quadratic equation must contain x^2 term");
        }
        
        // Calculate discriminant
        double discriminant = b * b - 4 * a * c;
        
        if (discriminant < 0) {
            // Complex solutions
            double realPart = -b / (2 * a);
            double imaginaryPart = sqrt(-discriminant) / (2 * a);
            
            // Handle negative zero
            if (abs(realPart) < 1e-10) {
                realPart = 0.0;
            }
            
            // Format the complex solutions
            std::ostringstream oss_real, oss_imag;
            oss_real << std::fixed << std::setprecision(10) << realPart;
            oss_imag << std::fixed << std::setprecision(10) << imaginaryPart;
            
            string realStr = oss_real.str();
            string imagStr = oss_imag.str();
            
            // Remove trailing zeros for cleaner output
            realStr = realStr.substr(0, realStr.find_last_not_of('0') + 1);
            if (realStr.back() == '.') realStr.pop_back();
            
            imagStr = imagStr.substr(0, imagStr.find_last_not_of('0') + 1);
            if (imagStr.back() == '.') imagStr.pop_back();
            
            // Special case: if imaginary part is 1 or -1, don't show the coefficient
            if (imagStr == "1" || imagStr == "1.0") {
                return "x1 = " + realStr + " + i, x2 = " + realStr + " - i";
            } else {
                return "x1 = " + realStr + " + " + imagStr + "i, x2 = " + realStr + " - " + imagStr + "i";
            }
        } else if (discriminant == 0) {
            // One real solution
            double solution = -b / (2 * a);
            Fraction solution_frac = Fraction::fromDouble(solution);
            if (std::fabs(solution - (static_cast<double>(solution_frac.numerator) / solution_frac.denominator)) < 1e-9) {
                return "x = " + solution_frac.toString();
            } else {
                std::ostringstream oss;
                oss << std::fixed << std::setprecision(10) << solution;
                return "x = " + oss.str();
            }
        } else {
            // Two real solutions
            double sqrt_discriminant = sqrt(discriminant);
            double solution1 = (-b - sqrt_discriminant) / (2 * a);
            double solution2 = (-b + sqrt_discriminant) / (2 * a);
            
            Fraction solution1_frac = Fraction::fromDouble(solution1);
            Fraction solution2_frac = Fraction::fromDouble(solution2);

            bool sol1_is_rational = std::fabs(solution1 - (static_cast<double>(solution1_frac.numerator) / solution1_frac.denominator)) < 1e-9;
            bool sol2_is_rational = std::fabs(solution2 - (static_cast<double>(solution2_frac.numerator) / solution2_frac.denominator)) < 1e-9;

            if (sol1_is_rational && sol2_is_rational) {
                return "x1 = " + solution1_frac.toString() + ", x2 = " + solution2_frac.toString();
            } else {
                std::ostringstream oss1, oss2;
                oss1 << std::fixed << std::setprecision(10) << solution1;
                oss2 << std::fixed << std::setprecision(10) << solution2;
                
                string solution1Str = oss1.str();
                string solution2Str = oss2.str();
                
                // Remove trailing zeros and decimal point if not needed
                solution1Str = solution1Str.substr(0, solution1Str.find_last_not_of('0') + 1);
                if (solution1Str.back() == '.') solution1Str.pop_back();
                
                solution2Str = solution2Str.substr(0, solution2Str.find_last_not_of('0') + 1);
                if (solution2Str.back() == '.') solution2Str.pop_back();
                
                return "x1 = " + solution1Str + ", x2 = " + solution2Str;
            }
        }
    }
    
    // Function to solve systems of linear equations with multiple variables
    string solveLinearSystem(const string& input) {
        
        // Remove equation2() wrapper
        if (input.length() < 12 || input.substr(0, 10) != "equation2(" || input.back() != ')') {
            throw runtime_error("Invalid equation2 format. Use: equation2(x+y=5,x-y=1)");
        }
        
        string content = input.substr(10, input.length() - 11);
        if (content.empty()) {
            throw runtime_error("No equations provided");
        }
        
        // Split equations by comma
        vector<string> equations;
        size_t start = 0;
        size_t commaPos = content.find(',');
        
        while (commaPos != string::npos) {
            equations.push_back(content.substr(start, commaPos - start));
            start = commaPos + 1;
            commaPos = content.find(',', start);
        }
        equations.push_back(content.substr(start));
        
        
        if (equations.size() < 2) {
            throw runtime_error("System must contain at least 2 equations");
        }
        
        // Detect variables in the system
        set<char> variables;
        for (const auto& eq : equations) {
            for (char c : eq) {
                if (c >= 'x' && c <= 'z') {  // Support x, y, z
                    variables.insert(c);
                }
            }
        }
        
        if (variables.size() > 3) {
            throw runtime_error("Currently supporting up to 3 variables (x, y, z)");
        }
        
        // Parse each equation
        vector<map<char, double>> coefficients;
        vector<double> constants;
        
        for (const auto& eq : equations) {
            size_t equalsPos = eq.find('=');
            if (equalsPos == string::npos) {
                throw runtime_error("Each equation must contain '=' sign");
            }
            
            string leftSide = eq.substr(0, equalsPos);
            string rightSide = eq.substr(equalsPos + 1);
            
            map<char, double> eqCoeff;
            double eqConst = 0;

            // --- Start of Refactored Parser ---
            size_t i = 0;
            while (i < leftSide.length()) {
                // 1. Skip leading whitespace for the term
                while (i < leftSide.length() && leftSide[i] == ' ') { i++; }
                if (i == leftSide.length()) break;

                // 2. Determine the sign of the term
                bool isNegative = false;
                if (leftSide[i] == '-') {
                    isNegative = true;
                    i++;
                } else if (leftSide[i] == '+') {
                    i++;
                }

                // 3. Parse the coefficient (number part)
                double coeff = 1.0;
                if (i < leftSide.length() && isdigit(leftSide[i])) {
                    string numStr;
                    while (i < leftSide.length() && (isdigit(leftSide[i]) || leftSide[i] == '.')) {
                        numStr += leftSide[i];
                        i++;
                    }
                    try {
                        coeff = std::stod(numStr);
                    } catch (const std::invalid_argument&) {
                        throw runtime_error("Invalid number format in equation: " + numStr);
                    }
                }

                // 4. Parse the variable part
                while (i < leftSide.length() && leftSide[i] == ' ') { i++; } // Skip space between coeff and var
                if (i < leftSide.length() && variables.count(leftSide[i])) {
                    char var = leftSide[i];
                    eqCoeff[var] += isNegative ? -coeff : coeff;
                    i++;
                } else { // If there's no variable, it was a constant on the left side
                    eqConst -= isNegative ? -coeff : coeff;
                }
            }
            // --- End of Refactored Parser ---
            
            // Parse right side
            Fraction rightValueFrac = evaluateExpression(rightSide);
            double rightValue = static_cast<double>(rightValueFrac.numerator) / rightValueFrac.denominator;
            eqConst += rightValue;
            
            coefficients.push_back(eqCoeff);
            constants.push_back(eqConst);
        }
        
        // Build matrix for Gaussian elimination
        int n = variables.size();
        vector<vector<double>> matrix(n, vector<double>(n + 1));
        vector<char> varList(variables.begin(), variables.end());
        sort(varList.begin(), varList.end());  // Sort for consistent ordering
        
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                matrix[i][j] = coefficients[i][varList[j]];
            }
            matrix[i][n] = constants[i];
        }
        
        // Gaussian elimination
        for (int i = 0; i < n; i++) {
            // Find pivot
            int maxRow = i;
            for (int k = i + 1; k < n; k++) {
                if (abs(matrix[k][i]) > abs(matrix[maxRow][i])) {
                    maxRow = k;
                }
            }
            
            // Swap rows
            swap(matrix[i], matrix[maxRow]);
            
            // Check for singular matrix
            if (abs(matrix[i][i]) < 1e-10) {
                throw runtime_error("System has no unique solution (singular matrix)");
            }
            
            // Eliminate column
            for (int k = i + 1; k < n; k++) {
                double factor = matrix[k][i] / matrix[i][i];
                for (int j = i; j <= n; j++) {
                    matrix[k][j] -= factor * matrix[i][j];
                }
            }
        }
        
        // Back substitution
        vector<double> solutions(n);
        bool all_rational = true;
        for (int i = n - 1; i >= 0; i--) {
            solutions[i] = matrix[i][n];
            for (int j = i + 1; j < n; j++) {
                solutions[i] -= matrix[i][j] * solutions[j];
            }
            solutions[i] /= matrix[i][i];
            
            // Handle negative zero
            if (abs(solutions[i]) < 1e-10) {
                solutions[i] = 0.0;
            }

            Fraction sol_frac = Fraction::fromDouble(solutions[i]);
            if (std::fabs(solutions[i] - (static_cast<double>(sol_frac.numerator) / sol_frac.denominator)) >= 1e-9) {
                all_rational = false;
            }
        }
        
        // Format result
        string result;
        for (int i = 0; i < n; i++) {
            if (i > 0) result += ", ";
            
            string valueStr;
            if (all_rational) {
                Fraction sol_frac = Fraction::fromDouble(solutions[i]);
                valueStr = sol_frac.toString();
            } else {
                std::ostringstream oss;
                oss << std::fixed << std::setprecision(10) << solutions[i];
                valueStr = oss.str();
                // Remove trailing zeros and decimal point if not needed
                valueStr = valueStr.substr(0, valueStr.find_last_not_of('0') + 1);
                if (valueStr.back() == '.') valueStr.pop_back();
            }
            
            char varName = varList[i];
            result += string(1, varName) + " = " + valueStr;
        }
        
        return result;
    }
    
    // Unified function to process any input string
    string processInput(const string& input) {
        try {
            // Check if it's a system of equations solving request
            if (input.length() >= 9 && input.substr(0, 9) == "equation2") {
                return solveLinearSystem(input);
            }
            
            // Check if it's an equation solving request
            if (input.length() >= 9 && input.substr(0, 9) == "equation(") {
                // Check if it's a quadratic equation (contains x^2)
                if (input.find("x^2") != string::npos) {
                    return solveQuadraticEquation(input);
                } else {
                    // Linear equation
                    return solveEquation(input);
                }
            }
            else {
                // Regular expression evaluation
                Fraction result = evaluateExpression(input);
                return result.toString();
            }
        } catch (const std::exception& e) {
            return "Error: " + string(e.what());
        }
    }
}