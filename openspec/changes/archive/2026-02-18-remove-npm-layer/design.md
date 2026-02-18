# Design: 移除 npm/JS 层

## Overview

移除 Node.js 包装层，使项目成为纯 Rust 分发。

## Architecture

### Before (混合架构)
```
用户
  ↓
npm install -g mathcalc-cli
  ↓
package.json + index.js (Node.js 包装器)
  ↓
bin/calculator_*-x86_64 (Rust 二进制)
```

### After (纯 Rust)
```
用户
  ↓
cargo install calculator
  ↓
或: curl -L .../releases/download/.../calculator-*-x86_64
  ↓
Rust 二进制 (直接运行)
```

## Files to Delete

| 文件 | 说明 |
|------|------|
| `package.json` | npm 包配置 |
| `package-lock.json` | npm 锁定文件 |
| `index.js` | Node.js 包装器脚本 |
| `scripts/` | npm 脚本目录 (4 个文件) |
| `PUBLISH.md` | npm 发布文档 |
| `test-json.js` | JS 测试脚本 |
| `test-build-syntax.js` | JS 构建测试 |
| `test-package.js` | JS 包测试 |

## Files to Keep

| 文件 | 说明 |
|------|------|
| `bin/` | 预编译二进制 (Windows + Linux) |
| `Cargo.toml` | Rust 项目配置 |
| `Cargo.lock` | Rust 依赖锁定 |
| `src/` | Rust 源代码 |
| `README.md` | 项目文档 (需更新) |

## Installation Methods

### 1. cargo install (推荐)
```bash
cargo install calculator
```

### 2. GitHub Releases 下载
```bash
# Linux
curl -L https://github.com/Mcas-996/calculator-cli/releases/latest/download/calculator-linux-x64 -o mathcalc
chmod +x mathcalc
./mathcalc "2+2"

# Windows
# 从 Releases 页面下载 calculator_windows-x86-64.exe
```

## Build for Release

```bash
# 构建当前平台
cargo build --release

# 交叉编译
# Linux:
cargo build --release --target x86_64-unknown-linux-gnu
# Windows:
cargo build --release --target x86_64-pc-windows-gnu
```

## Testing After Changes

1. `cargo build --release` - 确保编译通过
2. `cargo test` - 确保测试通过
3. `./target/release/calculator "2+2"` - 验证基本功能
4. 检查 `bin/` 目录下二进制是否存在
