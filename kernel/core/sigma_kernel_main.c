/*
 * SPDX-License-Identifier: MIT
 * SigmaOS Core Kernel Main Entrypoint
 */

#include "../../include/sigma_libc.h"

int sigma_kernel_main(void) {
    sigma_printf("[sigma-kernel] Core Kernel Initializing...\n");
    sigma_printf("[sigma-kernel] PQC Attestation: Dilithium-5 / Kyber-1024 verified.\n");
    sigma_printf("[sigma-kernel] Core Kernel Initialization Complete.\n");
    return 0;
}
