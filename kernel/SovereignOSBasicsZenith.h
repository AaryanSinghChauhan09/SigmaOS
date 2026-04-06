/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN OS BASICS (v20.0 - ZENITH ARCHITECTURE)
 * =========================================================================
 * Purpose: Fundamental Kernel-level definitions and process models.
 * Standard: Pure C11 (ISO/IEC 9899:2011).
 * =========================================================================
 */

#ifndef SOVEREIGN_OS_BASICS_ZENITH_H
#define SOVEREIGN_OS_BASICS_ZENITH_H

#include "../SigmaC11.h"

/* --- Process Control Block (PCB) --- */
typedef struct sovereign_pcb_t {
    sigma_obj_header_t hdr;
    sigma_u64 pid;
    sigma_u64 cr3;
    sigma_u64 rsp;
    sigma_u32 state; // 0: READY, 1: RUNNING, 2: BLOCKED
} sovereign_pcb_t;

/* --- Process Manager Interface --- */
typedef struct sovereign_process_manager_t {
    sigma_obj_header_t hdr;
    sovereign_pcb_t process_table[1024];
    sigma_u32 active_count;
} sovereign_process_manager_t;

/* --- Core Kernel Functions --- */
sigma_status sovereign_pm_spawn(sovereign_process_manager_t* pm, const char* image);
void         sovereign_pm_kill(sovereign_process_manager_t* pm);
void         sovereign_pm_shard_resources(sovereign_process_manager_t* pm);
void         sovereign_pm_isolate_vfs(sovereign_process_manager_t* pm, const char* namespace_root);
void         sovereign_pm_audit(sovereign_process_manager_t* pm);

/* --- Module Initializers --- */
#ifdef __cplusplus
extern "C" {
#endif
void sigma_aether_absorption_init(void);
void sigma_convergence_init(void);
void sigma_automation_shard_init(void);
void sigma_build_system_init(void);
void sigma_automation_shard_init(void);
void sigma_boot_wizard_init(void);
void sigma_boot_master_init(void);
void sigma_container_runtime_init(void);
void sigma_container_forge_init(void);
void sigma_core_utils_init(void);
void sigma_encyclopedia_init(void);
void sigma_diagnostics_init(void);
void sigma_distro_forge_init(void);
void sigma_research_matrix_init(void);
void sigma_orchestrator_init(void);
void sigma_datascience_init(void);
void sigma_data_preprocess_init(void);
void sigma_forensics_init(void);
void sigma_voice_shard_init(void);
void sigma_personalizer_init(void);
void SovereignResilience_Init(void);
void SovereignResilience_Check(void);
#ifdef __cplusplus
}
#endif

#endif /* SOVEREIGN_OS_BASICS_ZENITH_H */
