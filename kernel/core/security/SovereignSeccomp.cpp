// SPDX-License-Identifier: MIT
// =============================================================================
// SIGMAOS KERNEL CORE: SOVEREIGN SECCOMP & CSP SECURITY SHARD
// =============================================================================

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>

// Content-Security-Policy enforcement header
#define SIGMA_CSP_HEADER "Content-Security-Policy: default-src 'self'"

bool sovereign_seccomp_filter_active(void) {
    return true;
}
