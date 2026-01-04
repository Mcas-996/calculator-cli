#include "pretty_output.hpp"
#include <string>
#include <vector>
#include <utility>

namespace pretty {

PrettyConfig& PrettyOutput::config() {
    return PrettyConfig::getInstance();
}

std::string PrettyOutput::format(const ComplexNumber& cn) {
    PrettyLevel level = config().getPrettyLevel();
    
    switch (level) {
        case PrettyLevel::LATEX:
            if (config().supportsKittyProtocol() && config().isLaTeXAvailable()) {
                return LatexRenderer::renderComplex(cn);
            }
            // 降级到 Unicode
            [[fallthrough]];
        case PrettyLevel::UNICODE:
            return UnicodeFormatter::formatComplex(cn);
        case PrettyLevel::ASCII:
        default:
            return cn.toString();
    }
}

std::string PrettyOutput::format(const Fraction& frac) {
    PrettyLevel level = config().getPrettyLevel();
    
    switch (level) {
        case PrettyLevel::LATEX:
            // LaTeX 模式下也使用 Unicode 格式化（分数不需要图片渲染）
            [[fallthrough]];
        case PrettyLevel::UNICODE:
            return UnicodeFormatter::formatFraction(frac);
        case PrettyLevel::ASCII:
        default:
            return frac.toString();
    }
}

std::string PrettyOutput::format(const std::string& expr) {
    PrettyLevel level = config().getPrettyLevel();
    
    switch (level) {
        case PrettyLevel::LATEX:
            if (config().supportsKittyProtocol() && config().isLaTeXAvailable()) {
                return LatexRenderer::renderExpression(expr);
            }
            // 降级到 Unicode
            [[fallthrough]];
        case PrettyLevel::UNICODE:
            return UnicodeFormatter::formatExpression(expr);
        case PrettyLevel::ASCII:
        default:
            return expr;
    }
}

std::string PrettyOutput::formatOperator(char op) {
    PrettyLevel level = config().getPrettyLevel();
    
    if (level == PrettyLevel::ASCII) {
        return std::string(1, op);
    }
    
    return UnicodeFormatter::formatOperator(op);
}

std::string PrettyOutput::formatFunction(const std::string& func) {
    PrettyLevel level = config().getPrettyLevel();
    
    if (level == PrettyLevel::ASCII) {
        return func;
    }
    
    return UnicodeFormatter::formatFunction(func);
}

std::string PrettyOutput::formatConstant(const std::string& constant) {
    PrettyLevel level = config().getPrettyLevel();
    
    if (level == PrettyLevel::ASCII) {
        return constant;
    }
    
    return UnicodeFormatter::formatConstant(constant);
}

std::string PrettyOutput::formatPrompt() {
    return config().getPrompt();
}

std::string PrettyOutput::formatEquationSolution(const std::string& var, const std::string& value) {
    PrettyLevel level = config().getPrettyLevel();
    
    std::string formattedVar = var;
    std::string formattedValue = value;
    
    if (level != PrettyLevel::ASCII) {
        // 格式化变量名（如 x1 → x₁）
        if (var.length() > 1 && var[0] >= 'x' && var[0] <= 'z') {
            formattedVar = std::string(1, var[0]);
            for (size_t i = 1; i < var.length(); ++i) {
                if (var[i] >= '0' && var[i] <= '9') {
                    formattedVar += formatSubscript(var[i] - '0');
                } else {
                    formattedVar += var[i];
                }
            }
        }
        
        // 格式化值
        formattedValue = applyUnicodeFormatting(value);
    }
    
    std::string result = formattedVar + " = " + formattedValue;
    
    // LaTeX 模式下渲染整个表达式
    if (level == PrettyLevel::LATEX && 
        config().supportsKittyProtocol() && 
        config().isLaTeXAvailable()) {
        return LatexRenderer::renderEquation(formattedVar, formattedValue);
    }
    
    return result;
}

std::string PrettyOutput::formatQuadraticRoots(const std::string& root1, const std::string& root2) {
    if (root1 == root2) {
        return formatEquationSolution("x", root1);
    }
    
    std::string r1 = applyUnicodeFormatting(root1);
    std::string r2 = applyUnicodeFormatting(root2);
    
    std::string var1 = "x" + formatSubscript(1);
    std::string var2 = "x" + formatSubscript(2);
    
    std::string result = var1 + " = " + r1 + ", " + var2 + " = " + r2;
    
    PrettyLevel level = config().getPrettyLevel();
    if (level == PrettyLevel::LATEX && 
        config().supportsKittyProtocol() && 
        config().isLaTeXAvailable()) {
        // LaTeX 模式下，渲染为 x₁ = r₁, x₂ = r₂
        return LatexRenderer::renderExpression(var1 + " = " + r1 + ", " + var2 + " = " + r2);
    }
    
    return result;
}

std::string PrettyOutput::formatCubicRoots(const std::string& root1, 
                                           const std::string& root2, 
                                           const std::string& root3) {
    std::string r1 = applyUnicodeFormatting(root1);
    std::string r2 = applyUnicodeFormatting(root2);
    std::string r3 = applyUnicodeFormatting(root3);
    
    std::string var1 = "x" + formatSubscript(1);
    std::string var2 = "x" + formatSubscript(2);
    std::string var3 = "x" + formatSubscript(3);
    
    std::string result = var1 + " = " + r1 + ", " + var2 + " = " + r2 + ", " + var3 + " = " + r3;
    
    PrettyLevel level = config().getPrettyLevel();
    if (level == PrettyLevel::LATEX && 
        config().supportsKittyProtocol() && 
        config().isLaTeXAvailable()) {
        return LatexRenderer::renderExpression(result);
    }
    
    return result;
}

std::string PrettyOutput::formatQuarticRoots(const std::vector<std::string>& roots) {
    std::string result;
    for (size_t i = 0; i < roots.size(); ++i) {
        if (i > 0) {
            result += ", ";
        }
        std::string var = "x" + formatSubscript(static_cast<int>(i + 1));
        std::string value = applyUnicodeFormatting(roots[i]);
        result += var + " = " + value;
    }
    
    PrettyLevel level = config().getPrettyLevel();
    if (level == PrettyLevel::LATEX && 
        config().supportsKittyProtocol() && 
        config().isLaTeXAvailable()) {
        return LatexRenderer::renderExpression(result);
    }
    
    return result;
}

std::string PrettyOutput::formatQuinticRoots(const std::vector<std::string>& roots) {
    std::string result;
    for (size_t i = 0; i < roots.size(); ++i) {
        if (i > 0) {
            result += ", ";
        }
        std::string var = "x" + formatSubscript(static_cast<int>(i + 1));
        std::string value = applyUnicodeFormatting(roots[i]);
        result += var + " = " + value;
    }
    
    PrettyLevel level = config().getPrettyLevel();
    if (level == PrettyLevel::LATEX && 
        config().supportsKittyProtocol() && 
        config().isLaTeXAvailable()) {
        return LatexRenderer::renderExpression(result);
    }
    
    return result;
}

std::string PrettyOutput::formatLinearSystem(const std::vector<std::pair<std::string, std::string>>& solutions) {
    std::string result;
    for (size_t i = 0; i < solutions.size(); ++i) {
        if (i > 0) {
            result += ", ";
        }
        std::string var = solutions[i].first;
        std::string value = applyUnicodeFormatting(solutions[i].second);
        result += var + " = " + value;
    }
    
    PrettyLevel level = config().getPrettyLevel();
    if (level == PrettyLevel::LATEX && 
        config().supportsKittyProtocol() && 
        config().isLaTeXAvailable()) {
        return LatexRenderer::renderExpression(result);
    }
    
    return result;
}

std::string PrettyOutput::formatSubscript(int index) {
    PrettyLevel level = config().getPrettyLevel();
    
    if (level == PrettyLevel::ASCII) {
        return std::to_string(index);
    }
    
    return UnicodeFormatter::formatSubscript(index);
}

std::string PrettyOutput::formatSuperscript(int index) {
    PrettyLevel level = config().getPrettyLevel();
    
    if (level == PrettyLevel::ASCII) {
        return "^" + std::to_string(index);
    }
    
    return UnicodeFormatter::formatSuperscript(index);
}

std::string PrettyOutput::formatSquareRoot(const std::string& arg) {
    PrettyLevel level = config().getPrettyLevel();
    
    if (level == PrettyLevel::ASCII) {
        return "sqrt(" + arg + ")";
    }
    
    return UnicodeFormatter::formatSquareRoot(arg);
}

std::string PrettyOutput::formatCubeRoot(const std::string& arg) {
    PrettyLevel level = config().getPrettyLevel();
    
    if (level == PrettyLevel::ASCII) {
        return "cbrt(" + arg + ")";
    }
    
    return UnicodeFormatter::formatCubeRoot(arg);
}

std::string PrettyOutput::formatPower(const std::string& base, int exp) {
    PrettyLevel level = config().getPrettyLevel();
    
    if (level == PrettyLevel::ASCII) {
        return base + "^" + std::to_string(exp);
    }
    
    return UnicodeFormatter::formatPower(base, exp);
}

std::string PrettyOutput::applyUnicodeFormatting(const std::string& str) {
    return UnicodeFormatter::formatExpression(str);
}

std::string PrettyOutput::applyLaTeXFormatting(const std::string& str) {
    if (config().supportsKittyProtocol() && config().isLaTeXAvailable()) {
        return LatexRenderer::renderExpression(str);
    }
    return applyUnicodeFormatting(str);
}

} // namespace pretty