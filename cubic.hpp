#pragma once

#include <vector>
#include <cmath>
#include <stdexcept>
#include <algorithm>
#include <numeric>

using std::vector;
using std::acos;
using std::numbers;

// A small epsilon value for floating-point comparisons
const double CUBIC_EPSILON = 1e-9;

// Solves the cubic equation ax^3 + bx^2 + cx + d = 0
// and returns a vector containing the real roots.
// The implementation is all within this header file as requested.
inline vector<double> solve_cubic(double a, double b, double c, double d) {
    if (std::abs(a) < CUBIC_EPSILON) {
        throw std::invalid_argument("Coefficient 'a' cannot be zero in a cubic equation.");
    }

    // Define PI if not available
    #ifndef std::numbers::pi
    #endif

    // Normalize to x^3 + Bx^2 + Cx + D = 0
    double B = b / a;
    double C = c / a;
    double D = d / a;

    // Substitute x = y - B/3 to get a depressed cubic y^3 + py + q = 0
    double p = C - (B * B) / 3.0;
    double q = D + (2.0 * B * B * B) / 27.0 - (B * C) / 3.0;
    double offset = -B / 3.0;

    vector<double> roots;

    if (std::abs(p) < CUBIC_EPSILON) { // Special case: p is close to 0, y^3 = -q
        roots.push_back(std::cbrt(-q) + offset);
    } else {
        double discriminant = (q * q) / 4.0 + (p * p * p) / 27.0;

        if (discriminant >= -CUBIC_EPSILON) {
            // One real root (or three real roots with at least two being equal)
            double sqrt_discriminant = std::sqrt(std::max(0.0, discriminant));
            double term = -q / 2.0;
            
            double u = std::cbrt(term + sqrt_discriminant);
            double v = std::cbrt(term - sqrt_discriminant);
            
            roots.push_back(u + v + offset);
            
            // If discriminant is close to zero, we have multiple roots (a double or triple root)
            if (std::abs(discriminant) < CUBIC_EPSILON) {
                roots.push_back(- (u + v) / 2.0 + offset);
            }
        } else {
            // Three distinct real roots (trigonometric solution)
            double term1 = 2.0 * std::sqrt(-p / 3.0);
            double term2 = (3.0 * q) / (2.0 * p) * std::sqrt(-3.0 / p);
            // Clamp term2 to the [-1, 1] range due to potential floating point errors
            double acos_arg = std::max(-1.0, std::min(1.0, term2));
            double phi = acos(acos_arg);

            roots.push_back(term1 * std::cos(phi / 3.0) + offset);
            roots.push_back(term1 * std::cos((phi + 2.0 * numbers::pi) / 3.0) + offset);
            roots.push_back(term1 * std::cos((phi - 2.0 * numbers::pi) / 3.0) + offset);
        }
    }
    
    // Sort roots for consistent output
    std::sort(roots.begin(), roots.end());
    
    // Remove duplicates that might arise from floating point inaccuracies
    roots.erase(std::unique(roots.begin(), roots.end(), [](double r1, double r2){
        return std::abs(r1 - r2) < CUBIC_EPSILON;
    }), roots.end());

    return roots;
}
