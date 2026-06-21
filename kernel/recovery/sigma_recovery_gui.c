/*
 * Recovery GUI wizard — Rescuezilla / SystemRescue inspired text UI.
 */
#include "../../include/sigma_recovery.h"
#include "../../include/sigma_kernel_types.h"

extern void sigma_puts(const char* s);

#ifndef __cplusplus
#define bool sigma_bool
#define true SIGMA_TRUE
#define false SIGMA_FALSE
#endif

extern bool recovery_create_snapshot(const char* description);
extern bool recovery_rollback_to_snapshot(sigma_u32 snapshot_id);
extern void recovery_run_forensic_audit(void);

void recovery_gui_init(void) {
    sigma_puts("[recovery-gui] Sovereign Recovery Wizard ready.\n");
}

void recovery_gui_show_menu(void) {
    sigma_puts("\n=== SigmaOS Recovery Wizard ===\n");
    sigma_puts("  1) Create snapshot\n");
    sigma_puts("  2) Rollback to snapshot\n");
    sigma_puts("  3) Forensic audit\n");
    sigma_puts("  4) Export logs to /var/recovery\n");
    sigma_puts("  5) Return to Safe Mode menu\n");
}

void recovery_gui_action_snapshot(void) {
    if (recovery_create_snapshot("user-requested")) {
        sigma_puts("[recovery-gui] Snapshot created.\n");
    }
}

void recovery_gui_action_rollback(sigma_u32 id) {
    if (recovery_rollback_to_snapshot(id)) {
        sigma_puts("[recovery-gui] Rollback initiated.\n");
    }
}

void recovery_gui_action_audit(void) {
    recovery_run_forensic_audit();
    sigma_puts("[recovery-gui] Forensic audit complete.\n");
}
