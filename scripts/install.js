#!/usr/bin/env node

const path = require('path');
const fs = require('fs');
const os = require('os');

// Platform detection
const platform = os.platform();
console.log(`Detected platform: ${platform}`);
console.log(`Detected architecture: ${os.arch()}`);

// Map platforms to binary files
const binaries = {
  'win32': 'calculator_windows-x86-64.exe',
  'linux': 'calculator_linux-x64'
};

// Check if platform is supported
if (!binaries[platform]) {
  console.error(`
❌ Platform ${platform} is not supported by calculator-cli v0.1.0

Supported platforms:
- Windows (x64)
- Linux (x64)

For other platforms, please build from source:
git clone https://github.com/anomalyco/calculator-cli.git
cd calculator-cli
cargo build --release
  `);
  process.exit(1);
}

// Binary path
const binaryName = binaries[platform];
const binaryPath = path.join(__dirname, '..', 'bin', binaryName);

// Verify binary exists
if (!fs.existsSync(binaryPath)) {
  console.error(`❌ Binary not found: ${binaryPath}`);
  console.error('Please ensure the binary files are present in the bin directory.');
  console.error('For Windows users: Place calculator_windows-x86-64.exe in bin/');
  console.error('For Linux users: Place calculator_linux-x64 in bin/');
  process.exit(1);
}

// Create symlink or wrapper for node_modules/.bin
const targetDir = path.join(__dirname, '..', 'node_modules', '.bin');
const targetPath = path.join(targetDir, 'calculator');

try {
  // Ensure .bin directory exists
  if (!fs.existsSync(targetDir)) {
    fs.mkdirSync(targetDir, { recursive: true });
  }

  if (platform === 'win32') {
    // Create a batch file wrapper for Windows
    const batchContent = `@echo off
"${binaryPath}" %*`;
    fs.writeFileSync(`${targetPath}.cmd`, batchContent);
    console.log('✅ Created Windows batch file');
  } else {
    // Create symbolic link for Unix-like systems
    if (fs.existsSync(targetPath)) {
      fs.unlinkSync(targetPath);
    }
    fs.symlinkSync(binaryPath, targetPath);
    // Ensure executable permissions
    fs.chmodSync(targetPath, '755');
    console.log('✅ Created symbolic link with executable permissions');
  }
} catch (error) {
  console.error(`❌ Failed to create command link: ${error.message}`);
  process.exit(1);
}

console.log('✅ Installation complete!');
console.log(`You can now run 'calculator' in your terminal.`);
```

<file_path>
calculator-cli/scripts/postinstall.js
</file_path>

<edit_description>
Create postinstall validation script
</edit_description>

```javascript
#!/usr/bin/env node

const path = require('path');
const fs = require('fs');
const os = require('os');

console.log('✅ Validating calculator-cli installation...');

// Check for proper binary based on platform
const platform = os.platform();
const expectedBinary = platform === 'win32'
  ? 'calculator_windows-x86-64.exe'
  : 'calculator_linux-x64';

const binaryPath = path.join(__dirname, '..', 'bin', expectedBinary);

if (fs.existsSync(binaryPath)) {
  console.log(`✅ Binary found: ${expectedBinary}`);
} else {
  console.error(`❌ Binary not found: ${expectedBinary}`);
  console.error('Please ensure the binary files are present in the bin directory.');
  process.exit(1);
}

// Verify command is accessible
const targetPath = path.join(__dirname, '..', 'node_modules', '.bin');
const commandPath = platform === 'win32'
  ? path.join(targetPath, 'calculator.cmd')
  : path.join(targetPath, 'calculator');

if (fs.existsSync(commandPath)) {
  console.log('✅ Command link created successfully');
} else {
  console.error('❌ Command link not found');
  process.exit(1);
}

console.log('✅ Installation validated successfully!');
```

<file_path>
calculator-cli/scripts/test.js
</file_path>

<edit_description>
Create test script to validate the binary
</edit_description>

```javascript
#!/usr/bin/env node

const path = require('path');
const fs = require('fs');
const os = require('os');
const { execSync } = require('child_process');

console.log('🧪 Testing calculator-cli binary...');

// Platform detection
const platform = os.platform();
const binaryName = platform === 'win32'
  ? 'calculator_windows-x86-64.exe'
  : 'calculator_linux-x64';

const binaryPath = path.join(__dirname, '..', 'bin', binaryName);

// Check if binary exists
if (!fs.existsSync(binaryPath)) {
  console.error(`❌ Binary not found: ${binaryName}`);
  process.exit(1);
}

// Test basic calculation
try {
  console.log('🧮 Testing basic calculation: 2 + 2');
  const result = execSync(`"${binaryPath}" "2 + 2"`, { encoding: 'utf8' }).trim();

  if (result && result.includes('4')) {
    console.log(`✅ Basic calculation test passed: ${result}`);
  } else {
    console.log(`❌ Unexpected result: ${result}`);
    process.exit(1);
  }
} catch (error) {
  console.error(`❌ Failed to run binary: ${error.message}`);
  process.exit(1);
}

// Test equation solving
try {
  console.log('🧮 Testing equation solving: x^2-4=0');
  const equationResult = execSync(`"${binaryPath}" "x^2-4=0"`, { encoding: 'utf8' }).trim();

  console.log(`✅ Equation solving result: ${equationResult}`);
} catch (error) {
  console.error(`❌ Failed to solve equation: ${error.message}`);
  process.exit(1);
}

console.log('🎉 All tests passed! Calculator CLI is working correctly.');
```

<file_path>
calculator-cli/index.js
</file_path>

<edit_description>
Create simple entry point
</edit_description>

```javascript
#!/usr/bin/env node

// Entry point for the calculator-cli npm package
// This file is minimal because the actual binary handles all functionality

console.log('calculator-cli v0.1.0');
console.log('Run "calculator <expression>" to use the calculator');
```

<file_path>
calculator-cli/README.md
</file_path>

<edit_description>
Create simple README for v0.1 release
</edit_description>

```markdown
# Calculator CLI v0.1.0

A lightweight command-line calculator with symbolic math support, available for Windows and Linux x64 platforms.

## Quick Start

1. Install: `npm install -g calculator-cli`
2. Run: `calculator "2 + 2"` or `calculator "x^2-4=0"`
3. For interactive mode: `calculator` (no arguments)

## Features

- Basic arithmetic operations
- Equation solving
- Complex number support
- Fraction and symbolic output when possible

## Supported Platforms

- Windows (x64)
- Linux (x64)

## Usage Examples

```bash
# Basic calculations
calculator "2 + 2"
calculator "3 * (4 + 5)"

# Equation solving
calculator "x^2-4=0"
calculator "x+y=5, x-y=1"

# Complex numbers
calculator "sqrt(-4)"
calculator "(3+2i) * (1-i)"
```

## Installation Notes

This package includes precompiled binaries for Windows and Linux x64. No Rust toolchain is required.

## License

MIT License
