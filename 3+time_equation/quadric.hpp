#pragma once

#include <algorithm>
#include <array>
#include <cmath>
#include <complex>
#include <limits>
#include <stdexcept>

namespace quartic {

constexpr double QUARTIC_EPS = 1e-12;
constexpr double PI = 3.141592653589793238462643383279502884;

struct DurandKernerOptions {
    int max_iterations = 200;
    double tolerance = 1e-14;
    bool polish_roots = true;
    int polish_steps = 2;
};

struct QuarticResult {
    std::array<std::complex<double>, 4> roots{};
    bool converged = false;
    int iterations = 0;
    double max_step = 0.0;
    double max_residual = 0.0;
};

inline bool nearly_zero(double value, double eps = QUARTIC_EPS) {
    return std::abs(value) <= eps;
}

inline std::complex<double> eval_monic(const std::complex<double>& x,
                                       double A, double B, double C, double D) {
    return (((x + A) * x + B) * x + C) * x + D;
}

inline std::complex<double> eval_poly(const std::complex<double>& x,
                                      double a, double b, double c,
                                      double d, double e) {
    return ((((a * x) + b) * x + c) * x + d) * x + e;
}

inline std::complex<double> eval_poly_derivative(const std::complex<double>& x,
                                                 double a, double b,
                                                 double c, double d) {
    return (((4.0 * a) * x + 3.0 * b) * x + 2.0 * c) * x + d;
}

inline std::array<std::complex<double>, 4> initial_guesses(double radius) {
    std::array<std::complex<double>, 4> roots{};
    for (int i = 0; i < 4; ++i) {
        double angle = 2.0 * PI * static_cast<double>(i) / 4.0;
        roots[i] = std::polar(radius, angle);
        roots[i] += std::complex<double>(1e-3 * i, -1e-3 * i);
    }
    return roots;
}

inline QuarticResult solve_durand_kerner(double a, double b, double c,
                                         double d, double e,
                                         const DurandKernerOptions& options = {}) {
    if (nearly_zero(a)) {
        throw std::invalid_argument("Coefficient 'a' must not be zero for a quartic equation.");
    }

    double A = b / a;
    double B = c / a;
    double C = d / a;
    double D = e / a;

    double radius = 1.0 + std::max({std::abs(A), std::abs(B), std::abs(C), std::abs(D)});
    auto roots = initial_guesses(radius);

    QuarticResult result;
    result.roots = roots;

    double tol = std::max(options.tolerance, std::numeric_limits<double>::epsilon() * 10);
    bool converged = false;
    int iter = 0;
    double max_step = 0.0;

    for (; iter < options.max_iterations; ++iter) {
        max_step = 0.0;
        for (int i = 0; i < 4; ++i) {
            std::complex<double> denom(1.0, 0.0);
            for (int j = 0; j < 4; ++j) {
                if (i == j) continue;
                auto diff = result.roots[i] - result.roots[j];
                if (std::abs(diff) < QUARTIC_EPS) {
                    diff += std::complex<double>(QUARTIC_EPS, QUARTIC_EPS);
                }
                denom *= diff;
            }
            std::complex<double> delta = eval_monic(result.roots[i], A, B, C, D) / denom;
            result.roots[i] -= delta;
            max_step = std::max(max_step, std::abs(delta));
        }
        if (max_step < tol) {
            converged = true;
            ++iter;
            break;
        }
    }

    if (!converged) {
        iter = options.max_iterations;
    }
    result.iterations = iter;
    result.max_step = max_step;
    result.converged = converged;

    if (options.polish_roots) {
        for (auto& root : result.roots) {
            for (int k = 0; k < options.polish_steps; ++k) {
                auto value = eval_poly(root, a, b, c, d, e);
                auto derivative = eval_poly_derivative(root, a, b, c, d);
                if (std::abs(derivative) < QUARTIC_EPS) break;
                root -= value / derivative;
            }
        }
    }

    double max_residual = 0.0;
    for (const auto& root : result.roots) {
        max_residual = std::max(max_residual, std::abs(eval_poly(root, a, b, c, d, e)));
    }
    result.max_residual = max_residual;
    return result;
}

inline bool is_biquadratic(double b, double d, double eps = QUARTIC_EPS) {
    return nearly_zero(b, eps) && nearly_zero(d, eps);
}

inline std::array<std::complex<double>, 4> solve_biquadratic_core(double p, double q) {
    std::complex<double> discriminant = std::complex<double>(p * p - 4.0 * q, 0.0);
    std::complex<double> sqrt_disc = std::sqrt(discriminant);
    std::complex<double> y1 = (-p + sqrt_disc) * 0.5;
    std::complex<double> y2 = (-p - sqrt_disc) * 0.5;

    std::complex<double> r1 = std::sqrt(y1);
    std::complex<double> r2 = -r1;
    std::complex<double> r3 = std::sqrt(y2);
    std::complex<double> r4 = -r3;

    return {r1, r2, r3, r4};
}

inline QuarticResult solve_biquadratic(double a, double b, double c,
                                       double d, double e) {
    if (nearly_zero(a)) {
        throw std::invalid_argument("Coefficient 'a' must not be zero for a quartic equation.");
    }
    auto roots = solve_biquadratic_core(c / a, e / a);
    QuarticResult result;
    result.roots = roots;
    result.converged = true;
    result.iterations = 1;
    result.max_step = 0.0;
    double max_residual = 0.0;
    for (const auto& root : result.roots) {
        max_residual = std::max(max_residual, std::abs(eval_poly(root, a, b, c, d, e)));
    }
    result.max_residual = max_residual;
    return result;
}

inline QuarticResult solve(double a, double b, double c,
                           double d, double e,
                           const DurandKernerOptions& options = {}) {
    if (is_biquadratic(b, d)) {
        return solve_biquadratic(a, b, c, d, e);
    }
    return solve_durand_kerner(a, b, c, d, e, options);
}

} // namespace quartic
