# 🤖 iFlow CLI
![iFlow CLI Screenshot](./assets/iflow-cli.jpg)

**English** | [中文](README_CN.md) | [日本語](README_JA.md) | [한국어](README_KO.md) | [Français](README_FR.md) | [Deutsch](README_DE.md) | [Español](README_ES.md) | [Русский](README_RU.md)

iFlow CLI is a powerful AI assistant that runs directly in your terminal. This repository contains the source code for the iFlow CLI application, built as a monorepo with TypeScript, Node.js, and React (via Ink). This document is intended for developers who want to understand, contribute to, or extend the iFlow CLI codebase.

## 📦 Project Overview

iFlow CLI is structured as a monorepo using npm workspaces. It consists of three main packages:

- **`packages/cli`**: The command-line interface entry point, handling user interactions, command parsing, and terminal UI.
- **`packages/core`**: The core engine providing AI agent orchestration, tool execution, context management, and integration with MCP (Model Context Protocol) servers.
- **`packages/vscode-ide-companion`**: A VS Code extension that integrates iFlow CLI capabilities into the editor.

The application is built with modern JavaScript/TypeScript tooling, featuring a plugin-based architecture for extensibility.

## 📁 Project Structure

```
iflow-cli/
├── packages/                    # Monorepo packages (detailed below)
├── scripts/                    # Build and utility scripts
├── integration-tests/          # Integration test suites
├── docs/                       # Documentation
├── assets/                     # Images and static assets
├── vendors/                    # Bundled third‑party tools (ripgrep)
└── bundle/                     # Final bundled CLI (generated)
```

### Detailed Package Structure

#### `packages/cli` – Command‑Line Interface
```
cli/
├── src/
│   ├── commands/              # Command definitions
│   │   ├── agents/            # Agent‑related commands
│   │   ├── commands/          # Slash command implementations
│   │   ├── mcp/               # MCP command handlers
│   │   └── workflows/         # Workflow commands
│   ├── ui/                    # React components for terminal UI
│   │   ├── components/        # Reusable UI components
│   │   ├── contexts/          # React contexts (theme, state)
│   │   ├── hooks/             # Custom React hooks
│   │   ├── editors/           # Text‑editing components
│   │   └── themes/            # UI theme definitions
│   ├── services/              # CLI‑specific services
│   ├── utils/                 # Utility functions
│   ├── history/               # Conversation history management
│   └── config/                # Configuration handling
├── dist/                      # Compiled JavaScript (generated)
└── [tests]/                   # Unit tests (alongside source files)
```

#### `packages/core` – Core Engine
```
core/
├── src/
│   ├── tools/                 # Tool implementations
│   │   ├── task/              # Task tool & sub‑agent system
│   │   ├── *.ts               # Individual tools (read‑file, shell, etc.)
│   │   └── *.test.ts          # Tool unit tests
│   ├── core/                  # Core AI client & orchestration
│   │   ├── client.ts          # Gemini/iFlow API client
│   │   ├── contentGenerator.ts# Content generation logic
│   │   ├── coreToolScheduler.ts # Tool scheduling & concurrency
│   │   └── ...
│   ├── services/              # Background services
│   │   ├── fileDiscoveryService.ts
│   │   ├── gitService.ts
│   │   ├── shellExecutionService.ts
│   │   └── ...
│   ├── config/                # Configuration management
│   ├── mcp/                   # Model Context Protocol integration
│   ├── telemetry/             # OpenTelemetry instrumentation
│   ├── utils/                 # Shared utilities
│   └── ...
├── dist/                      # Compiled JavaScript (generated)
└── [tests]/                   # Unit tests (alongside source files)
```

#### `packages/vscode‑ide‑companion` – VS Code Extension
```
vscode-ide-companion/
├── src/
│   ├── extension.ts           # VS Code extension entry point
│   ├── ide‑server.ts          # Communication with iFlow CLI
│   ├── diff‑manager.ts        # File diff handling
│   └── open‑files‑manager.ts  # Open files state management
├── assets/                    # Extension icons & assets
├── .vscode/                   # VS Code configuration
└── dist/                      # Compiled extension (generated)
```

### Other Key Directories

