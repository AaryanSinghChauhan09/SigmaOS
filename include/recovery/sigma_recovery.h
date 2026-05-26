/*
 * =============================================================================
 * Σ SIGMAOS: SOVEREIGN RECOVERY SUITE
 * =============================================================================
 * Mission: Built-in system snapshotting, rollback, and cryptographic forensic 
 *          auditing to rival Rescuezilla and CAINE.
 * Standard: C11/C++17 — Zero external dependencies.
 * =============================================================================
 */

#ifndef SIGMA_RECOVERY_H
#define SIGMA_RECOVERY_H

#include "../sigma_kernel_types.h"

#define REC_MAX_SNAPSHOTS 32
#define REC_DESC_LEN      128

typedef struct {
    sigma_u32  id;
    char       description[REC_DESC_LEN];
    sigma_u64  timestamp_tsc;
    sigma_u64  zfs_transaction_group;
    sigma_bool is_bootable;
} sigma_snapshot_t;

#ifdef __cplusplus
extern "C" {
#endif

void      recovery_init(void);
sigma_u32 recovery_create_snapshot(const char* desc);
int       recovery_rollback(sigma_u32 snapshot_id);
void      recovery_list_snapshots(void);

/* Forensic Mode */
int       recovery_enter_forensic_mode(void);
int       recovery_generate_filesystem_hash(const char* mount_point, char* out_hash_hex);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_RECOVERY_H */
