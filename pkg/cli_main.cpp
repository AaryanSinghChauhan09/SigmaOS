#include <iostream>
#include <string>

// Forward declarations of backend package actions
extern "C" {
    void sigma_log_info(const char* fmt, ...);
    void sigma_log_error(const char* fmt, ...);
    int sigma_pkg_install(const char* pkg_name);
    int sigma_pkg_remove(const char* pkg_name);
    int sigma_pkg_update_all();
    int sigma_pkg_list_installed();
    int sigma_pkg_search(const char* query);
}

void print_help() {
    std::cout << "SigmaOS Sovereign Package Manager (spkg)\n";
    std::cout << "Usage:\n";
    std::cout << "  spkg install <pkg>  - Resolve, download, cryptographically verify, and install a package\n";
    std::cout << "  spkg remove <pkg>   - Uninstall a package and prune dependencies\n";
    std::cout << "  spkg search <query> - Search the sovereign registry for matching packages\n";
    std::cout << "  spkg list           - List all active installed packages in isolated shards\n";
    std::cout << "  spkg update         - Sync registry mirrors and perform atomic upgrades\n";
}

int main(int argc, char* argv[]) {
    if (argc < 2) {
        print_help();
        return 1;
    }

    std::string cmd = argv[1];

    if (cmd == "install") {
        if (argc < 3) {
            std::cout << "[spkg] Error: Please specify a package name to install.\n";
            return 1;
        }
        std::cout << "[spkg] Initiating post-quantum secure installation for: " << argv[2] << "\n";
        return sigma_pkg_install(argv[2]);
    } 
    else if (cmd == "remove") {
        if (argc < 3) {
            std::cout << "[spkg] Error: Please specify a package name to remove.\n";
            return 1;
        }
        std::cout << "[spkg] Initiating removal of: " << argv[2] << "\n";
        return sigma_pkg_remove(argv[2]);
    } 
    else if (cmd == "search") {
        if (argc < 3) {
            std::cout << "[spkg] Error: Please specify a query term.\n";
            return 1;
        }
        return sigma_pkg_search(argv[2]);
    } 
    else if (cmd == "list") {
        return sigma_pkg_list_installed();
    } 
    else if (cmd == "update") {
        std::cout << "[spkg] Fetching package index signed with Dilithium-5 keys...\n";
        return sigma_pkg_update_all();
    } 
    else {
        std::cout << "[spkg] Unknown package command: " << cmd << "\n";
        print_help();
        return 1;
    }
}
