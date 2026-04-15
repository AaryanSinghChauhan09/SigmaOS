/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN SIMULATION (Suite S24)
 * =========================================================================
 * Shard: Sovereign Simulation Engine
 * Parity: User-Mode Linux (UML), Windows Subsystem for Linux (WSL1), gVisor
 * Design: Secure kernel-in-kernel execution with state-machine modeling.
 * =========================================================================
 */

#ifndef SOVEREIGN_SIMULATION_H
#define SOVEREIGN_SIMULATION_H

#include "../../../include/SovereignCommon.h"

typedef struct {
    sigma_u64 pc;
    sigma_u64 sp;
    sigma_u64 regs[16];
    char      status[32];
} sim_context_t;

/* Public API */
void        sigma_sim_init(void);

/* Simulation Control */
sigma_u32   sigma_sim_create(void* entry_point);
sigma_err_t sigma_sim_step(sigma_u32 sim_id);
sigma_err_t sigma_sim_snapshot(sigma_u32 sim_id, void* buffer, sigma_sz_t size);

/* Telemetry */
void        sigma_sim_stats(void);

#endif /* SOVEREIGN_SIMULATION_H */
