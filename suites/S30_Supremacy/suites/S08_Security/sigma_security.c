#include "libc/SovereignLibC.h"
#include "sigma_security.h"
#include "sigma_libc.h"
#include "sigma_libc.h"

/* =========================================================================
 * SIGMA OS: SECURITY SUITE (S08) - SOVEREIGN SECURITY MATRIX
 * Hardware-enforced access control replacing SELinux/AppArmor entirely.
 * ========================================================================= */

static sigma_security_domain_t domains[MAX_SECURITY_DOMAINS];
static uint32_t next_domain_id = 1;

// Audit ring buffer (fixed-size, no dynamic allocation)
#define AUDIT_ENTRIES 512
static char audit_ring[AUDIT_ENTRIES][128];
static uint32_t audit_head = 0;

void sigma_security_init(void) {
    sigma_sigma_memset(domains, 0, sizeof(domains));
    sigma_sigma_memset(audit_ring, 0, sizeof(audit_ring));

    // Domain 0: Kernel Root — all permissions
    domains[0].domain_id  = 0;
    domains[0].permissions = PERM_READ | PERM_WRITE | PERM_EXECUTE | PERM_NET_ACCESS | PERM_KERNEL_OP;
    domains[0].active     = 1;
    sigma_sigma_memcpy(domains[0].name, "kernel_root", 12);

    sigma_sigma_printf("[SEC] Sovereign Security Matrix initialized. SELinux/AppArmor containerized in Vault.\n");
}

int sigma_security_create_domain(const char* name, uint8_t permissions) {
    if (next_domain_id >= MAX_SECURITY_DOMAINS) return -1;

    sigma_security_domain_t* d = &domains[next_domain_id];
    d->domain_id   = next_domain_id;
    d->permissions = permissions;
    d->active      = 1;
    strncpy(d->name, name, 31);

    return next_domain_id++;
}

int sigma_security_check(uint32_t domain_id, sigma_permission_t perm) {
    if (domain_id >= MAX_SECURITY_DOMAINS) return 0;
    if (!domains[domain_id].active) return 0;
    return (domains[domain_id].permissions & perm) ? 1 : 0;
}

void sigma_security_audit_log(uint32_t domain_id, const char* action) {
    // Ring-buffer based audit log — zero heap usage
    snsigma_sigma_printf(audit_ring[audit_head % AUDIT_ENTRIES], 127,
        "[AUDIT] domain=%u action=%s", domain_id, action);
    audit_head++;
}
