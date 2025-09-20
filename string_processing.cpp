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

namespace sp {
    // Helper function to perform arithmetic operations for Fractions
    Fraction applyOp(Fraction a, Fraction b, char op) {
        switch(op) {
            case '+': return a + b;
            case '-': return a - b;
            case '*': return a * b;
            case '/': return a / b;
            case '^': 
                // Exponentiation for fractions, converting to double for now, then back to fraction.
                // For integer exponents with Fraction objects, a more robust implementation would involve
                // repeated multiplication or a custom power function without converting to double.
                // Assuming base is not zero for now during conversion.
                if (a.denominator == 0) throw std::runtime_error("Invalid base for exponentiation: 0/0");
                if (b.denominator == 0 && b.numerator != 0) throw std::runtime_error("Division by zero in exponent");
                return Fraction::fromDouble(std::pow(static_cast<double>(a.numerator) / a.denominator, 
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
                if (b == 0) throw std::runtime_error("Division by zero");
                return a / b;
            case '^': return std::pow(a, b);
        }
        return 0; // Should not reach here
    }

    // Helper function to handle unary operations (like square root)
    // Still uses double for internal sqrt calculation, then converts to Fraction if needed
    double applyUnaryOp(double a, const std::string& op) {
        if(op == "sqrt") {
            if(a < 0) throw std::runtime_error("Square root of negative number");
            return std::sqrt(a);
        } else if(op == "abs") {
            return std::abs(a);
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

    // Helper function to parse numbers (including decimals and negative numbers)
    // Now returns a Fraction
    Fraction parseNumber(const std::string& expression, size_t& i) {
        double result = 0; // Temporarily parse as double
        bool isNegative = false;
        double decimalMultiplier = 0.1;
        
        // Handle leading negative sign
        if (i < expression.length() && expression[i] == '-') {
            isNegative = true;
            i++;
        }
        
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
        return Fraction::fromDouble(isNegative ? -result : result);
    }

    // Function to evaluate middle school math expressions
    // Now returns a Fraction
    Fraction evaluateExpression(const std::string& expression) {
        std::stack<Fraction> values;  // Stack for Fraction numbers
        std::stack<char> ops;    // Stack for operators
        
        for(size_t i = 0; i < expression.length(); i++) {
            // Skip whitespace
            if(expression[i] == ' ') continue;
            
            // Handle percentages
            if(expression[i] == '%') {
                if(values.empty()) throw std::runtime_error("Invalid percentage syntax");
                Fraction val = values.top();
                values.pop();
                values.push(val * Fraction(1, 100)); // Divide by 100
                continue;
            }
            
            // Handle natural constants pi and e
            else if(i + 2 <= expression.length() && expression.substr(i, 2) == "pi") {
                values.push(Fraction::fromDouble(3.14159265359));
                i += 1; // Move past "pi"
                continue;
            }
            else if(expression[i] == 'e' && (i == 0 || !isalpha(expression[i-1]))) {
                // Check if it's 'e' constant (not part of another word)
                if(i + 1 >= expression.length() || !isalpha(expression[i+1])) {
                    values.push(Fraction::fromDouble(2.71828182846));
                    continue;
                }
            }
            // If number (including decimals and negatives), parse it
            else if(isdigit(expression[i]) || (expression[i] == '-' && (i == 0 || (!isdigit(expression[i-1]) && expression[i-1] != ')')))) { 
                // A negative sign directly before a number or at start of expression is a unary minus.
                // A negative sign after an operator or '(' is also unary minus.
                // A negative sign after a number or ')' is a binary minus. This condition handles unary.
                Fraction val = parseNumber(expression, i);
                values.push(val);
                continue; // parseNumber already advanced i
            }
            // Handle sqrt function
            else if(i + 4 <= expression.length() && expression.substr(i, 4) == "sqrt") {
                i += 3; // Move to end of "sqrt"
                
                // Skip whitespace after sqrt
                while(i + 1 < expression.length() && expression[i + 1] == ' ') i++;
                
                // Expect opening parenthesis
                if(i + 1 >= expression.length() || expression[i + 1] != '(') {
                    throw std::runtime_error("sqrt requires parentheses");
                }
                i++; // Move past opening parenthesis
                
                // Find matching closing parenthesis
                int parenCount = 1;
                size_t startExpr = i; // Adjusted to be the start of the actual expression inside parenthesis
                size_t endExpr = i;
                
                while(endExpr + 1 < expression.length() && parenCount > 0) {
                    endExpr++;
                    if(expression[endExpr] == '(') parenCount++;
                    else if(expression[endExpr] == ')') parenCount--;
                }
                
                if(parenCount != 0) throw std::runtime_error("Unmatched parentheses in sqrt");
                
                // Extract expression inside sqrt
                std::string sqrtExpr = expression.substr(startExpr, endExpr - startExpr);
                // Recursively call evaluateExpression which returns a Fraction, then convert to double for std::sqrt
                Fraction innerResult = evaluateExpression(sqrtExpr);
                double sqrtVal = static_cast<double>(innerResult.numerator) / innerResult.denominator;
                
                if(sqrtVal < 0) throw std::runtime_error("Square root of negative number");
                values.push(Fraction::fromDouble(std::sqrt(sqrtVal)));
                
                i = endExpr; // Move to closing parenthesis
            }
            // If opening parenthesis, push to ops stack
            else if(expression[i] == '(') {
                ops.push(expression[i]);
            }
            // If closing parenthesis, solve entire brace
            else if(expression[i] == ')') {
                while(!ops.empty() && ops.top() != '(') {
                    if (values.size() < 2) throw std::runtime_error("Too few operands for operator " + std::string(1, ops.top()));
                    Fraction val2 = values.top();
                    values.pop();
                    
                    Fraction val1 = values.top();
                    values.pop();
                    
                    char op = ops.top();
                    ops.pop();
                    
                    values.push(applyOp(val1, val2, op));
                }
                if(!ops.empty()) ops.pop(); // Pop opening parenthesis
                else throw std::runtime_error("Unmatched closing parenthesis");
            }
            // If operator, process according to precedence
            else if(expression[i] == '+' || expression[i] == '-' || expression[i] == '*' || expression[i] == '/' || expression[i] == '^') {
                while(!ops.empty() && precedence(ops.top()) >= precedence(expression[i])) {
                    if (values.size() < 2) throw std::runtime_error("Too few operands for operator " + std::string(1, ops.top()));
                    Fraction val2 = values.top();
                    values.pop();
                    
                    Fraction val1 = values.top();
                    values.pop();
                    
                    char op = ops.top();
                    ops.pop();
                    
                    values.push(applyOp(val1, val2, op));
                }
                ops.push(expression[i]);
            } else {
                throw std::runtime_error("Invalid character in expression: " + std::string(1, expression[i]));
            }
        }
        
        // Process remaining operators
        while(!ops.empty()) {
            if(ops.top() == '(') throw std::runtime_error("Unmatched opening parenthesis");
            if (values.size() < 2) throw std::runtime_error("Too few operands for operator " + std::string(1, ops.top()));
            Fraction val2 = values.top();
            values.pop();
            
            Fraction val1 = values.top();
            values.pop();
            
            char op = ops.top();
            ops.pop();
            
            values.push(applyOp(val1, val2, op));
        }
        
        if(values.empty()) throw std::runtime_error("Invalid expression");
        // Ensure only one value remains in the stack
        if (values.size() != 1) throw std::runtime_error("Invalid expression format resulting in multiple values");
        return values.top();
    }
    
    // Function to solve simple linear equations
    double solveEquation(const std::string& equation) {
        // Check if equation starts with "equation(" and ends with ")"
        if (equation.length() < 11 || equation.substr(0, 9) != "equation(") {
            throw std::runtime_error("Invalid equation format. Use: equation(x+1=0)");
        }
        
        size_t endPos = equation.find_last_of(')');
        if (endPos == std::string::npos || endPos != equation.length() - 1) {
            throw std::runtime_error("Invalid equation format. Use: equation(x+1=0)");
        }
        
        // Extract the equation content
        std::string eqContent = equation.substr(9, endPos - 9);
        
        // Find the equals sign
        size_t equalsPos = eqContent.find('=');
        if (equalsPos == std::string::npos) {
            throw std::runtime_error("Equation must contain '=' sign");
        }
        
        std::string leftSide = eqContent.substr(0, equalsPos);
        std::string rightSide = eqContent.substr(equalsPos + 1);
        
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
                size_t start = i;
                bool isNegative = false;
                bool hasDecimal = false;
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
                    hasDecimal = true;
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
                throw std::runtime_error("Invalid character in equation: " + std::string(1, leftSide[i]));
            }
        }
        
        // Process right side (treat as constant)
        if (!rightSide.empty()) {
            Fraction rightValueFrac = evaluateExpression(rightSide);
            double rightValue = static_cast<double>(rightValueFrac.numerator) / rightValueFrac.denominator;
            b -= rightValue; // Move to left side: ax + b - rightValue = 0
        }
        
        if (!hasX) {
            throw std::runtime_error("Equation must contain variable x");
        }
        
        if (a == 0) {
            if (b == 0) {
                throw std::runtime_error("Infinite solutions (equation is 0 = 0)");
            } else {
                throw std::runtime_error("No solution (equation is " + std::to_string(b) + " = 0)");
            }
        }
        
        return -b / a;
    }
    
    // Function to solve quadratic equations
    std::string solveQuadraticEquation(const std::string& equation) {
        // Check if equation starts with "equation(" and ends with ")"
        if (equation.length() < 11 || equation.substr(0, 9) != "equation(") {
            throw std::runtime_error("Invalid equation format. Use: equation(x^2+2x+1=0)");
        }
        
        size_t endPos = equation.find_last_of(')');
        if (endPos == std::string::npos || endPos != equation.length() - 1) {
            throw std::runtime_error("Invalid equation format. Use: equation(x^2+2x+1=0)");
        }
        
        // Extract the equation content
        std::string eqContent = equation.substr(9, endPos - 9);
        
        // Find the equals sign
        size_t equalsPos = eqContent.find('=');
        if (equalsPos == std::string::npos) {
            throw std::runtime_error("Equation must contain '=' sign");
        }
        
        std::string leftSide = eqContent.substr(0, equalsPos);
        std::string rightSide = eqContent.substr(equalsPos + 1);
        
        // Parse coefficients for quadratic equation ax^2 + bx + c = 0
        double a = 0, b = 0, c = 0;
        bool hasX2 = false;
        bool hasX = false;
        
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
                hasX = true;
                b += 1.0; // coefficient is 1 if just 'x'
                i++;
            }
            else if (i + 1 < leftSide.length() && leftSide.substr(i, 2) == "-x" && (i + 2 >= leftSide.length() || leftSide[i + 2] != '^')) {
                hasX = true;
                b += -1.0;
                i += 2;
            }
            else if (i + 1 < leftSide.length() && leftSide.substr(i, 2) == "+x" && (i + 2 >= leftSide.length() || leftSide[i + 2] != '^')) {
                hasX = true;
                b += 1.0;
                i += 2;
            }
            else if (isdigit(leftSide[i]) || leftSide[i] == '-' || leftSide[i] == '+') {
                // Parse number
                size_t start = i;
                bool isNegative = false;
                bool hasDecimal = false;
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
                    hasDecimal = true;
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
                    hasX = true;
                    b += isNegative ? -num : num;
                    i++; // Skip x
                } else {
                    c += isNegative ? -num : num;
                }
            }
            else {
                throw std::runtime_error("Invalid character in equation: " + std::string(1, leftSide[i]));
            }
        }
        
