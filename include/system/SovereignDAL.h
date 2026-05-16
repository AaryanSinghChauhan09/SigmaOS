#ifndef SIGMA_SOVEREIGN_DAL_H
#define SIGMA_SOVEREIGN_DAL_H

#include "../core/sigma_types.h"
#include <string>
#include <map>
#include <vector>

namespace SigmaOS {
namespace Kernel {
namespace System {

enum class PackageProvider { APT, PACMAN, DNF, UNKNOWN };

struct PackageInfo {
    std::string name;
    std::string version;
    std::string description;
    bool        isInstalled;
};

/**
 * @brief SovereignDAL â€” Sovereign Distro Abstraction Layer
 * Provides a unified interface for package management across heterogeneous distros.
 */
class SovereignDAL {
public:
    static SovereignDAL& getInstance();

    void initialize();
    bool installPackage(const std::string& name);
    bool removePackage(const std::string& name);
    std::vector<PackageInfo> searchPackages(const std::string& query);

private:
    SovereignDAL();
    ~SovereignDAL() = default;
    SovereignDAL(const SovereignDAL&) = delete;
    SovereignDAL& operator=(const SovereignDAL&) = delete;

    void detectProvider();

    PackageProvider m_provider;
    std::map<std::string, PackageInfo> m_cache;
};

} // namespace System
} // namespace Kernel
} // namespace SigmaOS

#endif // SIGMA_SOVEREIGN_DAL_H
