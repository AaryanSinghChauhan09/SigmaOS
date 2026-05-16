/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ARMOR POLICY ENGINE (S-ARMOR)
 * =========================================================================
 * Mission: Mandatory Access Control (MAC) and Shard Sandboxing.
 * Inspired by SELinux / AppArmor / GRSecurity.
 * =========================================================================
 */

#ifndef SIGMA_ARMOR_H
#define SIGMA_ARMOR_H

#include "./sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    ARMOR_LEVEL_PERMISSIVE,
    ARMOR_LEVEL_ENFORCING,
    ARMOR_LEVEL_PARANOID
} sigma_armor_level_t;

typedef struct {
    char shard_id[32];
    bool allow_net;
    bool allow_storage;
    bool allow_ipc;
} sigma_armor_policy_t;

/* --- Armor Primitives --- */
void      armor_init(void);
void      armor_set_level(sigma_armor_level_t level);
bool      armor_check_permission(const char* shard_id, const char* action);
void      armor_enforce_policy(const sigma_armor_policy_t* policy);

#ifdef __cplusplus
}

namespace SigmaOS {
namespace Kernel {
namespace Security {

class SovereignArmorEngine {
public:
    static SovereignArmorEngine& getInstance() {
        static SovereignArmorEngine instance;
        return instance;
    }

    void init();
    void setLevel(sigma_armor_level_t level);
    bool checkPermission(const char* sid, const char* act);
    void applyPolicy(const sigma_armor_policy_t* policy);

private:
    SovereignArmorEngine() : m_current_level(ARMOR_LEVEL_ENFORCING) {}
    sigma_armor_level_t m_current_level;
};

} // namespace Security
} // namespace Kernel
} // namespace SigmaOS
#endif

#endif /* SIGMA_ARMOR_H */
