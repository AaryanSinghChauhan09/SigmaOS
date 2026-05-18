/*
 * =========================================================================
 * SigmaOS: SovereignVulkanLayer (sovereign_vulkan.h)
 * =========================================================================
 * Direct shader routing to GPU without Vulkan SDK or wrapper libs.
 * SPIR-V words are streamed to a memory-mapped command queue.
 * =========================================================================
 */
#ifndef SOVEREIGN_VULKAN_H
#define SOVEREIGN_VULKAN_H

#include "sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Result codes */
typedef enum {
    SVK_OK            = 0,
    SVK_ERR_NOT_READY = 1,
    SVK_ERR_TIMEOUT   = 2,
    SVK_ERR_INVALID   = 3
} svk_result_t;

/* Shader stage flags */
typedef enum {
    SVK_STAGE_VERTEX   = 0x01u,
    SVK_STAGE_FRAGMENT = 0x02u,
    SVK_STAGE_COMPUTE  = 0x04u
} svk_stage_t;

/* Shader handle */
typedef sigma_u32 svk_shader_t;
#define SVK_SHADER_INVALID 0xFFFFFFFFu

/*
 * svk_init — reset GPU command queue, enable clocks.
 */
void svk_init(void);

/*
 * svk_submit_shader — stream a pre-compiled SPIR-V blob to the GPU.
 * @spirv : pointer to SPIR-V words (must be 4-byte aligned)
 * @words : number of 32-bit words in the blob
 * @stage : shader pipeline stage
 * @out   : receives opaque shader handle on success
 * Returns SVK_OK on success.
 */
svk_result_t svk_submit_shader(const sigma_u32* spirv,
                                sigma_u32        words,
                                svk_stage_t      stage,
                                svk_shader_t*    out);

/*
 * svk_dispatch — trigger compute dispatch on the GPU.
 * @x, @y, @z : workgroup counts
 */
svk_result_t svk_dispatch(sigma_u32 x, sigma_u32 y, sigma_u32 z);

/*
 * svk_draw — trigger a draw call with @vertex_count vertices.
 */
svk_result_t svk_draw(sigma_u32 vertex_count);

/*
 * svk_wait_idle — spin until GPU reports IDLE.
 * Returns SVK_ERR_TIMEOUT after ~1M iterations.
 */
svk_result_t svk_wait_idle(void);

#ifdef __cplusplus
}
#endif

#endif /* SOVEREIGN_VULKAN_H */
