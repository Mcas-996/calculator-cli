# Change: Add npm distribution for calculator-cli

## Why
Currently users can only access the calculator-cli tool by building from source or downloading pre-compiled binaries. Publishing to npm would provide an additional convenient installation method for the millions of npm users, especially those already working in JavaScript/Node.js environments who might want a simple CLI calculator tool. This reduces friction for adoption as npm handles version management, updates, and platform-specific binary downloads automatically.

## What Changes
- Create npm package configuration (package.json with node-pre-gyp or prebuild support)
- Add precompiled binaries for Windows, macOS (Intel), and Linux (x64) platforms
- Implement automatic fallback to cargo build for ARM platforms (Apple Silicon, ARM64 Linux)
- Add GitHub Actions workflow to build and publish to npm on releases
- Update build system to generate platform-specific binaries for npm distribution
- Add installation instructions for npm in README files
- Configure npm registry metadata and publishing automation

## Impact
- Affected specs: distribution
- Affected code: build system, CI configuration, documentation
- **BREAKING**: No breaking changes to current functionality - this is additive only
- Effort: Medium (requires CI setup, build configuration)
- Platform Strategy: Provide precompiled binaries for major platforms (x64) and use cargo build for ARM platforms