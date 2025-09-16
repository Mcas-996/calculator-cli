# 计算器命令行工具

一个支持基本运算、括号、指数、百分比、平方根、数学常数、方程求解和线性方程组求解的命令行计算器应用程序。

## 功能特性

- 基本算术运算：`+`、`-`、`*`、`/`、`^`（指数）
- 括号用于表达式分组
- 负数和小数
- 百分比计算（例如，`50%` 转换为 `0.5`）
- 平方根函数：`sqrt(x)`
- 数学常数：`pi`（π）和 `e`
- 线性方程求解：`equation(x+1=0)`
- 二次方程求解：`equation(x^2+2x+1=0)`
- 线性方程组求解：`equation2(x+y=5,x-y=1)`
- 无效表达式的错误处理

## 支持的操作

```bash
# 基本算术
3 + 5 * (2 - 8)^2
-2.5 * 4 + 3^2

# 百分比
50% * 200

# 平方根
sqrt(16) + 3

# 常数
pi * 2
e^2

# 线性方程
equation(x+1=0)
equation(2x-3=7)

# 二次方程
equation(x^2+2x+1=0)
equation(x^2-5x+6=0)

# 线性方程组
equation2(x+y=5,x-y=1)
equation2(2x+3y=12,4x-y=5)
equation2(x+y+z=6,x-y+z=2,2x+y-z=3)
```

## 构建

### Windows
```bash
build_windows.bat
```

### Linux
```bash
chmod +x build_linux.sh
./build_linux.sh
```

### macOS
```bash
chmod +x build_macos.sh
./build_macos.sh
```

## 使用方法

```bash
# 基本用法
calculator "3 + 4 * 2"

# 显示帮助信息
calculator --help
# 或
calculator -h
```

## 系统要求

- 支持C++11的编译器（g++）
- 标准C++库
- 数学库

## 文件说明

- `main_cli.cpp` - 主应用程序入口点
- `string_processing.cpp` - 表达式求值逻辑
- `string_processing.hpp` - 计算器函数的头文件
- `build_*.sh/bat` - 平台特定的构建脚本