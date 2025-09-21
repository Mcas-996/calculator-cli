// fractions.hpp
#ifndef FRACTIONS_HPP
#define FRACTIONS_HPP

#include <numeric>   // For std::gcd
#include <string>
#include <sstream>   // For std::ostringstream
#include <cmath>     // For std::fabs, std::round
#include <utility>   // For std::swap
#include <iomanip>   // For std::fixed, std::setprecision

class Fraction {
public: // Make all members public, as they were in a struct
    long long numerator;
    long long denominator;

    // Constructor
    Fraction(long long num = 0, long long den = 1) : numerator(num), denominator(den) {
        if (denominator == 0) {
            // Handle division by zero, e.g., represent as 0/1 or throw an error
            // For a calculator, 0/1 might be a safer default than undefined.
            numerator = 0;
            denominator = 1;
        } else {
            // Ensure denominator is positive and simplify
            if (denominator < 0) {
                numerator = -numerator;
                denominator = -denominator;
            }
            simplify();
        }
    }

    // Simplify the fraction
    void simplify() {
        if (numerator == 0) {
            denominator = 1;
            return;
        }
        // Use std::gcd from <numeric>
        long long common_divisor = std::gcd(numerator, denominator);
        numerator /= common_divisor;
        denominator /= common_divisor;
    }

    // Arithmetic operators
    Fraction operator+(const Fraction& other) const {
        return Fraction(
            numerator * other.denominator + other.numerator * denominator,
            denominator * other.denominator
        );
    }

    Fraction operator-(const Fraction& other) const {
        return Fraction(
            numerator * other.denominator - other.numerator * denominator,
            denominator * other.denominator
        );
    }

    Fraction operator*(const Fraction& other) const {
        return Fraction(
            numerator * other.numerator,
            denominator * other.denominator
        );
    }

    Fraction operator/(const Fraction& other) const {
        if (other.numerator == 0) {
            // Division by zero, handle appropriately (e.g., return invalid fraction or throw)
            // For simplicity, returning 0/1 as an "error" state for now.
            return Fraction(0, 1);
        }
        return Fraction(
            numerator * other.denominator,
            denominator * other.numerator
        );
    }

    // Returns the inverse of the fraction (b/a for a/b)
    Fraction inverse() const {
        if (numerator == 0) {
            throw std::runtime_error("Cannot invert a zero fraction");
        }
        return Fraction(denominator, numerator);
    }

    // Conversion from double to Fraction
    static Fraction fromDouble(double value, double epsilon = 1e-9) {
        if (std::fabs(value - std::round(value)) < epsilon) {
            return Fraction(static_cast<long long>(std::round(value))); // It's an integer
        }

        // Try to find a good denominator
        const long long MAX_DENOMINATOR = 1000000; // Limit for finding a reasonable fraction
        long long bestN = 0;
        long long bestD = 1;
        double minDiff = std::fabs(value);

        for (long long d = 1; d <= MAX_DENOMINATOR; ++d) {
            long long n = static_cast<long long>(std::round(value * d));
            double diff = std::fabs(value - static_cast<double>(n) / d);
            if (diff < minDiff) {
                minDiff = diff;
                bestN = n;
                bestD = d;
            }
        }
        return Fraction(bestN, bestD);
    }

    // Convert Fraction to string for display
    std::string toString() const {
        std::ostringstream oss;
        if (denominator == 1) {
            oss << numerator;
        } else {
            // Convert to double for decimal representation
            oss << std::fixed << std::setprecision(10) << static_cast<double>(numerator) / denominator;
        }
        return oss.str();
    }
};

#endif // FRACTIONS_HPP