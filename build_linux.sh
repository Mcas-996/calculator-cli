#!/bin/bash
# Linux compilation script for calculator

# Check if g++ is available
if ! command -v g++ &> /dev/null; then
    echo "Error: g++ not found. Please install g++."
    echo "On Ubuntu/Debian: sudo apt install g++"
    echo "On Fedora/RHEL/CentOS: sudo dnf install gcc-c++"
    echo "On Arch Linux: sudo pacman -S gcc"
    exit 1
fi

echo "Compiling calculator..."
g++ -std=c++11 -Wall -Wextra -O2 main_cli.cpp string_processing.cpp -o calculator -lm

if [ $? -eq 0 ]; then
    echo "Compilation successful! Created calculator"
else
    echo "Compilation failed!"
    exit 1
fi