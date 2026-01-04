#include "latex_renderer.hpp"
#include "pretty_output_config.hpp"
#include <fstream>
#include <sstream>
#include <filesystem>
#include <vector>
#include <cstdint>

namespace pretty {

int LatexRenderer::tempFileCounter_ = 0;

std::string LatexRenderer::generateLatexSource(const std::string& expr) {
    std::ostringstream oss;
    oss << "\\documentclass[preview]{standalone}\n";
    oss << "\\usepackage{amsmath}\n";
    oss << "\\usepackage{amssymb}\n";
    oss << "\\begin{document}\n";
    oss << "\\[ " << asciiToLatex(expr) << " \\]\n";
    oss << "\\end{document}\n";
    return oss.str();
}

bool LatexRenderer::renderToImage(const std::string& latex, std::string& imagePath) {
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
    std::string compileCmd = "pdflatex -interaction=nonstopmode -output-directory=" + 
                            std::filesystem::path(texFile).parent_path().string() + 
                            " " + texFile + " > /dev/null 2>&1";
    
    int result = system(compileCmd.c_str());
    
    if (result != 0) {
        cleanupTempFile(texFile);
        cleanupTempFile(pdfFile);
        return false;
    }
    
    // 转换 PDF 为 PNG
    std::string convertCmd = "convert -density 300 " + pdfFile + " -quality 100 " + pngFile + " > /dev/null 2>&1";
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
    
    if (!renderToImage(latex, imagePath)) {
        // 渲染失败，返回 Unicode 格式
        return asciiToLatex(expr);
    }
    
    std::string kittyData = encodeImageForKitty(imagePath);
    cleanupTempFile(imagePath);
    
    return kittyData;
}

std::string LatexRenderer::renderComplex(const ComplexNumber& cn) {
    // 将复数转换为字符串，然后渲染
    std::string expr = cn.toString();
    return renderExpression(expr);
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

} // namespace pretty