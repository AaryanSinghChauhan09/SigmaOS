/*
 * =========================================================================
 * Σ SIGMAOS: MODULAR SYSCALL DISPATCHER (C Implementation)
 * =========================================================================
 * Mission: High-performance, O(1) syscall dispatch table with runtime
 * registry and validation rules.
 * =========================================================================
 */

#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_log.h"
#include "../../include/libc/SovereignLibC.h"

#ifdef __cplusplus
extern "C" {
#endif

// Forward declarations for shard entry points
void util_echo(const char* t);
char kbd_read(void);
sigma_i32 net_socket(sigma_i32 d, sigma_i32 t, sigma_i32 p);
void pkg_install(const char* name);

// Modular Syscall Registry
#define MAX_SYSCALLS 256

typedef sigma_u64 (*syscall_handler_t)(sigma_u64, sigma_u64, sigma_u64);

typedef struct {
    syscall_handler_t handler;
    sigma_bool active;
    const char* name;
} syscall_entry_t;

static syscall_entry_t syscall_table[MAX_SYSCALLS];

// Default fallback handler
static sigma_u64 sys_ni_syscall(sigma_u64 a1, sigma_u64 a2, sigma_u64 a3) {
    (void)a1; (void)a2; (void)a3;
    sigma_log_info("[SYSCALL] ENOSYS: Syscall not implemented or disabled.");
    return (sigma_u64)-1;
}

// Handlers
static sigma_u64 sys_write(sigma_u64 a1, sigma_u64 a2, sigma_u64 a3) {
    (void)a2; (void)a3;
    util_echo((const char*)(sigma_usize)a1);
    return 0;
}

static sigma_u64 sys_read(sigma_u64 a1, sigma_u64 a2, sigma_u64 a3) {
    (void)a1; (void)a2; (void)a3;
    return (sigma_u64)kbd_read();
}

static sigma_u64 sys_socket(sigma_u64 a1, sigma_u64 a2, sigma_u64 a3) {
    return (sigma_u64)net_socket((sigma_i32)a1, (sigma_i32)a2, (sigma_i32)a3);
}

static sigma_u64 sys_pkg_install(sigma_u64 a1, sigma_u64 a2, sigma_u64 a3) {
    (void)a2; (void)a3;
    pkg_install((const char*)(sigma_usize)a1);
    return 0;
}

// Registry API
void syscall_register(sigma_u32 id, syscall_handler_t handler, const char* name) {
    if (id < MAX_SYSCALLS) {
        syscall_table[id].handler = handler;
        syscall_table[id].active = SIGMA_TRUE;
        syscall_table[id].name = name;
        sigma_log_info("[SYSCALL] Registered 0x%02X: %s", id, name);
    }
}

void syscall_init(void) {
    sigma_log_info("[SYSCALL] Initializing Modular Syscall Dispatcher...");
    
    // Initialize with fallback
    for (int i = 0; i < MAX_SYSCALLS; i++) {
        syscall_table[i].handler = sys_ni_syscall;
        syscall_table[i].active = SIGMA_FALSE;
        syscall_table[i].name = "sys_ni_syscall";
    }
    
    // Register known syscalls
    syscall_register(0x01, sys_write, "sys_write");
    syscall_register(0x02, sys_read, "sys_read");
    syscall_register(0x05, sys_socket, "sys_socket");
    syscall_register(0x06, sys_pkg_install, "sys_pkg_install");
}

sigma_u64 sigma_syscall(sigma_u64 id, sigma_u64 a1, sigma_u64 a2, sigma_u64 a3) {
    if (id < MAX_SYSCALLS && syscall_table[id].active) {
        // Tracing hook can be added here
        return syscall_table[id].handler(a1, a2, a3);
    } else {
        sigma_log_info("[SYSCALL] Unknown or inactive ID 0x%llX dispatched.", id);
        return (sigma_u64)-1;
    }
}

#ifdef __cplusplus
}
#endif