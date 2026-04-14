// =============================================================================
// SigmaOS — S04_HAL — SovereignGraphicsBridge.c
// Unified High-Performance Graphics Abstraction Pipeline
// =============================================================================
// Competitor USPs Absorbed:
//   • Apple Metal      — Low-overhead, high-efficiency GPU access
//   • Vulkan (Khronos) — Universal, explicit GPU state control
//   • Windows DirectX12— DirectStorage integration and rich feature set
// Architecture:
//   • SigmaDraw: A unified, zero-dependency C11 Graphics DSL
//   • Shaders: Hot-swappable SPIR-V based kernel-side shaders
//   • Pipeline: Automatic frame-graph optimization before VRAM transfer
// =============================================================================

#include <sigma_types.h>


#define MAX_COMMANDS_PER_BLOCK 4096

typedef enum {
    GFX_CMD_FILL_RECT    = 0,
    GFX_CMD_DRAW_MESH    = 1,
    GFX_CMD_APPLY_SHADER = 2,
    GFX_CMD_BLIT         = 3
} GraphicsCommandType;

// ── Graphics Frame Command ────────────────────────────────────────────────────
typedef struct {
    GraphicsCommandType type;
    uint32_t            target_layer;
    uint8_t             payload[256];
} GraphicsCommand;

// ── Public API ────────────────────────────────────────────────────────────────

// Initialise the Graphics Bridge with GPU-Exclusive access (ASIO style)
void gfx_bridge_init(void);

// Submit a block of commands for hardware execution (Direct VRAM path)
void gfx_bridge_submit_block(GraphicsCommand* commands, uint32_t count);

// Load a SPIR-V or SigmaShader into GPU VRAM
uint32_t gfx_bridge_load_shader(const char* shader_path);

// Set real-time light sources for S02 Holographic Engine (visionOS parity)
void gfx_bridge_set_lights(float x, float y, float z, uint32_t color);

// Poll for GPU vBlank to ensure tear-free composition
void gfx_bridge_wait_sync(void);

// Benchmark current GPU draw-call throughput (Perf parity)
uint64_t gfx_bridge_get_throughput(void);


