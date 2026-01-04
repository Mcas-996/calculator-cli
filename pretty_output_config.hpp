#ifndef PRETTY_OUTPUT_CONFIG_HPP
#define PRETTY_OUTPUT_CONFIG_HPP

#include <cstdlib>
#include <iostream>
#include <string>

namespace pretty {

// 美化级别枚举
enum class PrettyLevel {
    ASCII,      // 仅 ASCII 符号
    UNICODE,    // Unicode 数学符号
    LATEX       // LaTeX 渲染（通过 Kitty 协议）
};

// 配置管理器（单例模式）
class PrettyConfig {
public:
    // 获取单例实例
    static PrettyConfig& getInstance();
    
    // 禁止拷贝和移动
    PrettyConfig(const PrettyConfig&) = delete;
    PrettyConfig& operator=(const PrettyConfig&) = delete;
    PrettyConfig(PrettyConfig&&) = delete;
    PrettyConfig& operator=(PrettyConfig&&) = delete;
    
    // 设置美化级别
    void setPrettyLevel(PrettyLevel level);
    
    // 获取当前美化级别
    PrettyLevel getPrettyLevel() const;
    
    // 检查是否支持 Kitty 协议
    bool supportsKittyProtocol() const;
    
    // 检查 LaTeX 是否可用
    bool isLaTeXAvailable() const;
    
    // 获取提示符
    std::string getPrompt() const;
    
    // 获取美化级别名称
    static std::string levelToString(PrettyLevel level);

private:
    PrettyConfig();  // 私有构造函数
    
    // 检测终端能力
    void detectTerminalCapabilities();
    
    // 检测 LaTeX 可用性
    void detectLaTeXAvailability();
    
    PrettyLevel currentLevel_;  // 当前美化级别
    bool supportsKitty_;        // 是否支持 Kitty 协议
    bool hasLaTeX_;             // 是否有 LaTeX
};

} // namespace pretty

#endif // PRETTY_OUTPUT_CONFIG_HPP