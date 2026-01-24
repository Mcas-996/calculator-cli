#!/usr/bin/env node

/*
 * Entry point for calculator-cli npm package
 * This is a thin wrapper that delegates to the binary
 */

const path = require('path');
const { spawn } = require('child_process');
const os = require('os');

// Determine the platform and architecture
const platform = os.platform();
const arch = os.arch();

// Check if this is an ARM platform
if (arch === 'arm64') {
  console.error(`
⚙️  ARM (Apple Silicon / ARM64) detected
calculator-cli doesn't include precompiled binaries for ARM architecture.
To install calculator-cli on ARM:

1. Ensure Rust toolchain is installed:
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

2. Install calculator-cli from source:
   cargo install calculator

For more information, visit: https://github.com/anomalyco/calculator-cli#installation
`);
  process.exit(1);
}

// Determine binary path based on platform
let binaryName;
switch (platform) {
  case 'win32':
    binaryName = 'calculator-win32-x64.exe';
    break;
  case 'darwin':
    binaryName = 'calculator-darwin-x64';
    break;
  case 'linux':
    binaryName = 'calculator-linux-x64';
    break;
  default:
    console.error(`❌ Unsupported platform: ${platform}`);
    process.exit(1);
}

// Path to the binary
const binaryPath = path.join(__dirname, 'bin', binaryName);

// Check if binary exists
if (require('fs').existsSync(binaryPath)) {
  // Spawn the binary process with all arguments
  const child = spawn(binaryPath, process.argv.slice(2), { stdio: 'inherit' });

  // Forward exit code
  child.on('exit', (code) => {
    process.exit(code);
  });
} else {
  console.error(`❌ Calculator binary not found: ${binaryPath}`);
  console.error('Please reinstall the package or build from source.');
  process.exit(1);
}
