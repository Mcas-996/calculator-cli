# LaTeX/Unicode 美化输出实现计划

## 目标
为计算器 CLI 添加数学符号美化输出功能，支持 Unicode 符号和 LaTeX 渲染（通过 Kitty 协议）。

## 实现策略
采用分层美化策略：ASCII → Unicode → LaTeX，自动检测终端能力并选择最佳格式。

## 需求确认
- **美化级别**：支持 Unicode 和 LaTeX 两种模式
- **启用方式**：命令行选项 `--pretty/-p`, `--unicode/-u`, `--ascii/-a`
- **美化范围**：输入提示符和输出都美化
- **方程显示**：LaTeX 风格（带下标，如 x₁ = 2）

## 实现步骤

### 步骤 1：创建配置管理器
**文件**：
- `pretty_output_config.hpp`
- `pretty_output_config.cpp`

**功能**：
- 管理美化级别（ASCII/Unicode/LaTeX）
- 终端能力检测（Kitty 协议支持）
- LaTeX 可用性检测

**关键类**：
```cpp
enum class PrettyLevel { ASCII, UNICODE, LATEX };

class PrettyConfig {
    static PrettyConfig& getInstance();
    void setPrettyLevel(PrettyLevel level);
    PrettyLevel getPrettyLevel() const;
    bool supportsKittyProtocol() const;
    bool isLaTeXAvailable() const;
    std::string getPrompt() const;
};
```

### 步骤 2：创建 Unicode 格式化器
**文件**：
- `unicode_formatter.hpp`
- `unicode_formatter.cpp`

**功能**：
- ASCII 到 Unicode 符号映射
- 数学表达式格式化
- 支持下标、上标、根号、分数

**符号映射**：
- `×`, `÷`, `π`, `√`
- `²`, `³` (上标)
- `₀`, `₁`, `₂` (下标)

### 步骤 3：创建 LaTeX 渲染器
**文件**：
- `latex_renderer.hpp`
- `latex_renderer.cpp`

**功能**：
- LaTeX 源码生成
- 调用 pdflatex 生成 PNG
- 通过 Kitty 协议显示图片

**渲染流程**：
1. 生成 LaTeX 源码
2. 编译为 PDF → PNG
3. Base64 编码
4. 通过 Kitty 协议显示

### 步骤 4：创建统一美化接口
**文件**：
- `pretty_output.hpp`
- `pretty_output.cpp`

**功能**：
- 根据配置自动选择输出格式
- 提供统一的格式化接口
- 处理不同类型数据的格式化

**关键方法**：
```cpp
class PrettyOutput {
    static std::string format(const ComplexNumber& cn);
    static std::string format(const Fraction& frac);
    static std::string format(const std::string& expr);
    static std::string formatPrompt();
    static std::string formatEquationSolution(const std::string& var, const std::string& value);
};
```

### 步骤 5：扩展命令行参数
**文件**：`main_cli.cpp`

**新增选项**：
- `--pretty/-p`：启用美化（自动选择最佳格式）
- `--unicode/-u`：强制 Unicode 模式
- `--ascii/-a`：强制 ASCII 模式

### 步骤 6：更新核心类
**文件**：
- `complex_number.hpp`
- `fractions.hpp`

**修改**：
- 添加 `toPrettyString()` 方法
- 保留 `toString()` 向后兼容

### 步骤 7：更新输出格式化
**文件**：`string_processing.cpp`

**修改点**：
- `formatCoefficient()` - 使用 PrettyOutput
- `polynomialToString()` - Unicode 符号
- `formatNumericRoots()` - 下标格式
- `formatSymbolicOutput()` - LaTeX 风格

### 步骤 8：更新交互式提示符
**文件**：`main_cli.cpp`

**修改**：
- 使用美化提示符（如 `➜ ` 或 `∫ `）
- 根据配置动态选择

### 步骤 9：更新构建系统
**文件**：`CMakeLists.txt`

**修改**：
- 添加新源文件到构建
- 添加 LaTeX 依赖检查（可选）

### 步骤 10：编写测试
**文件**：`pretty_output_tests.cpp`

**测试内容**：
- 配置管理器功能
- Unicode 格式化正确性
- LaTeX 渲染（可选）
- 统一接口功能

## 关键文件清单

### 新建文件
1. `pretty_output_config.hpp/cpp` - 配置管理器
2. `unicode_formatter.hpp/cpp` - Unicode 格式化器
3. `latex_renderer.hpp/cpp` - LaTeX 渲染器
4. `pretty_output.hpp/cpp` - 统一美化接口
5. `pretty_output_tests.cpp` - 单元测试

### 修改文件
1. `main_cli.cpp` - 命令行参数和提示符
2. `complex_number.hpp` - 添加美化输出方法
3. `fractions.hpp` - 添加美化输出方法
4. `string_processing.cpp` - 更新输出格式化
5. `CMakeLists.txt` - 添加新源文件

## 终端检测方案

### Kitty 协议检测
```cpp
bool detectKittySupport() {
    // 检查 TERM_PROGRAM=wezterm
    // 检查 TERM 包含 "kitty"
    // 可选：发送查询序列
}
```

### LaTeX 检测
```cpp
bool detectLaTeXAvailability() {
    // 检查 pdflatex 命令是否存在
    return system("which pdflatex > /dev/null 2>&1") == 0;
}
```

## 降级策略
1. LaTeX 不可用 → Unicode
2. Kitty 协议不支持 → Unicode
3. Unicode 显示异常 → ASCII

## 成功标准
- ✅ 命令行参数正确配置美化级别
- ✅ Unicode 符号正确显示
- ✅ LaTeX 渲染在支持的终端中工作
- ✅ 交互式提示符美化
- ✅ 方程结果使用 LaTeX 风格
- ✅ 自动检测终端能力
- ✅ 降级策略正常工作
- ✅ 跨平台兼容（Linux/macOS/Windows）
- ✅ 向后兼容

## 实现顺序
1. 步骤 1-4：核心美化功能
2. 步骤 5-8：集成到现有代码
3. 步骤 9-10：构建和测试