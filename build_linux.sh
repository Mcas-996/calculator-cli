#!/bin/bash
# Linux compilation script for calculator

# Check if clang++ is available
if ! command -v clang++ &> /dev/null; then
    echo "Error: clang++ not found. Please install clang++."
    echo "On Ubuntu/Debian: sudo apt install clang"
    echo "On Fedora/RHEL/CentOS: sudo dnf install clang"
    echo "On Arch Linux: sudo pacman -S clang"
    exit 1
fi

echo "Compiling calculator..."
clang++ -std=c++2a -Wall -Wextra -O2 main_cli.cpp string_processing.cpp -o calculator -lm

if [ $? -eq 0 ]; then
    echo "Compilation successful! Created calculator"
else
    echo "Compilation failed!"
    exit 1
fi
