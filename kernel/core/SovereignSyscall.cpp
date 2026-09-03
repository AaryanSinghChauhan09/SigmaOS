// SPDX-License-Identifier: MIT
// =============================================================================
// SIGMAOS KERNEL CORE: SOVEREIGN SYSCALL INTERFACE
// =============================================================================
// Hardened syscall dispatcher with out-of-bounds syscall ID traps and strict
// user-space pointer/argument validation routines.
// =============================================================================

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>

#define SYSCALL_ID_MAX     256u

// Canonical x86-64 user-space upper boundary (canonical bit 47 must be 0).
// Value: 0x0000_7FFF_FFFF_FFFF  (2^47 - 1)
// Fixed: was 0x00007FFFFFFFFFFF000ULL which added an erroneous trailing zero,
// shifting the limit by 3 bits and allowing ~8 bytes of kernel mapping to slip
// through the boundary check.
#define USER_SPACE_MAX_ADDR  0x00007FFFFFFFFFFFULL

// Error codes returned by the dispatcher
#define ESYSCALL_OK           0
#define ESYSCALL_INVALID_ID  (-1)
#define ESYSCALL_BAD_PTR     (-14)   // -EFAULT
#define ESYSCALL_UNIMPLEMENTED (-38) // -ENOSYS

typedef int64_t (*SyscallHandler)(
    uint64_t a1, uint64_t a2, uint64_t a3,
    uint64_t a4, uint64_t a5, uint64_t a6);

// ── Forward declarations for stub handlers ───────────────────────────────────
static int64_t sys_unimplemented(uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t);
static int64_t sys_read (uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t);
static int64_t sys_write(uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t);
static int64_t sys_open (uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t);
static int64_t sys_close(uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t);
static int64_t sys_exit (uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t);

// ── Dispatch table ────────────────────────────────────────────────────────────
// Indexed by Linux x86-64 syscall number.  Unimplemented entries return ENOSYS.
static SyscallHandler sigma_syscall_table[SYSCALL_ID_MAX];

// Populate table at runtime (called from kernel init).
void sigma_syscall_table_init(void) {
    // Initialise all entries to the "unimplemented" stub first.
    for (unsigned i = 0; i < SYSCALL_ID_MAX; ++i) {
        sigma_syscall_table[i] = sys_unimplemented;
    }
    // Wire in the implemented handlers.
    sigma_syscall_table[0]  = sys_read;
    sigma_syscall_table[1]  = sys_write;
    sigma_syscall_table[2]  = sys_open;
    sigma_syscall_table[3]  = sys_close;
    sigma_syscall_table[60] = sys_exit;
}

// ── Argument validation ───────────────────────────────────────────────────────

/**
 * @brief Validate a user-space pointer and length pair.
 *
 * Returns true only when the entire range [arg_ptr, arg_ptr+len) lies within
 * canonical user-space (below USER_SPACE_MAX_ADDR) with no arithmetic overflow.
 *
 * @param arg_ptr   User-supplied pointer value (not dereferenced here).
 * @param len       Number of bytes the pointer must cover.
 * @param is_write  Reserved for future page-permission checks (unused for now).
 */
bool arg_validate(uint64_t arg_ptr, size_t len, bool is_write) {
    (void)is_write; // Reserved for future TLB/permission check

    if (len == 0) {
        return true;
    }

    if (arg_ptr == 0) {
        return false; // NULL pointer
    }

    // Overflow check: arg_ptr + len must not wrap around.
    if (arg_ptr + (uint64_t)len < arg_ptr) {
        return false;
    }

    // Both the start and the end of the range must be in user-space.
    if (arg_ptr > USER_SPACE_MAX_ADDR ||
        (arg_ptr + (uint64_t)len) > USER_SPACE_MAX_ADDR) {
        return false;
    }

    return true;
}

// ── Main dispatcher ───────────────────────────────────────────────────────────

/**
 * @brief Sovereign syscall dispatcher.
 *
 * Validates the syscall ID, then forwards to the registered handler.
 * Returns negative errno on error.
 */
int64_t dispatch_sovereign_syscall(
    uint64_t syscall_id,
    uint64_t a1, uint64_t a2, uint64_t a3,
    uint64_t a4, uint64_t a5, uint64_t a6)
{
    if (syscall_id >= SYSCALL_ID_MAX) {
        return ESYSCALL_INVALID_ID;
    }

    SyscallHandler handler = sigma_syscall_table[syscall_id];
    if (handler == NULL) {
        return ESYSCALL_UNIMPLEMENTED;
    }

    return handler(a1, a2, a3, a4, a5, a6);
}

// ── Stub implementations ─────────────────────────────────────────────────────

static int64_t sys_unimplemented(
    uint64_t a1, uint64_t a2, uint64_t a3,
    uint64_t a4, uint64_t a5, uint64_t a6)
{
    (void)a1; (void)a2; (void)a3; (void)a4; (void)a5; (void)a6;
    return ESYSCALL_UNIMPLEMENTED;
}

static int64_t sys_read(
    uint64_t fd, uint64_t buf_ptr, uint64_t count,
    uint64_t a4, uint64_t a5, uint64_t a6)
{
    (void)a4; (void)a5; (void)a6;
    if (!arg_validate(buf_ptr, (size_t)count, /*is_write=*/true)) {
        return ESYSCALL_BAD_PTR;
    }
    (void)fd;
    // TODO: route through VFS layer
    return (int64_t)count;
}

static int64_t sys_write(
    uint64_t fd, uint64_t buf_ptr, uint64_t count,
    uint64_t a4, uint64_t a5, uint64_t a6)
{
    (void)a4; (void)a5; (void)a6;
    if (!arg_validate(buf_ptr, (size_t)count, /*is_write=*/false)) {
        return ESYSCALL_BAD_PTR;
    }
    (void)fd;
    // TODO: route through VFS layer
    return (int64_t)count;
}

static int64_t sys_open(
    uint64_t path_ptr, uint64_t flags, uint64_t mode,
    uint64_t a4, uint64_t a5, uint64_t a6)
{
    (void)a4; (void)a5; (void)a6;
    if (!arg_validate(path_ptr, 1, /*is_write=*/false)) {
        return ESYSCALL_BAD_PTR;
    }
    (void)flags; (void)mode;
    // TODO: route through VFS layer; return first available fd > 2
    return 3;
}

static int64_t sys_close(
    uint64_t fd,
    uint64_t a2, uint64_t a3, uint64_t a4, uint64_t a5, uint64_t a6)
{
    (void)a2; (void)a3; (void)a4; (void)a5; (void)a6;
    if ((int64_t)fd < 0) {
        return ESYSCALL_BAD_PTR;
    }
    // TODO: route through VFS/fd-table
    return ESYSCALL_OK;
}

static int64_t sys_exit(
    uint64_t exit_code,
    uint64_t a2, uint64_t a3, uint64_t a4, uint64_t a5, uint64_t a6)
{
    (void)a2; (void)a3; (void)a4; (void)a5; (void)a6;
    // TODO: signal process manager and scheduler
    return (int64_t)exit_code;
}
