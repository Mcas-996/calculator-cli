#!/usr/bin/env node

/*
 * Local build script for generating platform-specific binaries for npm distribution
 * This script is designed for local compilation across multiple platforms
 *
 * To build for all platforms locally, you'll need:
 * - Rust toolchains: x86_64-pc-windows-gnu, x86_64-apple-darwin, x86_64-unknown-linux-gnu
 * - Cross-compilation tools for Windows (mingw-w64) on Linux/macOS
 */

const path = require("path");
const { execSync } = require("child_process");
const fs = require("fs");

// Target platforms and their output files
const platforms = [
    {
        target: "x86_64-unknown-linux-gnu",
        outputFile: "calculator-linux-x64",
        canBuildOn: ["linux"],
    },
    {
        target: "x86_64-pc-windows-gnu",
        outputFile: "calculator-win32-x64.exe",
        canBuildOn: ["linux", "macos", "windows"],
        requires: {
            linux: "x86_64-w64-mingw32-gcc",
            macos: "x86_64-w64-mingw32-gcc",
        },
    },
    {
        target: "x86_64-apple-darwin",
        outputFile: "calculator-darwin-x64",
        canBuildOn: ["macos"],
    },
];

console.log("🔨 Building calculator-cli binaries for npm distribution...\n");

// Check if Cargo.toml exists
const projectRoot = path.join(__dirname, "..");
if (!fs.existsSync(path.join(projectRoot, "Cargo.toml"))) {
    console.error("❌ Cargo.toml not found in project root");
    console.log("Make sure this script is run from the correct directory");
    process.exit(1);
}

// Detect current platform and WSL environment
const detectCurrentPlatform = () => {
    const platform = process.platform;
    if (platform === "linux") return "linux";
    if (platform === "darwin") return "macos";
    if (platform === "win32") return "windows";
    return "unknown";
};

// Check if running in WSL
const isWSL = () => {
    try {
        return (
            fs
                .readFileSync("/proc/version", "utf8")
                .toLowerCase()
                .includes("microsoft") ||
            process.env.WSL_DISTRO_NAME !== undefined
        );
    } catch (e) {
        return false;
    }
};

const currentPlatform = detectCurrentPlatform();
const inWSL = currentPlatform === "linux" && isWSL();
console.log(
    `📍 Detected current platform: ${currentPlatform}${inWSL ? " (in WSL)" : ""}\n`,
);

// Helper function to check if a command is available
const commandExists = (command) => {
    try {
        execSync(`which ${command}`, { stdio: "ignore" });
        return true;
    } catch (e) {
        return false;
    }
};

// Helper function to check if a target is installed
const isTargetInstalled = (target) => {
    try {
        const list = execSync("rustup target list --installed", {
            encoding: "utf8",
        });
        return list.includes(target);
    } catch (e) {
        return false;
    }
};

