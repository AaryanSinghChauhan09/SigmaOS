#include "sigma_hal.h"
#include "SovereignLibC.h"
#ifndef SOVEREIGN_PACKAGE_HPP
#define SOVEREIGN_PACKAGE_HPP

#include "sigma_types.h"

class SovereignPackageEngine {
public:
    static SovereignPackageEngine& getInstance();
    void init();
    bool installPackage(const char* pkg_name);

private:
    SovereignPackageEngine() : packages_installed(0) {}
    sigma_u32 packages_installed;
};

extern "C" {
    void package_manager_init();
    bool package_manager_install(const char* name);
}

#endif

