// =============================================================================
// SigmaOS — S04_HAL — S04_01_GpuDriverStack.c
// Industrial-Grade Unified GPU Orchestration Shard
// =============================================================================

#include "suites/S01_Genesis/shards/sigma_types.h"


typedef struct {
    uint32_t device_id;
    uint8_t  vram_gb;
    bool     supports_ray_tracing;
    uintptr_t command_buffer_base;
} GpuNode;

// ── Public API ────────────────────────────────────────────────────────────────

// Register a hardware GPU (Intel/Nvidia/AMD/NPU) with the Sovereign Stack
void gpustack_register(GpuNode* gpu);

// Submit a CommandList to the local graphics capability (S04 hook)
void gpustack_dispatch(uint32_t gpu_id, void* cmd_list);

// Sync VRAM state across Hive peers (MeshDisplay hook)
void gpustack_sync_mesh(uint32_t gpu_id);



