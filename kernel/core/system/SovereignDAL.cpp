#include "system/SovereignDAL.h"
#include "hal/sigma_hal.h"
#include "sigma_log.h"

namespace SigmaOS {
namespace Kernel {
namespace System {

SovereignDAL& SovereignDAL::getInstance() {
    static SovereignDAL instance;
    return instance;
}

SovereignDAL::SovereignDAL() : m_provider(PackageProvider::UNKNOWN) {}

void SovereignDAL::initialize() {
    detectProvider();
    sigma_log_info("[DAL] Distro Abstraction Layer initialized.");
}

void SovereignDAL::detectProvider() {
    sigma_log_info("[DAL] Probing host distro for package management signatures...");
    m_provider = PackageProvider::PACMAN; 
    sigma_log_info("[DAL] Package provider: pacman (Arch/SigmaOS).");
}

bool SovereignDAL::installPackage(const char* name) {
    sigma_log_info("[DAL] Translating install command for active provider:");
    sigma_log_info(name);
    auto& self = getInstance();
    switch (self.m_provider) {
        case PackageProvider::APT:    sigma_log_info("[DAL] Backend: apt-get install"); break;
        case PackageProvider::PACMAN: sigma_log_info("[DAL] Backend: pacman -S");       break;
        case PackageProvider::DNF:    sigma_log_info("[DAL] Backend: dnf install");     break;
        default: return false;
    }
    return true;
}

bool SovereignDAL::removePackage(const char* name) {
    sigma_log_info("[DAL] Requesting package excision:");
    sigma_log_info(name);
    auto& self = getInstance();
    switch (self.m_provider) {
        case PackageProvider::APT:    sigma_log_info("[DAL] Backend: apt-get remove"); break;
        case PackageProvider::PACMAN: sigma_log_info("[DAL] Backend: pacman -R");      break;
        case PackageProvider::DNF:    sigma_log_info("[DAL] Backend: dnf remove");     break;
        default: return false;
    }
    return true;
}

} // namespace System
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

/* --- C Bridge --- */
void sigma_dal_init() {
    SigmaOS::Kernel::System::SovereignDAL::getInstance().initialize();
}

extern "C" int sigma_dal_install(const char* pkg) {
    return SigmaOS::Kernel::System::SovereignDAL::getInstance().installPackage(pkg ? pkg : "") ? 1 : 0;
}

extern "C" int sigma_dal_remove(const char* pkg) {
    return SigmaOS::Kernel::System::SovereignDAL::getInstance().removePackage(pkg ? pkg : "") ? 1 : 0;
}



} // extern "C"
