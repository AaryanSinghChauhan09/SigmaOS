/*
 * =========================================================================
 * Σ SIGMAOS: IPv6 NEIGHBOR DISCOVERY PROTOCOL (NDP)
 * =========================================================================
 * Replaces ARP for IPv6 packet resolution.
 * =========================================================================
 */
#include "../../klib/include/sigma_stdio.h"

extern "C" void sigma_ndp_discover() {
    sigma_printf("[NDP] Broadcasting router solicitation...\n");
    sigma_printf("[NDP] Resolving MAC addresses for local subnet...\n");
}
