/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN GPU COMPUTE DRIVER (S-GPU)
 * =========================================================================
 * Mission: Silicon-native GPU command-queue dispatch and compute sharding.
 * Competitor parity: Linux DRM/KMS, macOS Metal, Windows DirectX 12.
 * ZERO-DEPENDENCY: Direct PCIe BAR MMIO access; no userspace driver stack.
 * =========================================================================
 */

#ifndef SIGMA_GPU_H
#define SIGMA_GPU_H

#include "core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- GPU Vendor IDs (PCI) --- */
#define SIGMA_GPU_VENDOR_NVIDIA  0x10DEu
#define SIGMA_GPU_VENDOR_AMD     0x1002u
#define SIGMA_GPU_VENDOR_INTEL   0x8086u
#define SIGMA_GPU_VENDOR_VIRTUAL 0x1AF4u  /* VirtIO GPU              */

/* --- GPU Pipeline Modes --- */
#define SIGMA_GPU_MODE_RENDER    0x00u  /* 3D rasterisation         */
#define SIGMA_GPU_MODE_COMPUTE   0x01u  /* GPGPU / AI compute       */
#define SIGMA_GPU_MODE_DISPLAY   0x02u  /* Scanout / display engine */
#define SIGMA_GPU_MODE_ENCODE    0x03u  /* Hardware video encode    */

#define SIGMA_GPU_CMD_QUEUE_DEPTH 256u
#define SIGMA_GPU_NAME_LEN        48u

typedef struct {
    sigma_u32  cmd_type;      /* Opcode for this command buffer    */
    sigma_addr_t data_addr;   /* Physical address of data          */
    sigma_u32  data_size;
    sigma_u32  fence_id;      /* Synchronisation fence             */
} sigma_gpu_cmd_t;

typedef struct {
    sigma_u16  vendor_id;
    sigma_u16  device_id;
    sigma_u32  vram_mb;
    sigma_u32  compute_units;
    char       name[SIGMA_GPU_NAME_LEN];
    sigma_addr_t mmio_base;   /* PCI BAR0 base                     */
    sigma_u32  mode;          /* Current pipeline mode             */
} sigma_gpu_info_t;

typedef struct {
    sigma_gpu_info_t info;
    sigma_u64 cmds_submitted;
    sigma_u64 cmds_completed;
    sigma_u32 fence_counter;
    sigma_u32 initialized;
} sigma_gpu_state_t;

/* --- GPU Primitives --- */
void      gpu_init(sigma_addr_t mmio_base, sigma_u16 vendor_id, sigma_u16 device_id);
void      gpu_set_mode(sigma_u32 mode);
sigma_u32 gpu_submit_cmd(const sigma_gpu_cmd_t* cmd);
void      gpu_wait_fence(sigma_u32 fence_id);
const sigma_gpu_state_t* gpu_get_state(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_GPU_H */
