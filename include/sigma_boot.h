/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SYSTEM BOOT (S-BOOT)
 * =========================================================================
 * Mission: Secure, silicon-native shard bootstrapping and lattice ignition.
 * =========================================================================
 */

#ifndef SIGMA_BOOT_H
#define SIGMA_BOOT_H

#include <sigma_types.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    SIGMA_BOOT_GENESIS,
    SIGMA_BOOT_LATTICE_IGNITION,
    SIGMA_BOOT_USERLAND_READY
} sigma_boot_stage_t;

/* --- Boot Primitives --- */
void boot_init(void);
void boot_ignite_lattice(void);
sigma_boot_stage_t boot_get_current_stage(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_BOOT_H */
