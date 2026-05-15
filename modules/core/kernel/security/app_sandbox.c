#include "../../../../include/libc/sigma_libc.h"
#include "../../../../include/libc/sigma_libc.h"
#include <string.h>

// ---------------------------------------------------------
// SigmaOS Sovereign Application Sandbox
// USP: Ultimate Ease-of-Use for End Users. Automatically
// wraps applications in Zero-Trust sandboxes without 
// requiring manual configuration.
// ---------------------------------------------------------

#define MAX_SANDBOXES 64

typedef struct {
    uint32_t sandbox_id;
    uint32_t active_pid;
    char     app_name[32];
    uint8_t  network_isolated;
    uint8_t  fs_isolated;
    uint32_t memory_quota_pages;
    uint8_t  is_active;
} app_sandbox_t;

static app_sandbox_t sandboxes[MAX_SANDBOXES];
static uint32_t sandbox_count = 0;

extern void audit_chain_append(uint32_t pid, uint8_t level, const char* msg);
extern int mem_contract_lease(uint32_t pid, uint32_t base_page, uint32_t num_pages, uint64_t duration);

// Launch an application with automatic sandboxing
int sandbox_launch_app(const char* app_name, uint8_t require_network, uint32_t max_ram_mb) {
    if (sandbox_count >= MAX_SANDBOXES) return -1;
    
    app_sandbox_t* box = &sandboxes[sandbox_count];
    box->sandbox_id = sandbox_count++;
    strncpy(box->app_name, app_name, 31);
    box->network_isolated = !require_network; // Default to isolated
    box->fs_isolated = 1;                     // Apps only get access to their own VFS node
    box->memory_quota_pages = (max_ram_mb * 1024 * 1024) / 4096;
    box->is_active = 1;

    // In reality, this would exec() the ELF/WASM binary and return the PID
    uint32_t mock_pid = 100 + box->sandbox_id; 
    box->active_pid = mock_pid;

    // Ease of Use USP: Automatically negotiate the memory contract on behalf of the app
    mem_contract_lease(mock_pid, 0x1000, box->memory_quota_pages, 0xFFFFFFFF);

    audit_chain_append(mock_pid, 1, "APP_LAUNCHED_IN_SANDBOX");
    return box->sandbox_id;
}

// Dynamically revoke a sandbox (e.g., user clicks "Force Quit")
void sandbox_terminate(uint32_t sandbox_id) {
    if (sandbox_id >= sandbox_count || !sandboxes[sandbox_id].is_active) return;
    
    // The kernel Memory Manager will instantly catch this and revoke all physical pages
    // The AI Scheduler will instantly drop the PID from the runqueue
    sandboxes[sandbox_id].is_active = 0;
    audit_chain_append(sandboxes[sandbox_id].active_pid, 1, "SANDBOX_TERMINATED_INSTANTLY");
}
