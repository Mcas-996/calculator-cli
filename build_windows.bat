@echo off
REM Windows compilation script for calculator

REM Check if clang++ is available
where clang++ >nul 2>&1
if %errorlevel% neq 0 (
    echo Error: clang++ not found. Please ensure clang is installed and in PATH.
    pause
    exit /b 1
)

echo Compiling calculator...
clang++ -std=c++2a -Wall -Wextra -O2 main_cli.cpp string_processing.cpp -o calculator.exe -lm

if %errorlevel% equ 0 (
    echo Compilation successful! Created calculator.exe
) else (
    echo Compilation failed!
    pause
    exit /b 1
)

pause
