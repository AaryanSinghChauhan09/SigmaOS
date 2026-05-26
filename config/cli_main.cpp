#include <iostream>
#include <string>

extern "C" {
    void apply_declarative_config(const char* config_file);
    void commit_config_generation();
    void rollback_config_generation(int target_gen);
    void print_config_status();
}

void print_help() {
    std::cout << "SigmaOS Declarative Config Utility (sigma-config)\n";
    std::cout << "Usage:\n";
    std::cout << "  sigma-config apply <file>   - Parse and apply declarative configuration state\n";
    std::cout << "  sigma-config rollback <gen> - Revert globally to a previous system generation\n";
    std::cout << "  sigma-config status         - Display active configuration and generation\n";
}

int main(int argc, char* argv[]) {
    if (argc < 2) {
        print_help();
        return 1;
    }

    std::string cmd = argv[1];

    if (cmd == "apply") {
        if (argc < 3) {
            std::cout << "[sigma-config] Error: Please specify a SigmaConf file to apply.\n";
            return 1;
        }
        apply_declarative_config(argv[2]);
        commit_config_generation();
        return 0;
    } 
    else if (cmd == "rollback") {
        if (argc < 3) {
            std::cout << "[sigma-config] Error: Please specify target generation ID.\n";
            return 1;
        }
        int gen = std::stoi(argv[2]);
        rollback_config_generation(gen);
        return 0;
    } 
    else if (cmd == "status") {
        print_config_status();
        return 0;
    }
    else {
        std::cout << "[sigma-config] Unknown command: " << cmd << "\n";
        print_help();
        return 1;
    }
}
