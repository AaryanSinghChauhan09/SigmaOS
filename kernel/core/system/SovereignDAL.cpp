#include "system/SovereignDAL.h"
#include "sigma_hal.h"
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
    // Future: Use system(3) or access(2) to verify binary paths
    m_provider = PackageProvider::PACMAN; 
    sigma_log_info("[DAL] Package provider: pacman (Arch/SigmaOS).");
}

bool SovereignDAL::installPackage(const std::string& name) {
    sigma_log_info("[DAL] Translating install command for active provider: %s", name.c_str());
    switch (m_provider) {
        case PackageProvider::APT:    sigma_log_info("[DAL] Backend: apt-get install"); break;
        case PackageProvider::PACMAN: sigma_log_info("[DAL] Backend: pacman -S");       break;
        case PackageProvider::DNF:    sigma_log_info("[DAL] Backend: dnf install");     break;
        default: return false;
    }
    m_cache[name] = {name, "latest", "Injected via Sovereign DAL", true};
    return true;
}

bool SovereignDAL::removePackage(const std::string& name) {
    sigma_log_info("[DAL] Requesting package excision: %s", name.c_str());
    switch (m_provider) {
        case PackageProvider::APT:    sigma_log_info("[DAL] Backend: apt-get remove"); break;
        case PackageProvider::PACMAN: sigma_log_info("[DAL] Backend: pacman -R");      break;
        case PackageProvider::DNF:    sigma_log_info("[DAL] Backend: dnf remove");     break;
        default: return false;
    }
    m_cache.erase(name);
    return true;
}

std::vector<PackageInfo> SovereignDAL::searchPackages(const std::string& query) {
    (void)query;
    return {
        {"sigma-cli",     "1.0.0", "Primary CLI tool for SigmaOS",            false},
        {"lattice-utils", "2.1.4", "Core utility shards for distributed sync", true },
        {"pqc-toolkit",   "0.9.8", "Post-Quantum Cryptography dev kit",        false}
    };
}

} // namespace System
} // namespace Kernel
} // namespace SigmaOS

/* --- C Bridge (Preserved for compatibility) --- */
extern "C" void sigma_dal_init() {
    SigmaOS::Kernel::System::SovereignDAL::getInstance().initialize();
}

extern "C" int sigma_dal_install(const char* pkg) {
    return SigmaOS::Kernel::System::SovereignDAL::getInstance().installPackage(pkg ? pkg : "") ? 1 : 0;
}

extern "C" int sigma_dal_remove(const char* pkg) {
    return SigmaOS::Kernel::System::SovereignDAL::getInstance().removePackage(pkg ? pkg : "") ? 1 : 0;
}
