#include"string_processing.hpp"
#include<string>
#include<iostream>
using namespace std;

int main(int argc, char* argv[]) {
    if (argc != 2) {
        cout << "Usage: " << argv[0] << " \"expression\"" << endl;
        return 1;
    }
    
    string arg = argv[1];
    if (arg == "--help" || arg == "-h") {
        cout << "Math Expression Calculator" << endl;
        cout << "Usage: " << argv[0] << " \"expression\"" << endl;
        cout << endl;
        cout << "Supported operations:" << endl;
        cout << "  +, -, *, /, ^ (exponent)" << endl;
        cout << "  Parentheses for grouping" << endl;
        cout << "  Negative numbers and decimals" << endl;
        cout << "  Percentages (e.g., 50% converts to 0.5)" << endl;
        cout << "  sqrt() function for square roots" << endl;
        cout << "  Constants: pi (3.14159...), e (2.71828...)" << endl;
<<<<<<< HEAD
        cout << "  Equation solving: equation(x+1=0)" << endl;
        cout << "  Quadratic equations: equation(x^2+2x+1=0)" << endl;
=======
>>>>>>> 1917b2042a898d6b3f12b11e39489442d7ccb87d
        cout << endl;
        cout << "Examples:" << endl;
        cout << "  3 + 5 * (2 - 8)^2" << endl;
        cout << "  -2.5 * 4 + 3^2" << endl;
        cout << "  50% * 200" << endl;
        cout << "  sqrt(16) + 3" << endl;
        cout << "  pi * 2" << endl;
        cout << "  e^2" << endl;
<<<<<<< HEAD
        cout << "  equation(x+1=0)" << endl;
        cout << "  equation(2x-3=7)" << endl;
        cout << "  equation(x^2+2x+1=0)" << endl;
        cout << "  equation(x^2-5x+6=0)" << endl;
        return 0;
    }
    
    string result = sp::processInput(argv[1]);
    cout << result << endl;
    
    // Check if the result starts with "Error:" to determine exit code
    if (result.substr(0, 6) == "Error:") {
=======
        return 0;
    }
    
    try {
        double result = sp::evaluateExpression(argv[1]);
        cout << result << endl;
    } catch (const std::exception& e) {
        cout << "Error: " << e.what() << endl;
>>>>>>> 1917b2042a898d6b3f12b11e39489442d7ccb87d
        return 1;
    }
    return 0;
}