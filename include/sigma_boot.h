#pragma once
/*
 * SigmaOS: sigma_boot.h
 * Zero-dependency boot primitives used by SovereignBootEngine.
 */
#include "sigma_kernel_types.h"

#define SIGMA_BOOT_STAGE_INIT      0u
#define SIGMA_BOOT_STAGE_RECOVERY  1u
#define SIGMA_BOOT_STAGE_KERNEL    2u
#define SIGMA_BOOT_STAGE_USERLAND  3u

typedef sigma_u32 sigma_boot_stage_t;
