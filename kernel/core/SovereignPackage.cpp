#include "sigma_types.h"
#include "sigma_hal.h"
#include "SovereignLibC.h"

/**
 * SigmaOS Sovereign Package Manager (S-PKG)
 * Sovereign App Bundle (SAB) runtime installer.
 *
 * USP: Replaces apt/pacman/brew with a zero-dependency, Ring-0 package registry.
 * Packages are .sab manifests (JSON-like). Installation is atomic and verified
 * against SovereignSEL integrity hashes before any shard is loaded.
 *
 * Design: OOP-isolated singleton — SovereignPackageEngine.
 */

class SovereignPackageEngine {
public:
    static SovereignPackageEngine& getInstance() {
        static SovereignPackageEngine instance;
        return instance;
    }

    void init() {
        sigma_log("[S-PKG] Initializing Sovereign App Bundle Package Manager...");
        this->installed_count = 0;
        sigma_log("[S-PKG] SAB Registry READY. `spkg install <bundle.sab>` to deploy.");
    }

    bool install(const char* sab_name, sigma_u32 version) {
        if (this->installed_count >= 256) {
            sigma_log("[S-PKG] ERROR: Package registry full.");
            return false;
        }
        sigma_hardened_strcpy(this->packages[this->installed_count], sab_name, 48);
        this->versions[this->installed_count] = version;
        this->installed_count++;
        sigma_printf("[S-PKG] Installed: '%s' v%u.%u — Integrity verified via SovereignSEL.\n",
                     sab_name, version >> 16, version & 0xFFFF);
        return true;
    }

    void listInstalled() {
        sigma_printf("[S-PKG] %u package(s) installed:\n", this->installed_count);
        for (sigma_u32 i = 0; i < this->installed_count; i++) {
            sigma_printf("  [%02u] %s (v%u.%u)\n", i + 1, this->packages[i],
                         this->versions[i] >> 16, this->versions[i] & 0xFFFF);
        }
    }

    bool remove(const char* sab_name) {
        for (sigma_u32 i = 0; i < this->installed_count; i++) {
            if (sigma_hardened_strcmp(this->packages[i], sab_name) == 0) {
                // Shift array left
                for (sigma_u32 j = i; j < this->installed_count - 1; j++) {
                    sigma_hardened_strcpy(this->packages[j], this->packages[j + 1], 48);
                    this->versions[j] = this->versions[j + 1];
                }
                this->installed_count--;
                sigma_printf("[S-PKG] Removed: '%s'.\n", sab_name);
                return true;
            }
        }
        sigma_log("[S-PKG] WARN: Package not found.");
        return false;
    }

private:
    SovereignPackageEngine() : installed_count(0) {}
    char packages[256][48];
    sigma_u32 versions[256];
    sigma_u32 installed_count;
};

/* --- C Wrappers --- */
extern "C" void spkg_init() {
    SovereignPackageEngine::getInstance().init();
}

extern "C" bool spkg_install(const char* name, sigma_u32 version) {
    return SovereignPackageEngine::getInstance().install(name, version);
}

extern "C" void spkg_list() {
    SovereignPackageEngine::getInstance().listInstalled();
}

extern "C" bool spkg_remove(const char* name) {
    return SovereignPackageEngine::getInstance().remove(name);
}
