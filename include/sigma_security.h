#ifndef SIGMA_SECURITY_H
#define SIGMA_SECURITY_H

#include "suites/S01_Genesis/shards/sigma_types.h"

/* =========================================================================
 * SIGMA OS: SECURITY SUITE (S08) - SOVEREIGN SECURITY MATRIX
 * Hardware-enforced access control replacing SELinux/AppArmor entirely.
 * ========================================================================= */

#define MAX_SECURITY_DOMAINS 64

typedef enum {
    PERM_NONE       = 0x00,
    PERM_READ       = 0x01,
    PERM_WRITE      = 0x02,
    PERM_EXECUTE    = 0x04,
    PERM_NET_ACCESS = 0x08,
    PERM_KERNEL_OP  = 0x10,
} sigma_permission_t;

typedef struct {
    uint32_t domain_id;
    char     name[32];
    uint8_t  permissions;  // Bitfield of sigma_permission_t flags
    uint32_t allowed_syscalls[64]; // Allowlisted system call IDs
    uint8_t  active;
} __attribute__((packed)) sigma_security_domain_t;

void sigma_security_init(void);
int sigma_security_create_domain(const char* name, uint8_t permissions);
int sigma_security_check(uint32_t domain_id, sigma_permission_t perm);
void sigma_security_audit_log(uint32_t domain_id, const char* action);

#endif
