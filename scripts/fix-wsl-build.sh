#!/bin/bash

# WSL Windows Build Fix Script
# This script helps resolve E4025 and E4031 linker errors when building Windows binaries in WSL

echo "🐧 WSL Windows Build Fix Script"
echo "================================"
echo ""

# Check if we're in WSL
if ! grep -qi microsoft /proc/version; then
    echo "⚠️  This script is optimized for WSL but can run on other Linux systems"
fi

# Install complete MinGW toolchain
echo "🔧 Installing complete MinGW toolchain..."
sudo apt-get update
sudo apt-get install -y \
    mingw-w64 \
    gcc-mingw-w64-x86-64 \
    gcc-mingw-w64-i686 \
    binutils-mingw-w64-x86-64 \
    binutils-mingw-w64-i686 \
    mingw-w64-tools \
    mingw-w64-x86-64-dev \
    g++-mingw-w64-x86-64

# Create Cargo config directory if it doesn't exist
echo ""
echo "📝 Configuring Cargo for cross-compilation..."
mkdir -p ~/.cargo

# Create or update Cargo config.toml
cat > ~/.cargo/config.toml << 'EOF'
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
ar = "x86_64-w64-mingw32-ar"

[target.x86_64-pc-windows-gnu.env]
CC_x86_64_pc_windows_gnu = "x86_64-w64-mingw32-gcc"
CXX_x86_64_pc_windows_gnu = "x86_64-w64-mingw32-g++"
AR_x86_64_pc_windows_gnu = "x86_64-w64-mingw32-ar"

[target.i686-pc-windows-gnu]
linker = "i686-w64-mingw32-gcc"
ar = "i686-w64-mingw32-ar"

[target.i686-pc-windows-gnu.env]
CC_i686_pc_windows_gnu = "i686-w64-mingw32-gcc"
CXX_i686_pc_windows_gnu = "i686-w64-mingw32-g++"
AR_i686_pc_windows_gnu = "i686-w64-mingw32-ar"
EOF

# Set environment variables for current session
echo ""
echo "🔧 Setting environment variables..."
export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER=x86_64-w64-mingw32-gcc
export CC_x86_64_pc_windows_gnu=x86_64-w64-mingw32-gcc
export CXX_x86_64_pc_windows_gnu=x86_64-w64-mingw32-g++
export AR_x86_64_pc_windows_gnu=x86_64-w64-mingw32-ar

# Add to ~/.bashrc for persistence
echo ""
echo "💾 Adding environment variables to ~/.bashrc..."
cat >> ~/.bashrc << 'EOF'

# Rust cross-compilation for Windows in WSL
export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER=x86_64-w64-mingw32-gcc
export CC_x86_64_pc_windows_gnu=x86_64-w64-mingw32-gcc
export CXX_x86_64_pc_windows_gnu=x86_64-w64-mingw32-g++
export AR_x86_64_pc_windows_gnu=x86_64-w64-mingw32-ar
EOF

# Add to ~/.zshrc if zsh is used
if [ -f ~/.zshrc ]; then
    echo ""
    echo "💾 Adding environment variables to ~/.zshrc..."
    cat >> ~/.zshrc << 'EOF'

# Rust cross-compilation for Windows in WSL
export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER=x86_64-w64-mingw32-gcc
export CC_x86_64_pc_windows_gnu=x86_64-w64-mingw32-gcc
export CXX_x86_64_pc_windows_gnu=x86_64-w64-mingw32-g++
export AR_x86_64_pc_windows_gnu=x86_64-w64-mingw32-ar
EOF
fi

# Verify installation
echo ""
echo "🔍 Verifying installation..."
if command -v x86_64-w64-mingw32-gcc &> /dev/null; then
    echo "✅ MinGW GCC found"
    echo "   Version: $(x86_64-w64-mingw32-gcc --version | head -1)"
else
    echo "❌ MinGW GCC not found"
fi

if command -v x86_64-w64-mingw32-ar &> /dev/null; then
    echo "✅ MinGW AR found"
else
    echo "❌ MinGW AR not found"
fi

# Install or update Rust targets
echo ""
echo "🔧 Installing Rust targets..."
rustup target add x86_64-pc-windows-gnu
rustup target add i686-pc-windows-gnu

# Test compilation with a simple C program
echo ""
echo "🧪 Testing MinGW with a simple program..."
cat > test_minGW.c << 'EOF'
#include <stdio.h>

int main() {
    printf("Hello from Windows binary built in WSL!\n");
    return 0;
}
EOF

# Compile with MinGW
if x86_64-w64-mingw32-gcc -o test_minGW.exe test_minGW.c 2>/dev/null; then
    echo "✅ MinGW test compilation successful"
    echo "   Output: $(./test_minGW.exe 2>/dev/null || echo 'Binary created, needs Windows to run')"
    rm -f test_minGW.exe test_minGW.c
else
    echo "❌ MinGW test compilation failed"
    echo "   You may need to restart your terminal or source your shell configuration"
fi

# Clean up
rm -f test_minGW.c

echo ""
echo "🎉 WSL Windows build fix completed!"
echo ""
echo "📋 Next steps:"
echo "1. Restart your terminal or run: source ~/.bashrc"
echo "2. Try building Windows binary: npm run build-windows"
echo "3. If you still have issues, try: npm run build-windows-static"
echo ""
echo "🔧 Additional options if problems persist:"
echo "- Use MSVC toolchain: rustup toolchain install stable-x86_64-pc-windows-msvc"
echo "- Build in native Windows environment"
echo "- Use GitHub Actions for automated builds"
echo ""
echo "💡 For more help, see: WSL_BUILD_GUIDE.md"
