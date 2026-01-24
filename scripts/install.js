#!/usr/bin/env node
/*
 * Platform detection and installation script for calculator-cli npm package
 */

const os = require('os');
const fs = require('fs');
const path = require('path');
const https = require('https');

// Platform to binary mapping
const platforms = {
    'darwin-x64': 'calculator-macos-x64',
    'linux-x64': 'calculator-linux-x64',
    'win32-x64': 'calculator-win32-x64.exe'
};

// Detect platform
const platform = os.platform();
const arch = os.arch();
const platformKey = `${platform}-${arch}`;

console.log(`Detected platform: ${platformKey}`);

// Check if we have a precompiled binary for this platform
if (!platforms[platformKey]) {
    // ARM platforms or unsupported architectures
    console.log('');
    console.log('⚙️  ARM (Apple Silicon / ARM64) or unsupported platform detected');
    console.log('');
    console.log('calculator-cli doesn\'t include precompiled binaries for this platform.');
    console.log('To install calculator-cli on this platform:');
    console.log('');
    console.log('1. Install Rust toolchain:');
    console.log('   curl --proto \'=https\' --tlsv1.2 -sSf https://sh.rustup.rs | sh');
    console.log('   source ~/.cargo/env');
    console.log('');
    console.log('2. Install calculator-cli from source:');
    console.log('   cargo install calculator');
    console.log('');
    console.log('For more information, visit: https://github.com/anomalyco/calculator-cli');
    console.log('');

    process.exit(1);
}

// Determine binary path
const binaryName = platforms[platformKey];
const binaryPath = path.join(__dirname, '..', 'bin', binaryName);

// Check if binary already exists
if (fs.existsSync(binaryPath)) {
    console.log(`✅ Binary already exists: ${binaryPath}`);

    // On Unix systems, ensure it's executable
    if (platform !== 'win32') {
        try {
            fs.chmodSync(binaryPath, '755');
        } catch (err) {
            console.warn(`Warning: Could not set executable permissions on ${binaryPath}: ${err.message}`);
        }
    }

    process.exit(0);
}

// Extract binary from package
const packageBinaryPath = path.join(__dirname, '..', 'bin', binaryName);
const targetPath = path.join(__dirname, '..', 'bin', platform === 'win32' ? 'calculator.exe' : 'calculator');

try {
    // Create bin directory if it doesn't exist
    const binDir = path.dirname(targetPath);
    if (!fs.existsSync(binDir)) {
        fs.mkdirSync(binDir, { recursive: true });
    }

    // Copy the binary to the target location
    fs.copyFileSync(packageBinaryPath, targetPath);

    // On Unix systems, make it executable
    if (platform !== 'win32') {
        fs.chmodSync(targetPath, 0755);
    }

    console.log(`✅ Installed calculator binary to: ${targetPath}`);
} catch (err) {
    console.error(`❌ Failed to install binary: ${err.message}`);
    process.exit(1);
}
