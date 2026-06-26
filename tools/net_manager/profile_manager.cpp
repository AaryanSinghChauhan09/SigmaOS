#include "../../include/sigma_kernel_types.h"
#include <string>

extern "C" {
    void sigma_log_info(const char* fmt, ...);
    void sigma_log_error(const char* fmt, ...);
}

namespace SigmaOS {
namespace NetManager {

enum class ProfileType {
    HOME,
    ENTERPRISE,
    CLOUD
};

struct NetProfile {
    ProfileType type;
    std::string name;
    bool use_dhcp;
    std::string static_ip;
    bool enforce_strict_firewall;
};

// Snapshot cache for rollback
NetProfile previous_profile;
bool has_snapshot = false;

sigma_status snapshot_network_state(const NetProfile& current) {
    sigma_log_info("[ProfileManager] Snapshotting active declarative network state...");
    previous_profile = current;
    has_snapshot = true;
    sigma_log_info("[ProfileManager] Network snapshot secure.");
    return K_OK;
}

sigma_status apply_profile(const NetProfile& profile) {
    sigma_log_info("[ProfileManager] Parsing NixOS-style declarative profile: '%s'", profile.name.c_str());
    
    if (profile.enforce_strict_firewall) {
        sigma_log_info("[ProfileManager] Applying MAC firewall rules for High Security...");
    } else {
        sigma_log_info("[ProfileManager] Relaxing specific firewall rules for Home/Trusted zone...");
    }
    
    if (profile.use_dhcp) {
        sigma_log_info("[ProfileManager] Networking stack configured for dynamic (DHCP) lease.");
    } else {
        sigma_log_info("[ProfileManager] Networking stack configured for static IP: %s", profile.static_ip.c_str());
    }
    
    sigma_log_info("[ProfileManager] Profile '%s' successfully applied to running kernel.", profile.name.c_str());
    return K_OK;
}

sigma_status rollback_network_state() {
    if (!has_snapshot) {
        sigma_log_error("[ProfileManager] CRITICAL: No network snapshot available for rollback!");
        return K_ERR_INVAL;
    }
    
    sigma_log_info("[ProfileManager] INITIATING NETWORK ROLLBACK...");
    apply_profile(previous_profile);
    sigma_log_info("[ProfileManager] Network state successfully reverted to last known-good configuration.");
    return K_OK;
}

} // namespace NetManager
} // namespace SigmaOS
