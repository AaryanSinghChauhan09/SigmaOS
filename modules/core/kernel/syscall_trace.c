#include "sigma_libc.h"

// ---------------------------------------------------------
// SigmaOS Modular Syscall Tracing (Phase 10)
// ---------------------------------------------------------

void syscall_trace_log(uint32_t syscall_id, uint64_t* args) {
    sigma_shard_init();
    // [PHASE 10] Trace kernel-native syscalls for observability.
}