- **`scripts/`** – Build, release, and maintenance scripts (e.g., `build.js`, `telemetry.js`).
- **`integration‑tests/`** – End‑to‑end tests for tool interactions and sandbox behavior.
- **`docs/`** – Comprehensive documentation (architecture, CLI usage, troubleshooting).
- **`vendors/`** – Bundled third‑party binaries (ripgrep) used by tools.
- **`bundle/`** – Final bundled CLI artifact (created by `npm run bundle`).

## 🏛️ Architecture Deep Dive

iFlow CLI follows a layered, event‑driven architecture designed for extensibility and security. The core components are:

### Core Layer (`packages/core`)
- **Tool System**: Registration, discovery, and execution of tools (file operations, shell, web, etc.). Tools are defined as classes extending `BaseTool` with parameter validation and permission control.
- **Agent System**: Orchestration of sub‑agents (`general‑purpose`, `plan‑agent`, `explore‑agent`) for parallel task execution. Each agent has isolated tool permissions and MCP access.
- **MCP Integration**: Native support for Model Context Protocol servers, allowing dynamic tool discovery from external processes.
- **Event System**: Asynchronous communication between components (tool calls, agent lifecycle, UI updates).
- **Configuration & Auth**: Centralized config management and multiple authentication providers (iFlow native, OpenAI‑compatible).

