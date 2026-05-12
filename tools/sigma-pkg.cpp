#include "sigma_sdk.h"
#include <iostream>
#include <string>
#include <vector>

/**
 * SigmaOS Package Manager CLI (sigma-pkg)
 * Purpose: Professional interface for shard management and repository synchronization.
 * Usage: sigma-pkg install [shard-id] | sigma-pkg list | sigma-pkg sync
 */

void print_help() {
    std::cout << "Σ SigmaOS Package Manager (sigma-pkg) v15.0" << std::endl;
    std::cout << "Usage: sigma-pkg <command> [options]" << std::endl;
    std::cout << "Commands:" << std::endl;
    std::cout << "  install <id>   Install a professional shard from the lattice nexus." << std::endl;
    std::cout << "  remove  <id>   Decommission a shard from the local silicon node." << std::endl;
    std::cout << "  list           List all active professional shards." << std::endl;
    std::cout << "  sync           Synchronize local lattice with the global repository." << std::endl;
}

int main(int argc, char* argv[]) {
    if (argc < 2) {
        print_help();
        return 1;
    }

    std::string cmd = argv[1];

    if (cmd == "install" && argc > 2) {
        std::cout << "[S-PKG] Initializing PQC-signed download for shard: " << argv[2] << "..." << std::endl;
        // Hit & Trial: Bridge to LatticePackageNexus via SDK
        sigma_pkg_install(argv[2]);
        std::cout << "[S-PKG] Shard " << argv[2] << " integrated successfully." << std::endl;
    } else if (cmd == "list") {
        std::cout << "[S-PKG] Querying local lattice registry..." << std::endl;
        sigma_pkg_list();
    } else if (cmd == "sync") {
        std::cout << "[S-PKG] Synchronizing with Sovereign Repository (Lattice-Net)..." << std::endl;
        sigma_pkg_sync();
    } else {
        print_help();
    }

    return 0;
}
