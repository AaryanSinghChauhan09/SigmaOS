/* Σ SIGMAOS: SOVEREIGN GPU SHARD HEADER */
#ifndef SOVEREIGN_GPU_SHARD_H
#define SOVEREIGN_GPU_SHARD_H
#include "sigma_types.h"

typedef enum { GPU_CMD_TRANSFER, GPU_CMD_DRAW_TRI, GPU_CMD_COMPUTE, GPU_CMD_PRESENT } SigmaGPUCmd_t;

sigma_err_t sigma_gpu_submit_stream (const char* client, SigmaGPUCmd_t type, sigma_u32 count);
sigma_u32   sigma_gpu_alloc_vram    (sigma_u32 size_mb);
void        SovereignGPUShard_Init   (void);
void        SovereignGPU_Audit       (void);

#endif
