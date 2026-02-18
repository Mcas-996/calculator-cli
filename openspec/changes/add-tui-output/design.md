## Context

当前计算器 CLI 使用简单的 `stdin` 循环（`run_interactive_mode()`）进行交互，每次输入一行表达式并输出结果。用户需要更现代化的终端 UI 体验，包括：

- LaTeX 风格的数学公式显示
- 结果历史记录的可视化
- 多行表达式支持
- ans 变量保存

目标是用 ratatui 实现一个新的 TUI 交互界面。

## Goals / Non-Goals

**Goals:**
- 用 ratatui 实现完整的 TUI 交互界面
- 上方区域显示"LaTeX 卡片"列表（计算结果），支持滚动
- 下方固定输入框，支持多行表达式（Shift+Enter 换行，Ctrl+Enter 执行）
- 实现 LaTeX 风格渲染（用 Unicode 字符模拟分数横线）
- 支持 ans 变量保存上次计算结果
- 弃用旧的 `run_interactive_mode()`

**Non-Goals:**
- 不实现函数绘图功能
- 不实现自动补全
- 不实现命令历史（↑/↓ 导航）
- 不实现 Tab 补全

## Decisions

### 1. ratatui 版本选择

**决定**: 使用最新稳定版 ratatui (0.26+)

**理由**:
- 最新版 API 更加现代化
- 内置了更丰富的组件
- 与 Rust 2024 edition 兼容

### 2. LaTeX 渲染方案

**决定**: 自定义 LaTeX 渲染器，将 LaTeX 转换为 Unicode 艺术字风格

```
        -5 ± √(25-24)
x = ───────────── = {-2, -3}
              2
```

**理由**:
- 真正的 LaTeX 渲染需要外部工具或图片（kitty sixel 等），过于复杂
- 用 Unicode 字符模拟分数横线足够美观且跨平台
- 复用现有的 `UnicodeFormatter` 和 `LatexFormatter`，新增专门的 `TuiLatexRenderer`

### 3. 多行输入实现

**决定**: 使用 ratatui 的 `Textarea` 组件

- Shift+Enter: 插入换行符
- Ctrl+Enter (Unix) / Ctrl+Enter (Windows): 执行计算

**理由**:
- `Textarea` 原生支持多行编辑
- 可自定义 key binding

### 4. ans 变量保存逻辑

**决定**:
- 表达式计算 → 保存 `ans = <结果>`
- 线性方程 → 保存 `ans = <x的值>`
- 多项式方程 → 保存 `ans = <x1的值>`

**理由**:
- 简单明确，符合用户习惯
- 方程取第一个解是常见做法

### 5. 布局结构

**决定**: 垂直分栏布局

```
┌─────────────────────────────────────────┐
│  ┌─ Result 1 ───────────────────────┐  │
│  │  LaTeX 卡片                      │  │
│  └─────────────────────────────────┘  │
│  ┌─ Result 2 ───────────────────────┐  │
│  │  LaTeX 卡片                      │  │
│  └─────────────────────────────────┘  │
│  ... (可滚动)                          │
├─────────────────────────────────────────┤
│  > 输入框                             │
│  [Ctrl+Enter: 执行] [Shift+Enter: 换行]│
└─────────────────────────────────────────┘
```

**理由**:
- 结构简单，符合 AI Coding TUI 的常见模式
- 上方结果区域可滚动查看历史
- 下方输入框始终可见

### 6. 模块结构

**决定**: 新增 `src/tui/` 模块

```
src/tui/
├── mod.rs          # 模块入口
├── app.rs          # TUI 应用主逻辑
├── result_card.rs  # 结果卡片渲染
├── input.rs        # 输入框处理
└── latex.rs        # LaTeX 风格渲染器
```

**理由**:
- 模块化设计，职责清晰
- 与现有代码解耦

## Risks / Trade-offs

- [风险] ratatui 依赖 crossterm，在某些极端环境可能不兼容 →  Mitigation: crossterm 是跨平台的，主流终端都支持
- [风险] 多平台按键事件差异（Ctrl+Enter 在不同终端行为不同）→  Mitigation: 使用 crossterm 的标准化事件处理
- [风险] 大量历史结果可能导致内存占用增加 →  Mitigation: 限制历史记录数量（如最多 100 条）

## Migration Plan

1. 新增 `ratatui` 依赖
2. 创建 `src/tui/` 模块
3. 实现 TUI 应用主逻辑
4. 实现 LaTeX 风格渲染器
5. 修改 `main.rs`：用新的 TUI 模式替代旧的 `run_interactive_mode()`
6. 测试多平台兼容性

## Open Questions

- 是否需要支持鼠标操作？（如点击结果卡片复制）
- 结果卡片是否需要显示计算表达式？（当前设计只显示结果）
