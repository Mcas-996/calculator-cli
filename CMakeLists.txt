# CMake 最低版本要求
cmake_minimum_required(VERSION 3.10)

# 项目名称
project(Calculator)

# 添加可执行文件 (您的主程序)
add_executable(calculator main_cli.cpp string_processing.cpp)

# 启用测试
enable_testing()

# 添加测试可执行文件
add_executable(run_tests calculator_tests.cpp string_processing.cpp)

# 将测试添加到 CTest
add_test(NAME BasicArithmeticTest COMMAND run_tests)
