#!/usr/bin/env node

/*
 * WSL Setup Script for calculator-cli
 * This script installs all necessary tools and dependencies for building in WSL
 */

const { execSync } = require("child_process");
const path = require("path");

console.log("🐧 Setting up calculator-cli build environment in WSL...\n");

// Function to run a command and display its output
function runCommand(command, description) {
    console.log(`🔧 ${description}...`);
    try {
        execSync(command, { stdio: "inherit" });
        console.log("✅ Success\n");
        return true;
    } catch (error) {
        console.log(`❌ Failed: ${error.message}\n`);
        return false;
    }
}

// Update package lists
runCommand("sudo apt-get update", "Updating package lists");

// Install build tools
runCommand("sudo apt-get install -y build-essential", "Installing build tools");

// Install mingw-w64 for Windows cross-compilation
runCommand("sudo apt-get install -y mingw-w64", "Installing mingw-w64");

// Try installing additional mingw packages if needed
runCommand(
    "sudo apt-get install -y gcc-mingw-w64-x86-64",
    "Installing additional mingw packages",
);

// Install Rust targets
console.log("🔧 Installing Rust target platforms...");
try {
    // Windows target
    runCommand("rustup target add x86_64-pc-windows-gnu", {
        stdio: "inherit",
        maxBuffer: 1024 * 1024,
    });
    console.log("✅ Added x86_64-pc-windows-gnu target");

    // Linux target (usually already installed)
    try {
        execSync("rustup target add x86_64-unknown-linux-gnu", {
            stdio: "inherit",
            maxBuffer: 1024 * 1024,
        });
        console.log("✅ Added x86_64-unknown-linux-gnu target");
    } catch (e) {
        console.log("ℹ️  Linux target may already be installed");
    }
    console.log("✅ Rust targets installed\n");
} catch (error) {
    console.log(`❌ Failed to install Rust targets: ${error.message}\n`);
}

// Set up environment variables for cross-compilation
console.log("💡 Setting up environment variables...");
console.log(`
Add these lines to your ~/.bashrc or ~/.zshrc:
export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER=x86_64-w64-mingw32-gcc
export CC_x86_64_pc_windows_gnu=x86_64-w64-mingw32-gcc
export CXX_x86_64_pc_windows_gnu=x86_64-w64-mingw32-g++

Then run: source ~/.bashrc or source ~/.zshrc
`);

// Verify installation
console.log("🔍 Verifying installation...");
try {
    // Check mingw installation
    execSync("x86_64-w64-mingw32-gcc --version", {
        stdio: "inherit",
        maxBuffer: 1024 * 1024,
    });
    console.log("✅ MinGW-w64 is properly installed");
} catch (error) {
    console.log(
        "⚠️  MinGW-w64 verification failed, but installation may have succeeded",
    );
}

try {
    // Check Rust targets
    const targets = execSync("rustup target list --installed", {
        encoding: "utf8",
        maxBuffer: 1024 * 1024,
    });
    if (targets.includes("x86_64-pc-windows-gnu")) {
        console.log("✅ Windows target is installed");
    } else {
        console.log("⚠️  Windows target may not be properly installed");
    }
} catch (error) {
    console.log("⚠️  Could not verify Rust targets");
}

console.log("\n🎉 WSL setup completed!");
console.log("\n📦 Next steps:");
console.log("1. Set up environment variables (see instructions above)");
console.log("2. Run: npm run build-binaries");
console.log("3. Test the built binaries");
console.log("\n💡 For more information, see WSL_BUILD_GUIDE.md");
