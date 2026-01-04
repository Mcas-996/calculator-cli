# Minimum required CMake version
cmake_minimum_required(VERSION 3.20)

# Force use of clang compiler
set(CMAKE_C_COMPILER clang)
set(CMAKE_CXX_COMPILER clang++)

# Project definition
project(Calculator)

# Use C++20
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Enable fast cmake re-runs
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

set(CALCULATOR_HEADERS
    complex_number.hpp
    fractions.hpp
    cubic.hpp
    string_processing.hpp
    symbolic_solver.hpp
    pretty_output_config.hpp
    unicode_formatter.hpp
    latex_renderer.hpp
    pretty_output.hpp
)

set(CALCULATOR_SOURCES
    string_processing.cpp
    symbolic_solver.cpp
    pretty_output_config.cpp
    unicode_formatter.cpp
    latex_renderer.cpp
    pretty_output.cpp
)

add_library(calculator_core ${CALCULATOR_SOURCES} ${CALCULATOR_HEADERS})
target_include_directories(calculator_core PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_link_libraries(calculator_core PUBLIC m)

add_executable(calculator main_cli.cpp)
target_link_libraries(calculator PRIVATE calculator_core)

add_executable(calculator_tests_runner calculator_tests.cpp)
target_link_libraries(calculator_tests_runner PRIVATE calculator_core)

if(WIN32)
    target_compile_definitions(calculator_core PUBLIC _USE_MATH_DEFINES)
    target_compile_definitions(calculator_tests_runner PRIVATE _USE_MATH_DEFINES)
endif()

enable_testing()
# Use the generator expression to ensure CTest invokes the correct binary for
# the active configuration (e.g. Release on Windows multi-config generators).
add_test(NAME calculator_tests COMMAND $<TARGET_FILE:calculator_tests_runner>)
