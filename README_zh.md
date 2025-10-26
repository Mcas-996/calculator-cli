# 计算器命令行工具

一个用 C++20 编写的轻量级命令行计算器，支持实/复数混合运算、方程求解以及小型线性方程组处理。

## 快速上手

1. 克隆仓库：`git clone https://github.com/allen/calculator-cli && cd calculator-cli`。
2. 配置并编译 Release 版本：`cmake -B build -S . -DCMAKE_BUILD_TYPE=Release && cmake --build build --parallel`。
3. 立即体验：`./build/calculator "equation(x^2-5x+6=0)"`。

## 功能特性

### 表达式引擎
- 加、减、乘、除、乘方和括号。
- 百分号（`50% * 200`）、一元负号。
- 常量 `pi`、`e`、虚数单位 `i`，自动解析小数或分数（结果尽量以最简分数输出）。
- `sqrt()`、`abs()`、弧度制三角函数 `sin()/cos()`，以及角度制的 `sind()/cosd()`。
- 完整的复数运算，例如 `sqrt(-4)`、`(3+2i)*(1-i)`、`cosd(60+i)`。

### 方程求解
- 一元一次方程：`equation(2x+5=0)`
- 一元二次方程：`equation(x^2-5x+6=0)`（自动给出实数或复数根）
- 一元三次方程：`equation(x^3-6x^2+11x-6=0)`
- 线性方程组（最多 3 个变量）：`equation2(x+y=5,x-y=1)`

### 输出格式
- 若结果可表示为有理数则输出分数，否则回退到带指定精度的小数。
- 复数固定输出为 `a + bi` 形式，并根据系数简化为 `i` 或 `-i`。

## 使用方法

```bash
# 直接在命令行传入表达式
./calculator "3 + 5 * (2 - 8)^2"

# 复数
./calculator "(3+2i) * (1 - i)"
./calculator "sqrt(-9)"         # -> 3i

# 三角函数
./calculator "sin(pi / 6)"      # 弧度
./calculator "sind(30)"         # 角度

# 方程
./calculator "equation(x^2-5x+6=0)"
./calculator "equation2(x+y=5,x-y=1)"
```

`--help`/`--version` 可查看帮助和版本信息。若未提供参数，程序会提示正确用法。

## 构建

推荐使用 CMake（≥3.10）及支持 C++20 的编译器：

```bash
cmake -B build -S . -DCMAKE_BUILD_TYPE=Release
cmake --build build --parallel
ctest --output-on-failure --test-dir build   # 可选，运行 calculator_tests
```

- Windows：可使用 MSVC/clang-cl，必要时添加 `-A x64` 选择架构。
- macOS / Linux：同样的命令即可，确保安装 `cmake` 与 `g++` 或 `clang++`。
- 仍保留 `build_*.sh/.bat` 脚本，但以上 CMake 流程为主。

## 测试

`ctest` 会运行 `calculator_tests.cpp` 中的全部回归用例。构建完成后执行 `ctest --output-on-failure --test-dir build` 即可覆盖实数、复数以及符号路径；调试特定用例时可以使用 `ctest -R <name>`。

## macOS Gatekeeper

CI 产出的未签名二进制在 macOS 可能触发安全警告，可执行：

```bash
xattr -d com.apple.quarantine /path/to/calculator
```

或在 Finder 中右键 “打开” 并确认。

## 目录结构

- `complex_number.hpp`、`fractions.hpp`、`string_processing.*`：核心计算逻辑
- `main_cli.cpp`：命令行入口
- `calculator_tests.cpp`：测试用例（通过 CTest 运行）
- `.github/workflows/c-cpp.yml`：GitHub Actions 持续集成/发布流程

## 常见问题

- **CMake 找不到编译器**：在 Windows 安装最新 Visual Studio Build Tools，在 Linux/macOS 安装 `build-essential` 或 Xcode Command Line Tools。
- **首次构建时间较长**：SymEngine 需首次从源码编译，请耐心等待；之后构建会增量进行。
- **运行时报缺少 DLL**：在 Developer Command Prompt 中运行，或安装与所用编译器匹配的 MSVC 运行库。
- **小数点解析异常**：若系统使用逗号小数，请在运行前设置 `LC_ALL=C`。

## 许可

MIT
