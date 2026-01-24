## Technical Design for npm Distribution

### Context
The calculator-cli is a Rust-based command-line tool currently distributed through GitHub releases and source compilation. To increase accessibility and adoption, we need to distribute it via npm, the JavaScript ecosystem's package manager. We will optimize the package size by including precompiled binaries only for the most common platforms (x64), while ARM users will need to build from source using cargo.

### Goals / Non-Goals
**Goals:**
- Provide seamless installation via npm for majority of users (x64 platforms)
- Leverage npm's version management and update capabilities
- Minimize package size by bundling only essential binaries
- Maintain current functionality and behavior
- Automate the publishing process to reduce maintenance overhead

**Non-Goals:**
- Provide precompiled binaries for all platforms
- Rewrite any core functionality in JavaScript
- Add JavaScript-specific features to the calculator
- Change the command-line interface or behavior

### Decisions

#### 1. Binary Packaging Approach
**Decision:** Use a hybrid approach with precompiled binaries only for x64 platforms (Windows, macOS Intel, Linux x64), with cargo build fallback for ARM platforms.

**Rationale:**
- Reduces package size significantly by excluding less common platforms
- x64 platforms represent the vast majority of npm users
- ARM users are typically developers comfortable with cargo build
- npm install scripts can cleanly detect architecture and provide appropriate instructions

#### 2. Platform Detection
**Decision:** Implement platform detection in the install script to provide precompiled binaries for x64 platforms and clear instructions for ARM users.

**Rationale:**
- Provides seamless user experience for majority of users
- Maintains accessibility for ARM users without bloating the package
- Clear messaging about supported platforms helps set user expectations
- Follows established patterns from other Rust-based CLI tools in npm

#### 3. CI/CD Integration
**Decision:** Use GitHub Actions to build binaries only for the supported platforms (x64) and publish to npm automatically on tagged releases.

**Rationale:**
- Simplifies the build process to focus on majority platforms
- Reduces CI resource usage and time
- Ensures consistent builds across supported platforms
- Automates release process while maintaining quality

#### 4. Package Naming
**Decision:** Publish as `calculator-cli` on npm if available, with a fallback name prepared.

**Rationale:**
- Matches the project's existing branding
- Clear indication of tool's purpose
- Easy to discover in npm searches

### Technical Implementation

#### Platform Support Matrix
```
| Platform    | Architecture | Distribution Method | Status |
|-------------|--------------|---------------------|--------|
| Windows     | x64          | Precompiled binary  | ✅ Supported |
| macOS       | x64 (Intel)  | Precompiled binary  | ✅ Supported |
| macOS       | arm64 (M1/M2)| Cargo build required| ⚙️ Supported with cargo |
| Linux       | x64          | Precompiled binary  | ✅ Supported |
| Linux       | arm64        | Cargo build required| ⚙️ Supported with cargo |
| Other       | Any          | Cargo build required| ⚙️ Supported with cargo |
```

#### Package Structure
```
calculator-cli/
├── bin/
│   ├── calculator-linux-x64
│   ├── calculator-darwin-x64
│   └── calculator-win32-x64.exe
├── scripts/
│   ├── install.js  # Platform detection and binary setup
│   └── postinstall.js  # Symlink creation and ARM instructions
├── package.json
└── README.md
```

#### package.json Configuration
```json
{
  "name": "calculator-cli",
  "version": "2.0.0",
  "description": "A command-line calculator with symbolic math support",
  "main": "index.js",
  "bin": {
    "calculator": "./bin/calculator"
  },
  "scripts": {
    "preinstall": "node scripts/install.js",
    "postinstall": "node scripts/postinstall.js",
    "test": "node scripts/test.js"
  },
  "os": ["darwin", "linux", "win32"],
  "cpu": ["x64", "arm64"],
  "files": [
    "bin/",
    "scripts/",
    "README.md"
  ]
}
```

#### Install Script Logic
1. Detect platform (OS, architecture) using `process.platform` and `process.arch`
2. If the platform is x64:
   - Map to appropriate precompiled binary filename
   - Extract and install the precompiled binary
   - Set executable permissions on Unix systems
   - Create symlink or wrapper script in `bin/calculator`
3. If the platform is ARM:
   - Display informative message about requiring cargo build
   - Provide clear instructions for building from source
   - Exit with appropriate code to indicate manual steps needed

#### ARM User Experience
When ARM users attempt to install via npm, they will see:
```
⚙️  ARM (Apple Silicon / ARM64) detected
calculator-cli doesn't include precompiled binaries for ARM architecture.
To install calculator-cli on ARM:

1. Ensure Rust toolchain is installed:
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

2. Install calculator-cli from source:
   cargo install calculator

For more information, visit: https://github.com/anomalyco/calculator-cli#installation
```

#### CI Workflow Steps
1. Trigger on tag push matching pattern `v*`
2. Separate build jobs for x64 platforms (linux, macos, windows)
3. Compile Rust release binary for each platform
4. Upload build artifacts between stages
5. Publish stage downloads all artifacts and publishes to npm
6. Update package.json version to match the tag

### Migration Plan
1. Prepare all CI configuration and scripts in a feature branch
2. Test package creation on a test npm account
3. After successful testing, merge to main and publish first version
4. Monitor for any issues and create follow-up patches as needed
5. Update documentation with platform-specific installation instructions

### Risks / Trade-offs
- **Risk:** ARM users may have reduced user experience (need to use cargo)
  - **Mitigation:** Provide clear, friendly instructions and links to Rust installer
- **Risk:** Users might be confused about platform differences
  - **Mitigation:** Clearly document supported platforms and installation methods
- **Trade-off:** Reduced package size vs immediate installation for all platforms
  - **Justification:** x64 platforms represent the majority of npm users (estimated >90%)
- **Trade-off:** Additional complexity in install scripts
  - **Mitigation:** Use established patterns from other npm packages

### Open Questions
- Should we provide a simple wrapper npm script that automates cargo install for ARM users?
- How should we handle version synchronization between Cargo.toml and package.json?
- Should we implement npm version updates to automatically trigger new releases?
- Should we add a check for Rust availability in the install script for ARM platforms?