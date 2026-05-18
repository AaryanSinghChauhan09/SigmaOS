#pragma once
#include <stdint.h>
#include "libc/sigma_libc.h"

namespace SigmaOS {
namespace Userland {

// Sprint 6: Userland Utilities - Lightweight File Manager
class SigmaFileManager {
private:
    bool encryption_plugin_loaded;
    bool compression_plugin_loaded;

public:
    SigmaFileManager() : encryption_plugin_loaded(false), compression_plugin_loaded(false) {
        sigma_log("[FM] Sigma File Manager (sigma-fm) Initialized.");
    }

    void list_directory(const char* path) {
        sigma_print("[FM] Listing directory: ");
        sigma_print(path);
        sigma_print("\n");
        // Emulate VFS call
        sigma_print("  drwxr-xr-x  root  root   4096  Jan 1 00:00 .\n");
        sigma_print("  -rw-r--r--  user  user   1024  Jan 1 00:00 config.json\n");
    }

    void copy_file(const char* src, const char* dest) {
        sigma_print("[FM] Copying ");
        sigma_print(src);
        sigma_print(" -> ");
        sigma_print(dest);
        sigma_print("\n");
    }

    void load_plugin(const char* plugin_name) {
        if (sigma_strcmp(plugin_name, "encryption") == 0) {
            encryption_plugin_loaded = true;
            sigma_log("[FM] Encryption plugin loaded successfully.");
        }
    }
};

} // namespace Userland
} // namespace SigmaOS
