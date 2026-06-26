/*
 * =============================================================================
 * Σ SIGMAOS: SOVEREIGN INIT SYSTEM (sigma-init v1.0)
 * =============================================================================
 * Mission: Service manager with dependency resolution, boot stage orchestration,
 *          and automatic restart with exponential backoff.
 * Absorbs: systemd service management, OpenRC simplicity, runit supervision.
 * Standard: C11/C++17 — Zero external dependencies.
 * =============================================================================
 */

#ifndef SIGMA_INIT_SYSTEM_H
#define SIGMA_INIT_SYSTEM_H

#include "../sigma_kernel_types.h"

#define INIT_MAX_SERVICES     64
#define INIT_MAX_DEPS          8
#define INIT_SERVICE_NAME_LEN 48

typedef enum {
    BOOT_STAGE_FIRMWARE  = 0,
    BOOT_STAGE_KERNEL    = 1,
    BOOT_STAGE_DRIVERS   = 2,
    BOOT_STAGE_SERVICES  = 3,
    BOOT_STAGE_USERLAND  = 4,
    BOOT_STAGE_COMPLETE  = 5
} sigma_boot_stage_init_t;

typedef enum {
    SERVICE_STOPPED    = 0,
    SERVICE_STARTING   = 1,
    SERVICE_RUNNING    = 2,
    SERVICE_FAILED     = 3,
    SERVICE_RESTARTING = 4
} sigma_service_state_t;

typedef enum {
    RESTART_NEVER      = 0,
    RESTART_ON_FAILURE = 1,
    RESTART_ALWAYS     = 2
} sigma_restart_policy_t;

typedef struct {
    sigma_u32              id;
    char                   name[INIT_SERVICE_NAME_LEN];
    sigma_service_state_t  state;
    sigma_restart_policy_t restart_policy;
    sigma_u32              pid;               /* PID of the service process */
    sigma_u32              restart_count;
    sigma_u64              start_time_tsc;
    sigma_u64              uptime_us;
    sigma_boot_stage_init_t boot_stage;       /* stage at which this service starts */
    sigma_u32              dep_count;
    sigma_u32              deps[INIT_MAX_DEPS]; /* service IDs this depends on */
    sigma_bool             critical;          /* if true, boot halts on failure */
} sigma_service_t;

#ifdef __cplusplus
extern "C" {
#endif

void       init_system_init(void);
void       init_boot(void);
sigma_u32  service_register(const char* name, sigma_boot_stage_init_t stage,
                            sigma_restart_policy_t policy, sigma_bool critical);
int        service_add_dependency(sigma_u32 service_id, sigma_u32 dep_id);
int        service_start(sigma_u32 service_id);
int        service_stop(sigma_u32 service_id);
sigma_service_state_t service_status(sigma_u32 service_id);
void       init_print_boot_log(void);
void       init_print_service_tree(void);
sigma_u32  init_get_service_count(void);
sigma_u64  init_get_boot_time_us(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_INIT_SYSTEM_H */
