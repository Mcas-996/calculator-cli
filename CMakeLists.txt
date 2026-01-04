# Minimum required CMake version
cmake_minimum_required(VERSION 3.30)

# Force use of clang compiler
set(CMAKE_C_COMPILER clang)
set(CMAKE_CXX_COMPILER clang++)

# Project definition
project(Calculator)

# Use C++23 to unlock std::numbers, std::complex helpers, and modules
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Enable C++ modules support
set(CMAKE_CXX_EXTENSIONS OFF)
if(CMAKE_CXX_COMPILER_ID STREQUAL "GNU")
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fmodules-ts")
elseif(CMAKE_CXX_COMPILER_ID STREQUAL "Clang")
    if(WIN32)
        # For clang-cl on Windows, try to enable modules
        set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fmodules")
    else()
        set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fmodules")
    endif()
endif()

# Enable fast cmake re-runs
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

set(CALCULATOR_HEADERS
    complex_number.hpp
    fractions.hpp
    cubic.hpp
    string_processing.hpp
    # symbolic_solver.hpp  # Disabled due to SymEngine dependency
)

set(CALCULATOR_SOURCES
    string_processing.cpp
    # symbolic_solver.cpp  # Disabled due to SymEngine dependency
)

add_library(calculator_core ${CALCULATOR_SOURCES} ${CALCULATOR_HEADERS})
target_include_directories(calculator_core PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})

# Disable SymEngine for now due to clang-cl compatibility issues
# TODO: Re-enable SymEngine once header conflicts are resolved
message(STATUS "SymEngine disabled for clang compatibility - building basic calculator only")

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
