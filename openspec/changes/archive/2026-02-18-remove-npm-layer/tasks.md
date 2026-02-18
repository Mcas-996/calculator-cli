# Tasks: 移除 npm/JS 层

## 1. 删除 JS/TS 文件
- [x] 1.1 删除 `package.json`
- [x] 1.2 删除 `package-lock.json`
- [x] 1.3 删除 `index.js`
- [x] 1.4 删除 `scripts/` 目录及其所有内容
  - `scripts/postinstall.js`
  - `scripts/install.js`
  - `scripts/test.js`
  - `scripts/setup-wsl.js`
- [x] 1.5 删除测试相关 JS 文件
  - `test-json.js`
  - `test-build-syntax.js`
  - `test-package.js`

## 2. 删除 npm 发布相关文件
- [x] 2.1 删除 `PUBLISH.md`
- [x] 2.2 检查并删除 `.npmignore` (如果存在) - 不存在

## 3. 保留 Rust 分发文件
- [x] 3.1 保留 `bin/` 目录 (预编译二进制)
- [x] 3.2 保留 Cargo.toml (Rust 项目配置)
- [x] 3.3 保留 Cargo.lock

## 4. 更新文档
- [x] 4.1 更新 README.md
  - 移除 npm 安装说明
  - 添加 cargo install 说明
  - 添加从 GitHub Releases 下载说明
  - 更新示例命令
- [x] 4.2 检查并更新其他文档
  - 已删除 LOCAL_BUILD_GUIDE.md (npm 构建指南)
  - 已删除 IFLOW.md (空文件)

## 5. 清理项目配置
- [x] 5.1 检查并更新 .gitignore (移除 npm 相关条目)
- [x] 5.2 检查 openspec 配置是否需要更新 - 不需要

## 6. 验证
- [x] 6.1 验证 cargo build 正常工作
- [x] 6.2 验证 cargo test 通过 (14 个预存在的测试失败，与 npm 移除无关)
- [x] 6.3 确认 bin/ 目录下二进制仍然存在
- [x] 6.4 手动测试 CLI 功能
