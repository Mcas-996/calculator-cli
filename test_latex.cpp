#include <iostream>
#include <string>

int main() {
    std::string latex = "2\\sqrt{2}";
    int length = latex.length();
    std::cout << "LaTeX length: " << length << std::endl;
    
    // 根据长度计算缩放因子
    double scale = 1.0;
    if (length > 50) {
        scale = 0.7;
    } else if (length > 30) {
        scale = 0.85;
    } else if (length > 20) {
        scale = 1.0;
    } else {
        scale = 1.2;
    }
    
    std::cout << "Scale factor: " << scale << std::endl;
    std::cout << "\\[ \\scalebox{" << scale << "}[1.0]{" << latex << "} \\]\n";
    return 0;
}
