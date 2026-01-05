#include "pretty_output_config.hpp"
#include <cstdlib>
#include <iostream>
#include <string>
#include <algorithm>
#include <cctype>

namespace pretty {

PrettyConfig::PrettyConfig()
    : currentLevel_(PrettyLevel::ASCII)
    , supportsKitty_(false)
    , hasLaTeX_(false)
{
    detectTerminalCapabilities();
    detectLaTeXAvailability();
}

PrettyConfig& PrettyConfig::getInstance() {
    static PrettyConfig instance;
    return instance;
}

void PrettyConfig::setPrettyLevel(PrettyLevel level) {
    currentLevel_ = level;
}

PrettyLevel PrettyConfig::getPrettyLevel() const {
    return currentLevel_;
}

bool PrettyConfig::supportsKittyProtocol() const {
    return supportsKitty_;
}

bool PrettyConfig::isLaTeXAvailable() const {
    return hasLaTeX_;
}

std::string PrettyConfig::getPrompt() const {
    switch (currentLevel_) {
        case PrettyLevel::LATEX:
            return "∫ ";  // 积分符号
        case PrettyLevel::UNICODE:
            return "➜ ";  // 箭头符号
        case PrettyLevel::ASCII:
        default:
            return ">>> ";  // 原始提示符
    }
}

std::string PrettyConfig::levelToString(PrettyLevel level) {
    switch (level) {
        case PrettyLevel::ASCII:
            return "ASCII";
        case PrettyLevel::UNICODE:
            return "Unicode";
        case PrettyLevel::LATEX:
            return "LaTeX";
        default:
            return "Unknown";
    }
}

void PrettyConfig::detectTerminalCapabilities() {
    // 方法 1：检查 TERM_PROGRAM 环境变量
    const char* termProgram = std::getenv("TERM_PROGRAM");
    if (termProgram) {
        std::string termProg(termProgram);
        // 转换为小写进行比较
        std::transform(termProg.begin(), termProg.end(), termProg.begin(), ::tolower);
        if (termProg == "wezterm" || termProg == "kitty" || termProg == "iterm.app") {
            supportsKitty_ = true;
            return;
        }
    }
    
    // 方法 2：检查 TERM 环境变量
    const char* term = std::getenv("TERM");
    if (term) {
        std::string termStr(term);
        // 转换为小写进行比较
        std::transform(termStr.begin(), termStr.end(), termStr.begin(), ::tolower);
        if (termStr.find("kitty") != std::string::npos ||
            termStr.find("wezterm") != std::string::npos) {
            supportsKitty_ = true;
            return;
        }
    }
    
    // 方法 3：检查 KITTY_WINDOW_ID 环境变量（kitty 特定）
    const char* kittyWindowId = std::getenv("KITTY_WINDOW_ID");
    if (kittyWindowId) {
        supportsKitty_ = true;
        return;
    }
    
    // 默认不支持
    supportsKitty_ = false;
}

void PrettyConfig::detectLaTeXAvailability() {
    // 检查 xelatex 命令是否存在
#ifdef _WIN32
    // Windows 平台
    hasLaTeX_ = (system("where xelatex >nul 2>&1") == 0);
#else
    // Linux/macOS 平台
    hasLaTeX_ = (system("which xelatex > /dev/null 2>&1") == 0);
#endif
}

} // namespace pretty