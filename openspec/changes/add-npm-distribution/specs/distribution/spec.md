# Distribution Specification

## ADDED Requirements

### Requirement: npm Package Distribution
The calculator-cli tool SHALL be distributable via the npm package registry to enable installation through standard npm workflows for x64 platforms, with cargo build required for ARM platforms.

#### Scenario: Successful npm Installation on x64
- **WHEN** a user on an x64 platform runs `npm install -g calculator-cli`
- **THEN** the calculator binary is installed with correct permissions
- **AND** the binary is available system-wide in the user's PATH
- **AND** the binary functionality matches the standard release build

#### Scenario: ARM Platform Detection and Fallback
- **WHEN** a user on an ARM platform (Apple Silicon, ARM64 Linux) runs `npm install -g calculator-cli`
- **THEN** the installation script detects the ARM architecture
- **AND** displays clear instructions for building from source using cargo
- **AND** the installation terminates gracefully without trying to install non-existent binaries

#### Scenario: Platform-specific Binary Selection
- **WHEN** a user installs the package on a supported x64 platform
- **THEN** the installation script detects the user's operating system and architecture
- **AND** the appropriate pre-compiled binary is installed
- **AND** fallback behavior handles unsupported platforms with clear messaging

### Requirement: Version Synchronization
The calculator-cli npm package SHALL maintain version consistency with the Rust crate releases.

#### Scenario: Version Alignment
- **WHEN** a new release is tagged in the repository
- **THEN** the automated CI publishes to npm with the matching version number
- **AND** the package.json version is automatically updated to match Cargo.toml

### Requirement: Platform-specific Distribution Strategy
The calculator-cli npm package SHALL support different installation methods based on platform architecture.

#### Scenario: x64 Platform Support
- **WHEN** a user on Windows (x64), macOS (Intel/x64), or Linux (x64) runs npm install
- **THEN** a precompiled binary appropriate for their platform is installed
- **AND** the tool is immediately available after installation completes

#### Scenario: ARM Platform Handling
- **WHEN** a user on macOS (Apple Silicon) or Linux (ARM64) runs npm install
- **THEN** they receive clear instructions for cargo installation
- **AND** the instructions include Rust toolchain setup if needed
- **AND** the instructions reference the project's build documentation

### Requirement: Installation Documentation
The calculator-cli project SHALL include platform-specific installation instructions in documentation.

#### Scenario: General User Documentation
- **WHEN** a user views the README.md or README_zh.md files
- **THEN** npm installation instructions are clearly documented for x64 platforms
- **AND** cargo build instructions are clearly documented for ARM platforms
- **AND** a platform support matrix is provided showing which method to use

#### Scenario: ARM Platform Documentation
- **WHEN** an ARM user reviews installation documentation
- **THEN** they find specific instructions for their platform
- **AND** they are directed to cargo install with all necessary prerequisites
- **AND** common issues on ARM platforms are addressed

### Requirement: Package Size Optimization
The calculator-cli npm package SHALL be optimized for size by including only x64 binaries.

#### Scenario: Package Composition
- **WHEN** the npm package is built
- **THEN** it includes only precompiled binaries for x64 platforms
- **AND** it excludes ARM binaries to minimize download size
- **AND** it includes platform detection logic to handle ARM users appropriately