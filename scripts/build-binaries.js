#!/usr/bin/env node

/*
 * Local build script for generating platform-specific binaries for npm distribution
 * This script builds calculator-cli for x64 platforms and places them in the appropriate bin directory
 * Intended for local execution by maintainers before publishing to npm
 */

const path = require("path");
const { execSync } = require("child_process");
const fs = require("fs");

// Target platforms and their output files
const platforms = [
    {
        target: "x86_64-unknown-linux-gnu",
        outputFile: "calculator-linux-x64",
    },
    {
        target: "x86_64-pc-windows-gnu",
        outputFile: "calculator-win32-x64.exe",
    },
    {
        target: "x86_64-apple-darwin",
        outputFile: "calculator-darwin-x64",
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
        console.log(`Created bin directory: ${binDir}`);
    }

    // Build for each target platform
    for (const platform of platforms) {
        console.log(`Building for ${platform.target}...`);

        try {
            // Build the binary
            execSync(`cargo build --release --target ${platform.target}`, {
                stdio: "inherit",
                cwd: projectRoot,
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

            console.log(`✅ Built and copied to: ${destPath}`);
        } catch (error) {
            console.error(
                `❌ Failed to build for ${platform.target}: ${error.message}`,
            );

            // Check if the target toolchain is installed
            if (error.message.includes("could not find")) {
                console.log(`\n💡 To install the target toolchain, run:`);
                console.log(`   rustup target add ${platform.target}`);
            }
        }

        console.log(""); // Add empty line for readability
    }

    console.log("🎉 Local build process completed!");
    console.log("\n📦 Next steps for publishing:");
    console.log("1. Verify binaries are working: node test-package.js");
    console.log("2. Create package: npm run prepare-package");
    console.log("3. Test locally: npm run local-test");
    console.log("4. Publish to npm: npm publish");
    console.log(
        "\n💡 Version should be updated in both Cargo.toml and package.json!",
    );
} catch (error) {
    console.error(`❌ Build process failed: ${error.message}`);
    if (error.message.includes("could not find")) {
        console.log("\n💡 To install missing target toolchains, run:");
        for (const platform of platforms) {
            if (!platform.built) {
                console.log(`   rustup target add ${platform.target}`);
            }
        }
    }
    process.exit(1);
}
