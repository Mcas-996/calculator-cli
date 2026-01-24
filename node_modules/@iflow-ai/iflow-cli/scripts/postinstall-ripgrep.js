/**
 * @license
 * Copyright 2025 Google LLC
 * SPDX-License-Identifier: Apache-2.0
 */

/**
 * Post-install script to remove macOS quarantine attributes from ripgrep binaries.
 * This prevents ENOEXEC errors when spawning the ripgrep process.
 *
 * The quarantine attributes are added by macOS Gatekeeper when files are downloaded
 * from the internet, and can cause spawn() to fail with ENOEXEC intermittently.
 */

import { execSync } from 'child_process';
import { existsSync, readdirSync, statSync, readFileSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const vendorsDir = join(__dirname, '..', 'vendors', 'ripgrep');

/**
 * Checks if a file is a Git LFS pointer file
 * @param {string} filePath - Path to the file
 * @returns {boolean} True if the file is a Git LFS pointer
 */
function isGitLFSPointer(filePath) {
  try {
    const stat = statSync(filePath);
    // Git LFS pointers are small text files (usually < 200 bytes)
    if (stat.size > 500) {
      return false;
    }

    const content = readFileSync(filePath, 'utf8');
    return content.startsWith('version https://git-lfs.github.com/spec/');
  } catch {
    return false;
  }
}

/**
 * Removes problematic extended attributes from a file on macOS
 * This includes quarantine and cache attributes that can cause ENOEXEC
 * Note: com.apple.provenance cannot be removed but doesn't cause issues
 * @param {string} filePath - Path to the file
 */
function removeQuarantineAttribute(filePath) {
  if (process.platform !== 'darwin') {
    return; // Only needed on macOS
  }

  const attributesToRemove = [
    'com.apple.quarantine',
    'CachedFileMimeType',
    'CachedFileType',
    'FileXRayCachedResultInEA',
  ];

  let removed = false;
  for (const attr of attributesToRemove) {
    try {
      execSync(`xattr -d "${attr}" "${filePath}" 2>/dev/null`, {
        stdio: 'ignore',
      });
      removed = true;
    } catch {
      // Attribute might not exist, continue
    }
  }

  if (removed) {
    console.log(`  ✓ Removed extended attributes from ${filePath}`);
  }
}

/**
 * Recursively finds and processes ripgrep binaries
 * @param {string} dir - Directory to search
 */
function processRipgrepBinaries(dir) {
  if (!existsSync(dir)) {
    console.log(
      'Ripgrep vendors directory not found, skipping quarantine removal.',
    );
    return;
  }

  const entries = readdirSync(dir);
  let hasGitLFSPointers = false;

  for (const entry of entries) {
    const fullPath = join(dir, entry);
    const stat = statSync(fullPath);

    if (stat.isDirectory()) {
      // Recurse into subdirectories
      processRipgrepBinaries(fullPath);
    } else if (entry === 'rg' || entry === 'rg.exe') {
      // Check if this is a Git LFS pointer file
      if (isGitLFSPointer(fullPath)) {
        console.error(
          `  ✗ Error: ${fullPath} is a Git LFS pointer file, not a real binary!`,
        );
        console.error(
          `    This will cause ENOEXEC errors when trying to execute ripgrep.`,
        );
        console.error(
          `    Please ensure Git LFS is installed and run: git lfs pull`,
        );
        hasGitLFSPointers = true;
      } else {
        // Found a real ripgrep binary
        removeQuarantineAttribute(fullPath);
      }
    }
  }

  if (hasGitLFSPointers) {
    console.error(
      '\n⚠️  WARNING: Some ripgrep binaries are Git LFS pointer files.',
    );
    console.error(
      '   The search functionality will not work until these are replaced with real binaries.',
    );
    console.error('   To fix this, run: git lfs install && git lfs pull\n');
  }
}

// Main execution
if (process.platform === 'darwin') {
  console.log('Removing macOS quarantine attributes from ripgrep binaries...');
  processRipgrepBinaries(vendorsDir);
  console.log('Done!');
} else {
  console.log('Skipping quarantine removal (not on macOS)');
}
