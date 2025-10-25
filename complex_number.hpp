#ifndef COMPLEX_NUMBER_HPP
#define COMPLEX_NUMBER_HPP

#include <cmath>
#include <complex>
#include <iomanip>
#include <sstream>
#include <string>
#include <stdexcept>

#include "fractions.hpp"

constexpr double COMPLEX_EPSILON = 1e-9;

class ComplexNumber {
public:
    double real;
    double imag;

    ComplexNumber(double r = 0.0, double i = 0.0) : real(r), imag(i) {}

    ComplexNumber operator+(const ComplexNumber& other) const {
        return ComplexNumber(real + other.real, imag + other.imag);
    }

    ComplexNumber operator-(const ComplexNumber& other) const {
        return ComplexNumber(real - other.real, imag - other.imag);
    }

    ComplexNumber operator*(const ComplexNumber& other) const {
        return ComplexNumber(
            real * other.real - imag * other.imag,
            real * other.imag + imag * other.real
        );
    }

    ComplexNumber operator/(const ComplexNumber& other) const {
        double denominator = other.real * other.real + other.imag * other.imag;
        if (std::fabs(denominator) < COMPLEX_EPSILON) {
            throw std::runtime_error("Division by zero");
        }
        return ComplexNumber(
            (real * other.real + imag * other.imag) / denominator,
            (imag * other.real - real * other.imag) / denominator
        );
    }

    ComplexNumber pow(const ComplexNumber& exponent) const {
        std::complex<double> base(real, imag);
        std::complex<double> exp(exponent.real, exponent.imag);
        std::complex<double> result = std::pow(base, exp);
        return ComplexNumber(result.real(), result.imag());
    }

    ComplexNumber sqrtPrincipal() const {
        std::complex<double> value(real, imag);
        std::complex<double> result = std::sqrt(value);
        return ComplexNumber(result.real(), result.imag());
    }

    ComplexNumber sin() const {
        std::complex<double> value(real, imag);
        std::complex<double> result = std::sin(value);
        return ComplexNumber(result.real(), result.imag());
    }

    ComplexNumber cos() const {
        std::complex<double> value(real, imag);
        std::complex<double> result = std::cos(value);
        return ComplexNumber(result.real(), result.imag());
    }

    bool isApproximatelyReal(double epsilon = COMPLEX_EPSILON) const {
        return std::fabs(imag) < epsilon;
    }

    std::string toString(double epsilon = COMPLEX_EPSILON) const {
        auto formatComponent = [](double value) -> std::string {
            Fraction frac = Fraction::fromDouble(value);
            double fracValue = static_cast<double>(frac.numerator) / frac.denominator;
            if (std::fabs(value - fracValue) < 1e-9) {
                return frac.toString();
            }
            std::ostringstream oss;
            oss << std::fixed << std::setprecision(10) << value;
            std::string str = oss.str();
            while (!str.empty() && str.back() == '0') {
                str.pop_back();
            }
            if (!str.empty() && str.back() == '.') {
                str.pop_back();
            }
            if (str == "-0") {
                str = "0";
            }
            if (str.empty()) {
                str = "0";
            }
            return str;
        };

        bool realZero = std::fabs(real) < epsilon;
        bool imagZero = std::fabs(imag) < epsilon;

        if (realZero && imagZero) {
            return "0";
        }

        if (imagZero) {
            return formatComponent(real);
        }

        if (realZero) {
            std::string imagStr = formatComponent(imag);
            if (imagStr == "1") return "i";
            if (imagStr == "-1") return "-i";
            return imagStr + "i";
        }

        std::string realStr = formatComponent(real);
        std::string imagStr = formatComponent(std::fabs(imag));
        std::string sign = imag >= 0 ? " + " : " - ";

        if (imagStr == "1") {
            imagStr = "";
        }

        return realStr + sign + (imagStr.empty() ? "i" : imagStr + "i");
    }
};

#endif // COMPLEX_NUMBER_HPP