        // Process right side (treat as constant)
        if (!rightSide.empty()) {
            Fraction rightValueFrac = evaluateExpression(rightSide);
            double rightValue = static_cast<double>(rightValueFrac.numerator) / rightValueFrac.denominator;
            c -= rightValue; // Move to left side: ax^2 + bx + c - rightValue = 0
        }
        
        if (!hasX2) {
            throw std::runtime_error("Quadratic equation must contain x^2 term");
        }
        
        // Calculate discriminant
        double discriminant = b * b - 4 * a * c;
        
        if (discriminant < 0) {
            // Complex solutions
            double realPart = -b / (2 * a);
            double imaginaryPart = std::sqrt(-discriminant) / (2 * a);
            
            // Handle negative zero case
            if (std::abs(realPart) < 1e-10) {
                realPart = 0.0;
            }
            
            // Format the complex solutions
            std::string realStr = std::to_string(realPart);
            std::string imagStr = std::to_string(imaginaryPart);
            
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
            
            return "x1 = " + realStr + " + " + imagStr + "i, x2 = " + realStr + " - " + imagStr + "i";
        } else if (discriminant == 0) {
            // One real solution
            double solution = -b / (2 * a);
            
            // Format solution nicely
            std::string solutionStr = std::to_string(solution);
            solutionStr = solutionStr.substr(0, solutionStr.find_last_not_of('0') + 1);
            if (solutionStr.back() == '.') solutionStr.pop_back();
            
            return "x = " + solutionStr;
        } else {
            // Two real solutions
            double sqrt_discriminant = std::sqrt(discriminant);
            double solution1 = (-b - sqrt_discriminant) / (2 * a);
            double solution2 = (-b + sqrt_discriminant) / (2 * a);
            
            // Format solutions nicely
            std::string solution1Str = std::to_string(solution1);
            std::string solution2Str = std::to_string(solution2);
            
            solution1Str = solution1Str.substr(0, solution1Str.find_last_not_of('0') + 1);
            if (solution1Str.back() == '.') solution1Str.pop_back();
            
            solution2Str = solution2Str.substr(0, solution2Str.find_last_not_of('0') + 1);
            if (solution2Str.back() == '.') solution2Str.pop_back();
            
            return "x1 = " + solution1Str + ", x2 = " + solution2Str;
        }
    }
    
    // Function to solve systems of linear equations with multiple variables
    std::string solveLinearSystem(const std::string& input) {
        
        // Remove equation2() wrapper
        if (input.length() < 12 || input.substr(0, 10) != "equation2(" || input.back() != ')') {
            throw std::runtime_error("Invalid equation2 format. Use: equation2(x+y=5,x-y=1)");
        }
        
        std::string content = input.substr(10, input.length() - 11);
        if (content.empty()) {
            throw std::runtime_error("No equations provided");
        }
        
        // Split equations by comma
        std::vector<std::string> equations;
        size_t start = 0;
        size_t commaPos = content.find(',');
        
        while (commaPos != std::string::npos) {
            equations.push_back(content.substr(start, commaPos - start));
            start = commaPos + 1;
            commaPos = content.find(',', start);
        }
        equations.push_back(content.substr(start));
        
        
        if (equations.size() < 2) {
            throw std::runtime_error("System must contain at least 2 equations");
        }
        
        // Detect variables in the system
        std::set<char> variables;
        for (const auto& eq : equations) {
            for (char c : eq) {
                if (c >= 'x' && c <= 'z') {  // Support x, y, z
                    variables.insert(c);
                }
            }
        }
        
        if (variables.size() != equations.size()) {
            throw std::runtime_error("Number of variables must equal number of equations");
        }
        
        if (variables.size() > 3) {
            throw std::runtime_error("Currently supporting up to 3 variables (x, y, z)");
        }
        
        // Parse each equation
        std::vector<std::map<char, double>> coefficients;
        std::vector<double> constants;
        
        for (const auto& eq : equations) {
            size_t equalsPos = eq.find('=');
            if (equalsPos == std::string::npos) {
                throw std::runtime_error("Each equation must contain '=' sign");
            }
            
            std::string leftSide = eq.substr(0, equalsPos);
            std::string rightSide = eq.substr(equalsPos + 1);
            
            std::map<char, double> eqCoeff;
            double eqConst = 0;
            
            // Parse left side
            size_t i = 0;
            while (i < leftSide.length()) {
                if (leftSide[i] == ' ') {
                    i++;
                    continue;
                }
                
                bool isNegative = false;
                double coeff = 1.0;
                
                if (leftSide[i] == '-') {
                    isNegative = true;
                    i++;
                } else if (leftSide[i] == '+') {
                    i++;
                }
                
                // Parse coefficient number
                bool hasCoeff = false;
                if (i < leftSide.length() && isdigit(leftSide[i])) {
                    coeff = 0;
                    hasCoeff = true;
                    while (i < leftSide.length() && isdigit(leftSide[i])) {
                        coeff = coeff * 10 + (leftSide[i] - '0');
                        i++;
                    }
                    
                    // Parse decimal part
                    if (i < leftSide.length() && leftSide[i] == '.') {
                        i++;
                        double decimalMultiplier = 0.1;
                        while (i < leftSide.length() && isdigit(leftSide[i])) {
                            coeff += (leftSide[i] - '0') * decimalMultiplier;
                            decimalMultiplier *= 0.1;
                            i++;
                        }
                    }
                }
                
                // Check for variable
                if (i < leftSide.length() && variables.count(leftSide[i])) {
                    char var = leftSide[i];
                    if (!hasCoeff) coeff = 1.0;  // No coefficient means coefficient is 1
                    eqCoeff[var] += isNegative ? -coeff : coeff;
                    i++;
                } else if (hasCoeff || isNegative) {
                    // It's a constant term
                    eqConst -= isNegative ? -coeff : coeff;
                }
            }
            
            // Parse right side
            Fraction rightValueFrac = evaluateExpression(rightSide);
            double rightValue = static_cast<double>(rightValueFrac.numerator) / rightValueFrac.denominator;
            eqConst += rightValue;
            
            coefficients.push_back(eqCoeff);
            constants.push_back(eqConst);
        }
        
        // Build matrix for Gaussian elimination
        int n = variables.size();
        std::vector<std::vector<double>> matrix(n, std::vector<double>(n + 1));
        std::vector<char> varList(variables.begin(), variables.end());
        std::sort(varList.begin(), varList.end());  // Sort for consistent ordering
        
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
                if (std::abs(matrix[k][i]) > std::abs(matrix[maxRow][i])) {
                    maxRow = k;
                }
            }
            
            // Swap rows
            std::swap(matrix[i], matrix[maxRow]);
            
            // Check for singular matrix
            if (std::abs(matrix[i][i]) < 1e-10) {
                throw std::runtime_error("System has no unique solution (singular matrix)");
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
        std::vector<double> solutions(n);
        for (int i = n - 1; i >= 0; i--) {
            solutions[i] = matrix[i][n];
            for (int j = i + 1; j < n; j++) {
                solutions[i] -= matrix[i][j] * solutions[j];
            }
            solutions[i] /= matrix[i][i];
            
            // Handle negative zero
            if (std::abs(solutions[i]) < 1e-10) {
                solutions[i] = 0.0;
            }
        }
        
        // Format result with x1, y1, z1 naming convention
        std::string result;
        for (int i = 0; i < n; i++) {
            if (i > 0) result += ", ";
            
            std::string valueStr = std::to_string(solutions[i]);
            // Remove trailing zeros and decimal point if not needed
            valueStr = valueStr.substr(0, valueStr.find_last_not_of('0') + 1);
            if (valueStr.back() == '.') valueStr.pop_back();
            
            // Use x1, y1, z1 convention instead of x, y, z
            char varName = varList[i];
            result += std::string(1, varName) + "1=" + valueStr;
        }
        
        return result;
    }
    
    // Unified function to process any input string
    std::string processInput(const std::string& input) {
        try {
            // Check if it's a system of equations solving request
            if (input.length() >= 9 && input.substr(0, 9) == "equation2") {
                return solveLinearSystem(input);
            }
            
            // Check if it's an equation solving request
            if (input.length() >= 9 && input.substr(0, 9) == "equation(") {
                // Check if it's a quadratic equation (contains x^2)
                if (input.find("x^2") != std::string::npos) {
                    return solveQuadraticEquation(input);
                } else {
                    // Linear equation
                    double result = solveEquation(input);
                    
                    // Handle negative zero
                    if (std::abs(result) < 1e-10) {
                        result = 0.0;
                    }
                    
                    // Format result nicely (remove trailing zeros)
                    std::string resultStr = std::to_string(result);
                    resultStr = resultStr.substr(0, resultStr.find_last_not_of('0') + 1);
                    if (resultStr.back() == '.') resultStr.pop_back();
                    
                    return "x = " + resultStr;
                }
            } else {
                // Regular expression evaluation
                Fraction result = evaluateExpression(input);
                return result.toString();
            }
        } catch (const std::exception& e) {
            return "Error: " + std::string(e.what());
        }
    }
}