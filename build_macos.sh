#!/bin/bash
# macOS compilation script for calculator

# Check if g++ is available
if ! command -v g++ &> /dev/null; then
    echo "Error: g++ not found. Please install Xcode command line tools."
    echo "Run: xcode-select --install"
    exit 1
fi

echo "Compiling calculator..."
g++ -std=c++2a -Wall -Wextra -O2 main_cli.cpp string_processing.cpp -o calculator -lm

if [ $? -eq 0 ]; then
    echo "Compilation successful! Created calculator"
else
    echo "Compilation failed!"
    exit 1
fi
