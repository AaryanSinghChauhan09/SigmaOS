#include <iostream>
#include <string>

int main(int argc, char* argv[]) {
    std::cout << "[S-CLI] Executing Sovereign Command..." << std::endl;
    for (int i = 1; i < argc; ++i) {
        std::cout << "  Arg " << i << ": " << argv[i] << std::endl;
    }
    std::cout << "[S-CLI] Command executed successfully." << std::endl;
    return 0;
}
