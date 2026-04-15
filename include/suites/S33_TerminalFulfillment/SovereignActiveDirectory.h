/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN ACTIVE DIRECTORY (v1.0 — C11)
 * =========================================================================
 * Absorbed USPs from: Windows Server Active Directory / LDAP / Kerberos
 *
 * Features implemented:
 *   ✓ Lightweight Directory Access Protocol (LDAP) emulation
 *   ✓ Domain Controller (DC) tracking and joining
 *   ✓ Group Policy Objects (GPO) propagation
 *   ✓ Kerberos Key Distribution Center (KDC) ticket granting
 * =========================================================================
 */

#ifndef SOVEREIGN_AD_H
#define SOVEREIGN_AD_H

#include "suites/S01_Genesis/shards/sigma_types.h"

#define SIGMA_AD_DOMAIN_MAX 128

typedef struct {
    char username[64];
    char display_name[128];
    char groups[8][64];
} SigmaADUser_t;

sigma_err_t sigma_ad_promote_to_dc(const char *domain_name);
sigma_err_t sigma_ad_join_domain(const char *domain_name, const char *dc_ip, const char *admin_user, const char *admin_pass);
sigma_err_t sigma_ad_ldap_query(const char *query, SigmaADUser_t *out_user);
sigma_err_t sigma_ad_kdc_request_ticket(const char *spn, char *ticket_out);
sigma_err_t sigma_ad_apply_gpo(const char *policy_file);

void SovereignActiveDirectory_Init(void);

#endif /* SOVEREIGN_AD_H */
