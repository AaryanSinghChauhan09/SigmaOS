/**
 * SigmaOS Sovereign Enforcement Layer (S-SEL)
 * v29.0 Zenith Foundation — Mandatory Access Control (MAC)
 * ZERO-DEPENDENCY: Strictly bare-metal access policy enforcement.
 */

#include "sigma_hal.h"
#include "sigma_types.h"

#define MAX_POLICIES 128

typedef struct {
    uint32_t subject_shard_id;
    uint32_t object_shard_id;
    uint8_t allowed_actions; // Bitmask: 1=READ, 2=WRITE, 4=EXECUTE
} sigma_sel_policy_t;

class SovereignSELEngine {
public:
    static SovereignSELEngine& getInstance() {
        static SovereignSELEngine instance;
        return instance;
    }

    void init() {
        sigma_log("[SEL] Initializing Sovereign Enforcement Layer (MAC algorithm)...");
        this->policy_count = 0;
        
        // Define default strict policies
        this->addPolicy(0, 0, 7); // Kernel (0) has all access to itself
    }

    bool addPolicy(uint32_t subject, uint32_t object, uint8_t actions) {
        if (this->policy_count >= MAX_POLICIES) {
            sigma_printf("[SEL] ERROR: Policy limit reached.\n");
            return false;
        }

        this->policies[this->policy_count].subject_shard_id = subject;
        this->policies[this->policy_count].object_shard_id = object;
        this->policies[this->policy_count].allowed_actions = actions;
        this->policy_count++;
        
        sigma_printf("[SEL] Policy Added: Subj(S%02u) -> Obj(S%02u) | Actions: 0x%02X\n", 
                     subject, object, actions);
        return true;
    }

    bool checkAccess(uint32_t subject, uint32_t object, uint8_t action) const {
        // Enforce Mandatory Access Control (MAC)
        for (uint32_t i = 0; i < this->policy_count; i++) {
            const sigma_sel_policy_t& p = this->policies[i];
            if (p.subject_shard_id == subject && p.object_shard_id == object) {
                if ((p.allowed_actions & action) == action) {
                    return true; // Access granted by policy
                } else {
                    sigma_printf("[SEL] [VIOLATION] Subj(S%02u) denied action %u on Obj(S%02u)\n", 
                                 subject, action, object);
                    return false; // Access explicitly denied
                }
            }
        }
        
        // Default deny if no policy matched
        sigma_printf("[SEL] [VIOLATION] Default DENY for Subj(S%02u) on Obj(S%02u)\n", subject, object);
        return false; 
    }

private:
    SovereignSELEngine() : policy_count(0) {}

    sigma_sel_policy_t policies[MAX_POLICIES];
    uint32_t policy_count;
};

/* --- C Wrappers --- */
extern "C" void sel_init() {
    SovereignSELEngine::getInstance().init();
}

extern "C" bool sel_add_policy(uint32_t subject, uint32_t object, uint8_t actions) {
    return SovereignSELEngine::getInstance().addPolicy(subject, object, actions);
}

extern "C" bool sel_check_access(uint32_t subject, uint32_t object, uint8_t action) {
    return SovereignSELEngine::getInstance().checkAccess(subject, object, action);
}
