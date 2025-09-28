# CMake 最低版本要求
cmake_minimum_required(VERSION 3.10)

# 项目名称
project(Calculator)

# 设置C++标准为C++17以支持std::gcd
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# 添加可执行文件 (您的主程序)
add_executable(calculator main_cli.cpp string_processing.cpp)

# 启用测试
enable_testing()

# 添加测试可执行文件
add_executable(run_tests calculator_tests.cpp string_processing.cpp)

# Windows特定配置
if(WIN32)
    # 在Windows上定义M_PI常量以避免编译错误
    add_definitions(-D_USE_MATH_DEFINES)
endif()