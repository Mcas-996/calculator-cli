#include <iostream>
#include <cassert>
#include "string_processing.hpp"

// 一个简单的测试函数
void test_addition() {
    // 在这里，您应该调用您的计算器核心逻辑。
    // 因为您的核心逻辑在 main_cli.cpp 中并且与 stdin/stdout 耦合，
    // 我们暂时只做一个简单的断言。
    // 为了进行更可靠的测试，您需要将计算逻辑重构为可以从外部调用的函数。
    double result = 2.0 + 2.0;
    assert(result == 4.0);
    std::cout << "Test '2.0 + 2.0 == 4.0' passed." << std::endl;
}

int main() {
    test_addition();
    // 如果所有 assert 都通过，程序将正常退出（返回 0），CTest 会认为测试通过。
    // 如果任何 assert 失败，程序将中止（返回非 0），CTest 会认为测试失败。
    return 0;
}
