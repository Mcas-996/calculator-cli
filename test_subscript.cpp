#include <iostream>
#include <string>

std::string formatSubscript(int index) {
    const std::string subscripts = "₀₁₂₃₄₅₆₇₈₉";
    if (index >= 0 && index <= 9) {
        return std::string(1, subscripts[index]);
    }
    // 对于多位数字，逐位转换
    std::string result;
    std::string str = std::to_string(index);
    for (char c : str) {
        int digit = c - '0';
        if (digit >= 0 && digit <= 9) {
            result += subscripts[digit];
        }
    }
    return result;
}

int main() {
    for (int i = 1; i <= 3; i++) {
        std::cout << "x" << formatSubscript(i) << " = " << i << std::endl;
    }
    return 0;
}
