/**
 * @license
 * Copyright 2025 Google LLC
 * SPDX-License-Identifier: Apache-2.0
 */

import { execSync } from 'child_process';
import { readdirSync, existsSync, statSync } from 'fs';
import { join, dirname, resolve } from 'path';
import { fileURLToPath } from 'url';
import os from 'os';

// 获取当前脚本的目录
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// 验证路径是否安全
function validatePath(path) {
  if (!path || typeof path !== 'string') {
    return false;
  }

  // 防止路径遍历攻击
  const normalizedPath = resolve(path);
  return !normalizedPath.includes('..');
}

// 检查文件是否存在且可执行
function isExecutable(filePath) {
  try {
    const stats = statSync(filePath);
    return stats.isFile() && (stats.mode & parseInt('111', 8)) !== 0;
  } catch {
    return false;
  }
}

// 获取 VS Code 命令的跨平台函数
function getVSCodeCommand() {
  const platform = os.platform();

  // 首先尝试在 PATH 中查找
  const pathCommand = platform === 'win32' ? 'where code' : 'which code';
  try {
    const result = execSync(pathCommand, {
      stdio: 'pipe',
      encoding: 'utf8',
      timeout: 5000 // 5秒超时
    }).trim();

    if (result && validatePath(result)) {
      return platform === 'win32' ? `"${result?.split(os.EOL)[0]}"` : result;
    }
  } catch (_error) {
    // 继续尝试其他路径
  }

  switch (platform) {
    case 'darwin': { // macOS
      const macPaths = [
        '/usr/local/bin/code',
        '/Applications/Visual Studio Code.app/Contents/Resources/app/bin/code'
      ];

      for (const path of macPaths) {
        if (validatePath(path) && isExecutable(path)) {
          return `"${path}"`;
        }
      }
      return 'code';
    }
    case 'win32': { // Windows
      const username = process.env.USERNAME || process.env.USER || '';
      const windowsPaths = [
        `C:\\Users\\${username}\\AppData\\Local\\Programs\\Microsoft VS Code\\bin\\code.cmd`,
        'C:\\Program Files\\Microsoft VS Code\\bin\\code.cmd',
        'C:\\Program Files (x86)\\Microsoft VS Code\\bin\\code.cmd'
      ];

      for (const path of windowsPaths) {
        if (validatePath(path) && existsSync(path)) {
          return `"${path}"`;
        }
      }
      return 'code';
    }
    case 'linux': { // Linux
      const linuxPaths = [
        '/usr/bin/code',
        '/usr/local/bin/code',
        '/opt/visual-studio-code/bin/code',
        '/snap/bin/code'
      ];

      for (const path of linuxPaths) {
        if (validatePath(path) && isExecutable(path)) {
          return `"${path}"`;
        }
      }
      return 'code';
    }
    default:
      return 'code';
  }
}

function installVSCodeExtension() {
  try {
    // 获取当前模块的安装目录
    const possibleDirs = [
      join(__dirname, '..', 'bundle'),  // 本地开发
      join(process.cwd(), 'bundle')  // 备用路径
    ].filter(dir => validatePath(dir));

    let actualBundleDir = null;
    for (const dir of possibleDirs) {
      try {
        if (existsSync(dir)) {
          const stats = statSync(dir);
          if (stats.isDirectory()) {
            actualBundleDir = dir;
            break;
          }
        }
      } catch (error) {
        console.warn(`[VS Code Extension] Cannot access directory ${dir}:`, error.message);
        continue;
      }
    }

    if (!actualBundleDir) {
      console.warn('[VS Code Extension] Bundle directory not found, skipping installation');
      return;
    }

    console.info(`[VS Code Extension] Using bundle directory: ${actualBundleDir}`);

    const codeCommand = getVSCodeCommand();

    // 检查 VS Code 是否可用
    try {
      execSync(`${codeCommand} --version`, {
        stdio: 'ignore',
        timeout: 10000 // 10秒超时
      });
    } catch (error) {
      console.info('[VS Code Extension] VS Code not installed or code command unavailable, skipping extension installation');
      console.debug(`[VS Code Extension] VS Code check failed:`, error.message);
      return;
    }

    // 查找 vsix 文件
    let files;
    try {
      files = readdirSync(actualBundleDir);
    } catch (error) {
      console.error(`[VS Code Extension] Cannot read bundle directory:`, error.message);
      return;
    }

    const vsixFiles = files
      .filter(file => {
        if (!file || typeof file !== 'string') return false;
        return file.toLowerCase().endsWith('.vsix') && !file.includes('..');
      })
      .filter(file => {
        const fullPath = join(actualBundleDir, file);
        return validatePath(fullPath) && existsSync(fullPath);
      });

    if (vsixFiles.length === 0) {
      console.warn('[VS Code Extension] No valid VS Code extension files (.vsix) found');
      return;
    }

    console.info(`[VS Code Extension] Found ${vsixFiles.length} extension(s) to install`);

    // 安装每个找到的 vsix 文件
    let successCount = 0;
    let failCount = 0;

    for (const vsixFile of vsixFiles) {
      const vsixPath = join(actualBundleDir, vsixFile);
      console.info(`[VS Code Extension] Installing: ${vsixFile}`);

      try {
        execSync(`${codeCommand} --install-extension "${vsixPath}"`, {
          stdio: 'inherit',
          timeout: 30000 // 30秒超时
        });
        console.info(`[VS Code Extension] ✔ Successfully installed: ${vsixFile}`);
        successCount++;
      } catch (installError) {
        console.error(`[VS Code Extension] ❌ Failed to install: ${vsixFile}`);
        console.debug(`[VS Code Extension] Install error:`, installError.message);
        failCount++;
      }
    }

    console.info(`[VS Code Extension] Installation complete: ${successCount} succeeded, ${failCount} failed`);
  } catch (error) {
    console.error('[VS Code Extension] Unexpected error during VS Code extension installation:', error.message);
    console.debug('[VS Code Extension] Stack trace:', error.stack);
  }
}

// 只在非 CI 环境执行
if (!process.env.CI) {
  installVSCodeExtension();
}
