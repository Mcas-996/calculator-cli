#ifndef PRETTY_OUTPUT_HPP
#define PRETTY_OUTPUT_HPP

#include <string>
#include <vector>
#include <utility>

#include "complex_number.hpp"
#include "fractions.hpp"
#include "pretty_output_config.hpp"
#include "unicode_formatter.hpp"
#include "latex_renderer.hpp"

namespace pretty {

// 统一美化输出接口
class PrettyOutput {
public:
    // 格式化复数
    static std::string format(const ComplexNumber& cn);
    
    // 格式化分数
    static std::string format(const Fraction& frac);
    
    // 格式化表达式字符串
    static std::string format(const std::string& expr);
    
    // 格式化运算符
    static std::string formatOperator(char op);
    
    // 格式化函数名
    static std::string formatFunction(const std::string& func);
    
    // 格式化常量
    static std::string formatConstant(const std::string& constant);
    
    // 格式化提示符
    static std::string formatPrompt();
    
    // 格式化方程解（x = value）
    static std::string formatEquationSolution(const std::string& var, const std::string& value);
    
    // 格式化二次方程根（x1 = value1, x2 = value2）
    static std::string formatQuadraticRoots(const std::string& root1, const std::string& root2);
    
    // 格式化三次方程根（x1 = v1, x2 = v2, x3 = v3）
    static std::string formatCubicRoots(const std::string& root1, 
                                         const std::string& root2, 
                                         const std::string& root3);
    
    // 格式化四次方程根
    static std::string formatQuarticRoots(const std::vector<std::string>& roots);
    
    // 格式化五次方程根
    static std::string formatQuinticRoots(const std::vector<std::string>& roots);
    
    // 格式化线性方程组解
    static std::string formatLinearSystem(const std::vector<std::pair<std::string, std::string>>& solutions);
    
    // 格式化下标索引（1 → ₁）
    static std::string formatSubscript(int index);
    
    // 格式化上标指数（2 → ²）
    static std::string formatSuperscript(int index);
    
    // 格式化平方根
    static std::string formatSquareRoot(const std::string& arg);
    
    // 格式化立方根
    static std::string formatCubeRoot(const std::string& arg);
    
    // 格式化幂运算
    static std::string formatPower(const std::string& base, int exp);
    
    // 应用 Unicode 格式化
    static std::string applyUnicodeFormatting(const std::string& str);
    
    // 应用 LaTeX 格式化
    static std::string applyLaTeXFormatting(const std::string& str);

private:
    // 获取当前配置实例
    static PrettyConfig& config();
};

} // namespace pretty

#endif // PRETTY_OUTPUT_HPP