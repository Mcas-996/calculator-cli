#include "string_processing.hpp"
#include <cassert>
#include <cmath>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>
// Helper function to compare double results with a tolerance
bool compare_doubles(double d1, double d2, double epsilon = 0.00001) {
    return std::abs(d1 - d2) < epsilon;
}

// Helper function to parse quadratic equation solutions and compare them regardless of order
bool compare_quadratic_solutions(const std::string& actual, const std::string& expected) {
    // Expected format: "x1 = val1, x2 = val2"
    // Actual format: "x1 = valA, x2 = valB" or "x = val"

    // Handle single solution case first
    if (expected.find("x = ") != std::string::npos) {
        return actual == expected;
    }

    // For two solutions, parse both actual and expected
    std::vector<double> actual_vals;
    std::vector<double> expected_vals;

    // Parse actual string
    size_t pos = actual.find("x1 = ");
    if (pos != std::string::npos) {
        size_t comma_pos = actual.find(", x2 = ", pos);
        if (comma_pos != std::string::npos) {
            actual_vals.push_back(std::stod(actual.substr(pos + 5, comma_pos - (pos + 5))));
            actual_vals.push_back(std::stod(actual.substr(comma_pos + 6)));
        }
    }

    // Parse expected string
    pos = expected.find("x1 = ");
    if (pos != std::string::npos) {
        size_t comma_pos = expected.find(", x2 = ", pos);
        if (comma_pos != std::string::npos) {
            expected_vals.push_back(std::stod(expected.substr(pos + 5, comma_pos - (pos + 5))));
            expected_vals.push_back(std::stod(expected.substr(comma_pos + 6)));
        }
    }

    if (actual_vals.size() != 2 || expected_vals.size() != 2) {
        // This should not happen if formats are consistent, but as a safeguard
        return false;
    }

    // Sort both vectors to compare regardless of order
    std::sort(actual_vals.begin(), actual_vals.end());
    std::sort(expected_vals.begin(), expected_vals.end());

    // Compare sorted values with tolerance
    return compare_doubles(actual_vals[0], expected_vals[0]) &&
           compare_doubles(actual_vals[1], expected_vals[1]);
}

// Helper function to parse linear system solutions and compare them with tolerance
bool compare_linear_system_solutions(const std::string& actual, const std::string& expected) {
    // Expected and Actual format: "x = val1, y = val2"
    std::istringstream iss_actual(actual);
    std::istringstream iss_expected(expected);

    std::string token;
    double actual_x, actual_y;
    double expected_x, expected_y;

    // Parse actual x
    std::getline(iss_actual, token, '='); // Read "x "
    iss_actual >> actual_x; // Read value
    std::getline(iss_actual, token, '='); // Read ", y "
    iss_actual >> actual_y; // Read value

    // Parse expected x
    std::getline(iss_expected, token, '='); // Read "x "
    iss_expected >> expected_x; // Read value
    std::getline(iss_expected, token, '='); // Read ", y "
    iss_expected >> expected_y; // Read value

    return compare_doubles(actual_x, expected_x) &&
           compare_doubles(actual_y, expected_y);
}

void run_all_tests() {
    std::cout << "Running all calculator tests..." << std::endl;

    // Test 1: Basic arithmetic
    std::string result1 = sp::processInput("3 + 5 * (2 - 8)^2");
    std::cout << "Actual result for '3 + 5 * (2 - 8)^2': " << result1 << std::endl;
    assert(result1 == "183");

    std::string result2 = sp::processInput("-2.5 * 4 + 3^2");
    std::cout << "Actual result for '-2.5 * 4 + 3^2': " << result2 << std::endl;
    assert(result2 == "-1");
    std::cout << "Basic arithmetic tests passed." << std::endl;

    // Test 2: Percentages
    std::string result3 = sp::processInput("50% * 200");
    std::cout << "Actual result for '50% * 200': " << result3 << std::endl;
    assert(result3 == "100");

    std::string result4 = sp::processInput("25% + 75%");
    std::cout << "Actual result for '25% + 75%': " << result4 << std::endl;
    assert(result4 == "1"); // 0.25 + 0.75
    std::cout << "Percentage tests passed." << std::endl;

    // Test 3: Square roots
    std::string result5 = sp::processInput("sqrt(16) + 3");
    std::cout << "Actual result for 'sqrt(16) + 3': " << result5 << std::endl;
    assert(result5 == "7");

    std::string result6 = sp::processInput("sqrt(9) * 2");
    std::cout << "Actual result for 'sqrt(9) * 2': " << result6 << std::endl;
    assert(result6 == "6");
    std::cout << "Square root tests passed." << std::endl;

    // Test 4: Constants
    double pi_test_val = std::stod(sp::processInput("pi * 2"));
    std::cout << "Actual result for 'pi * 2': " << pi_test_val << std::endl;
    assert(compare_doubles(pi_test_val, M_PI * 2));

    double e_test_val = std::stod(sp::processInput("e^2"));
    std::cout << "Actual result for 'e^2': " << e_test_val << std::endl;
    assert(compare_doubles(e_test_val, M_E * M_E));
    std::cout << "Constant tests passed." << std::endl;

    // Test 5: Linear equations
    std::string result7 = sp::processInput("equation(x+1=0)");
    std::cout << "Actual result for 'equation(x+1=0)': " << result7 << std::endl;
    assert(result7 == "x = -1");

    std::string result8 = sp::processInput("equation(2x-3=7)");
    std::cout << "Actual result for 'equation(2x-3=7)': " << result8 << std::endl;
    assert(result8 == "x = 5");
    std::cout << "Linear equation tests passed." << std::endl;

    // Test 6: Quadratic equations
    std::string result9 = sp::processInput("equation(x^2+2x+1=0)");
    std::cout << "Actual result for 'equation(x^2+2x+1=0)': " << result9 << std::endl;
    assert(result9 == "x = -1");

    std::string result10 = sp::processInput("equation(x^2-5x+6=0)");
    std::cout << "Actual result for 'equation(x^2-5x+6=0)': " << result10 << std::endl;
    assert(compare_quadratic_solutions(result10, "x1 = 3, x2 = 2"));
    std::cout << "Quadratic equation tests passed." << std::endl;

    // Test 7: Systems of linear equations
    std::string result11 = sp::processInput("equation2(x+y=5,x-y=1)");
    std::cout << "Actual result for 'equation2(x+y=5,x-y=1)': " << result11 << std::endl;
    assert(result11 == "x = 3, y = 2");

    std::string result12 = sp::processInput("equation2(2x+3y=12,4x-y=5)");
    std::cout << "Actual result for 'equation2(2x+3y=12,4x-y=5)': " << result12 << std::endl;
    assert(compare_linear_system_solutions(result12, "x = 1.928571, y = 2.714286")); // Use helper for comparison
    std::cout << "Systems of linear equations tests passed." << std::endl;

    // Test 8: Error handling (example - division by zero)
    std::string error_result = sp::processInput("1 / 0");
    std::cout << "Actual result for '1 / 0': " << error_result << std::endl;
    assert(error_result.find("Error: Division by zero") != std::string::npos);
    std::cout << "Error handling test passed." << std::endl;

    std::cout << "All tests completed successfully!" << std::endl;
}

int main() {
    run_all_tests();
    return 0;
}
