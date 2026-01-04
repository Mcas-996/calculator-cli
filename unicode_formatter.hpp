#ifndef UNICODE_FORMATTER_HPP
#define UNICODE_FORMATTER_HPP

#include <cmath>
#include <iomanip>
#include <sstream>
#include <string>
#include <vector>

#include "complex_number.hpp"
#include "fractions.hpp"

namespace pretty {

// Unicode 格式化器
class UnicodeFormatter {
public:
    // 格式化运算符
    static std::string formatOperator(char op);
    
    // 格式化函数名
    static std::string formatFunction(const std::string& func);
    
    // 格式化常量
    static std::string formatConstant(const std::string& constant);
    
    // 格式化表达式（简单替换）
    static std::string formatExpression(const std::string& expr);
    
    // 格式化下标（0-9）
    static std::string formatSubscript(int index);
    
    // 格式化上标（0-9）
    static std::string formatSuperscript(int index);
    
    // 格式化复数
    static std::string formatComplex(const ComplexNumber& cn);
    
    // 格式化分数
    static std::string formatFraction(const Fraction& frac);
    
    // 格式化幂运算（如 x^2 → x²）
    static std::string formatPower(const std::string& base, int exp);
    
    // 格式化平方根（如 sqrt(x) → √x）
    static std::string formatSquareRoot(const std::string& arg);
    
    // 格式化立方根（如 cbrt(x) → ³√x）
    static std::string formatCubeRoot(const std::string& arg);
    
    // 简化根式（如 2.8284271247 → 2√2）
    static std::string simplifyRadical(double value);

private:
    // 检查字符串是否为数字
    static bool isNumber(const std::string& str);
    
    // 查找匹配的括号
    static size_t findMatchingParen(const std::string& str, size_t start);
};

} // namespace pretty

#endif // UNICODE_FORMATTER_HPP