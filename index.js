#!/usr/bin/env node

// Entry point for the calculator-cli npm package
// This is a minimal wrapper that delegates to the compiled binary

const path = require("path");
const { spawnSync } = require("child_process");
const os = require("os");

// Determine the appropriate binary based on platform
const platform = os.platform();
const isWindows = platform === "win32";

// Binary file names
const binaryName = isWindows
    ? "calculator_windows-x86-64.exe"
    : "calculator_linux-x64";
const binaryPath = path.join(__dirname, "bin", binaryName);

function runCalculator(args) {
    if (!require("fs").existsSync(binaryPath)) {
        console.error(`Error: Calculator binary not found for ${platform}`);
        process.exit(1);
    }

    try {
        const result = spawnSync(binaryPath, args, { stdio: "inherit" });
        process.exit(result.status);
    } catch (error) {
        console.error(`Error: ${error.message}`);
        process.exit(1);
    }
}

// If called from npm scripts, pass all arguments to the binary
if (require.main === module) {
    runCalculator(process.argv.slice(2));
}

module.exports = { runCalculator };
