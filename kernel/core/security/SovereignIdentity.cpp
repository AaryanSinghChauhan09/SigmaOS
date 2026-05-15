#include "../../../include/sigma_types.h"
#include "../../../include/sigma_log.h"
#include "../../../include/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {
namespace Security {

struct SovereignUser {
    sigma_u32 uid;
    sigma_u32 gid;
    char username[32];
    bool is_root;
};

class SovereignIdentityManager : public SigmaOS::SigmaObject, public SigmaOS::SigmaSingleton<SovereignIdentityManager> {
    friend class SigmaOS::SigmaSingleton<SovereignIdentityManager>;
public:
    const char* type_name() const noexcept override { return "SovereignIdentityManager"; }

    void init() {
        sigma_log_info("[S-AUTH] Initializing Sovereign Identity & Permission Matrix...");
        // Default root user
        m_users[0] = { 0, 0, "root", true };
        m_users[1] = { 1000, 1000, "sovereign", false };
        m_current_user = &m_users[1];
        sigma_log_info("[S-AUTH] User 'sovereign' (UID 1000) logged into industrial lattice.");
    }

    sigma_u32 get_current_uid() const { return m_current_user->uid; }
    sigma_u32 get_current_gid() const { return m_current_user->gid; }
    bool is_current_root() const { return m_current_user->is_root; }

    void switch_user(sigma_u32 uid) {
        sigma_log_info("[S-AUTH] Transitioning to UID %u...", uid);
        for(int i=0; i<2; i++) {
            if (m_users[i].uid == uid) {
                m_current_user = &m_users[i];
                sigma_log_info("[S-AUTH] Identity Switch: SUCCESS (%s)", m_users[i].username);
                return;
            }
        }
        sigma_log_info("[S-AUTH] Identity Switch: FAILED (UID not found)");
    }

private:
    SovereignIdentityManager() = default;
    SovereignUser m_users[16];
    SovereignUser* m_current_user;
};

} // namespace Security
} // namespace Kernel
} // namespace SigmaOS

extern "C" {
    void auth_init() {
        SigmaOS::Kernel::Security::SovereignIdentityManager::getInstance().init();
    }
    sigma_u32 auth_get_uid() {
        return SigmaOS::Kernel::Security::SovereignIdentityManager::getInstance().get_current_uid();
    }
}
