#include "latex_renderer.hpp"
#include "pretty_output_config.hpp"
#include <fstream>
#include <sstream>
#include <filesystem>
#include <vector>
#include <cstdint>
#include <cmath>
#include <iomanip>
#include <algorithm>

namespace pretty {

int LatexRenderer::tempFileCounter_ = 0;

std::string LatexRenderer::generateLatexSource(const std::string& expr) {
    std::ostringstream oss;
    oss << "\\documentclass[preview]{standalone}\n";
    oss << "\\usepackage{amsmath}\n";
    oss << "\\usepackage{amssymb}\n";
    oss << "\\usepackage{unicode-math}\n";
    oss << "\\setmainfont{Latin Modern Math}\n";
    oss << "\\begin{document}\n";
    oss << "\\[ " << expr << " \\]\n";
    oss << "\\end{document}\n";
    return oss.str();
}

bool LatexRenderer::renderToImage(const std::string& latex, std::string& imagePath, int dpi) {
    // 检查 LaTeX 是否可用
    if (!isAvailable()) {
        return false;
    }
    
    // 创建临时文件
    std::string texFile = createTempFile(".tex");
    std::string pdfFile = texFile.substr(0, texFile.length() - 4) + ".pdf";
    std::string pngFile = texFile.substr(0, texFile.length() - 4) + ".png";
    
    // 写入 LaTeX 源码
    {
        std::ofstream out(texFile);
        out << latex;
        out.close();
    }
    
    // 编译为 PDF
    std::string compileCmd = "xelatex -interaction=nonstopmode -output-directory=" + 
                            std::filesystem::path(texFile).parent_path().string() + 
                            " " + texFile + " > /dev/null 2>&1";
    
    int result = system(compileCmd.c_str());
    
    if (result != 0) {
        cleanupTempFile(texFile);
        cleanupTempFile(pdfFile);
        return false;
    }
    
    // 转换 PDF 为 PNG
    // 使用 pdftoppm 替代 ImageMagick 的 convert
    std::string ppmFile = pdfFile.substr(0, pdfFile.length() - 4);
    std::string convertCmd = "pdftoppm -png -singlefile -r " + std::to_string(dpi) + " " + pdfFile + " " + ppmFile + " > /dev/null 2>&1";
    result = system(convertCmd.c_str());
    
    // 清理临时文件
    cleanupTempFile(texFile);
    cleanupTempFile(pdfFile);
    
    if (result != 0) {
        cleanupTempFile(pngFile);
        return false;
    }
    
    imagePath = pngFile;
    return true;
}

std::string LatexRenderer::encodeImageForKitty(const std::string& imagePath) {
    // 读取图片文件
    std::ifstream file(imagePath, std::ios::binary);
    if (!file) {
        return "";
    }
    
    std::vector<uint8_t> buffer((std::istreambuf_iterator<char>(file)), 
                                  std::istreambuf_iterator<char>());
    file.close();
    
    // Base64 编码
    std::string base64 = base64Encode(buffer);
    
    // 构造 Kitty 协议序列
    return "\033]1337;File=inline=1:" + base64 + "\033\\";
}

std::string LatexRenderer::renderExpression(const std::string& expr) {
    std::string latex = generateLatexSource(expr);
    std::string imagePath;
    
    if (!renderToImage(latex, imagePath, 400)) {
        // 渲染失败，返回 Unicode 格式
        return asciiToLatex(expr);
    }
    
    std::string kittyData = encodeImageForKitty(imagePath);
    cleanupTempFile(imagePath);
    
    return kittyData;
}

std::string LatexRenderer::renderComplexCode(const ComplexNumber& cn) {
    // 将复数转换为 LaTeX 格式
    std::string latex = complexToLatex(cn);
    
    // 返回 LaTeX 代码文本
    return "\\[" + latex + "\\]";
}

std::string LatexRenderer::renderComplex(const ComplexNumber& cn, bool tryRender) {
    // 将复数转换为 LaTeX 格式
    std::string latex = complexToLatex(cn);
    
    if (tryRender) {
        // 尝试渲染为图片
        std::ostringstream oss;
        oss << "\\documentclass[preview, fontsize=14pt]{standalone}\n";
        oss << "\\usepackage{amsmath}\n";
        oss << "\\usepackage{amssymb}\n";
        oss << "\\usepackage{graphicx}\n";
        oss << "\\begin{document}\n";
        // 根据内容长度计算缩放因子
        double scale = 1.0;
        int latexLen = latex.length();
        if (latexLen > 50) scale = 0.7;
        else if (latexLen > 30) scale = 0.85;
        else if (latexLen > 20) scale = 1.0;
        else scale = 1.2;
        // 将 scalebox 包裹整个数学块
        oss << "\\scalebox{" << scale << "}[1.0]{$\\displaystyle " << latex << "$}\n";
        oss << "\\end{document}\n";
        std::string latexSource = oss.str();
        
        std::string imagePath;
        if (renderToImage(latexSource, imagePath, 400)) {
            std::string kittyData = encodeImageForKitty(imagePath);
            cleanupTempFile(imagePath);
            return kittyData;
        }
    }
    
    // 渲染失败或不尝试渲染，返回 LaTeX 代码文本
    return "\\[" + latex + "\\]";
}

std::string LatexRenderer::renderEquation(const std::string& lhs, const std::string& rhs) {
    std::string expr = lhs + " = " + rhs;
    return renderExpression(expr);
}

std::string LatexRenderer::renderSolution(const std::string& var, const std::string& value) {
    std::string expr = var + " = " + value;
    return renderExpression(expr);
}

bool LatexRenderer::isAvailable() {
    return PrettyConfig::getInstance().isLaTeXAvailable();
}

std::string LatexRenderer::createTempFile(const std::string& suffix) {
    std::string tempDir = "/tmp";
    const char* tmpDirEnv = std::getenv("TMPDIR");
    if (tmpDirEnv) {
        tempDir = tmpDirEnv;
    }
    
    std::string filename = "calc_latex_" + std::to_string(tempFileCounter_++) + suffix;
    return tempDir + "/" + filename;
}

void LatexRenderer::cleanupTempFile(const std::string& path) {
    try {
        if (std::filesystem::exists(path)) {
            std::filesystem::remove(path);
        }
    } catch (...) {
        // 忽略删除错误
    }
}

std::string LatexRenderer::base64Encode(const std::vector<uint8_t>& data) {
    static const std::string base64_chars = 
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    
    std::string result;
    int i = 0;
    unsigned char char_array_3[3];
    unsigned char char_array_4[4];
    
    size_t in_len = data.size();
    size_t pos = 0;
    
    while (in_len--) {
        char_array_3[i++] = data[pos++];
        if (i == 3) {
            char_array_4[0] = (char_array_3[0] & 0xfc) >> 2;
            char_array_4[1] = ((char_array_3[0] & 0x03) << 4) + ((char_array_3[1] & 0xf0) >> 4);
            char_array_4[2] = ((char_array_3[1] & 0x0f) << 2) + ((char_array_3[2] & 0xc0) >> 6);
            char_array_4[3] = char_array_3[2] & 0x3f;
            
            for (i = 0; i < 4; i++) {
                result += base64_chars[char_array_4[i]];
            }
            i = 0;
        }
    }
    
    if (i) {
        for (int j = i; j < 3; j++) {
            char_array_3[j] = '\0';
        }
        
        char_array_4[0] = (char_array_3[0] & 0xfc) >> 2;
        char_array_4[1] = ((char_array_3[0] & 0x03) << 4) + ((char_array_3[1] & 0xf0) >> 4);
        char_array_4[2] = ((char_array_3[1] & 0x0f) << 2) + ((char_array_3[2] & 0xc0) >> 6);
        char_array_4[3] = char_array_3[2] & 0x3f;
        
        for (int j = 0; j < i + 1; j++) {
            result += base64_chars[char_array_4[j]];
        }
        
        while (i++ < 3) {
            result += '=';
        }
    }
    
    return result;
}

std::string LatexRenderer::asciiToLatex(const std::string& expr) {
    std::string result;
    size_t i = 0;
    
    while (i < expr.length()) {
        // 处理 sqrt()
        if (i + 4 <= expr.length() && expr.substr(i, 4) == "sqrt") {
            i += 4;
            if (i < expr.length() && expr[i] == '(') {
                size_t end = i + 1;
                int depth = 1;
                while (end < expr.length() && depth > 0) {
                    if (expr[end] == '(') depth++;
                    else if (expr[end] == ')') depth--;
                    end++;
                }
                std::string arg = expr.substr(i + 1, end - i - 2);
                result += "\\sqrt{" + asciiToLatex(arg) + "}";
                i = end;
            } else {
                result += "\\sqrt{";
            }
        }
        // 处理 cbrt()
        else if (i + 4 <= expr.length() && expr.substr(i, 4) == "cbrt") {
            i += 4;
            if (i < expr.length() && expr[i] == '(') {
                size_t end = i + 1;
                int depth = 1;
                while (end < expr.length() && depth > 0) {
                    if (expr[end] == '(') depth++;
                    else if (expr[end] == ')') depth--;
                    end++;
                }
                std::string arg = expr.substr(i + 1, end - i - 2);
                result += "\\sqrt[3]{" + asciiToLatex(arg) + "}";
                i = end;
            } else {
                result += "\\sqrt[3]{";
            }
        }
        // 处理 pi
        else if (i + 2 <= expr.length() && expr.substr(i, 2) == "pi") {
            result += "\\pi";
            i += 2;
        }
        // 处理幂运算
        else if (expr[i] == '^') {
            result += "^{";
            i++;
            // 跳过空格
            while (i < expr.length() && expr[i] == ' ') i++;
            // 读取指数
            while (i < expr.length() && expr[i] != ' ' && expr[i] != ')' && expr[i] != ',') {
                result += expr[i];
                i++;
            }
            result += "}";
        }
        // 处理分数（如 1/2）
        else if (expr[i] == '/' && i > 0 && i + 1 < expr.length()) {
            // 查找前面的数字
            size_t numStart = i - 1;
            while (numStart > 0 && (std::isdigit(expr[numStart - 1]) || expr[numStart - 1] == '.')) {
                numStart--;
            }
            std::string numerator = expr.substr(numStart, i - numStart);
            
            // 查找后面的数字
            size_t denEnd = i + 1;
            while (denEnd < expr.length() && (std::isdigit(expr[denEnd]) || expr[denEnd] == '.')) {
                denEnd++;
            }
            std::string denominator = expr.substr(i + 1, denEnd - i - 1);
            
            result = result.substr(0, numStart);
            result += "\\frac{" + numerator + "}{" + denominator + "}";
            i = denEnd;
        }
        // 处理下标（如 x1, x2）
        else if (i > 0 && expr[i] >= '1' && expr[i] <= '9' && 
                 (expr[i-1] == 'x' || expr[i-1] == 'y' || expr[i-1] == 'z')) {
            result += "_{" + std::string(1, expr[i]) + "}";
            i++;
        }
        // 处理虚数单位
        else if (expr[i] == 'i' && 
                 (i == 0 || !std::isalpha(expr[i-1])) && 
                 (i + 1 >= expr.length() || !std::isalpha(expr[i+1]))) {
            result += "i";
            i++;
        }
        // 其他字符
        else {
            result += expr[i];
            i++;
        }
    }
    
    return result;
}

std::string LatexRenderer::complexToLatex(const ComplexNumber& cn) {
    const double epsilon = 1e-9;
    bool realZero = std::fabs(cn.real) < epsilon;
    bool imagZero = std::fabs(cn.imag) < epsilon;

    if (realZero && imagZero) return "0";
    if (imagZero) return numberToLatex(cn.real);
    if (realZero) {
        double imagMag = std::fabs(cn.imag);
        if (std::fabs(imagMag - 1.0) < epsilon) {
            return cn.imag < 0 ? "-i" : "i";
        }
        return numberToLatex(cn.imag) + "i";
    }

    std::string realStr = numberToLatex(cn.real);
    std::string imagStr = numberToLatex(std::fabs(cn.imag));
    std::string sign = cn.imag >= 0 ? "+" : "-";
    
    if (std::fabs(std::fabs(cn.imag) - 1.0) < epsilon) {
        imagStr = "";
    }
    
    return realStr + " " + sign + " " + (imagStr.empty() ? "i" : imagStr + "i");
}

std::string LatexRenderer::numberToLatex(double value) {
    // 检查是否为整数
    double roundedInt = std::round(value);
    if (std::fabs(value - roundedInt) < 1e-9) {
        return std::to_string(static_cast<long long>(roundedInt));
    }
    
    // 检查是否可以表示为根式（优先级高于分数）
    std::string radical = simplifyRadical(value);
    if (!radical.empty()) {
        return radical;
    }
    
    // 检查是否为简单分数
    Fraction frac = Fraction::fromDouble(value);
    double fracValue = static_cast<double>(frac.numerator) / frac.denominator;
    if (std::fabs(value - fracValue) < 1e-9) {
        if (frac.denominator == 1) {
            return std::to_string(frac.numerator);
        }
        return "\\frac{" + std::to_string(frac.numerator) + "}{" + std::to_string(frac.denominator) + "}";
    }
    
    // 小数表示
    std::ostringstream oss;
    oss << std::fixed << std::setprecision(10) << value;
    std::string str = oss.str();
    while (!str.empty() && str.back() == '0') {
        str.pop_back();
    }
    if (!str.empty() && str.back() == '.') {
        str.pop_back();
    }
    return str;
}

std::string LatexRenderer::simplifyRadical(double value) {
    const double epsilon = 1e-9;
    
    // 处理负数
    bool isNegative = value < 0;
    double absValue = std::fabs(value);
    
    // 尝试找到整数 n 和 m，使得 absValue = n * sqrt(m)
    for (int m = 2; m <= 100; ++m) {
        double sqrtM = std::sqrt(m);
        if (std::fabs(sqrtM * sqrtM - m) > epsilon) continue;
        
        double n = absValue / sqrtM;
        double nRounded = std::round(n);
        
        if (std::fabs(n - nRounded) < epsilon) {
            int nInt = static_cast<int>(nRounded);
            std::string result;
            
            if (isNegative) {
                if (nInt == 1) {
                    result = "-\\sqrt{" + std::to_string(m) + "}";
                } else {
                    result = "-" + std::to_string(nInt) + "\\sqrt{" + std::to_string(m) + "}";
                }
            } else {
                if (nInt == 1) {
                    result = "\\sqrt{" + std::to_string(m) + "}";
                } else {
                    result = std::to_string(nInt) + "\\sqrt{" + std::to_string(m) + "}";
                }
            }
            
            return result;
        }
    }
    
    return "";
}

} // namespace pretty