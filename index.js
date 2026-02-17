#!/usr/bin/env node

const os = require("os");
const path = require("path");
const fs = require("fs");
const { spawnSync } = require("child_process");

// Determine platform and appropriate binary
const platform = os.platform();
const isWindows = platform === "win32";
const arch = os.arch();

if (!arch.includes("64")) {
    console.error("❌ Calculator CLI only supports 64-bit architectures");
    process.exit(1);
}

let binaryPath, calculatorName;

if (isWindows) {
    calculatorName = "calculator_windows-x86-64.exe";
    binaryPath = path.join(__dirname, "bin", calculatorName);
} else if (platform === "linux") {
    calculatorName = "calculator-linux-x64";
    binaryPath = path.join(__dirname, "bin", calculatorName);
} else {
    console.error("❌ Platform not supported:", platform);
    console.log(
        "For macOS (Apple Silicon) or other platforms, please install from source:",
    );
    console.log("  cargo install calculator");
    process.exit(1);
}

// Check if binary exists
if (!fs.existsSync(binaryPath)) {
    console.error(`❌ Calculator binary not found: ${calculatorName}`);
    console.log(
        "Please ensure the binary files are present in the bin directory.",
    );
    console.log("\nFor Windows:");
    console.log("  Place calculator_windows-x86-64.exe in bin/");
    console.log("\nFor Linux:");
    console.log("  Place calculator-linux-x64 in bin/");
    console.log("\nDownload releases from:");
    console.log("  https://github.com/Mcas-996/calculator-cli/releases");
    process.exit(1);
}

// Make sure binary is executable (Unix-like systems)
if (!isWindows) {
    try {
        fs.chmodSync(binaryPath, "755");
    } catch (err) {
        console.warn(
            `⚠️ Warning: Could not set executable permissions: ${err.message}`,
        );
    }
}

// Handle command line arguments
const args = process.argv.slice(2);

if (args.length === 0) {
    // Display help when no arguments provided
    console.log("Calculator CLI v0.2.0 - Command Line Calculator");
    console.log("");
    console.log("Usage:");
    console.log('  mathcalc "expression"    Calculate an expression');
    console.log('  mathcalc "equation"     Solve an equation');
    console.log("  mathcalc                Interactive mode");
    console.log("");
    console.log("Examples:");
    console.log('  mathcalc "2 + 2"        # => 4');
    console.log('  mathcalc "x^2-4=0"      # => x = 2, -2');
    console.log('  mathcalc "(3+2i)*(1-i)" # => 5 + i');
    console.log("");
    console.log("For more help:");
    console.log("  mathcalc --help");
    process.exit(0);
}

try {
    const result = spawnSync(binaryPath, args, { stdio: "inherit" });
    process.exit(result.status || 0);
} catch (error) {
    console.error(`Error: ${error.message}`);
    process.exit(1);
}