### CLI Layer (`packages/cli`)
- **React‑based UI**: Built with [Ink](https://github.com/vadimdemedes/ink) for terminal rendering, using hooks and components for interactive prompts, streaming output, and real‑time status.
- **Command Parser**: Handles slash commands (`/init`), direct agent calls (`$explore‑agent`), and natural‑language queries.
- **State Management**: React hooks manage conversation history, tool call status, and user preferences.

### Integration Layer
- **Sandbox Environment**: Optional Docker/Podman isolation for risky tool executions.
- **Telemetry**: OpenTelemetry‑based metrics, traces, and logs for debugging and performance monitoring.
- **Git Integration**: Automatic detection of repositories, commit message generation, and diff viewing.

For a comprehensive architecture overview, see [Architecture Documentation](./docs/architecture/overview.md).

## 🛠️ Technology Stack

- **Runtime**: Node.js ≥20
- **Language**: TypeScript with strict mode
- **UI Framework**: React via [Ink](https://github.com/vadimdemedes/ink) for terminal rendering
- **Build Tool**: esbuild for bundling, tsc for type checking
- **Package Manager**: npm (workspaces)
- **Testing**: Vitest for unit tests, custom integration test runner
- **Linting/Formatting**: ESLint, Prettier

## 🏗️ Building the Project

### Build All Packages

```bash
npm install && npm run build
```

This command runs the TypeScript compiler for each package and produces output in `packages/*/dist`.

### Build with Bundling (Beta)

```bash
npm run release:version [version]
npm pack
```

The `bundle` script generates the final standalone CLI artifact in the `bundle/` directory, which is what gets published to npm.



## 🧪 Testing

### Unit Tests

test packages/cli module:
```bash
npm run build --workspaces && npm run typecheck --workspace=@iflow-ai/iflow-cli && npm run test:ci --workspace=@iflow-ai/iflow-cli
```

test packages/core module:
```bash
npm run build --workspaces && npm run typecheck --workspace=@iflow-ai/iflow-cli-core && npm run test:ci --workspace=@iflow-ai/iflow-cli-core
```

Runs Vitest tests across all packages.

### Integration Tests

Integration tests verify tool interactions and sandbox behavior. They can run with different sandbox backends:

```bash
# No sandbox
npm run test:integration:sandbox:none

# With Docker sandbox
npm run test:integration:sandbox:docker

# With Podman sandbox
npm run test:integration:sandbox:podman
```

### End‑to‑End Tests

```bash
npm run test:e2e
```

### Linting and Formatting

```bash
npm run lint          # ESLint check
npm run lint:fix      # ESLint auto‑fix
npm run format        # Prettier formatting
npm run typecheck     # TypeScript type checking
```

## 🔌 Extending iFlow CLI

### Adding a New Tool

1. **Create a new tool file** in `packages/core/src/tools/` (e.g., `my-tool.ts`).
2. **Extend `BaseTool`** and implement the required methods:
   ```typescript
   export class MyTool extends BaseTool<MyParams, ToolResult> {
     constructor() {
       super('my_tool', 'My Tool', 'Description for the AI', Icon.Hammer);
     }
     // Define parameter schema, validation, execution logic
   }
   ```
3. **Register the tool** in `packages/core/src/tools/tools.ts` by adding it to the `coreTools` array.
4. **Write tests** following existing patterns (e.g., `my-tool.test.ts`).

See [Tool System Design](./docs/architecture/tool-system-design.md) and [Tools API](./docs/core/tools-api.md) for detailed guidance.

### Adding a New Agent Type

1. **Define agent metadata** in `packages/core/src/tools/task/agentRegistry.ts` under `AGENT_DEFINITIONS`.
2. **Specify allowed tools**, MCP servers, and execution constraints.
3. **The `task` tool** automatically handles agent lifecycle, parallel execution, and result aggregation.

### Adding a New CLI Command

1. **Add command definition** in `packages/cli/src/commands/` (e.g., create `my-command.ts`).
2. **Register the command** in `packages/cli/src/commands/commands.ts`.
3. **Implement command logic** using the existing CLI services and UI components.

### Integrating an MCP Server

1. **Configure the server** in user or project settings (`~/.iflow/settings.json`):
   ```json
   "mcpServers": {
     "my_server": {
       "command": "npx",
       "args": ["-y", "my-mcp-server"]
     }
   }
   ```
2. **Tools from the server** will be automatically discovered and prefixed with the server name (e.g., `my_server__tool_name`).

## 🔄 Development Workflow

### Typical Feature Development

1. **Explore existing code** using the built‑in `$explore‑agent`:
   ```bash
   $explore‑agent "How is the tool system organized?"
   ```
2. **Create a feature branch** from `main`.
3. **Implement changes** following the coding standards and architectural patterns.
4. **Run tests** locally (`npm test`, `npm run test:integration:sandbox:none`).
5. **Update documentation** if the change affects user‑facing behavior or adds new APIs.
6. **Submit a PR** with a clear description and link to any related issues.

### Code Review Checklist

- [ ] TypeScript types are strict and accurate.
- [ ] New tools have proper parameter validation and user confirmation.
- [ ] Added tests cover positive and negative scenarios.
- [ ] No secrets or sensitive data are committed.
- [ ] Bundle size impact is considered for new dependencies.
- [ ] Documentation is updated (user guides, API docs, architecture notes).

### Performance Profiling

- Use `DEBUG=true` to capture detailed timing logs.
- Monitor memory usage via crash reports (`crash-*.json`).
- For suspected bottlenecks, run the CLI with Node.js inspector and profile CPU/memory in Chrome DevTools.

## 🧩 Contributing

We welcome contributions! Please follow these steps:

1. **Fork** the repository and create a feature branch.
2. **Ensure your changes pass** the existing tests and linting rules.
3. **Add tests** for new functionality.
4. **Update documentation** if needed.
5. **Submit a pull request** with a clear description of the changes.

### Code Style

- Use TypeScript with strict mode.
- Follow the existing ESLint and Prettier configuration.
- Write meaningful commit messages (conventional commits are appreciated but not required).
- Keep the bundle size in mind when adding dependencies.

### Commit Hooks

The project uses Husky to run lint‑staged checks on commit. This ensures code consistency before pushing.


## 📚 Documentation

- **[Architecture](./docs/architecture/overview.md)**: High‑level design and component interactions.
- **[Tool System](./docs/architecture/tool-system-design.md)**: How tools are implemented and scheduled.
- **[MCP Integration](./docs/architecture/mcp-integration-guide.md)**: Integrating Model Context Protocol servers.
- **[CLI Commands](./docs/cli/commands.md)**: User‑facing command reference.
- **[Contributing Guide](./CONTRIBUTING.md)**: Detailed contribution instructions.

## 🤝 Community

- **Issues**: Report bugs or request features on [GitHub Issues](https://github.com/iflow-ai/iflow-cli/issues).
- **WeChat Group**: Join the community discussion via the QR code below.

![WeChat group](./assets/iflow-wechat.jpg)

## 📄 License

iFlow CLI is open‑source under the [MIT License](./LICENSE).
