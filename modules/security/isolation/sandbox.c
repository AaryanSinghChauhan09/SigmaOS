#include <stdint.h>
#include <stddef.h>

// ---------------------------------------------------------
// SigmaOS Process Sandboxing / Isolation Prototype
// ---------------------------------------------------------

// Sandbox Policy Flags
#define SANDBOX_NO_NETWORK   0x01
#define SANDBOX_NO_DISK      0x02
#define SANDBOX_NO_IPC       0x04
#define SANDBOX_READ_ONLY_FS 0x08
#define SANDBOX_MAX_RESTRICT 0x0F  // All restrictions

typedef struct {
    uint32_t pid;
    uint32_t policy;             // Bitmask of restrictions
    uint32_t cap_table_offset;   // Index into the global capability pool for this sandbox
    uint32_t max_memory_pages;   // Hard cap on physical pages
    uint32_t current_pages;
} sandbox_t;

#define MAX_SANDBOXES 64
static sandbox_t sandboxes[MAX_SANDBOXES];
static uint32_t sandbox_count = 0;

// Create a sandbox for a process
int sandbox_create(uint32_t pid, uint32_t policy, uint32_t max_pages) {
    if (sandbox_count >= MAX_SANDBOXES) return -1;
    sandbox_t* sb = &sandboxes[sandbox_count++];
    sb->pid = pid;
    sb->policy = policy;
    sb->max_memory_pages = max_pages;
    sb->current_pages = 0;
    return sandbox_count - 1;
}

// Enforce a policy check before a syscall is executed
int sandbox_check_syscall(uint32_t pid, uint32_t syscall_id) {
    for (int i = 0; i < sandbox_count; i++) {
        if (sandboxes[i].pid != pid) continue;
        // Block network syscalls if SANDBOX_NO_NETWORK is set
        if ((sandboxes[i].policy & SANDBOX_NO_NETWORK) && (syscall_id >= 100 && syscall_id <= 120)) {
            return 0; // Blocked
        }
        // Block disk syscalls
        if ((sandboxes[i].policy & SANDBOX_NO_DISK) && (syscall_id >= 50 && syscall_id <= 80)) {
            return 0; // Blocked
        }
    }
    return 1; // Allowed
}

// Check if a page allocation is within sandbox limits
int sandbox_check_memory(uint32_t pid) {
    for (int i = 0; i < sandbox_count; i++) {
        if (sandboxes[i].pid == pid) {
            if (sandboxes[i].current_pages >= sandboxes[i].max_memory_pages) {
                return 0; // Memory limit exceeded → deny
            }
            sandboxes[i].current_pages++;
            return 1;
        }
    }
    return 1; // No sandbox → allow
}

// Destroy sandbox (called when process exits)
void sandbox_destroy(uint32_t pid) {
    for (int i = 0; i < sandbox_count; i++) {
        if (sandboxes[i].pid == pid) {
            sandboxes[i].pid = 0;
            sandboxes[i].policy = 0;
        }
    }
}
