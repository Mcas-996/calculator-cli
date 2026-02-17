# 计算器命令行工具

一个轻量级的Rust命令行计算器，支持实数/复数表达式求值、方程求解（支持到五次方程的精确符号解）以及处理小型线性方程组。

## 快速上手

1. 克隆项目：`git clone https://github.com/allen/calculator-cli && cd calculator-cli`。
2. 构建Release版本：`cargo build --release`。
3. 直接在Shell中运行表达式：`./target/release/calculator "x^2-5x+6=0"`。

## 安装和使用

### npm全局安装（推荐）
```bash
npm install -g mathcalc-cli
mathcalc "2 + 2"
mathcalc "x^2-5x+6=0"
```

### npx方式（无需全局安装）
```bash
npx mathcalc-cli "2 + 2"
npx mathcalc-cli "x^2-5x+6=0"
```

### 本地项目使用
```bash
npm install mathcalc-cli
npx mathcalc-cli "2 + 2"
```

### 从GitHub Release下载
```bash
https://github.com/Mcas-996/calculator-cli/releases
```

下载对应平台的二进制文件

### ARM系统（Apple Silicon、ARM64 Linux）
npm包仅包含x64系统的预编译二进制文件。对于ARM系统：

```bash
# 1. 如果尚未安装Rust：
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 2. 从源码安装calculator-cli：
cargo install calculator
```

### 平台支持
| 平台 | 架构 | 安装方式 |
|------|------|----------|
| Windows | x64 | npm install |
| macOS | x64 (Intel) | npm install |
| macOS | ARM (Apple Silicon) | cargo install |
| Linux | x64 | npm install |
| Linux | ARM64 | cargo install |

## 功能特性

### 表达式引擎
- 加法、减法、乘法、除法、乘方运算
- 百分号（`50% * 200`）、括号、一元负号
- 常量`pi`、`e`、虚数单位`i`，支持小数或分数输入（结果自动简化为最简分数）
- 平方根`sqrt()`、绝对值`abs()`、弧度制三角函数`sin()/cos()`，以及角度制变体`sind()/cosd()`
- 完整的复数运算，例如`sqrt(-4)`、`(3+2i)*(1-i)`、`cosd(60+i)`

### 方程求解
- 一元一次方程：`2x+5=0`
- 一元二次方程：`x^2-5x+6=0`（实数根或复数根）
- 一元三次方程：`x^3-6x^2+11x-6=0`
- 一元四次方程：`x^4-2=0`（通过`sqrt`/`cbrt`获得符号根，数值解使用Durand–Kerner方法作为回退）
- 一元五次方程：`x^5+2x^4+...=0` → 通过Durand-Kerner方法进行数值近似
- 线性方程组（最多3个变量）：`x+y=5, x-y=1`

### 输出格式
- 结果在可能的情况下优先使用精确分数（例如`1/3`保持有理数形式），仅在必要时回退到小数
- 复数以`a + bi`形式输出，`i`/`-i`自动简化
- 多种输出格式：ASCII、Unicode和LaTeX

## 使用方法

```bash
# 基本用法（表达式作为命令行参数）
./target/release/calculator "3 + 5 * (2 - 8)^2"

# 复数运算
./target/release/calculator "(3+2i) * (1 - i)"
./target/release/calculator "sqrt(-9)"        # -> 3i

# 三角函数
./target/release/calculator "sin(pi / 6)"     # 弧度
./target/release/calculator "sind(30)"        # 角度

# 方程求解
./target/release/calculator "x^2-5x+6=0"
./target/release/calculator "x+y=5, x-y=1"

# 输出格式
./target/release/calculator --unicode "sqrt(16)"
./target/release/calculator --latex "pi"
./target/release/calculator --ascii "3 + 4"
```

使用`--help`或`--version`可查看CLI信息。若未提供参数，程序将进入交互模式。

## 构建

项目使用Rust和Cargo构建。

```bash
# 构建Release版本
cargo build --release

# 运行测试
cargo test

# 使用clippy进行额外检查
cargo clippy
```

- 需要Rust 1.75.0或更高版本
- Cargo会自动处理所有依赖
- 二进制文件位于`target/release/calculator`

## 交互模式

不传入参数运行计算器即可进入交互模式：

```bash
./target/release/calculator
```

输入表达式后按Enter键求值。输入`exit`或`quit`退出，或按Ctrl+D。

## 项目结构

- `src/core/` - 核心数据类型（ComplexNumber、Fraction、Expression）
- `src/parser/` - 表达式解析器和词法分析器
- `src/solver/` - 方程求解器（一次、二次、三次、四次、五次方程）
- `src/output/` - 输出格式化器（ASCII、Unicode、LaTeX）
- `src/main.rs` - 命令行入口

## 常见问题

- **找不到Cargo**：从https://rustup.rs/安装Rust
- **构建失败**：请确保已安装Rust 1.75.0或更高版本
- **依赖区域设置的解析问题**：在运行前强制使用C区域设置（`LC_ALL=C ./calculator`）

## 许可

MIT许可证 - 详见LICENSE文件

## 从C++版本迁移

此计算器最初使用C++20和SymEngine实现。Rust版本提供：
- 更快的构建时间（数分钟缩短到数秒）
- 无外部依赖（不再需要374MB的SymEngine vendored代码）
- 无需垃圾回收的内存安全性
- 更小的二进制体积
- 更好的跨平台支持
