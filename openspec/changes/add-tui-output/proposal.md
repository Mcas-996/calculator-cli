## Why

当前计算器 CLI 使用简单的 stdin 循环进行交互式计算，缺少现代化的终端 UI 体验。用户需要更好的视觉反馈、分层布局以及 LaTeX 风格的数学公式显示。ratatui 是 Rust 生态中成熟的 TUI 库，适合实现这个改进。

## What Changes

- 使用 ratatui 重写交互式终端 UI，替代现有的 `run_interactive_mode()`
- 上方区域：显示计算结果的"LaTeX 卡片"列表，支持滚动
- 下方区域：固定输入框，支持多行表达式
- 实现真正的 LaTeX 风格渲染（用 Unicode 字符模拟分数横线）
- 支持 `ans` 变量保存上次计算结果
- 弃用旧的简单 stdin 交互模式

## Capabilities

### New Capabilities

- `tui-interface`: 完整的 TUI 交互界面，包含结果卡片渲染、输入框、多行表达式支持

### Modified Capabilities

- `cli-interface`: 现有的 CLI 接口需要更新，交互模式改用 TUI 实现

## Impact

- 新增依赖：`ratatui`
- 修改 `src/main.rs`：`run_interactive_mode()` 改为 TUI 实现
- 新增 `src/tui/` 模块：TUI 相关的组件和逻辑
- 输出格式化模块保持不变，继续使用现有的 formatter 系统
