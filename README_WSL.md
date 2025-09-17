# Using Calculator CLI in WSL

This document explains how to use the Calculator CLI in Windows Subsystem for Linux (WSL).

## Prerequisites

1. WSL2 with a Linux distribution installed (such as Ubuntu)
2. g++ compiler installed in your system

## Installing g++ (Ubuntu example)

```bash
sudo apt update
sudo apt install g++
```

## Building the Project

1. Copy project files to the WSL file system:
   ```bash
   mkdir -p ~/calculator-cli
   cd ~/calculator-cli
   cp /mnt/d/[path-to-project]/calculator-cli/* .
   ```

2. Run the Linux build script:
   ```bash
   chmod +x build_linux.sh
   ./build_linux.sh
   ```

## Using the Calculator

After successful compilation, you can run the calculator with the following command:

```bash
./calculator "expression"
```

### Examples

```bash
# Basic arithmetic operations
./calculator "2+3*4"

# Equation solving
./calculator "equation(x^2-5x+6=0)"

# System of linear equations
./calculator "equation2(x+y=5,x-y=1)"
```

## Running the Windows Version Directly in WSL

You can also run the Windows-compiled version directly in WSL:

```bash
/mnt/d/[path-to-project]/calculator-cli/calculator.exe "expression"
```

For example:
```bash
/mnt/d/allen\'s/code/repositories/cal_cli/1/calculator-cli/calculator.exe "2+3*4"
```

## Troubleshooting

If you encounter any issues:

1. Make sure the build scripts have LF line endings (not CRLF)
2. Ensure g++ is properly installed
3. Check that all source files are present in the directory

The calculator should work correctly in WSL with these steps.