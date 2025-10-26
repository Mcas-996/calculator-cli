#pragma once

#include <string>
#include <vector>

namespace symbolic {

// Attempts to solve a polynomial \sum_{k=0}^{n} coeffs[k] * x^k = 0 symbolically.
// coeffs[0] is the constant term; coeffs.back() must be non-zero.
// Returns one string per root in a deterministic ordering.
std::vector<std::string> solvePolynomialSymbolically(const std::vector<double>& coeffs);

} // namespace symbolic
