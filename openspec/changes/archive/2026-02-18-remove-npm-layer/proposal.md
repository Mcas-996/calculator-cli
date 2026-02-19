# Change: 移除 npm/JS 层，纯 Rust 分发

## Why

当前架构是混合的：
```
npm install → index.js → bin/calculator_(linux|windows)-x86_64
```

问题：
- 需要维护 Node.js 环境和 npm 包发布流程
- 每次 Rust 改动需要重新构建并上传二进制到 GitHub Releases
- npm postinstall 脚本需要动态下载二进制，增加复杂性
- 用户安装的是 JS 包装器，而非原生二进制

Rust 重构已完成，现在是纯 Rust 的好时机。

## What Changes

1. **移除 JS/TS 文件**
   - 删除 `package.json`
   - 删除 `package-lock.json`
   - 删除 `index.js`
   - 删除 `scripts/` 目录
   - 删除 `*.js` 测试脚本

2. **移除 npm 发布相关文件**
   - 删除 `PUBLISH.md`
   - 删除测试相关的 JS 文件:
     - `test-json.js`
     - `test-build-syntax.js`
     - `test-package.js`

3. **纯 Rust 分发方式**
   - 保留 `bin/` 预编译二进制 (用户可直接下载)
   - 支持 `cargo install calculator` 从 crates.io 安装
   - 用户可从 GitHub Releases 下载对应平台的二进制

4. **更新文档**
   - 更新 README.md，移除 npm 安装说明
   - 添加 cargo install 和直接下载说明

## Impact

- **受影响文件**:
  - `package.json`
  - `package-lock.json`
  - `index.js`
  - `scripts/`
  - `PUBLISH.md`
  - `test-json.js`
  - `test-build-syntax.js`
  - `test-package.js`
- **用户安装方式变化**:
  ```
  旧: npm install -g mathcalc-cli
  新: cargo install calculator
  或: curl -L https://github.com/.../releases/download/v2.0.0/calculator-linux-x64 -o mathcalc
  ```
- **二进制兼容性**: 保持不变，用户体验不变
- **构建流程**: 简化，不再需要 npm publish
