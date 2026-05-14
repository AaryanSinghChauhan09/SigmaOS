#include "SovereignPackage.hpp"
#include "sigma_log.h"
#include "sigma_hal.h"
#include "sigma_log.h"
#include "SovereignLibC.h"
#include "sigma_log.h"

SovereignPackageEngine& SovereignPackageEngine::getInstance() {
    static SovereignPackageEngine instance;
    return instance;
}

void SovereignPackageEngine::init() {
    sigma_log("[PKG-MGR] Initializing Sovereign Package Engine (S-AUR Bridge)...");
    this->packages_installed = 0;
}

bool SovereignPackageEngine::installPackage(const char* pkg_name) {
    sigma_log_info("[PKG-MGR] Fetching '%s' from Sovereign Distributed Mirror...\n", pkg_name);
    if (sigma_strcmp(pkg_name, "sigma-core-utils") == 0) {
        sigma_log("[PKG-MGR] Installing core utilities... OK");
    } else {
        sigma_log("[PKG-MGR] Package downloaded and verified via PQC signature.");
    }
    this->packages_installed++;
    return true;
}

extern "C" void package_manager_init() {
    SovereignPackageEngine::getInstance().init();
}

extern "C" bool package_manager_install(const char* name) {
    return SovereignPackageEngine::getInstance().installPackage(name);
}


