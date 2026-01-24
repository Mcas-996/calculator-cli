# Local Build Guide for calculator-cli npm Package

This guide explains how to build and publish the calculator-cli npm package using local compilation instead of CI/CD pipelines.

## Overview

The calculator-cli npm package includes precompiled binaries for x64 platforms (Windows, macOS Intel, Linux x64). This guide explains how to build these binaries locally and publish the package to npm.

## Prerequisites

### Required Tools

1. **Rust toolchain** (latest stable version)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Node.js** (version 12 or higher)
   ```bash
   # Using nvm is recommended
   nvm install 18
   nvm use 18
   ```

3. **npm account** with publishing permissions for calculator-cli
   - Sign up at [npmjs.com](https://www.npmjs.com)
   - Generate an access token with "Automation" permissions

### Cross-Compilation Tools (Optional)

If you want to build for platforms other than your current one:

- **For Windows on Linux/macOS**: mingw-w64
  ```bash
  # Ubuntu/Debian
  sudo apt-get install mingw-w64
  
  # macOS
  brew install mingw-w64
  ```

- **For macOS on Linux/Windows**: Not fully supported
  - The most reliable approach is to build on macOS itself

- **For Linux on Windows**: Use WSL
  - Install Ubuntu from Microsoft Store
  - Install Rust and other tools in WSL

## Build Process

### 1. Prepare the Source Code

1. Update the version in `Cargo.toml`:
   ```toml
   [package]
   version = "2.0.1"  # Update this number
   ```

2. Sync the version with `package.json`:
   ```bash
   # Extract version from Cargo.toml and update package.json
   VERSION=$(grep '^version = ' Cargo.toml | head -1 | cut -d '"' -f 2)
   npm version $VERSION --no-git-tag-version
   ```

### 2. Build Cross-Platform Binaries

1. Add the required Rust targets:
   ```bash
   rustup target add x86_64-unknown-linux-gnu
   rustup target add x86_64-apple-darwin
   rustup target add x86_64-pc-windows-gnu
   ```

2. Build the binaries:
   ```bash
   npm run build-binaries
   ```

   The script will:
   - Attempt to build for all three platforms
   - Skip platforms that are not supported on your system
   - Report any missing toolchains with instructions

### 3. Test the Package

1. Validate the package structure:
   ```bash
   node test-package.js
   ```

2. Create and test the npm package:
   ```bash
   # Create the packge file
   npm run prepare-package
   
   # Test local installation
   npm run local-test
   ```

### 4. Publish to npm

1. Log in to npm (if not already):
   ```bash
   npm login
   ```

2. Publish the package:
   ```bash
   npm publish
   ```

3. Verify the publication:
   - Check the package page at [npmjs.com/package/calculator-cli](https://www.npmjs.com/package/calculator-cli)
   - Test installation in a clean environment

## Platform-Specific Instructions

### Building on Linux

Linux is the most straightforward platform for cross-compilation:

```bash
# Install dependencies
sudo apt-get update
sudo apt-get install build-essential mingw-w64

# Add targets
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-pc-windows-gnu

# Build (macOS binary will be skipped)
npm run build-binaries
```

### Building on macOS

macOS can build for itself and Windows (with extra tools):

```bash
# Install Homebrew if not already
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install dependencies
brew install mingw-w64

# Add targets
rustup target add x86_64-apple-darwin
rustup target add x86_64-pc-windows-gnu

# Build (Linux binary will be skipped)
npm run build-binaries
```

### Building on Windows

Windows can build for itself:

```bash
# Add targets
rustup target add x86_64-pc-windows-gnu

# Build (Linux and macOS binaries will be skipped)
npm run build-binaries
```

For cross-compilation on Windows, use WSL:
```bash
# In WSL/Ubuntu
sudo apt-get update
sudo apt-get install build-essential mingw-w64
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Add targets
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-pc-windows-gnu

# Build (macOS binary will be skipped)
npm run build-binaries
```

## Troubleshooting

### Common Issues

1. **Target not found errors**:
   ```
   error: could not find `x86_64-pc-windows-gnu`
   ```
   Solution: Install the missing target with `rustup target add <target-name>`

2. **Linker errors (E4025)**:
   ```
   error: linking with `linker` failed: exit code: 1
   = note: /usr/bin/ld: cannot find -lgcc_s
   ```
   This error occurs when cross-compiling, especially for Windows targets. Solutions:
   
   **On Linux**:
   ```bash
   # Install mingw-w64 for Windows cross-compilation
   sudo apt-get update
   sudo apt-get install mingw-w64 gcc-mingw-w64-x86-64
   
   # Reinstall the Windows target
   rustup target remove x86_64-pc-windows-gnu
   rustup target add x86_64-pc-windows-gnu
   ```
   
   **On macOS**:
   ```bash
   # Install mingw-w64 using Homebrew
   brew install mingw-w64
   
   # Reinstall the Windows target
   rustup target remove x86_64-pc-windows-gnu
   rustup target add x86_64-pc-windows-gnu
   ```
   
   **Alternative solutions**:
   - Build on the native platform (Windows for Windows binaries)
   - Use a CI service for specific platforms
   - Focus on platforms that build successfully on your system

3. **Permission denied on script execution**:
   ```bash
   chmod +x scripts/build-binaries.js
   ```

4. **Binary not found after building**:
   - Check if you have the required Rust target installed
   - Verify the build completed successfully
   - Check the target directory: `target/<target-name>/release/`

5. **Package size too large**:
   - Verify only the necessary files are in the `bin/` directory
   - Check `.npmignore` if you have one
   - Ensure `.gitignore` isn't affecting the build

### Verification Steps

After each release, verify:

1. **Package size** should be reasonable (typically < 50MB)
2. **Installation works** on at least one x64 platform:
   ```bash
   npm install -g calculator-cli
   calculator "1+1"  # Should output "2"
   ```
3. **ARM detection** shows appropriate error message on ARM platforms
4. **Package.json version** matches the release tag

## Release Process Checklist

1. [ ] Update version in `Cargo.toml`
2. [ ] Sync version with `package.json`
3. [ ] Add any missing Rust targets
4. [ ] Build binaries: `npm run build-binaries`
5. [ ] Verify package structure: `node test-package.js`
6. [ ] Create package: `npm run prepare-package`
7. [ ] Test local installation: `npm run local-test`
8. [ ] Commit and tag release: `git tag vx.x.x`
9. [ ] Push tag: `git push origin vx.x.x`
10. [ ] Publish to npm: `npm publish`

## Maintenance Notes

- **Frequency**: Publish updates when new features are added or bugs are fixed
- **Versioning**: Follow semantic versioning (MAJOR.MINOR.PATCH)
- **Compatibility**: Test on multiple platforms before publishing
- **Fallback**: Keep ARM build instructions up-to-date as Rust toolchain evolves

## Security Considerations

- Store npm tokens in environment variables, not in code
- Use npm two-factor authentication if available
- Vet all dependencies before publishing
- Keep build tools and dependencies up to date

## Alternative: Docker-based Builds

For reproducible builds, you can use Docker:

```bash
# Create a Dockerfile with all required tools
docker build -t calculator-cli-builder .

# Build binaries
docker run -v $(pwd):/workspace calculator-cli-builder npm run build-binaries
```

This ensures consistent build environments across machines.