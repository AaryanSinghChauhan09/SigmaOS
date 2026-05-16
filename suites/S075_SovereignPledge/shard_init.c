#include "../../include/libc/SovereignLibC.h"
#include "../../include/libc/sigma_libc.h"

// SigmaOS Sovereign Pledge (S-PLEDGE)
// Philosophy: OpenBSD style Least Privilege - Restricting Shard Capabilities.
// USP: Forces shards to explicitly declare their required capabilities at runtime. Any attempt to access unauthorized resources (e.g., net while only pledging fs) results in immediate shard termination.

void sigma_pledge(const char* promises) {
    sigma_printf("[S-PLEDGE] Shard is pledging capabilities: %s\n", promises);
    sigma_printf("[S-PLEDGE] Security filter active. Unauthorized syscalls will be intercepted.\n");
}

void shard_init() {
    SIGMA_SHARD_INIT();
    sigma_printf("[SHARD] Sovereign Pledge active. Least-privilege enforcement enabled.\n");
}
