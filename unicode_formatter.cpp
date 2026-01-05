#include "unicode_formatter.hpp"
#include <cmath>
#include <iomanip>
#include <sstream>
#include <string>

namespace pretty {

std::string UnicodeFormatter::formatOperator(char op) {
    switch (op) {
        case '*': return "×";
        case '/': return "÷";
        case '+': return "+";
        case '-': return "-";
        case '^': return "^";
        case '=': return "=";
        default: return std::string(1, op);
    }
}

std::string UnicodeFormatter::formatFunction(const std::string& func) {
    if (func == "sqrt") return "√";
    if (func == "cbrt") return "³√";
    if (func == "sin") return "sin";
    if (func == "cos") return "cos";
    if (func == "tan") return "tan";
    if (func == "sind") return "sind";
    if (func == "cosd") return "cosd";
    if (func == "tand") return "tand";
    if (func == "abs") return "|";
    if (func == "log") return "log";
    if (func == "ln") return "ln";
    if (func == "exp") return "exp";
    return func;
}

std::string UnicodeFormatter::formatConstant(const std::string& constant) {
    if (constant == "pi" || constant == "π") return "π";
    if (constant == "e") return "e";
    return constant;
}

std::string UnicodeFormatter::formatExpression(const std::string& expr) {
    std::string result;
    size_t i = 0;
    
    while (i < expr.length()) {
        // 处理 sqrt()
        if (i + 4 <= expr.length() && expr.substr(i, 4) == "sqrt") {
            i += 4;
            if (i < expr.length() && expr[i] == '(') {
                size_t end = findMatchingParen(expr, i);
                if (end != std::string::npos) {
                    std::string arg = expr.substr(i + 1, end - i - 1);
                    result += formatSquareRoot(formatExpression(arg));
                    i = end + 1;
                } else {
                    result += "sqrt";
                }
            } else {
                result += "sqrt";
            }
        }
        // 处理 cbrt()
        else if (i + 4 <= expr.length() && expr.substr(i, 4) == "cbrt") {
            i += 4;
            if (i < expr.length() && expr[i] == '(') {
                size_t end = findMatchingParen(expr, i);
                if (end != std::string::npos) {
                    std::string arg = expr.substr(i + 1, end - i - 1);
                    result += formatCubeRoot(formatExpression(arg));
                    i = end + 1;
                } else {
                    result += "cbrt";
                }
            } else {
                result += "cbrt";
            }
        }
        // 处理 pi
        else if (i + 2 <= expr.length() && expr.substr(i, 2) == "pi") {
            result += "π";
            i += 2;
        }
        // 处理幂运算 x^2, x^3
        else if (expr[i] == '^' && i + 1 < expr.length()) {
            i++;
            // 检查是否是简单的数字指数
            if (i < expr.length() && expr[i] >= '0' && expr[i] <= '9') {
                int exp = expr[i] - '0';
                result += formatSuperscript(exp);
                i++;
            } else {
                result += "^";
            }
        }
        // 处理运算符
        else if (expr[i] == '*' || expr[i] == '/') {
            result += formatOperator(expr[i]);
            i++;
        }
        // 其他字符直接复制
        else {
            result += expr[i];
            i++;
        }
    }
    
    return result;
}

std::string UnicodeFormatter::formatSubscript(int index) {
    const std::string subscripts = "₀₁₂₃₄₅₆₇₈₉";
    if (index >= 0 && index <= 9) {
        // 每个 Unicode 字符占用 3 个字节（UTF-8 编码）
        return subscripts.substr(index * 3, 3);
    }
    // 对于多位数字，逐位转换
    std::string result;
    std::string str = std::to_string(index);
    for (char c : str) {
        int digit = c - '0';
        if (digit >= 0 && digit <= 9) {
            result += subscripts.substr(digit * 3, 3);
        }
    }
    return result;
}

std::string UnicodeFormatter::formatSuperscript(int index) {
    const std::string superscripts = "⁰¹²³⁴⁵⁶⁷⁸⁹";
    if (index >= 0 && index <= 9) {
        // 每个 Unicode 字符占用 3 个字节（UTF-8 编码）
        return superscripts.substr(index * 3, 3);
    }
    // 对于多位数字，逐位转换
    std::string result;
    std::string str = std::to_string(index);
    for (char c : str) {
        int digit = c - '0';
        if (digit >= 0 && digit <= 9) {
            result += superscripts.substr(digit * 3, 3);
        }
    }
    return result;
}

std::string UnicodeFormatter::formatComplex(const ComplexNumber& cn) {
    auto formatComponent = [](double value) -> std::string {
        // 1. 首先尝试简化根式（如 2.8284271247 → 2√2）
        std::string simplified = simplifyRadical(std::fabs(value));
        if (!simplified.empty()) {
            if (value < 0) {
                return "-" + simplified;
            }
            return simplified;
        }
        
        // 2. 尝试分数表示（仅当分母较小时）
        Fraction frac = Fraction::fromDouble(value);
        double fracValue = static_cast<double>(frac.numerator) / frac.denominator;
        if (std::fabs(value - fracValue) < 1e-9 && frac.denominator <= 100) {
            // 使用 Unicode 分数符号
            return formatFraction(frac);
        }
        
        // 3. 否则使用小数
        std::ostringstream oss;
        oss << std::fixed << std::setprecision(10) << value;
        std::string str = oss.str();
        while (!str.empty() && str.back() == '0') {
            str.pop_back();
        }
        if (!str.empty() && str.back() == '.') {
            str.pop_back();
        }
        if (str == "-0") {
            str = "0";
        }
        return str;
    };

    const double epsilon = 1e-9;
    bool realZero = std::fabs(cn.real) < epsilon;
    bool imagZero = std::fabs(cn.imag) < epsilon;

    if (realZero && imagZero) return "0";
    if (imagZero) return formatComponent(cn.real);
    if (realZero) {
        std::string imagStr = formatComponent(cn.imag);
        if (imagStr == "1") return "i";
        if (imagStr == "-1") return "-i";
        return imagStr + "i";
    }

    std::string realStr = formatComponent(cn.real);
    std::string imagStr = formatComponent(std::fabs(cn.imag));
    std::string sign = cn.imag >= 0 ? " + " : " - ";

    if (imagStr == "1") {
        imagStr = "";
    }
    return realStr + sign + (imagStr.empty() ? "i" : imagStr + " i");
}

std::string UnicodeFormatter::formatFraction(const Fraction& frac) {
    if (frac.denominator == 1) {
        return std::to_string(frac.numerator);
    }
    // 使用 Unicode 分数符号（仅支持常见分数）
    if (frac.numerator == 1 && frac.denominator == 2) return "½";
    if (frac.numerator == -1 && frac.denominator == 2) return "-½";
    if (frac.numerator == 1 && frac.denominator == 3) return "⅓";
    if (frac.numerator == 2 && frac.denominator == 3) return "⅔";
    if (frac.numerator == -1 && frac.denominator == 3) return "-⅓";
    if (frac.numerator == -2 && frac.denominator == 3) return "-⅔";
    if (frac.numerator == 1 && frac.denominator == 4) return "¼";
    if (frac.numerator == 3 && frac.denominator == 4) return "¾";
    if (frac.numerator == -1 && frac.denominator == 4) return "-¼";
    if (frac.numerator == -3 && frac.denominator == 4) return "-¾";
    if (frac.numerator == 1 && frac.denominator == 5) return "⅕";
    if (frac.numerator == 2 && frac.denominator == 5) return "⅖";
    if (frac.numerator == 3 && frac.denominator == 5) return "⅗";
    if (frac.numerator == 4 && frac.denominator == 5) return "⅘";
    if (frac.numerator == 1 && frac.denominator == 6) return "⅙";
    if (frac.numerator == 5 && frac.denominator == 6) return "⅚";
    if (frac.numerator == -1 && frac.denominator == 6) return "-⅙";
    if (frac.numerator == -5 && frac.denominator == 6) return "-⅚";
    if (frac.numerator == 1 && frac.denominator == 7) return "⅐";
    if (frac.numerator == -1 && frac.denominator == 7) return "-⅐";
    if (frac.numerator == 1 && frac.denominator == 8) return "⅛";
    if (frac.numerator == 3 && frac.denominator == 8) return "⅜";
    if (frac.numerator == 5 && frac.denominator == 8) return "⅝";
    if (frac.numerator == 7 && frac.denominator == 8) return "⅞";
    if (frac.numerator == -1 && frac.denominator == 8) return "-⅛";
    if (frac.numerator == -3 && frac.denominator == 8) return "-⅜";
    if (frac.numerator == -5 && frac.denominator == 8) return "-⅝";
    if (frac.numerator == -7 && frac.denominator == 8) return "-⅞";
    if (frac.numerator == 1 && frac.denominator == 9) return "⅑";
    if (frac.numerator == -1 && frac.denominator == 9) return "-⅑";
    if (frac.numerator == 1 && frac.denominator == 10) return "⅒";
    if (frac.numerator == -1 && frac.denominator == 10) return "-⅒";
    
    // 其他分数使用小数
    std::ostringstream oss;
    oss << std::fixed << std::setprecision(10) 
        << static_cast<double>(frac.numerator) / frac.denominator;
    std::string str = oss.str();
    while (!str.empty() && str.back() == '0') {
        str.pop_back();
    }
    if (!str.empty() && str.back() == '.') {
        str.pop_back();
    }
    return str;
}

std::string UnicodeFormatter::formatPower(const std::string& base, int exp) {
    return base + formatSuperscript(exp);
}

std::string UnicodeFormatter::formatSquareRoot(const std::string& arg) {
    return "√" + arg;
}

std::string UnicodeFormatter::formatCubeRoot(const std::string& arg) {
    return "³√" + arg;
}

std::string UnicodeFormatter::simplifyRadical(double value) {
    // 首先检查是否为整数或简单分数
    double rounded = std::round(value);
    if (std::fabs(value - rounded) < 1e-9) {
        // 是整数，不进行根式简化
        return "";
    }
    
    // 检查是否为简单分数
    Fraction frac = Fraction::fromDouble(value);
    double fracValue = static_cast<double>(frac.numerator) / frac.denominator;
    if (std::fabs(value - fracValue) < 1e-9 && frac.denominator <= 100) {
        // 是简单分数，不进行根式简化
        return "";
    }
    
    // 常见的无理数因子：√2, √3, √5, √6, √7, √10
    const double sqrt2 = std::sqrt(2.0);
    const double sqrt3 = std::sqrt(3.0);
    const double sqrt5 = std::sqrt(5.0);
    const double sqrt6 = std::sqrt(6.0);
    const double sqrt7 = std::sqrt(7.0);
    const double sqrt10 = std::sqrt(10.0);
    
    struct Radical {
        double factor;
        int base;
        std::string symbol;
    };
    
    std::vector<Radical> radicals = {
        {sqrt2, 2, "√2"},
        {sqrt3, 3, "√3"},
        {sqrt5, 5, "√5"},
        {sqrt6, 6, "√6"},
        {sqrt7, 7, "√7"},
        {sqrt10, 10, "√10"}
    };
    
    // 首先检查 value 是否直接等于某个根式
    for (const auto& rad : radicals) {
        if (std::fabs(value - rad.factor) < 1e-6) {
            return rad.symbol;
        }
    }
    
    // 尝试匹配每个无理数因子
    for (const auto& rad : radicals) {
        double coeff = value / rad.factor;
        
        // 检查系数是否接近整数
        double rounded = std::round(coeff);
        if (std::fabs(coeff - rounded) < 1e-6 && rounded != 0) {
            // 整数系数
            if (std::fabs(rounded) == 1) {
                return (rounded > 0 ? "" : "-") + rad.symbol;
            } else {
                return std::to_string(static_cast<long long>(rounded)) + rad.symbol;
            }
        }
    }
    
    // 如果没有找到整数系数，尝试简单分数系数（分母 <= 10）
    for (const auto& rad : radicals) {
        double coeff = value / rad.factor;
        
        // 检查是否为简单分数
        Fraction frac = Fraction::fromDouble(coeff);
        double fracValue = static_cast<double>(frac.numerator) / frac.denominator;
        if (std::fabs(coeff - fracValue) < 1e-6 && frac.denominator <= 10) {
            if (frac.denominator == 1) {
                if (std::abs(frac.numerator) == 1) {
                    return (frac.numerator > 0 ? "" : "-") + rad.symbol;
                } else {
                    return std::to_string(frac.numerator) + rad.symbol;
                }
            }
            // 简单分数系数（如 1/2, 3/2 等）
            std::string sign = (frac.numerator > 0 ? "" : "-");
            if (std::abs(frac.numerator) == 1) {
                return sign + "√" + std::to_string(rad.base) + "/" + std::to_string(frac.denominator);
            } else {
                return std::to_string(frac.numerator) + "√" + std::to_string(rad.base) + "/" + std::to_string(frac.denominator);
            }
        }
    }
    
    // 无法简化，返回空字符串
    return "";
}

bool UnicodeFormatter::isNumber(const std::string& str) {
    if (str.empty()) return false;
    size_t start = 0;
    if (str[0] == '-') {
        if (str.length() == 1) return false;
        start = 1;
    }
    bool hasDecimal = false;
    for (size_t i = start; i < str.length(); ++i) {
        if (str[i] == '.') {
            if (hasDecimal) return false;
            hasDecimal = true;
        } else if (!std::isdigit(str[i])) {
            return false;
        }
    }
    return true;
}

size_t UnicodeFormatter::findMatchingParen(const std::string& str, size_t start) {
    if (start >= str.length() || str[start] != '(') {
        return std::string::npos;
    }
    
    int depth = 1;
    for (size_t i = start + 1; i < str.length(); ++i) {
        if (str[i] == '(') {
            depth++;
        } else if (str[i] == ')') {
            depth--;
            if (depth == 0) {
                return i;
            }
        }
    }
    
    return std::string::npos;
}

} // namespace pretty