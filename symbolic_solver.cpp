#include "symbolic_solver.hpp"

#include "fractions.hpp"

#include <cctype>
#include <cmath>
#include <stdexcept>
#include <symengine/add.h>
#include <symengine/basic.h>
#include <symengine/functions.h>
#include <symengine/mul.h>
#include <symengine/pow.h>
#include <symengine/printers/strprinter.h>
#include <symengine/sets.h>
#include <symengine/solve.h>
#include <symengine/symbol.h>
#include <symengine/symengine_rcp.h>

namespace {

constexpr double SYMBOLIC_EPSILON = 1e-12;
constexpr const char NON_FINITE_PREFIX[] = "NON_FINITE::";

inline bool nearly_zero(double value) {
    return std::fabs(value) <= SYMBOLIC_EPSILON;
}

bool isTokenChar(char c) {
    return std::isalnum(static_cast<unsigned char>(c)) || c == '_' || c == '.';
}

std::string replaceDoubleStar(const std::string& input) {
    std::string result;
    result.reserve(input.size());
    for (size_t i = 0; i < input.size();) {
        if (i + 1 < input.size() && input[i] == '*' && input[i + 1] == '*') {
            result.push_back('^');
            i += 2;
        } else {
            result.push_back(input[i]);
            ++i;
        }
    }
    return result;
}

std::string convertExponentNotation(const std::string& input) {
    std::string caretString = replaceDoubleStar(input);
    std::string finalResult;
    size_t last = 0;
    for (size_t i = 0; i + 5 < caretString.size(); ++i) {
        if (caretString.compare(i, 6, "^(1/3)") != 0) {
            continue;
        }

        size_t caretPos = i;
        size_t baseStart = 0;
        size_t baseEnd = caretPos;
        std::string base;

        if (caretPos == 0) {
            continue;
        }

        if (caretString[caretPos - 1] == ')') {
            size_t depth = 1;
            size_t j = caretPos - 2;
            while (true) {
                if (caretString[j] == ')') {
                    ++depth;
                } else if (caretString[j] == '(') {
                    --depth;
                    if (depth == 0) {
                        baseStart = j;
                        break;
                    }
                }
                if (j == 0) {
                    break;
                }
                --j;
            }

            if (depth == 0 && baseStart + 1 < baseEnd - 1) {
                size_t innerStart = baseStart + 1;
                size_t innerLen = baseEnd - innerStart - 1;
                finalResult.append(caretString, last, baseStart - last);
                base = caretString.substr(innerStart, innerLen);
                finalResult += "cbrt(" + base + ")";
                last = caretPos + 6;
                i = caretPos + 5;
                continue;
            }
        }

        size_t j = caretPos;
        while (j > 0 && isTokenChar(caretString[j - 1])) {
            --j;
        }
        if (j > 0 && caretString[j - 1] == '-' && (j == 1 || !isTokenChar(caretString[j - 2]))) {
            --j;
        }

        baseStart = j;
        base = caretString.substr(baseStart, baseEnd - baseStart);
        finalResult.append(caretString, last, baseStart - last);
        finalResult += "cbrt(" + base + ")";
        last = caretPos + 6;
        i = caretPos + 5;
    }

    if (last < caretString.size()) {
        finalResult.append(caretString, last, caretString.size() - last);
    }
    return finalResult;
}

SymEngine::RCP<const SymEngine::Basic> makeExactNumber(double value) {
    using namespace SymEngine;
    if (nearly_zero(value)) {
        return zero;
    }

    Fraction frac = Fraction::fromDouble(value);
    auto numerator = integer(frac.numerator);
    if (frac.denominator == 1) {
        return numerator;
    }
    auto denominator = integer(frac.denominator);
    return div(numerator, denominator);
}

SymEngine::vec_basic buildCoeffVector(const std::vector<double>& coeffs) {
    SymEngine::vec_basic basicCoeffs;
    basicCoeffs.reserve(coeffs.size());
    for (double value : coeffs) {
        basicCoeffs.push_back(makeExactNumber(value));
    }
    return basicCoeffs;
}

SymEngine::set_basic collectFiniteSolutions(const SymEngine::RCP<const SymEngine::Set>& set) {
    using namespace SymEngine;
    if (is_a<FiniteSet>(*set)) {
        return down_cast<const FiniteSet &>(*set).get_container();
    }
    if (is_a<Union>(*set)) {
        set_basic aggregated;
        const auto& subsets = down_cast<const Union &>(*set).get_container();
        for (const auto& subset : subsets) {
            auto partial = collectFiniteSolutions(subset);
            aggregated.insert(partial.begin(), partial.end());
        }
        return aggregated;
    }
    StrPrinter printer;
    throw std::runtime_error(std::string(NON_FINITE_PREFIX) + convertExponentNotation(printer.apply(*set)));
}

std::vector<std::string> stringifySolutions(const SymEngine::RCP<const SymEngine::Set>& set) {
    using namespace SymEngine;
    auto finiteContainer = collectFiniteSolutions(set);
    std::vector<std::string> formatted;
    formatted.reserve(finiteContainer.size());
    StrPrinter printer;
    for (const auto& root : finiteContainer) {
        formatted.push_back(convertExponentNotation(printer.apply(*root)));
    }
    return formatted;
}

} // namespace

namespace symbolic {

std::vector<std::string> solvePolynomialSymbolically(const std::vector<double>& coeffs) {
    using namespace SymEngine;
    if (coeffs.size() < 2) {
        throw std::invalid_argument("At least a constant and leading coefficient are required");
    }

    auto leading = coeffs.back();
    if (nearly_zero(leading)) {
        throw std::invalid_argument("Leading coefficient must be non-zero for symbolic solving");
    }

    auto basicCoeffs = buildCoeffVector(coeffs);
    auto domain = universalset();
    RCP<const Set> solutionSet;

    switch (basicCoeffs.size()) {
        case 2:
            solutionSet = solve_poly_linear(basicCoeffs, domain);
            break;
        case 3:
            solutionSet = solve_poly_quadratic(basicCoeffs, domain);
            break;
        case 4:
            solutionSet = solve_poly_cubic(basicCoeffs, domain);
            break;
        case 5:
            solutionSet = solve_poly_quartic(basicCoeffs, domain);
            break;
        default: {
            auto x = symbol("x");
            RCP<const Basic> polynomial = zero;
            for (size_t i = 0; i < basicCoeffs.size(); ++i) {
                if (eq(*(basicCoeffs[i]), *zero)) {
                    continue;
                }
                RCP<const Basic> term;
                if (i == 0) {
                    term = basicCoeffs[i];
                } else if (i == 1) {
                    term = mul(basicCoeffs[i], x);
                } else {
                    term = mul(basicCoeffs[i], pow(x, integer(static_cast<long long>(i))));
                }
                polynomial = add(polynomial, term);
            }
            solutionSet = solve_poly(polynomial, x, domain);
            break;
        }
    }

    return stringifySolutions(solutionSet);
}

} // namespace symbolic
