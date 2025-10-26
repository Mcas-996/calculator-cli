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
    symbolic_solver.hpp
)

set(CALCULATOR_SOURCES
    string_processing.cpp
    symbolic_solver.cpp
)

add_library(calculator_core ${CALCULATOR_SOURCES} ${CALCULATOR_HEADERS})
target_include_directories(calculator_core PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})

# Always build SymEngine to guarantee symbolic outputs.
set(BUILD_TESTS no CACHE BOOL "Build SymEngine tests" FORCE)
set(BUILD_BENCHMARKS no CACHE BOOL "Build SymEngine benchmarks" FORCE)
set(BUILD_BENCHMARKS_GOOGLE no CACHE BOOL "Build SymEngine google benchmarks" FORCE)

if(APPLE)
    set(CMAKE_PREFIX_PATH "/opt/homebrew" ${CMAKE_PREFIX_PATH})
    set(GMP_ROOT "/opt/homebrew" CACHE PATH "Path to GMP")
    target_include_directories(calculator_core PUBLIC "/opt/homebrew/include")
endif()

add_subdirectory(third-part/symengine third-part/symengine/build_integration EXCLUDE_FROM_ALL)
set(SYMENGINE_BINARY_DIR ${CMAKE_CURRENT_BINARY_DIR}/third-part/symengine/build_integration)

target_link_libraries(calculator_core PUBLIC symengine)
target_include_directories(calculator_core PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/third-part/symengine
    ${SYMENGINE_BINARY_DIR}
    ${SYMENGINE_BINARY_DIR}/symengine/utilities/teuchos)
target_compile_definitions(calculator_core PUBLIC CALC_WITH_SYMENGINE)

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
