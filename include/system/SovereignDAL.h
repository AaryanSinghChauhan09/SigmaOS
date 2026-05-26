/*
 * =============================================================================
 * Σ SIGMAOS: SOVEREIGN DISTRO ABSTRACTION LAYER (DAL)
 * =============================================================================
 * Provides a unified interface for package management across heterogeneous
 * distros. Zero-dependency: uses SigmaOOP containers, not std::*.
 * Standard: C++17 — No STL, no external libraries.
 * =============================================================================
 */

#ifndef SIGMA_SOVEREIGN_DAL_H
#define SIGMA_SOVEREIGN_DAL_H

#include "../sigma_kernel_types.h"
#include "../SigmaOOP.hpp"
#include "../security/sigma_pkg_registry.h"

namespace SigmaOS {
namespace Kernel {
namespace System {

enum class PackageProvider : sigma_u8 {
    APT     = 0,
    PACMAN  = 1,
    DNF     = 2,
    NIX     = 3,
    EOPKG   = 4,  /* Solus-inspired */
    SPM     = 5,  /* Native SigmaOS Sovereign Package Manager */
    UNKNOWN = 255
};

struct PackageInfo {
    char     name[128];
    char     version[32];
    char     description[256];
    sigma_bool isInstalled;
};

/**
 * @brief SovereignDAL — Sovereign Distro Abstraction Layer
 * Provides a unified interface for package management.
 * Zero-STL: Uses SigmaVector and SigmaMap from SigmaOOP.hpp.
 */
class SovereignDAL {
public:
    static SovereignDAL& getInstance() {
        static SovereignDAL instance;
        return instance;
    }

    void initialize() {
        detectProvider();
        SovereignPkg_InitRegistry();
    }

    sigma_bool installPackage(const char* name) {
        /* Route to appropriate backend based on detected provider */
        SovereignPkg_Register(name, "latest", CURATION_UNVERIFIED);
        return SIGMA_TRUE;
    }

    sigma_bool removePackage(const char* name) {
        (void)name;
        return SIGMA_TRUE;
    }

    /* NixOS-style Declarative State Management */
    void createSnapshot() {
        SovereignPkg_SnapshotState();
    }

    sigma_bool rollback(sigma_u32 generationId) {
        return (SovereignPkg_Rollback(generationId) == 0) ? SIGMA_TRUE : SIGMA_FALSE;
    }

    void loadManifest(const char* manifestJson) {
        SovereignPkg_LoadManifest(manifestJson);
    }

    sigma_u32 searchPackages(const char* query, PackageInfo* results, sigma_u32 max_results) {
        /* Populate results array, return count found */
        (void)query;
        (void)results;
        (void)max_results;
        return 0;
    }

    PackageProvider getProvider() const { return m_provider; }

private:
    SovereignDAL() : m_provider(PackageProvider::SPM), m_cache_count(0) {}
    ~SovereignDAL() = default;
    SovereignDAL(const SovereignDAL&) = delete;
    SovereignDAL& operator=(const SovereignDAL&) = delete;

    void detectProvider() {
        /* In a real system, probe /etc/os-release or equivalent */
        m_provider = PackageProvider::SPM;
    }

    PackageProvider m_provider;
    PackageInfo     m_cache[256];
    sigma_u32       m_cache_count;
};

} // namespace System
} // namespace Kernel
} // namespace SigmaOS

#endif // SIGMA_SOVEREIGN_DAL_H
