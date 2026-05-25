#include "../../hal/SovereignHAL.hpp"

#include <iostream>
#include <string>

// External declarations for mock functions
extern "C" {
    void sigma_log_info(const char* fmt, ...);
    void sigma_log_error(const char* fmt, ...);
}

namespace SigmaOS {
namespace DriverManager {
    struct RepoPackage {
        sigma_u32 driver_id;
        const char* package_name;
        const char* signature;
        const sigma_u8* binary_blob;
        sigma_usize blob_size;
    };
    sigma_status fetch_driver(sigma_u32 requested_id, RepoPackage* out_pkg);
    bool verify_driver_signature(const RepoPackage& pkg);
    sigma_status snapshot_current_driver_state(sigma_u32 driver_id);
    sigma_status restore_driver_snapshot(sigma_u32 driver_id);
}
}

using namespace SigmaOS;
using namespace SigmaOS::HAL;
using namespace SigmaOS::DriverManager;

void print_help() {
    std::cout << "SigmaOS Sovereign Driver Manager (sigma-driver)\n";
    std::cout << "Usage:\n";
    std::cout << "  sigma-driver scan          - Enumerate hardware and identify missing drivers\n";
    std::cout << "  sigma-driver install <id>  - Fetch, cryptographically verify, and install driver\n";
    std::cout << "  sigma-driver rollback <id> - Revert to the last known-good driver snapshot\n";
}

void cmd_scan() {
    std::cout << "[Sigma-Driver] Initiating Sovereign Hardware Scan...\n";
    BoardTelemetry telemetry = SovereignHAL::getInstance().getSystemTelemetry();
    
    std::cout << "\n--- PCI Bus Devices ---\n";
    for(sigma_u32 i = 0; i < telemetry.pci_device_count; i++) {
        std::cout << "Device ID: " << telemetry.pci_devices[i].device_id 
                  << " | Vendor ID: " << telemetry.pci_devices[i].vendor_id << "\n";
    }

    std::cout << "\n--- USB Bus Devices ---\n";
    for(sigma_u32 i = 0; i < telemetry.usb_device_count; i++) {
        std::cout << "Product ID: " << telemetry.usb_devices[i].idProduct 
                  << " | Vendor ID: " << telemetry.usb_devices[i].idVendor << "\n";
    }
    
    std::cout << "\n[Sigma-Driver] Hardware scan complete.\n";
}

void cmd_install(sigma_u32 driver_id) {
    std::cout << "[Sigma-Driver] Initiating installation for Driver ID: " << driver_id << "\n";
    
    // 1. Snapshot current state
    if (snapshot_current_driver_state(driver_id) != K_OK) {
        std::cout << "[ERR] Failed to create driver snapshot. Aborting update for safety.\n";
        return;
    }
    
    // 2. Fetch driver package
    RepoPackage pkg;
    if (fetch_driver(driver_id, &pkg) != K_OK) {
        std::cout << "[ERR] Failed to securely fetch driver from Sovereign Ledger.\n";
        return;
    }
    
    // 3. Cryptographic Verification
    if (!verify_driver_signature(pkg)) {
        std::cout << "[ERR] CRITICAL: Cryptographic verification failed! Discarding compromised package.\n";
        return;
    }
    
    // 4. Install logic (Mocked)
    std::cout << "[Sigma-Driver] Package validated. Installing '" << pkg.package_name << "' into sovereign module space...\n";
    std::cout << "[Sigma-Driver] Installation successful!\n";
}

void cmd_rollback(sigma_u32 driver_id) {
    std::cout << "[Sigma-Driver] Initiating emergency rollback for Driver ID: " << driver_id << "\n";
    
    if (restore_driver_snapshot(driver_id) == K_OK) {
        std::cout << "[Sigma-Driver] Rollback complete. Previous known-good state restored.\n";
    } else {
        std::cout << "[ERR] CRITICAL: Failed to restore snapshot!\n";
    }
}

int main(int argc, char* argv[]) {
    if (argc < 2) {
        print_help();
        return 1;
    }

    std::string cmd = argv[1];

    // Ensure HAL is initialized to bootstrap the telemetry
    SovereignHAL::getInstance().initializeHAL();

    if (cmd == "scan") {
        cmd_scan();
    } else if (cmd == "install") {
        if (argc < 3) {
            std::cout << "Missing driver ID.\n";
            return 1;
        }
        cmd_install(std::stoi(argv[2]));
    } else if (cmd == "rollback") {
        if (argc < 3) {
            std::cout << "Missing driver ID.\n";
            return 1;
        }
        cmd_rollback(std::stoi(argv[2]));
    } else {
        std::cout << "Unknown command: " << cmd << "\n";
        print_help();
        return 1;
    }

    return 0;
}
