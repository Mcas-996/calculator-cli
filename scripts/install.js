#!/usr/bin/env node

const path = require("path");
const fs = require("fs");
const os = require("os");

// Platform detection
const platform = os.platform();
console.log(`Detected platform: ${platform}`);
console.log(`Detected architecture: ${os.arch()}`);

// Map platforms to binary files
const binaries = {
    win32: "calculator_windows-x86-64.exe",
    linux: "calculator-linux-x64",
};

// Check if platform is supported
if (!binaries[platform]) {
    console.error(`
❌ Platform ${platform} is not supported by mathcalc-cli v0.2.0

Supported platforms:
- Windows (x64)
- Linux (x64)

For other platforms, please build from source:
git clone https://github.com/Mcas-996/calculator-cli.git
cd calculator-cli
cargo build --release
  `);
    process.exit(1);
}

// Binary path
const binaryName = binaries[platform];
const binaryPath = path.join(__dirname, "..", "bin", binaryName);

// Verify binary exists
if (!fs.existsSync(binaryPath)) {
    console.error(`❌ Binary not found: ${binaryPath}`);
    console.error(
        "Please ensure the binary files are present in the bin directory.",
    );
    console.error(
        "For Windows users: Place calculator_windows-x86-64.exe in bin/",
    );
    console.error("For Linux users: Place calculator-linux-x64 in bin/");
    process.exit(1);
}

// Create symlink or wrapper for node_modules/.bin
const targetDir = path.join(__dirname, "..", "node_modules", ".bin");
const targetPath = path.join(targetDir, "mathcalc");

try {
    // Ensure .bin directory exists
    if (!fs.existsSync(targetDir)) {
        fs.mkdirSync(targetDir, { recursive: true });
    }

    if (platform === "win32") {
        // Create a batch file wrapper for Windows
        const batchContent = `@echo off
"${binaryPath}" %*`;
        fs.writeFileSync(`${targetPath}.cmd`, batchContent);
        console.log("✅ Created Windows batch file");
    } else {
        // Create symbolic link for Unix-like systems
        if (fs.existsSync(targetPath)) {
            fs.unlinkSync(targetPath);
        }
        fs.symlinkSync(binaryPath, targetPath);
        // Ensure executable permissions
        fs.chmodSync(targetPath, "755");
        console.log("✅ Created symbolic link with executable permissions");
    }
} catch (error) {
    console.error(`❌ Failed to create command link: ${error.message}`);
    console.error("Creating alternative wrapper script...");

    try {
        // Create a wrapper script instead
        const wrapperContent = `#!/usr/bin/env node
const { spawnSync } = require('child_process');
const result = spawnSync('${binaryPath}', process.argv.slice(2), { stdio: 'inherit' });
process.exit(result.status || 0);
`;
        fs.writeFileSync(targetPath, wrapperContent);
        fs.chmodSync(targetPath, "755");
        console.log("✅ Created wrapper script");
    } catch (wrapperError) {
        console.error(`❌ Failed to create wrapper: ${wrapperError.message}`);
        process.exit(1);
    }
}

console.log("✅ Installation complete!");
console.log(`You can now run 'mathcalc' in your terminal.`);
