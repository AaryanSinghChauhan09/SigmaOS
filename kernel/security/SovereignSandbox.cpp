/**
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SECURITY SANDBOX (CBAC)
 * =========================================================================
 * Implementation of the Capability-Based Access Control engine.
 * =========================================================================
 */

#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_log.h"
#include "../../include/kernel/sigma_sandbox.h"

#define MAX_SANDBOX_PROFILES 64
#define MAX_TRACKED_PIDS     4096

namespace SigmaOS {
namespace Security {

class SovereignSandbox {
public:
    static SovereignSandbox& getInstance() {
        static SovereignSandbox instance;
        return instance;
    }

    void init() {
        m_profile_count = 0;
        
        for (sigma_u32 i = 0; i < MAX_TRACKED_PIDS; i++) {
            m_pid_profiles[i] = 0; /* 0 means strictly isolated / no capabilities */
        }
        
        sigma_log("[Sandbox] Sovereign Security Sandbox (CBAC) initialized.");
        
        /* Create default profiles */
        m_root_profile_id = createProfile(0xFFFFFFFFFFFFFFFFULL); /* System Admin */
        m_user_profile_id = createProfile(CAP_FS_READ | CAP_FS_WRITE | CAP_IPC_SEND | CAP_IPC_RECV);
        m_guest_profile_id = createProfile(CAP_FS_READ);
        
        /* Grant PID 1 (Init) root capabilities */
        applyProfile(1, m_root_profile_id);
    }

    sigma_u32 createProfile(sigma_u64 capabilities) {
        if (m_profile_count >= MAX_SANDBOX_PROFILES) return 0;
        
        sigma_u32 id = m_profile_count + 1;
        sigma_sandbox_profile_t& p = m_profiles[id - 1];
        p.profile_id = id;
        p.capability_mask = capabilities;
        p.enforce_memory_isolation = SIGMA_TRUE;
        p.drop_privileges_on_exec = SIGMA_TRUE;
        
        m_profile_count++;
        sigma_log_info("[Sandbox] Profile %u created with capability mask: 0x%llX\n", id, (unsigned long long)capabilities);
        return id;
    }

    int applyProfile(sigma_u32 pid, sigma_u32 profile_id) {
        if (pid >= MAX_TRACKED_PIDS) return K_ERR_INVAL;
        if (profile_id == 0 || profile_id > m_profile_count) return K_ERR_NOTFOUND;
        
        m_pid_profiles[pid] = profile_id;
        sigma_log_info("[Sandbox] Enforcing Profile %u on Process %u\n", profile_id, pid);
        return K_OK;
    }

    sigma_bool checkCapability(sigma_u32 pid, sigma_u64 requested_cap) {
        if (pid >= MAX_TRACKED_PIDS) {
            sigma_log_info("[Sandbox] AUDIT DENY: Invalid PID %u requested capability 0x%llX\n", pid, (unsigned long long)requested_cap);
            return SIGMA_FALSE;
        }
        
        sigma_u32 prof_id = m_pid_profiles[pid];
        if (prof_id == 0 || prof_id > m_profile_count) {
            sigma_log_info("[Sandbox] AUDIT DENY: Unprofiled PID %u requested capability 0x%llX\n", pid, (unsigned long long)requested_cap);
            return SIGMA_FALSE;
        }
        
        sigma_sandbox_profile_t& p = m_profiles[prof_id - 1];
        
        if ((p.capability_mask & requested_cap) == requested_cap) {
            return SIGMA_TRUE; /* Granted */
        }
        
        sigma_log_info("[Sandbox] AUDIT DENY: PID %u (Profile %u) blocked requesting capability 0x%llX\n", 
                       pid, prof_id, (unsigned long long)requested_cap);
        return SIGMA_FALSE;
    }

private:
    SovereignSandbox() : m_profile_count(0) {}

    sigma_sandbox_profile_t m_profiles[MAX_SANDBOX_PROFILES];
    sigma_u32               m_profile_count;
    
    sigma_u32               m_pid_profiles[MAX_TRACKED_PIDS];
    
    sigma_u32               m_root_profile_id;
    sigma_u32               m_user_profile_id;
    sigma_u32               m_guest_profile_id;
};

} // namespace Security
} // namespace SigmaOS

/* --- C Wrappers --- */
extern "C" {
void sandbox_init(void) { SigmaOS::Security::SovereignSandbox::getInstance().init(); }
sigma_u32 sandbox_create_profile(sigma_u64 caps) { return SigmaOS::Security::SovereignSandbox::getInstance().createProfile(caps); }
int sandbox_apply_profile(sigma_u32 pid, sigma_u32 prof_id) { return SigmaOS::Security::SovereignSandbox::getInstance().applyProfile(pid, prof_id); }
sigma_bool sandbox_check_capability(sigma_u32 pid, sigma_u64 cap) { return SigmaOS::Security::SovereignSandbox::getInstance().checkCapability(pid, cap); }
}
