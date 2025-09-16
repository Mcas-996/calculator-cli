#include "string_processing.hpp"
#include <stack>
#include <cctype>
#include <stdexcept>
#include <cmath>
#include <string>

namespace sp {
    // Helper function to perform arithmetic operations
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
        return 0;
    }

    // Helper function to handle unary operations (like square root)
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
    double parseNumber(const std::string& expression, size_t& i) {
        double result = 0;
        bool isNegative = false;
        bool hasDecimal = false;
        double decimalMultiplier = 0.1;
        
        // Check for negative sign
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
            hasDecimal = true;
            i++;
            while (i < expression.length() && isdigit(expression[i])) {
                result += (expression[i] - '0') * decimalMultiplier;
                decimalMultiplier *= 0.1;
                i++;
            }
        }
        
        i--; // Adjust for extra increment in the main loop
        return isNegative ? -result : result;
    }

    // Function to evaluate middle school math expressions
    double evaluateExpression(const std::string& expression) {
        std::stack<double> values;  // Stack for numbers (changed to double)
        std::stack<char> ops;    // Stack for operators
        
        for(size_t i = 0; i < expression.length(); i++) {
            // Skip whitespace
            if(expression[i] == ' ') continue;
            
            // Handle percentages
            if(expression[i] == '%') {
                if(values.empty()) throw std::runtime_error("Invalid percentage syntax");
                double val = values.top();
                values.pop();
                values.push(val / 100.0);
                continue;
            }
            
            // Handle natural constants pi and e
            else if(i + 2 <= expression.length() && expression.substr(i, 2) == "pi") {
                values.push(3.14159265359);
                i += 1; // Move past "pi"
                continue;
            }
            else if(expression[i] == 'e' && (i == 0 || !isalpha(expression[i-1]))) {
                // Check if it's 'e' constant (not part of another word)
                if(i + 1 >= expression.length() || !isalpha(expression[i+1])) {
                    values.push(2.71828182846);
                    continue;
                }
            }
            // If number (including decimals and negatives), parse it
            else if(isdigit(expression[i]) || (expression[i] == '-' && (i == 0 || !isdigit(expression[i-1])))) {
                size_t startPos = i;
                double val = parseNumber(expression, i);
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
                size_t startExpr = i + 1;
                size_t endExpr = i;
                
                while(endExpr + 1 < expression.length() && parenCount > 0) {
                    endExpr++;
                    if(expression[endExpr] == '(') parenCount++;
                    else if(expression[endExpr] == ')') parenCount--;
                }
                
                if(parenCount != 0) throw std::runtime_error("Unmatched parentheses in sqrt");
                
                // Extract expression inside sqrt
                std::string sqrtExpr = expression.substr(startExpr, endExpr - startExpr);
                double sqrtResult = evaluateExpression(sqrtExpr);
                
                if(sqrtResult < 0) throw std::runtime_error("Square root of negative number");
                values.push(std::sqrt(sqrtResult));
                
                i = endExpr; // Move to closing parenthesis
            }
            // If opening parenthesis, push to ops stack
            else if(expression[i] == '(') {
                ops.push(expression[i]);
            }
            // If closing parenthesis, solve entire brace
            else if(expression[i] == ')') {
                while(!ops.empty() && ops.top() != '(') {
                    double val2 = values.top();
                    values.pop();
                    
                    double val1 = values.top();
                    values.pop();
                    
                    char op = ops.top();
                    ops.pop();
                    
                    values.push(applyOp(val1, val2, op));
                }
                if(!ops.empty()) ops.pop(); // Pop opening parenthesis
            }
            // If operator, process according to precedence
            else if(expression[i] == '+' || expression[i] == '-' || expression[i] == '*' || expression[i] == '/' || expression[i] == '^') {
                while(!ops.empty() && precedence(ops.top()) >= precedence(expression[i])) {
                    double val2 = values.top();
                    values.pop();
                    
                    double val1 = values.top();
                    values.pop();
                    
                    char op = ops.top();
                    ops.pop();
                    
                    values.push(applyOp(val1, val2, op));
                }
                ops.push(expression[i]);
            }
        }
        
        // Process remaining operators
        while(!ops.empty()) {
            double val2 = values.top();
            values.pop();
            
            double val1 = values.top();
            values.pop();
            
            char op = ops.top();
            ops.pop();
            
            values.push(applyOp(val1, val2, op));
        }
        
        if(values.empty()) throw std::runtime_error("Invalid expression");
        return values.top();
    }
}