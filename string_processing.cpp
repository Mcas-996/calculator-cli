#include "string_processing.hpp"
#include "symbolic_solver.hpp"
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
#include <numeric> // For std::gcd
#include <numbers>
#include <functional>
#include <complex>

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
    // Helper function to perform arithmetic operations for ComplexNumbers
    ComplexNumber applyOp(const ComplexNumber& a, const ComplexNumber& b, char op) {
        switch(op) {
            case '+': return a + b;
            case '-': return a - b;
            case '*': return a * b;
            case '/': 
                return a / b;
            case '^': 
                return a.pow(b);
        }
        return ComplexNumber();
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

    // Helper function to determine operator precedence
    int precedence(char op) {
        if(op == '+' || op == '-') return 1;
        if(op == '*' || op == '/') return 2;
        if(op == '^') return 3;  // Exponents have highest precedence
        return 0;
    }

    // Helper function to parse numbers (including decimals and imaginary suffix)
    ComplexNumber parseNumber(const string& expression, size_t& i) {
        double result = 0;
        double decimalMultiplier = 0.1;
        
        while (i < expression.length() && isdigit(expression[i])) {
            result = result * 10 + (expression[i] - '0');
            i++;
        }
        
        if (i < expression.length() && expression[i] == '.') {
            i++;
            while (i < expression.length() && isdigit(expression[i])) {
                result += (expression[i] - '0') * decimalMultiplier;
                decimalMultiplier *= 0.1;
                i++;
            }
        }

        bool isImaginary = false;
        if (i < expression.length() && expression[i] == 'i') {
            isImaginary = true;
        } else {
            i--;
        }

        if (isImaginary) {
            return ComplexNumber(0.0, result);
        }
        return ComplexNumber(result, 0.0);
    }

double extractRealComponent(const ComplexNumber& value, const string& errorMessage) {
    if (!value.isApproximatelyReal()) {
        throw runtime_error(errorMessage);
    }
    return value.real;
}

ComplexNumber degreesToRadians(const ComplexNumber& degrees) {
    static const double RAD_PER_DEG = std::numbers::pi / 180.0;
    return degrees * ComplexNumber(RAD_PER_DEG, 0.0);
}

    namespace {
        constexpr double POLY_EPSILON = 1e-12;
        constexpr int MAX_DK_ITERATIONS = 200;
        constexpr double DK_TOLERANCE = 1e-12;

        std::string formatCoefficient(double value, bool allowOmitOne = false) {
            double roundedInt = std::round(value);
            if (std::fabs(value - roundedInt) < 1e-9) {
                long long intValue = static_cast<long long>(roundedInt);
                if (allowOmitOne && std::abs(intValue) == 1) {
                    return intValue < 0 ? "-" : "";
                }
                return std::to_string(intValue);
            }

            Fraction frac = Fraction::fromDouble(value);
            double fracValue = static_cast<double>(frac.numerator) / frac.denominator;
            if (std::fabs(value - fracValue) < 1e-9) {
                if (allowOmitOne && std::abs(frac.numerator) == frac.denominator) {
                    return (frac.numerator < 0) ? "-" : "";
                }
                if (frac.denominator == 1) {
                    return std::to_string(frac.numerator);
                }
                return std::to_string(frac.numerator) + "/" + std::to_string(frac.denominator);
            }

            std::ostringstream oss;
            oss << std::fixed << std::setprecision(10) << value;
            std::string s = oss.str();
            while (!s.empty() && s.back() == '0') s.pop_back();
            if (!s.empty() && s.back() == '.') s.pop_back();
            return s.empty() ? "0" : s;
        }

        std::string polynomialToString(const std::vector<double>& coeffs) {
            std::string result;
            bool firstTerm = true;
            for (int power = static_cast<int>(coeffs.size()) - 1; power >= 0; --power) {
                double coeff = coeffs[static_cast<size_t>(power)];
                if (std::fabs(coeff) < POLY_EPSILON) continue;

                bool isNegative = coeff < 0;
                double magnitude = isNegative ? -coeff : coeff;
                std::string term;

                if (power == 0) {
                    term = formatCoefficient(magnitude);
                } else {
                    std::string coeffStr = formatCoefficient(magnitude, true);
                    if (coeffStr.empty()) {
                        term = "x";
                    } else {
                        term = coeffStr + "x";
                    }
                    if (power > 1) {
                        term += "^" + std::to_string(power);
                    }
                }

                if (firstTerm) {
                    result += (isNegative ? "-" : "") + term;
                    firstTerm = false;
                } else {
                    result += isNegative ? " - " : " + ";
                    result += term;
                }
            }
            if (result.empty()) {
                result = "0";
            }
            return result;
        }

        std::string formatRootOfSolutions(const std::string& polynomialExpr, size_t degree) {
            std::string output;
            for (size_t idx = 0; idx < degree; ++idx) {
                if (idx > 0) output += ", ";
                output += "x" + std::to_string(static_cast<int>(idx + 1)) +
                          " = RootOf(" + polynomialExpr + ", " + std::to_string(idx) + ")";
            }
            return output;
        }

        std::vector<double> normalizeCoefficients(const std::vector<double>& coeffs) {
            std::vector<double> normalized = coeffs;
            double leading = normalized.back();
            for (double& c : normalized) {
                c /= leading;
            }
            return normalized;
        }

        std::complex<double> evaluatePolynomial(const std::vector<double>& coeffs, const std::complex<double>& x) {
            std::complex<double> result = coeffs.back();
            for (int i = static_cast<int>(coeffs.size()) - 2; i >= 0; --i) {
                result = result * x + coeffs[static_cast<size_t>(i)];
            }
            return result;
        }

        std::vector<std::complex<double>> durandKerner(const std::vector<double>& coeffs) {
            size_t degree = coeffs.size() - 1;
            if (degree == 0) {
                throw runtime_error("Polynomial degree must be at least 1");
            }

            auto monic = normalizeCoefficients(coeffs);

            double radius = 1.0;
            for (size_t i = 0; i < degree; ++i) {
                radius = std::max(radius, 1.0 + std::abs(monic[i]));
            }

            std::vector<std::complex<double>> roots(degree);
            const double angleStep = 2.0 * std::numbers::pi / static_cast<double>(degree);
            for (size_t i = 0; i < degree; ++i) {
                double angle = angleStep * static_cast<double>(i);
                roots[i] = std::polar(radius, angle);
                roots[i] += std::complex<double>(0.001 * static_cast<double>(i), -0.001 * static_cast<double>(degree - i));
            }

            for (int iter = 0; iter < MAX_DK_ITERATIONS; ++iter) {
                double maxStep = 0.0;
                for (size_t i = 0; i < degree; ++i) {
                    std::complex<double> denom(1.0, 0.0);
                    for (size_t j = 0; j < degree; ++j) {
                        if (i == j) continue;
                        auto diff = roots[i] - roots[j];
                        if (std::abs(diff) < 1e-15) {
                            diff += std::complex<double>(1e-12, 1e-12);
                        }
                        denom *= diff;
                    }
                    auto delta = evaluatePolynomial(monic, roots[i]) / denom;
                    roots[i] -= delta;
                    maxStep = std::max(maxStep, std::abs(delta));
                }
                if (maxStep < DK_TOLERANCE) {
                    break;
                }
            }
            return roots;
        }

        std::string formatNumericRoots(const std::vector<std::complex<double>>& roots) {
            string output;
            for (size_t idx = 0; idx < roots.size(); ++idx) {
                if (idx > 0) output += ", ";
                ComplexNumber cn(roots[idx].real(), roots[idx].imag());
                output += "x" + to_string(static_cast<int>(idx + 1)) + " = " + cn.toString();
            }
            return output;
        }

        std::vector<double> parsePolynomialLeftSide(const string& leftSide, int maxDegree, bool& hasLeadingTerm) {
            std::vector<double> coefficients(static_cast<size_t>(maxDegree + 1), 0.0);
            size_t i = 0;

            while (i < leftSide.length()) {
                if (leftSide[i] == ' ') {
                    ++i;
                    continue;
                }

                double sign = 1.0;
                if (leftSide[i] == '+') {
                    ++i;
                } else if (leftSide[i] == '-') {
                    sign = -1.0;
                    ++i;
                }

                while (i < leftSide.length() && leftSide[i] == ' ') {
                    ++i;
                }

                size_t termStart = i;
                bool hasDigits = false;
                double value = 0.0;

                while (i < leftSide.length() && isdigit(leftSide[i])) {
                    hasDigits = true;
                    value = value * 10 + (leftSide[i] - '0');
                    ++i;
                }

                if (i < leftSide.length() && leftSide[i] == '.') {
                    ++i;
                    double decimalMultiplier = 0.1;
                    while (i < leftSide.length() && isdigit(leftSide[i])) {
                        hasDigits = true;
                        value += (leftSide[i] - '0') * decimalMultiplier;
                        decimalMultiplier *= 0.1;
                        ++i;
                    }
                }

                while (i < leftSide.length() && leftSide[i] == ' ') {
                    ++i;
                }

                bool hasVariable = false;
                int exponent = 0;
                if (i < leftSide.length() && leftSide[i] == 'x') {
                    hasVariable = true;
                    exponent = 1;
                    ++i;
                    if (i < leftSide.length() && leftSide[i] == '^') {
                        ++i;
                        if (i >= leftSide.length() || !isdigit(leftSide[i])) {
                            throw runtime_error("Invalid character in equation: ^");
                        }
                        exponent = 0;
                        while (i < leftSide.length() && isdigit(leftSide[i])) {
                            exponent = exponent * 10 + (leftSide[i] - '0');
                            ++i;
                        }
                    }
                }

                if (!hasDigits && !hasVariable) {
                    if (termStart < leftSide.length()) {
                        throw runtime_error("Invalid character in equation: " + string(1, leftSide[termStart]));
                    }
                    throw runtime_error("Invalid equation format.");
                }

                if (!hasDigits) {
                    value = 1.0;
                }

                if (!hasVariable) {
                    exponent = 0;
                }

                if (exponent > maxDegree) {
                    throw runtime_error("Polynomial degree exceeds supported maximum x^" + to_string(maxDegree));
                }

                coefficients[static_cast<size_t>(exponent)] += sign * value;
                if (exponent == maxDegree && std::fabs(coefficients[static_cast<size_t>(exponent)]) > POLY_EPSILON) {
                    hasLeadingTerm = true;
                }
            }

            return coefficients;
        }

        std::vector<double> parsePolynomialEquationCoefficients(const string& equation,
                                                                int degree,
                                                                const string& formatHint,
                                                                const string& rhsRealMessage,
                                                                const string& missingLeadingMessage) {
            if (equation.length() < 11 || equation.substr(0, 9) != "equation(") {
                throw runtime_error("Invalid equation format. Use: " + formatHint);
            }

            size_t endPos = equation.find_last_of(')');
            if (endPos == string::npos || endPos != equation.length() - 1) {
                throw runtime_error("Invalid equation format. Use: " + formatHint);
            }

            string eqContent = equation.substr(9, endPos - 9);
            size_t equalsPos = eqContent.find('=');
            if (equalsPos == string::npos) {
                throw runtime_error("Equation must contain '=' sign");
            }

            string leftSide = eqContent.substr(0, equalsPos);
            string rightSide = eqContent.substr(equalsPos + 1);

            bool hasLeading = false;
            auto coefficients = parsePolynomialLeftSide(leftSide, degree, hasLeading);

            if (!rightSide.empty()) {
                ComplexNumber rightValue = evaluateExpression(rightSide);
                double realValue = extractRealComponent(rightValue, rhsRealMessage);
                coefficients[0] -= realValue;
            }

            if (!hasLeading) {
                throw runtime_error(missingLeadingMessage);
            }

            return coefficients;
        }

        std::string formatSymbolicOutput(const std::vector<std::string>& roots) {
            if (roots.empty()) {
                return "No solution";
            }
            if (roots.size() == 1) {
                return "x = " + roots.front();
            }

            std::string result;
            for (size_t i = 0; i < roots.size(); ++i) {
                if (i > 0) {
                    result += ", ";
                }
                result += "x" + std::to_string(i + 1) + " = " + roots[i];
            }
            return result;
        }
    } // namespace

    // Function to evaluate middle school math expressions
    // Now returns a ComplexNumber
    ComplexNumber evaluateExpression(const string& expression) {
        stack<ComplexNumber> values;  // Stack for numbers
        stack<char> ops;    // Stack for operators
        bool expecting_operand = true; // Flag to distinguish unary minus
        
        auto handleFunctionCall = [&](const string& funcName, size_t& index, size_t funcLength,
                                      const std::function<ComplexNumber(const ComplexNumber&)>& funcEvaluator) {
            size_t len = expression.length();
            index += funcLength - 1; // move to end of function name
            while (index + 1 < len && expression[index + 1] == ' ') index++;
            if (index + 1 >= len || expression[index + 1] != '(') {
                throw runtime_error(funcName + " requires parentheses");
            }
            index += 2; // move past '(' to first char of inner expression
            size_t startExpr = index;
            int parenCount = 1;
            size_t endExpr = index;
            
            while(endExpr < len && parenCount > 0) {
                endExpr++;
                if(endExpr >= len) throw runtime_error("Unmatched parentheses in " + funcName);
                if(expression[endExpr] == '(') parenCount++;
                else if(expression[endExpr] == ')') parenCount--;
            }
            
            string innerExpr = expression.substr(startExpr, endExpr - startExpr);
            ComplexNumber innerResult = evaluateExpression(innerExpr);
            values.push(funcEvaluator(innerResult));
            
            index = endExpr; // Move 'i' to the matching ')'
            expecting_operand = false; // After a function call, expect an operator
        };

        for(size_t i = 0; i < expression.length(); i++) {
            // Skip whitespace
            if(expression[i] == ' ') continue;
            
            // Handle percentages
            if(expression[i] == '%') {
                if(values.empty()) throw runtime_error("Invalid percentage syntax");
                ComplexNumber val = values.top();
                values.pop();
                values.push(val * ComplexNumber(0.01, 0.0)); // Divide by 100
                expecting_operand = false; // After a number/percentage, expect an operator
                continue;
            }
            
            // Handle natural constants pi and e
            else if(i + 2 <= expression.length() && expression.substr(i, 2) == "pi") {
                ComplexNumber pi_val(acos(-1.0), 0.0);
                values.push(pi_val);
                i += 1; // Move past "pi"
                expecting_operand = false; // After a constant, expect an operator
                continue;
            }
            else if(expression[i] == 'e' && (i == 0 || !isalpha(expression[i-1]))) {
                // Check if it's 'e' constant (not part of another word)
                if(i + 1 >= expression.length() || !isalpha(expression[i+1])) {
                    values.push(ComplexNumber(2.71828182846, 0.0));
                    expecting_operand = false; // After a constant, expect an operator
                    continue;
                }
            }
            else if(expression[i] == 'i' && (i == 0 || !isalpha(expression[i-1]))) {
                if (i + 1 >= expression.length() || !isalpha(expression[i + 1])) {
                    values.push(ComplexNumber(0.0, 1.0));
                    expecting_operand = false;
                    continue;
                }
            }
            // If number, parse it
            else if(isdigit(expression[i])) { 
                ComplexNumber val = parseNumber(expression, i);
                values.push(val);
                expecting_operand = false; // After a number, expect an operator
                continue; // parseNumber already advanced i
            }
            // Handle unary minus (e.g., -5, -(2+3))
            else if (expression[i] == '-' && expecting_operand) {
                values.push(ComplexNumber(0.0, 0.0)); // Push a zero
                ops.push('-'); // Push the minus operator
                expecting_operand = false; // After pushing an operator, expect an operand
            }
            // Handle supported functions
            else if(i + 4 <= expression.length() && expression.substr(i, 4) == "sqrt") {
                handleFunctionCall("sqrt", i, 4, [](const ComplexNumber& value) {
                    return value.sqrtPrincipal();
                });
            }
            else if(i + 4 <= expression.length() && expression.substr(i, 4) == "sind") {
                handleFunctionCall("sind", i, 4, [&](const ComplexNumber& value) {
                    return degreesToRadians(value).sin();
                });
            }
            else if(i + 3 <= expression.length() && expression.substr(i, 3) == "sin") {
                handleFunctionCall("sin", i, 3, [](const ComplexNumber& value) {
                    return value.sin();
                });
            }
            else if(i + 4 <= expression.length() && expression.substr(i, 4) == "cosd") {
                handleFunctionCall("cosd", i, 4, [&](const ComplexNumber& value) {
                    return degreesToRadians(value).cos();
                });
            }
            else if(i + 3 <= expression.length() && expression.substr(i, 3) == "cos") {
                handleFunctionCall("cos", i, 3, [](const ComplexNumber& value) {
                    return value.cos();
                });
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
                    ComplexNumber val2 = values.top();
                    values.pop();
                    
                    ComplexNumber val1 = values.top();
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
                    ComplexNumber val2 = values.top();
                    values.pop();
                    
                    ComplexNumber val1 = values.top();
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
            ComplexNumber val2 = values.top();
            values.pop();
            
            ComplexNumber val1 = values.top();
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
            ComplexNumber rightValueComplex = evaluateExpression(rightSide);
            double rightValue = extractRealComponent(rightValueComplex, "Linear equations require real constants");
            b -= rightValue; // Move to left side: ax + b - rightValue = 0
        }

        if (hasX) {
            try {
                std::vector<double> coeffs = {b, a};
                auto symbolicRoots = symbolic::solvePolynomialSymbolically(coeffs);
                if (!symbolicRoots.empty()) {
                    return formatSymbolicOutput(symbolicRoots);
                }
            } catch (const std::exception&) {
                // Fallback to numeric handling below.
            }
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
            ComplexNumber rightValueComplex = evaluateExpression(rightSide);
            double rightValue = extractRealComponent(rightValueComplex, "Quadratic equations require real constants");
            c -= rightValue; // Move to left side: ax^2 + bx + c - rightValue = 0
        }

        if (hasX2) {
            try {
                std::vector<double> coeffs = {c, b, a};
                auto symbolicRoots = symbolic::solvePolynomialSymbolically(coeffs);
                if (!symbolicRoots.empty()) {
                    return formatSymbolicOutput(symbolicRoots);
                }
            } catch (const std::exception&) {
                // Continue with numeric solution
            }
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
    
    string solveQuarticEquation(const string& equation) {
        auto coefficients = parsePolynomialEquationCoefficients(
            equation,
            4,
            "equation(x^4+x^3+x^2+x+1=0)",
            "Quartic equations require real constants",
            "Quartic equation must contain x^4 term");

        if (std::abs(coefficients[4]) < quartic::QUARTIC_EPS) {
            throw runtime_error("Quartic equation must contain x^4 term");
        }

        try {
            auto symbolicRoots = symbolic::solvePolynomialSymbolically(coefficients);
            if (!symbolicRoots.empty()) {
                return formatSymbolicOutput(symbolicRoots);
            }
        } catch (const std::exception&) {
            // Fall back to numeric Durand–Kerner solver below.
        }

        double a = coefficients[4];
        double b = coefficients[3];
        double c = coefficients[2];
        double d = coefficients[1];
        double e = coefficients[0];

        auto result = quartic::solve(a, b, c, d, e);
        if (!result.converged) {
            throw runtime_error("Quartic solver failed to converge within " + std::to_string(result.iterations) + " iterations");
        }

        string output;
        for (size_t idx = 0; idx < result.roots.size(); ++idx) {
            if (idx > 0) {
                output += ", ";
            }
            ComplexNumber root(result.roots[idx].real(), result.roots[idx].imag());
            output += "x" + to_string(static_cast<int>(idx + 1)) + " = " + root.toString();
        }

        return output;
    }

    string solveQuinticEquation(const string& equation) {
        auto coefficients = parsePolynomialEquationCoefficients(
            equation,
            5,
            "equation(x^5+x^4+x^3+x^2+x+1=0)",
            "Quintic equations require real constants",
            "Quintic equation must contain x^5 term");

        if (std::fabs(coefficients[5]) < POLY_EPSILON) {
            throw runtime_error("Quintic equation must contain x^5 term");
        }

        try {
            auto symbolicRoots = symbolic::solvePolynomialSymbolically(coefficients);
            if (symbolicRoots.empty()) {
                throw runtime_error("SymEngine returned no symbolic quintic roots");
            }
            return formatSymbolicOutput(symbolicRoots);
        } catch (const std::exception& e) {
            const std::string errMessage = e.what();
            const std::string prefix = "NON_FINITE::";
            if (errMessage.rfind(prefix, 0) == 0) {
                std::string polyExpr = polynomialToString(coefficients);
                std::string rootOfOutput = formatRootOfSolutions(polyExpr, coefficients.size() - 1);
                return rootOfOutput;
            }
            try {
                auto numericRoots = durandKerner(coefficients);
                return formatNumericRoots(numericRoots);
            } catch (const std::exception& numericErr) {
                throw runtime_error(string("SymEngine quintic solve failed: ") + errMessage + " / numeric fallback failed: " + numericErr.what());
            }
        }
    }
    
    // Function to solve cubic equations
    string solveCubicEquation(const string& equation) {
        // Check if equation starts with "equation(" and ends with ")"
        if (equation.length() < 11 || equation.substr(0, 9) != "equation(") {
            throw runtime_error("Invalid equation format. Use: equation(x^3-6x^2+11x-6=0)");
        }
        
        size_t endPos = equation.find_last_of(')');
        if (endPos == string::npos || endPos != equation.length() - 1) {
            throw runtime_error("Invalid equation format. Use: equation(x^3-6x^2+11x-6=0)");
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
        
        // Parse coefficients for cubic equation ax^3 + bx^2 + cx + d = 0
        double a = 0, b = 0, c = 0, d = 0;
        bool hasX3 = false;
        
        // Process left side
        size_t i = 0;
        while (i < leftSide.length()) {
            // Skip whitespace
            if (leftSide[i] == ' ') {
                i++;
                continue;
            }
            
            // Look for x^3 terms
            if (leftSide[i] == 'x' && i + 1 < leftSide.length() && leftSide[i + 1] == '^' && i + 2 < leftSide.length() && leftSide[i + 2] == '3') {
                hasX3 = true;
                a += 1.0; // coefficient is 1 if just 'x^3'
                i += 3; // Skip x^3
            }
            else if (i + 3 < leftSide.length() && leftSide.substr(i, 3) == "x^3") {
                hasX3 = true;
                a += 1.0;
                i += 3;
            }
            else if (i + 4 < leftSide.length() && leftSide.substr(i, 4) == "-x^3") {
                hasX3 = true;
                a += -1.0;
                i += 4;
            }
            else if (i + 4 < leftSide.length() && leftSide.substr(i, 4) == "+x^3") {
                hasX3 = true;
                a += 1.0;
                i += 4;
            }
            // Look for x^2 terms
            else if (leftSide[i] == 'x' && i + 1 < leftSide.length() && leftSide[i + 1] == '^' && i + 2 < leftSide.length() && leftSide[i + 2] == '2') {
                b += 1.0; // coefficient is 1 if just 'x^2'
                i += 3; // Skip x^2
            }
            else if (i + 3 < leftSide.length() && leftSide.substr(i, 3) == "x^2") {
                b += 1.0;
                i += 3;
            }
            else if (i + 4 < leftSide.length() && leftSide.substr(i, 4) == "-x^2") {
                b += -1.0;
                i += 4;
            }
            else if (i + 4 < leftSide.length() && leftSide.substr(i, 4) == "+x^2") {
                b += 1.0;
                i += 4;
            }
            // Look for x terms
            else if (leftSide[i] == 'x' && (i + 1 >= leftSide.length() || (leftSide[i + 1] != '^' && leftSide[i + 1] != '3' && leftSide[i + 1] != '2'))) {
                c += 1.0; // coefficient is 1 if just 'x'
                i++;
            }
            else if (i + 1 < leftSide.length() && leftSide.substr(i, 2) == "-x" && (i + 2 >= leftSide.length() || (leftSide[i + 2] != '^' && leftSide[i + 2] != '3' && leftSide[i + 2] != '2'))) {
                c += -1.0;
                i += 2;
            }
            else if (i + 1 < leftSide.length() && leftSide.substr(i, 2) == "+x" && (i + 2 >= leftSide.length() || (leftSide[i + 2] != '^' && leftSide[i + 2] != '3' && leftSide[i + 2] != '2'))) {
                c += 1.0;
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
                
                // Check if this number is multiplied by x^3
                if (i + 2 < leftSide.length() && leftSide[i] == 'x' && leftSide[i + 1] == '^' && leftSide[i + 2] == '3') {
                    hasX3 = true;
                    a += isNegative ? -num : num;
                    i += 3; // Skip x^3
                }
                // Check if this number is multiplied by x^2
                else if (i + 2 < leftSide.length() && leftSide[i] == 'x' && leftSide[i + 1] == '^' && leftSide[i + 2] == '2') {
                    b += isNegative ? -num : num;
                    i += 3; // Skip x^2
                }
                // Check if this number is multiplied by x
                else if (i < leftSide.length() && leftSide[i] == 'x' && (i + 1 >= leftSide.length() || (leftSide[i + 1] != '^' && leftSide[i + 1] != '3' && leftSide[i + 1] != '2'))) {
                    c += isNegative ? -num : num;
                    i++; // Skip x
                } else {
                    d += isNegative ? -num : num;
                }
            }
            else {
                throw runtime_error("Invalid character in equation: " + string(1, leftSide[i]));
            }
        }
        
        // Process right side (treat as constant)
        if (!rightSide.empty()) {
            ComplexNumber rightValueComplex = evaluateExpression(rightSide);
            double rightValue = extractRealComponent(rightValueComplex, "Cubic equations require real constants");
            d -= rightValue; // Move to left side: ax^3 + bx^2 + cx + d - rightValue = 0
        }

        if (hasX3) {
            try {
                std::vector<double> coeffs = {d, c, b, a};
                auto symbolicRoots = symbolic::solvePolynomialSymbolically(coeffs);
                if (!symbolicRoots.empty()) {
                    return formatSymbolicOutput(symbolicRoots);
                }
            } catch (const std::exception&) {
                // Continue with numeric fallback.
            }
        }
        
        if (!hasX3) {
            throw runtime_error("Cubic equation must contain x^3 term");
        }
        
        // Normalize coefficients
        b /= a;
        c /= a;
        d /= a;
        
        // Use Cardano's method to solve cubic equation
        // Convert to depressed cubic: t^3 + pt + q = 0
        double p = c - b * b / 3.0;
        double q = (2.0 * b * b * b - 9.0 * b * c + 27.0 * d) / 27.0;
        
        // Calculate discriminant
        double discriminant = q * q / 4.0 + p * p * p / 27.0;
        
        vector<double> realRoots;
        vector<std::pair<double, double>> complexRoots; // Real and imaginary parts
        
        if (discriminant > 0) {
            // One real root and two complex conjugate roots
            double sqrt_discriminant = sqrt(discriminant);
            double u = cbrt(-q / 2.0 + sqrt_discriminant);
            double v = cbrt(-q / 2.0 - sqrt_discriminant);
            double realRoot = u + v - b / 3.0;
            realRoots.push_back(realRoot);
            
            // Calculate complex roots
            double realPart = -(u + v) / 2.0 - b / 3.0;
            double imaginaryPart = (u - v) * sqrt(3.0) / 2.0;
            complexRoots.push_back({realPart, imaginaryPart});
            complexRoots.push_back({realPart, -imaginaryPart});
        } else if (abs(discriminant) < 1e-12) { // discriminant == 0
            // All roots are real and at least two are equal
            if (abs(q) < 1e-12) {
                // Triple root
                realRoots.push_back(-b / 3.0);
                realRoots.push_back(-b / 3.0);
                realRoots.push_back(-b / 3.0);
            } else {
                // One single root and one double root
                double u = cbrt(-q / 2.0);
                realRoots.push_back(2.0 * u - b / 3.0);
                realRoots.push_back(-u - b / 3.0);
                realRoots.push_back(-u - b / 3.0);
            }
        } else {
            // Three distinct real roots (Casus irreducibilis)
            double rho = sqrt(-p * p * p / 27.0);
            double theta = acos(-q / (2.0 * rho));
            double cbrt_rho = cbrt(rho);
            
            realRoots.push_back(2.0 * cbrt_rho * cos(theta / 3.0) - b / 3.0);
            realRoots.push_back(2.0 * cbrt_rho * cos((theta + 2.0 * std::numbers::pi) / 3.0) - b / 3.0);
            realRoots.push_back(2.0 * cbrt_rho * cos((theta + 4.0 * std::numbers::pi) / 3.0) - b / 3.0);
        }
        
        // Format result
        string result;
        int rootIndex = 1;
        
        // Add real roots to result
        for (size_t i = 0; i < realRoots.size(); i++) {
            if (rootIndex > 1) result += ", ";
            
            // Handle negative zero
            if (abs(realRoots[i]) < 1e-10) {
                realRoots[i] = 0.0;
            }
            
            Fraction root_frac = Fraction::fromDouble(realRoots[i]);
            if (std::fabs(realRoots[i] - (static_cast<double>(root_frac.numerator) / root_frac.denominator)) < 1e-9) {
                result += "x" + to_string(rootIndex) + " = " + root_frac.toString();
            } else {
                std::ostringstream oss;
                oss << std::fixed << std::setprecision(10) << realRoots[i];
                string rootStr = oss.str();
                // Remove trailing zeros and decimal point if not needed
                rootStr = rootStr.substr(0, rootStr.find_last_not_of('0') + 1);
                if (rootStr.back() == '.') rootStr.pop_back();
                result += "x" + to_string(rootIndex) + " = " + rootStr;
            }
            rootIndex++;
        }
        
        // Add complex roots to result
        for (size_t i = 0; i < complexRoots.size(); i++) {
            if (rootIndex > 1) result += ", ";
            
            double realPart = complexRoots[i].first;
            double imaginaryPart = complexRoots[i].second;
            
            // Handle negative zero
            if (abs(realPart) < 1e-10) realPart = 0.0;
            if (abs(imaginaryPart) < 1e-10) imaginaryPart = 0.0;
            
            std::ostringstream oss_real, oss_imag;
            oss_real << std::fixed << std::setprecision(10) << realPart;
            oss_imag << std::fixed << std::setprecision(10) << abs(imaginaryPart);
            
            string realStr = oss_real.str();
            string imagStr = oss_imag.str();
            
            // Remove trailing zeros for cleaner output
            realStr = realStr.substr(0, realStr.find_last_not_of('0') + 1);
            if (realStr.back() == '.') realStr.pop_back();
            
            imagStr = imagStr.substr(0, imagStr.find_last_not_of('0') + 1);
            if (imagStr.back() == '.') imagStr.pop_back();
            
            // Format complex number
            if (abs(realPart) < 1e-10) {
                // Pure imaginary
                if (abs(abs(imaginaryPart) - 1.0) < 1e-10) {
                    if (imaginaryPart >= 0) {
                        result += "x" + to_string(rootIndex) + " = i";
                    } else {
                        result += "x" + to_string(rootIndex) + " = -i";
                    }
                } else {
                    if (imaginaryPart >= 0) {
                        result += "x" + to_string(rootIndex) + " = " + imagStr + "i";
                    } else {
                        result += "x" + to_string(rootIndex) + " = -" + imagStr + "i";
                    }
                }
            } else {
                // Complex number with both real and imaginary parts
                string formattedRoot = realStr;
                if (imaginaryPart >= 0) {
                    if (abs(abs(imaginaryPart) - 1.0) < 1e-10) {
                        formattedRoot += " + i";
                    } else {
                        formattedRoot += " + " + imagStr + "i";
                    }
                } else {
                    if (abs(abs(imaginaryPart) - 1.0) < 1e-10) {
                        formattedRoot += " - i";
                    } else {
                        formattedRoot += " - " + imagStr + "i";
                    }
                }
                result += "x" + to_string(rootIndex) + " = " + formattedRoot;
            }
            rootIndex++;
        }
        
        return result;
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
            ComplexNumber rightValueComplex = evaluateExpression(rightSide);
            double rightValue = extractRealComponent(rightValueComplex, "Systems of equations require real constants");
            eqConst += rightValue;
            
            coefficients.push_back(eqCoeff);
            constants.push_back(eqConst);
        }
        
        // Build matrix for Gaussian elimination
        int n = static_cast<int>(variables.size());
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
                // Check if it's a quintic equation (contains x^5)
                if (input.find("x^5") != string::npos) {
                    return solveQuinticEquation(input);
                }
                // Check if it's a quartic equation (contains x^4)
                if (input.find("x^4") != string::npos) {
                    return solveQuarticEquation(input);
                }
                // Check if it's a cubic equation (contains x^3)
                if (input.find("x^3") != string::npos) {
                    return solveCubicEquation(input);
                }
                // Check if it's a quadratic equation (contains x^2)
                else if (input.find("x^2") != string::npos) {
                    return solveQuadraticEquation(input);
                } else {
                    // Linear equation
                    return solveEquation(input);
                }
            }
            else {
                // Regular expression evaluation
                ComplexNumber result = evaluateExpression(input);
                return result.toString();
            }
        } catch (const std::exception& e) {
            return "Error: " + string(e.what());
        }
    }
}
