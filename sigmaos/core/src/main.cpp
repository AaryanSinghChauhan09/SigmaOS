#include "sigma_core.h"
#include <iostream>
#include <string>

int main(int argc, char** argv) {
    std::cout << "========================================" << std::endl;
    std::cout << "   SigmaOS Sovereign Native Entrypoint  " << std::endl;
    std::cout << "========================================" << std::endl;

    if (argc < 2) {
        std::cout << "Usage: s-os-native <command> [args]" << std::endl;
        return 1;
    }

    std::string cmd = argv[1];

    if (cmd == "boot") {
        std::cout << "[NativeOS] Initializing subsystems..." << std::endl;
        ui_init();
        sec_audit();
        auto_run_all(auto_init());
    } else if (cmd == "audit") {
        ledger_audit();
        comp_audit();
    } else {
        std::cout << "[NativeOS] Unknown command: " << cmd << std::endl;
    }

    return 0;
}
