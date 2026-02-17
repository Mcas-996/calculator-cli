#!/usr/bin/env node

const path = require("path");
const fs = require("fs");
const os = require("os");

console.log("🔍 Validating mathcalc-cli installation...");

// Platform detection
const platform = os.platform();
const expectedBinary =
    platform === "win32"
        ? "calculator_windows-x86-64.exe"
        : "calculator-linux-x64";

const binaryPath = path.join(__dirname, "..", "bin", expectedBinary);

if (!fs.existsSync(binaryPath)) {
    console.error(`❌ Required binary not found: ${expectedBinary}`);
    process.exit(1);
}

if (!os.arch().includes("64")) {
    console.warn("⚠️ Warning: 64-bit architecture recommended");
}

console.log("✅ MathCalc CLI v0.2.0 is ready!");
