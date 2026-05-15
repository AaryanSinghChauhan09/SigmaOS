#ifndef SIGMA_BOOT_H
#define SIGMA_BOOT_H
#include "../include/core/sigma_types.h"
#ifdef __cplusplus
extern "C" {
#endif
typedef enum { SIGMA_BOOT_STAGE_INIT, SIGMA_BOOT_STAGE_KERNEL, SIGMA_BOOT_STAGE_USERLAND, SIGMA_BOOT_STAGE_RECOVERY } sigma_boot_stage_t;
#define SIGMA_BOOT_GENESIS 0
#define SIGMA_BOOT_LATTICE_IGNITION 1
#define SIGMA_BOOT_USERLAND_READY 2
#ifdef __cplusplus
}
#endif
#endif
