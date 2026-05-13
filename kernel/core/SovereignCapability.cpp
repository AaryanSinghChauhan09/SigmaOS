#include "sigma_types.h"
#include "../../../include/sigma_log.h"
#include "sigma_hal.h"
#include "../../../include/sigma_log.h"
#include "SovereignLibC.h"
#include "../../../include/sigma_log.h"

/**
 * SigmaOS Sovereign Capability Engine
 * Per-process capability bitmask for syscall gating.
 *
 * USP: Replaces Linux's coarse-grained `sudo` with a mathematically precise
 * per-process capability vector. Each shard only possesses the capabilities
 * it explicitly declared — zero privilege escalation possible.
 *
 * Design: OOP-isolated singleton — SovereignCapabilityEngine.
 */

typedef sigma_u64 sigma_caps_t;

// Capability bitmask definitions
#define SIGMA_CAP_NET_SEND    (1ULL << 0)
#define SIGMA_CAP_FS_WRITE    (1ULL << 1)
#define SIGMA_CAP_EXEC_CHILD  (1ULL << 2)
#define SIGMA_CAP_GPU_ACCESS  (1ULL << 3)
#define SIGMA_CAP_AUDIO       (1ULL << 4)
#define SIGMA_CAP_ENCLAVE     (1ULL << 5)

class SovereignCapabilityEngine {
public:
    static SovereignCapabilityEngine& getInstance() {
        static SovereignCapabilityEngine instance;
        return instance;
    }

    void init() {
        sigma_log("[CAPABILITY] Initializing Sovereign Per-Process Capability Engine...");
        this->process_count = 0;
    }

    void grantCapabilities(sigma_u32 pid, sigma_caps_t caps) {
        if (this->process_count >= 512) return;
        this->pids[this->process_count] = pid;
        this->caps[this->process_count] = caps;
        this->process_count++;
        sigma_log_info("[CAPABILITY] PID %u granted capability mask: 0x%016llX\n",
                     pid, (unsigned long long)caps);
    }

    bool checkCapability(sigma_u32 pid, sigma_caps_t required_cap) {
        for (sigma_u32 i = 0; i < this->process_count; i++) {
            if (this->pids[i] == pid) {
                bool granted = (this->caps[i] & required_cap) != 0;
                if (!granted) {
                    sigma_log_info("[CAPABILITY] ACCESS DENIED: PID %u lacks cap 0x%llX\n",
                                 pid, (unsigned long long)required_cap);
                }
                return granted;
            }
        }
        sigma_log_info("[CAPABILITY] PID %u not registered — DENY ALL.\n", pid);
        return false;
    }

private:
    SovereignCapabilityEngine() : process_count(0) {}
    sigma_u32 pids[512];
    sigma_caps_t caps[512];
    sigma_u32 process_count;
};

/* --- C Wrappers --- */
extern "C" void capability_init() {
    SovereignCapabilityEngine::getInstance().init();
}

extern "C" void capability_grant(sigma_u32 pid, sigma_u64 caps) {
    SovereignCapabilityEngine::getInstance().grantCapabilities(pid, caps);
}

extern "C" bool capability_check(sigma_u32 pid, sigma_u64 required) {
    return SovereignCapabilityEngine::getInstance().checkCapability(pid, required);
}


