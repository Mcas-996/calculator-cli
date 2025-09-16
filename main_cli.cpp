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
        cout << endl;
        cout << "Examples:" << endl;
        cout << "  3 + 5 * (2 - 8)^2" << endl;
        cout << "  -2.5 * 4 + 3^2" << endl;
        cout << "  50% * 200" << endl;
        cout << "  sqrt(16) + 3" << endl;
        cout << "  pi * 2" << endl;
        cout << "  e^2" << endl;
        return 0;
    }
    
    try {
        double result = sp::evaluateExpression(argv[1]);
        cout << result << endl;
    } catch (const std::exception& e) {
        cout << "Error: " << e.what() << endl;
        return 1;
    }
    return 0;
}