@echo off
REM Windows compilation script for calculator

REM Check if g++ is available
where g++ >nul 2>&1
if %errorlevel% neq 0 (
    echo Error: g++ not found. Please install MinGW or MinGW-w64.
    echo You can download MinGW-w64 from: https://www.mingw-w64.org/
    pause
    exit /b 1
)

echo Compiling calculator...
g++ -std=c++2a -Wall -Wextra -O2 main_cli.cpp string_processing.cpp -o calculator.exe -lm

if %errorlevel% equ 0 (
    echo Compilation successful! Created calculator.exe
) else (
    echo Compilation failed!
    pause
    exit /b 1
)

pause