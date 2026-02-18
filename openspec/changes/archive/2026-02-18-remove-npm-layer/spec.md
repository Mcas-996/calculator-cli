# Spec: 纯 Rust 分发

## CHANGED Requirements

### Requirement: 分发方式

#### Scenario: 用户安装
- **GIVEN** 用户想要安装 calculator
- **WHEN** 使用 `cargo install calculator` 或下载预编译二进制
- **THEN** 直接获得 Rust 原生二进制，无需 Node.js 环境

#### Scenario: 移除 npm 依赖
- **GIVEN** 项目中不再需要 npm/node 环境
- **WHEN** 删除 package.json 和相关 JS 文件
- **THEN** 项目仅包含 Rust 代码和预编译二进制

### Requirement: 文档更新

#### Scenario: README 更新
- **GIVEN** README.md 需要反映新的安装方式
- **WHEN** 更新安装说明
- **THEN** 包含以下安装方式:
  - `cargo install calculator` (从 crates.io)
  - 从 GitHub Releases 下载预编译二进制

## UNCHANGED Requirements

- CLI 参数接口保持不变 (`-p`, `-u`, `-l`, `-a`, `-d`, `-e`)
- 交互模式保持不变
- 所有数学功能保持不变
- 输出格式保持不变
