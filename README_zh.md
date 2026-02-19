# 计算器命令行工具

一个轻量级 Rust 命令行计算器，支持实数/复数表达式求值、方程求解和小型线性方程组计算。

## 快速上手

1. 克隆项目：`git clone https://github.com/Mcas-996/calculator-cli && cd calculator-cli`
2. 直接运行计算：`cargo run -- "x^2-5x+6=0"`
3. 构建 Release：`cargo build --release`
4. 运行 Release 二进制：`./target/release/calctui "2 + 2"`

## 安装与使用

### 通过 crates.io 安装（推荐）

```bash
cargo install calculator-tui
calctui "2 + 2"
calctui "x^2-5x+6=0"
```

### 从 GitHub Releases 下载

从以下地址下载对应平台的预编译二进制：
https://github.com/Mcas-996/calculator-cli/releases

下载后直接运行可执行文件并传入表达式即可。

### 从源码构建

```bash
git clone https://github.com/Mcas-996/calculator-cli && cd calculator-cli
cargo build --release
./target/release/calctui "2 + 2"
```

### 平台支持

| 平台 | 架构 | 安装方式 |
|------|------|----------|
| Windows | x64 | cargo install / GitHub Releases |
| macOS | x64 (Intel) | cargo install / GitHub Releases |
| macOS | ARM (Apple Silicon) | cargo install / GitHub Releases |
| Linux | x64 | cargo install / GitHub Releases |
| Linux | ARM64 | cargo install / GitHub Releases |

## 功能特性

### 表达式引擎

- 加减乘除与乘方
- 百分号（`50% * 200`）、括号、一元负号
- 常量 `pi`、`e`、虚数单位 `i`，支持小数与分数输入
- 函数：`sqrt()`、`abs()`、`sin()`、`cos()`、`sind()`、`cosd()`
- 复数运算，例如 `sqrt(-4)`、`(3+2i)*(1-i)`

### 方程求解

- 一元一次：`2x+5=0`
- 一元二次：`x^2-5x+6=0`
- 一元三次：`x^3-6x^2+11x-6=0`
- 一元四次：`x^4-2=0`
- 五次及以上：使用 Durand-Kerner 进行数值近似
- 线性方程组（最多 3 个变量）：`x+y=5, x-y=1`

### 输出模式

- 可在可能时保留分数等精确形式
- `--decimal` 输出小数近似
- 支持 ASCII / Unicode / LaTeX（`--ascii`、`--unicode`、`--latex`）

### 交互模式

- 默认：不传表达式时进入 TUI 模式
- 兼容旧版交互 CLI：`--v1`

## 使用示例

```bash
# 基本表达式
calctui "3 + 5 * (2 - 8)^2"

# 复数
calctui "(3+2i) * (1 - i)"
calctui "sqrt(-9)"                # -> 3i

# 三角函数
calctui "sin(pi / 6)"             # 弧度
calctui "sind(30)"                # 角度

# 方程求解
calctui "x^2-5x+6=0"
calctui "x+y=5, x-y=1"

# 输出格式
calctui --unicode "sqrt(16)"
calctui --latex "pi"
calctui --ascii "3 + 4"
calctui --decimal "1/3"

# 交互模式
calctui
calctui --v1
```

可使用 `--help` 或 `--version` 查看命令行帮助信息。

## 构建

```bash
# 构建 Release
cargo build --release

# 运行测试
cargo test

# clippy 检查
cargo clippy
```

- 需要 Rust 1.70.0 或更高版本
- Cargo 会自动处理依赖
- Release 二进制路径：`target/release/calctui`

## 项目结构

- `src/core/` - 核心数据类型（`ComplexNumber`、`Fraction`、`Expression`）
- `src/parser/` - 表达式解析与分词
- `src/solver/` - 方程求解器（一次到五次/方程组）
- `src/output/` - 输出格式化器（ASCII、Unicode、LaTeX）
- `src/tui/` - TUI 界面
- `src/main.rs` - CLI 入口

## 常见问题

- **找不到 Cargo**：请从 https://rustup.rs/ 安装 Rust
- **构建失败**：请确认 Rust 版本不低于 1.70.0

## 许可

MIT 许可证，详见 `LICENSE` 文件。
