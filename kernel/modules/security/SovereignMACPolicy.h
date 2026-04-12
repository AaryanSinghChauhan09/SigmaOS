#ifndef SOVEREIGN_MAC_POLICY_H
#define SOVEREIGN_MAC_POLICY_H

#include "../../../include/sigma_kernel.h"

typedef struct {
    char entity_name[64];
    int security_context_level;
    int can_execute;
    int can_write;
    int can_read;
} SovereignMACPolicy_t;

void mac_policy_init(void);
sigma_err_t mac_add_policy(const char* entity, int level, int exec, int write, int read);
SovereignMACPolicy_t* mac_find_policy(const char* entity);

#endif /* SOVEREIGN_MAC_POLICY_H */
