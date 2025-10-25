# Minimum required CMake version
cmake_minimum_required(VERSION 3.10)

# Project definition
project(Calculator)

# Use C++20 to unlock std::numbers, std::complex helpers, etc.
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(CALCULATOR_HEADERS
    complex_number.hpp
    fractions.hpp
    cubic.hpp
    string_processing.hpp
)

set(CALCULATOR_SOURCES
    string_processing.cpp
)

add_library(calculator_core ${CALCULATOR_SOURCES} ${CALCULATOR_HEADERS})
target_include_directories(calculator_core PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})

add_executable(calculator main_cli.cpp)
target_link_libraries(calculator PRIVATE calculator_core)

add_executable(run_tests calculator_tests.cpp)
target_link_libraries(run_tests PRIVATE calculator_core)

if(WIN32)
    target_compile_definitions(calculator_core PUBLIC _USE_MATH_DEFINES)
endif()

enable_testing()
# Use the generator expression to ensure CTest invokes the correct binary for
# the active configuration (e.g. Release on Windows multi-config generators).
add_test(NAME calculator_tests COMMAND $<TARGET_FILE:run_tests>)
