#ifndef SOVEREIGN_PACKAGE_MANAGER_H
#define SOVEREIGN_PACKAGE_MANAGER_H

#include "../sigma_types.h"
#include "../SigmaOOP.hpp"

namespace SigmaOS {
namespace System {
namespace PackageManagement {

struct PackageManifest {
    char name[64];
    char version[16];
    char author[64];
    sigma_u64 install_size;
    bool is_verified;
};

class SovereignPackageManager : public SigmaObject, public SigmaSingleton<SovereignPackageManager> {
public:
    void init();
    
    // Core Operations
    sigma_status install(const char* shard_id);
    sigma_status uninstall(const char* shard_id);
    void list_installed();
    void sync_repository();
    
    // Security & Atomicity
    bool verify_signature(const char* shard_id);
    void create_rollback_point();

    virtual const char* type_name() const noexcept override { return "SovereignPackageManager"; }

private:
    friend class SigmaSingleton<SovereignPackageManager>;
    SovereignPackageManager() = default;
    
    PackageManifest m_installed_shards[128];
    sigma_u32 m_shard_count;
};

} // namespace PackageManagement
} // namespace System
} // namespace SigmaOS

extern "C" {
    void spkg_init();
    void spkg_install(const char* id);
    void spkg_list();
    void spkg_sync();
}

#endif // SOVEREIGN_PACKAGE_MANAGER_H
