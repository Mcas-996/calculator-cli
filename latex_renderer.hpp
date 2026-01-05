#ifndef LATEX_RENDERER_HPP
#define LATEX_RENDERER_HPP

#include <cstdint>
#include <fstream>
#include <sstream>
#include <string>
#include <vector>

#include "complex_number.hpp"
#include "fractions.hpp"

namespace pretty {

// LaTeX 渲染器
class LatexRenderer {
public:
    // 生成 LaTeX 源码
    static std::string generateLatexSource(const std::string& expr);
    
    // 渲染为图片文件
    static bool renderToImage(const std::string& latex, std::string& imagePath, int dpi = 300);
    
    // 编码图片为 Kitty 协议格式
    static std::string encodeImageForKitty(const std::string& imagePath);
    
    // 渲染表达式（返回 Kitty 协议字符串）
    static std::string renderExpression(const std::string& expr);
    
    // 渲染复数
    static std::string renderComplex(const ComplexNumber& cn, bool tryRender = false);
    
    // 渲染复数为 LaTeX 代码文本（不尝试渲染）
    static std::string renderComplexCode(const ComplexNumber& cn);
    
    // 渲染方程（lhs = rhs）
    static std::string renderEquation(const std::string& lhs, const std::string& rhs);
    
    // 渲染方程解（x = value）
    static std::string renderSolution(const std::string& var, const std::string& value);
    
    // 检查是否可用
    static bool isAvailable();

private:
    // 创建临时文件
    static std::string createTempFile(const std::string& suffix);
    
    // 清理临时文件
    static void cleanupTempFile(const std::string& path);
    
    // Base64 编码
    static std::string base64Encode(const std::vector<uint8_t>& data);
    
    // 将 ASCII 表达式转换为 LaTeX 格式
    static std::string asciiToLatex(const std::string& expr);
    
    // 将复数转换为 LaTeX 格式
    static std::string complexToLatex(const ComplexNumber& cn);
    
    // 将数字转换为 LaTeX 格式
    static std::string numberToLatex(double value);
    
    // 简化根式
    static std::string simplifyRadical(double value);
    
    // 临时文件计数器
    static int tempFileCounter_;
};

} // namespace pretty

#endif // LATEX_RENDERER_HPP