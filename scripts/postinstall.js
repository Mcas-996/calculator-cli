#!/usr/bin/env node
/*
 * Post-install script for calculator-cli npm package
 * Ensures proper execution permissions and creates symlinks
 */

const fs = require('fs');
const path = require('path');
const os = require('os');

const platform = os.platform();
const calculatorBinary = platform === 'win32' ? 'calculator.exe' : 'calculator';
const binaryPath = path.join(__dirname, '..', 'bin', calculatorBinary);

try {
    // Verify the binary exists
    if (!fs.existsSync(binaryPath)) {
        console.error('❌ Calculator binary not found after installation');
        process.exit(1);
    }

    // On Unix systems, ensure executable permissions
    if (platform !== 'win32') {
        try {
            fs.chmodSync(binaryPath, '755');
            console.log('✅ Set executable permissions');
        } catch (err) {
            console.warn(`⚠️  Could not set executable permissions: ${err.message}`);
        }
    }

    console.log('✅ Calculator CLI ready to use!');
} catch (error) {
    console.error(`❌ Post-installation failed: ${error.message}`);
    process.exit(1);
}
