#include"string_processing.hpp"
#include"pretty_output_config.hpp"
#include "latex_renderer.hpp"
#include <iostream>
#include <string>
using namespace std;

int main(int argc, char* argv[]) {
    bool outputLatexCode = false;
    if (argc == 1) {
        cout << "Interactive mode (Ctrl+D to exit)" << endl;
        string line;
        while (true) {
            cout << ">>> " << flush;
            if (!getline(cin, line)) {
                cout << endl;
                break;
            }
            if (line.empty()) {
                continue;
            }
            string result = sp::processInput(line);
            cout << result << endl;
        }
        return 0;
    }
    // Process command line arguments
    string expression;
    bool hasExpression = false;

    for (int i = 1; i < argc; i++) {
        string arg = argv[i];
        
        if (arg == "--version" || arg == "-v") {
            cout << "1.0.0" << endl;
            return 0;
        }
        else if (arg == "--pretty" || arg == "-p") {
            // Enable pretty output (auto-detect best format)
            pretty::PrettyConfig::getInstance().setPrettyLevel(pretty::PrettyLevel::UNICODE);
            // Try to upgrade to LaTeX if available
            if (pretty::PrettyConfig::getInstance().supportsKittyProtocol() &&
                pretty::PrettyConfig::getInstance().isLaTeXAvailable()) {
                pretty::PrettyConfig::getInstance().setPrettyLevel(pretty::PrettyLevel::LATEX);
            }
        }
        else if (arg == "--unicode" || arg == "-u") {
            pretty::PrettyConfig::getInstance().setPrettyLevel(pretty::PrettyLevel::UNICODE);
        }
        else if (arg == "--latex" || arg == "-l") {
            pretty::PrettyConfig::getInstance().setPrettyLevel(pretty::PrettyLevel::UNICODE);
            // 标记需要输出 LaTeX 代码
            outputLatexCode = true;
        }
        else if (arg == "--ascii" || arg == "-a") {
            pretty::PrettyConfig::getInstance().setPrettyLevel(pretty::PrettyLevel::ASCII);
        }
        else if (arg == "--help" || arg == "-h") {
            cout << "Math Expression Calculator" << endl;
            cout << "Usage: " << argv[0] << " [OPTIONS] \"expression\"" << endl;
            cout << endl;
            cout << "Options:" << endl;
            cout << "  --pretty, -p     Enable pretty output (auto-detect best format)" << endl;
            cout << "  --unicode, -u    Force Unicode math symbols output" << endl;
            cout << "  --latex, -l      Force LaTeX output format" << endl;
            cout << "  --ascii, -a      Force ASCII output (default)" << endl;
            cout << "  --help, -h       Show this help message" << endl;
            cout << "  --version, -v    Show version information" << endl;
            cout << endl;
            cout << "Supported operations:" << endl;
            cout << "  +, -, *, /, ^ (exponent)" << endl;
            cout << "  Parentheses for grouping" << endl;
            cout << "  Negative numbers and decimals" << endl;
            cout << "  Percentages (e.g., 50% converts to 0.5)" << endl;
            cout << "  sqrt() function for square roots" << endl;
            cout << "  sin(), cos() (radians) and sind(), cosd() (degrees, complex-friendly)" << endl;
            cout << "  Constants: pi (3.14159...), e (2.71828...)" << endl;
            cout << "  Equation solving: equation(x+1=0)" << endl;
            cout << "  Quadratic equations: equation(x^2+2x+1=0)" << endl;
            cout << "  System of linear equations: equation2(x+y=5,x-y=1)" << endl;
            cout << endl;
            cout << "Examples:" << endl;
            cout << "  " << argv[0] << " \"3 + 5 * (2 - 8)^2\"" << endl;
            cout << "  " << argv[0] << " --pretty \"sqrt(16) + 3\"" << endl;
            cout << "  " << argv[0] << " -2.5 * 4 + 3^2" << endl;
            cout << "  " << argv[0] << " \"50% * 200\"" << endl;
            cout << "  " << argv[0] << " \"sqrt(16) + 3\"" << endl;
            cout << "  " << argv[0] << " \"pi * 2\"" << endl;
            cout << "  " << argv[0] << " \"e^2\"" << endl;
            cout << "  " << argv[0] << " \"equation(x+1=0)\"" << endl;
            cout << "  " << argv[0] << " \"equation(2x-3=7)\"" << endl;
            cout << "  " << argv[0] << " \"equation(x^2+2x+1=0)\"" << endl;
            cout << "  " << argv[0] << " \"equation(x^2-5x+6=0)\"" << endl;
            cout << "  " << argv[0] << " \"equation2(x+y=5,x-y=1)\"" << endl;
            cout << "  " << argv[0] << " \"equation2(2x+3y=12,4x-y=5)\"" << endl;
            cout << "  " << argv[0] << " \"equation2(x+y+z=6,x-y+z=2,2x+y-z=3)\"" << endl;
            return 0;
        }
        else {
            // Assume it's an expression
            expression = arg;
            hasExpression = true;
        }
    }
    
    // If no expression provided, enter interactive mode
    if (!hasExpression) {
        cout << "Interactive mode (Ctrl+D to exit)" << endl;
        string line;
        while (true) {
            cout << pretty::PrettyConfig::getInstance().getPrompt() << flush;
            if (!getline(cin, line)) {
                cout << endl;
                break;
            }
            if (line.empty()) {
                continue;
            }
            string result = sp::processInput(line);
            cout << result << endl;
        }
        return 0;
    }
    
    // Evaluate the expression
    string result = sp::processInput(expression);
    
    // 如果需要输出 LaTeX 代码，将结果转换为 LaTeX 格式
    if (outputLatexCode) {
        // 将结果包装在 LaTeX 数学模式中
        result = "\\[" + result + "\\]";
    }
    
    cout << result << endl;
    
    // Check if the result starts with "Error:" to determine exit code
    if (result.substr(0, 6) == "Error:") {
        return 1;
    }
    return 0;
}