try {
    // Get version from Cargo.toml
    const cargoContent = fs.readFileSync(
        path.join(projectRoot, "Cargo.toml"),
        "utf8",
    );
    const versionMatch = cargoContent.match(/^version\s*=\s*"([^"]+)"/m);
    const version = versionMatch ? versionMatch[1] : "2.0.0";
    console.log(`Found version: ${version}\n`);

    // Create bin directory if it doesn't exist
    const binDir = path.join(projectRoot, "bin");
    if (!fs.existsSync(binDir)) {
        fs.mkdirSync(binDir, { recursive: true });
        console.log(`Created bin directory: ${binDir}\n`);
    }

    // Filter platforms that can be built on current platform
    const buildablePlatforms = platforms.filter((platform) =>
        platform.canBuildOn.includes(currentPlatform),
    );

    if (buildablePlatforms.length === 0) {
        console.log(`⚠️  No platforms can be built on ${currentPlatform}`);
        console.log("This script supports:");

        platforms.forEach((platform) => {
            console.log(
                `- ${platform.target}: ${platform.canBuildOn.join(", ")}`,
            );
        });

        console.log("\n💡 Consider using a different platform or a CI service");
        process.exit(1);
    }

    let successfulBuilds = 0;

    // Build for each buildable target platform
    for (const platform of buildablePlatforms) {
        console.log(`📦 Building for ${platform.target}...`);

        // Check if the target is installed
        if (!isTargetInstalled(platform.target)) {
            console.log(`❌ Rust target '${platform.target}' not installed`);
            console.log(`💡 Installing target now...`);

            try {
                execSync(`rustup target add ${platform.target}`, {
                    stdio: "inherit",
                    maxBuffer: 10 * 1024 * 1024,
                });
                console.log(
                    `✅ Target '${platform.target}' installed successfully\n`,
                );
            } catch (error) {
                console.log(`❌ Failed to install target: ${error.message}`);
                console.log(
                    `💡 Please install manually: rustup target add ${platform.target}\n`,
                );
                continue;
            }
        }

        // Check for required tools
        if (platform.requires && platform.requires[currentPlatform]) {
            const requiredTool = platform.requires[currentPlatform];
            if (!commandExists(requiredTool)) {
                console.log(`❌ Required tool '${requiredTool}' not found`);

                // Provide installation instructions
                if (
                    currentPlatform === "linux" &&
                    platform.target.includes("windows")
                ) {
                    console.log("💡 To install on Linux/Ubuntu:");
                    console.log("   sudo apt-get update");
                    console.log("   sudo apt-get install mingw-w64");
                    console.log("   # For Debian:");
                    console.log("   sudo apt-get install gcc-mingw-w64-x86-64");
                } else if (
                    currentPlatform === "macos" &&
                    platform.target.includes("windows")
                ) {
                    console.log("💡 To install on macOS:");
                    console.log("   brew install mingw-w64");
                }

                console.log("");
                continue;
            }
        }

        try {
            // Set environment variables for cross-compilation if needed
            const env = { ...process.env };

            // Configure environment for cross-compilation
            const env = { ...process.env };
            if (
                platform.target === "x86_64-pc-windows-gnu" &&
                currentPlatform !== "windows"
            ) {
                // Set the linker for Windows cross-compilation
                env.CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER =
                    "x86_64-w64-mingw32-gcc";
                env.CC_x86_64_pc_windows_gnu = "x86_64-w64-mingw32-gcc";
                env.CXX_x86_64_pc_windows_gnu = "x86_64-w64-mingw32-g++";

                // WSL-specific settings
                if (inWSL) {
                    console.log(
                        "   🐧 Applying WSL-specific settings for Windows target",
                    );
                    // Additional linker flags for WSL
                    env.CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUSTFLAGS =
                        "-C link-arg=-Wl,--subsystem,console -C link-arg=-static";

                    // Try alternative linking strategy for WSL
                    if (process.env.USE_MSVC_TOOLCHAIN) {
                        console.log(
                            "   Using MSVC toolchain for Windows target",
                        );
                        env.RUSTUP_TOOLCHAIN = "stable-x86_64-pc-windows-msvc";
                    }
                }
            }

            // Build the binary
            const buildCmd = `cargo build --release --target ${platform.target}`;
            console.log(`   Running: ${buildCmd}`);

            // Use inherit stdio and increased buffer to avoid ENOBUFS errors
            execSync(buildCmd, {
                stdio: "inherit",
                env,
                cwd: projectRoot,
                maxBuffer: 10 * 1024 * 1024, // 10MB buffer
            });

            // Copy the binary to the bin directory
            const sourcePath = path.join(
                projectRoot,
                "target",
                platform.target,
                "release",
                "calculator",
            );

            // For Windows, the binary has .exe extension
            const windowsSourcePath = path.join(
                projectRoot,
                "target",
                platform.target,
                "release",
                "calculator.exe",
            );

            const finalSourcePath = platform.target.includes("windows")
                ? windowsSourcePath
                : sourcePath;
            const destPath = path.join(binDir, platform.outputFile);

            // Verify source exists
            if (!fs.existsSync(finalSourcePath)) {
                throw new Error(
                    `Binary not found at expected path: ${finalSourcePath}`,
                );
            }

            // Copy binary
            fs.copyFileSync(finalSourcePath, destPath);

            // On Unix-like systems, make sure the binary is executable
            if (!platform.target.includes("windows")) {
                try {
                    fs.chmodSync(destPath, "755");
                } catch (error) {
                    console.warn(
                        `Warning: Could not set executable permissions: ${error.message}`,
                    );
                }
            }

            console.log(`✅ Built and copied to: ${destPath}\n`);
            successfulBuilds++;
        } catch (error) {
            console.error(
                `❌ Failed to build for ${platform.target}: ${error.message}`,
            );

            // Provide specific help for common errors
            if (
                error.message.includes("E0425") ||
                error.message.includes("E0431") ||
                error.message.includes("linker") ||
                error.message.includes("undefined reference")
            ) {
                console.log("");
                console.log(
                    "💡 This appears to be a linker error (E0425/E0431). Solutions:",
                );

                if (platform.target.includes("windows")) {
                    console.log("1. For WSL MinGW issues, try these steps:");
                    if (inWSL) {
                        console.log("   🐧 WSL-specific solutions:");
                        console.log("   a) Install complete MinGW toolchain:");
                        console.log(
                            "      sudo apt-get update && sudo apt-get install mingw-w64 gcc-mingw-w64-x86-64",
                        );
                        console.log("");
                        console.log("   b) Set environment variables for WSL:");
                        console.log(
                            "      export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER=x86_64-w64-mingw32-gcc",
                        );
                        console.log(
                            "      export CC_x86_64_pc_windows_gnu=x86_64-w64-mingw32-gcc",
                        );
                        console.log(
                            "      export CXX_x86_64_pc_windows_gnu=x86_64-w64-mingw32-g++",
                        );
                        console.log("");
                        console.log(
                            "   c) Alternative: Use Windows native Rust:",
                        );
                        console.log(
                            "      rustup toolchain install stable-x86_64-pc-windows-msvc",
                        );
                        console.log(
                            "      rustup default stable-x86_64-pc-windows-msvc",
                        );
                        console.log(
                            "      rustup target add x86_64-pc-windows-msvc",
                        );
                        console.log("");
                        console.log(
                            "   d) For persistent configuration, add to ~/.bashrc:",
                        );
                        console.log("      [target.x86_64-pc-windows-gnu]");
                        console.log("      linker = 'x86_64-w64-mingw32-gcc'");
                        console.log("      ");
                        console.log("      [target.x86_64-pc-windows-msvc]");
                        console.log("      linker = 'lld-link'");
                    } else {
                        console.log(
                            "   sudo apt-get update && sudo apt-get install mingw-w64",
                        );
                    }

                    console.log("");
                    console.log(
                        "2. Try reinstalling and reconfiguring Rust targets:",
                    );
                    console.log(
                        "   rustup target remove --toolchain stable x86_64-pc-windows-gnu",
                    );
                    console.log(
                        "   rustup target add --toolchain stable x86_64-pc-windows-gnu",
                    );
                    console.log("   # Or try MSVC target:");
                    console.log("   rustup target add x86_64-pc-windows-msvc");

                    if (inWSL) {
                        console.log("");
                        console.log("🐧 WSL Specific Tips:");
                        console.log(
                            "   - WSL MinGW builds can be tricky. Consider using:",
                        );
                        console.log(
                            "   * Windows Subsystem for Linux with Windows Rust installed in Windows",
                        );
                        console.log(
                            "   * The MSVC target which might be more stable in WSL",
                        );
                        console.log(
                            "   * Running the build directly in Windows instead of WSL",
                        );
                    }

                    console.log("");
                    console.log("3. Alternative approaches:");
                    console.log("   - Build in native Windows environment");
                    console.log(
                        "   - Use GitHub Actions or another CI service",
                    );
                    console.log(
                        "   - Focus on platforms that build successfully",
                    );
                }
            } else if (error.message.includes("could not find")) {
                console.log("");
                console.log(`💡 Rust target not found. To install:`);
                console.log(`   rustup target add ${platform.target}`);
                console.log("");
                console.log("To see all installed targets:");
                console.log("   rustup target list --installed");
            }

            console.log("");
        }
    }

    // Build summary
    console.log("📋 Build Summary:");
    platforms.forEach((platform) => {
        const destPath = path.join(binDir, platform.outputFile);
        if (fs.existsSync(destPath)) {
            console.log(`✅ ${platform.outputFile}`);
        } else {
            const canBuild = platform.canBuildOn.includes(currentPlatform)
                ? "buildable"
                : "requires different platform";
            console.log(`❌ ${platform.outputFile} (${canBuild})`);
        }
    });

    console.log(
        `\nSuccessfully built ${successfulBuilds} out of ${buildablePlatforms.length} platforms`,
    );

    if (successfulBuilds === 0) {
        console.log("\n❌ No binaries were built successfully");
        console.log("\n💡 Troubleshooting:");
        console.log("1. Install missing Rust targets:");
        buildablePlatforms.forEach((platform) => {
            console.log(`   rustup target add ${platform.target}`);
        });
        console.log("\n2. Install cross-compilation tools:");
        if (currentPlatform === "linux") {
            console.log(
                "   sudo apt-get update && sudo apt-get install mingw-w64",
            );
        } else if (currentPlatform === "macos") {
            console.log("   brew install mingw-w64");
        }
        console.log("\n3. See LOCAL_BUILD_GUIDE.md for detailed instructions");
        console.log("\n4. Consider building only for your current platform");
    } else {
        console.log("\n✅ Build completed successfully!");
    }

    console.log("\n📦 Next steps:");
    if (successfulBuilds > 0) {
        console.log("1. Test the package: node test-package.js");
        console.log("2. Create package: npm run prepare-package");
        console.log("3. Test locally: npm run local-test");
        console.log("4. Publish to npm: npm publish");
        console.log(
            "\n💡 Remember to update the version number in Cargo.toml before publishing!",
        );

        if (inWSL) {
            console.log("\n🐧 WSL-Specific Tips:");
            console.log("- Windows binaries are accessible from Windows at:");
            console.log(
                `  \\\\wsl$\\$(hostname)\\${path
                    .resolve(projectRoot)
                    .replace(/^\/mnt\//, "")
                    .replace(/\//g, "\\")}\\bin`,
            );
            console.log(
                "- Test Windows binaries directly in Windows Command Prompt or PowerShell",
            );
        }
    } else {
        console.log("1. Fix the build issues shown above");
        console.log("2. Try building for fewer platforms");
        console.log("3. Consider building only for your current platform");

        if (inWSL) {
            console.log("\n🐧 WSL-Specific Tips:");
            console.log("- Ensure you have the latest mingw-w64 packages");
            console.log(
                "- Try running the build script from /mnt/c/ to avoid WSL path limitations",
            );
        }
    }
} catch (error) {
    console.error(`❌ Build process failed: ${error.message}`);
    process.exit(1);
}
