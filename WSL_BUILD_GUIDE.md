# Building calculator-cli in WSL (Windows Subsystem for Linux)

This guide provides specific instructions for building the calculator-cli npm package in a WSL environment. WSL offers unique advantages and considerations for cross-compilation to Windows.

## Advantages of Building in WSL

1. **Native Windows Access**: Binaries built for Windows in WSL work perfectly in Windows
2. **File System Integration**: Easy access to built binaries from Windows Explorer
3. **Linux Tooling**: Access to powerful Linux tools and scripting
4. **Seamless Path Mapping**: WSL paths map directly to Windows file system

## Prerequisites in WSL

### 1. Update WSL Package Lists

```bash
sudo apt-get update
sudo apt-get upgrade
```

### 2. Install Required Tools

```bash
# Install build tools
sudo apt-get install build-essential

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install cross-compilation tools for Windows
sudo apt-get install mingw-w64

# If above fails, try:
sudo apt-get install gcc-mingw-w64-x86-64

# If still fails, try the development headers:
sudo apt-get install mingw-w64-x86-64-dev
```

### 3. Install Rust Targets

```bash
# Add Windows target
rustup target add x86_64-pc-windows-gnu

# Add Linux target
rustup target add x86_64-unknown-linux-gnu

# Verify targets are installed
rustup target list --installed
```

## Building in WSL

### 1. Prepare the Project

```bash
# Navigate to your project (assuming it's in a Windows drive)
cd /mnt/c/path/to/your/calculator-cli

# Install Node.js dependencies
npm install
```

### 2. Build the Binaries

```bash
# Run the build script
npm run build-binaries
```

The script will automatically detect WSL and provide appropriate instructions.

### 3. Test the Windows Binary

You can test the Windows binary directly from WSL:

```bash
# Test the Windows binary
./bin/calculator-win32-x64.exe "1+1"
```

Or from Windows Command Prompt/PowerShell:

```
# Access the binary through WSL path mapping
\\wsl$\Ubuntu\mnt\c\path\to\your\calculator-cli\bin\calculator-win32-x64.exe "1+1"
```

## WSL-Specific Troubleshooting

### 1. MinGW Installation Issues

If you're having trouble with MinGW installation:

```bash
# Try this specific package
sudo apt-get install mingw-w64-x86-64-dev

# Or try from backports if on Ubuntu
sudo apt-get install -t focal-backports mingw-w64

# Verify installation
x86_64-w64-mingw32-gcc --version
```

### 2. Linker Errors

If you encounter linker errors (E0425/E4031) when building for Windows, these are common in WSL environment:

#### Quick Fix with the Helper Script
The easiest solution is to run our WSL build fix script:

```bash
npm run fix-wsl
```

This script will:
- Install all necessary MinGW packages
- Configure Cargo for cross-compilation
- Set up environment variables
- Test the compilation settings

#### Manual Fix Options

If you prefer to fix manually:

1. **Install additional MinGW packages:**
   ```bash
   sudo apt-get update
   sudo apt-get install -y mingw-w64-tools mingw-w64-x86-64-dev g++-mingw-w64-x86-64
   ```

2. **Configure Cargo:**
   ```bash
   mkdir -p ~/.cargo
   echo "[target.x86_64-pc-windows-gnu]" > ~/.cargo/config.toml
   echo "linker = \"x86_64-w64-mingw32-gcc\"" >> ~/.cargo/config.toml
   echo "ar = \"x86_64-w64-mingw32-ar\"" >> ~/.cargo/config.toml
   ```

3. **Set environment variables:**
   ```bash
   export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER=x86_64-w64-mingw32-gcc
   export CC_x86_64_pc_windows_gnu=x86_64-w64-mingw32-gcc
   export CXX_x86_64_pc_windows_gnu=x86_64-w64-mingw32-g++
   export AR_x86_64_pc_windows_gnu=x86_64-w64-mingw32-ar
   ```

4. **Add to ~/.bashrc for persistence:**
   ```bash
   cat >> ~/.bashrc << 'EOF'
   
   # Windows cross-compilation for calculator-cli
   export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER=x86_64-w64-mingw32-gcc
   export CC_x86_64_pc_windows_gnu=x86_64-w64-mingw32-gcc
   export CXX_x86_64_pc_windows_gnu=x86_64-w64-mingw32-g++
   export AR_x86_64_pc_windows_gnu=x86_64-w64-mingw32-ar
   EOF
   
   source ~/.bashrc
   ```

#### Alternative: Use Static Linking
Static linking can resolve many E4031 errors:

```bash
npm run build-windows-static
```

This uses `RUSTFLAGS='-C target-feature=+crt-static'` to create a self-contained Windows binary.

### 3. Path Issues

If path-related issues occur:

```bash
# Move the project to WSL filesystem instead of Windows drive
cp -r /mnt/c/path/to/your/calculator-cli ~/calculator-cli
cd ~/calculator-cli

# Or create a symbolic link
ln -s /mnt/c/path/to/your/calculator-cli ~/calculator-cli
```

## Optimizing Your WSL Environment

### 1. WSL2 Performance

```bash
# Move your project to WSL filesystem for better performance
rsync -av /mnt/c/path/to/your/calculator-cli ~/calculator-cli

# Then work from within WSL
cd ~/calculator-cli
```

### 2. VS Code Integration

If using VS Code with WSL:

1. Install the WSL extension
2. Open the project in WSL: `code .`
3. Ensure you're using the WSL terminal

### 3. Windows Integration

Create a Windows batch file to easily access WSL-built binaries:

```batch
@echo off
REM Run this in Windows Command Prompt
REM Path to your WSL-built Windows binary
\\wsl$\Ubuntu\home\yourusername\calculator-cli\bin\calculator-win32-x64.exe %*
```

## Testing the npm Package in WSL

### 1. Test Package Structure

```bash
node test-package.js
```

### 2. Create and Test Package

```bash
# Create the package
npm run prepare-package

# Install globally for testing
npm install -g calculator-cli-*.tgz

# Test the installation
calculator "1+1"
```

### 3. Test in Windows

The npm installation will be available in Windows as well:

1. Open Windows Command Prompt or PowerShell
2. Run `calculator "1+1"`
3. Verify the output is correct

## Publishing from WSL

Publishing from WSL works the same as from other platforms:

```bash
# Login to npm (if not already)
npm login

# Publish the package
npm publish
```

## WSL Best Practices

1. **Project Location**: Keep frequently modified projects in WSL filesystem for better performance
2. **File Access**: Use WSL paths (`/home/...`) for active development
3. **Path Mapping**: Remember that `/mnt/c` is slower than `/home/` in WSL2
4. **Tool Updates**: Regularly update both WSL packages and Rust toolchain

## Alternative: Dual Installation

For the most flexibility, consider installing Node.js and Rust both in WSL and Windows:

```bash
# In WSL
curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash -
sudo apt-get install -y nodejs

# In Windows Command Prompt
# Download and install from https://nodejs.org/
# Download and install from https://rustup.rs/
```

This allows you to:
- Build and test from the environment that works best
- Switch between Windows and WSL seamlessly
- Take advantage of each environment's strengths

WSL provides an excellent environment for developing cross-platform applications while maintaining easy access to Windows tools and file systems.