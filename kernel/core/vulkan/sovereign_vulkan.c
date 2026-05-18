/*
 * =========================================================================
 * SigmaOS: SovereignVulkanLayer Implementation (sovereign_vulkan.c)
 * =========================================================================
 * Direct SPIR-V streaming to GPU via MMIO command queue.
 * No Vulkan SDK, no wrapper libs — silicon-direct.
 * =========================================================================
 */
#include "sovereign_vulkan.h"
#include "../hal/hal.h"
#include "sigma_log.h"

/* ── GPU MMIO map (adjust base for real hardware) ───────────────────── */
#define GPU_BASE           ((sigma_paddr_t)0xFEE00000u)
#define GPU_REG_CTRL       (GPU_BASE + 0x000u)  /* control register    */
#define GPU_REG_STATUS     (GPU_BASE + 0x004u)  /* status register     */
#define GPU_REG_STAGE      (GPU_BASE + 0x008u)  /* shader stage select */
#define GPU_REG_WORD_COUNT (GPU_BASE + 0x00Cu)  /* SPIR-V word count   */
#define GPU_CMD_QUEUE      (GPU_BASE + 0x100u)  /* command FIFO base   */
#define GPU_REG_DISPATCH_X (GPU_BASE + 0x200u)
#define GPU_REG_DISPATCH_Y (GPU_BASE + 0x204u)
#define GPU_REG_DISPATCH_Z (GPU_BASE + 0x208u)
#define GPU_REG_DRAW_CNT   (GPU_BASE + 0x300u)

/* Control bits */
#define GPU_CTRL_RESET      0x01u
#define GPU_CTRL_CLK_EN     0x02u
#define GPU_CTRL_SHADER_LD  0x04u
#define GPU_CTRL_DISPATCH   0x08u
#define GPU_CTRL_DRAW       0x10u

/* Status bits */
#define GPU_STATUS_IDLE     0x01u
#define GPU_STATUS_BUSY     0x02u
#define GPU_STATUS_ERROR    0x80u

/* Next shader handle counter */
static sigma_u32 g_next_handle = 1u;
static sigma_bool g_initialised = SIGMA_FALSE;

/* ── svk_init ────────────────────────────────────────────────────────── */
void svk_init(void)
{
    /* Soft-reset GPU */
    HAL_MMIO_W32(GPU_REG_CTRL, GPU_CTRL_RESET);
    /* Enable clock */
    HAL_MMIO_W32(GPU_REG_CTRL, GPU_CTRL_CLK_EN);
    g_initialised = SIGMA_TRUE;
    sigma_log_info("[SVK] SovereignVulkanLayer initialised. Queue@0x%llx", (sigma_u64)GPU_CMD_QUEUE);
}

/* ── svk_wait_idle ───────────────────────────────────────────────────── */
svk_result_t svk_wait_idle(void)
{
    sigma_u32 retries = 1000000u;
    while (retries--) {
        sigma_u32 status = HAL_MMIO_R32(GPU_REG_STATUS);
        if (status & GPU_STATUS_ERROR)  return SVK_ERR_NOT_READY;
        if (status & GPU_STATUS_IDLE)   return SVK_OK;
        cpu_pause();
    }
    return SVK_ERR_TIMEOUT;
}

/* ── svk_submit_shader ───────────────────────────────────────────────── */
svk_result_t svk_submit_shader(const sigma_u32* spirv,
                                sigma_u32        words,
                                svk_stage_t      stage,
                                svk_shader_t*    out)
{
    if (!g_initialised || !spirv || !words || !out)
        return SVK_ERR_INVALID;

    svk_result_t rc = svk_wait_idle();
    if (rc != SVK_OK) return rc;

    /* Select shader stage */
    HAL_MMIO_W32(GPU_REG_STAGE, (sigma_u32)stage);
    /* Announce word count */
    HAL_MMIO_W32(GPU_REG_WORD_COUNT, words);

    /* Stream SPIR-V words into the command FIFO */
    for (sigma_u32 i = 0u; i < words; ++i) {
        HAL_MMIO_W32(GPU_CMD_QUEUE + (sigma_paddr_t)(i * 4u), spirv[i]);
    }

    /* Trigger shader load */
    HAL_MMIO_W32(GPU_REG_CTRL, GPU_CTRL_SHADER_LD | GPU_CTRL_CLK_EN);

    rc = svk_wait_idle();
    if (rc != SVK_OK) return rc;

    *out = g_next_handle++;
    sigma_log_info("[SVK] Shader loaded: handle=%u stage=%u words=%u",
                   *out, (sigma_u32)stage, words);
    return SVK_OK;
}

/* ── svk_dispatch ────────────────────────────────────────────────────── */
svk_result_t svk_dispatch(sigma_u32 x, sigma_u32 y, sigma_u32 z)
{
    if (!g_initialised) return SVK_ERR_NOT_READY;

    svk_result_t rc = svk_wait_idle();
    if (rc != SVK_OK) return rc;

    HAL_MMIO_W32(GPU_REG_DISPATCH_X, x);
    HAL_MMIO_W32(GPU_REG_DISPATCH_Y, y);
    HAL_MMIO_W32(GPU_REG_DISPATCH_Z, z);
    HAL_MMIO_W32(GPU_REG_CTRL, GPU_CTRL_DISPATCH | GPU_CTRL_CLK_EN);

    sigma_log_info("[SVK] Dispatch: %ux%ux%u workgroups", x, y, z);
    return SVK_OK;
}

/* ── svk_draw ────────────────────────────────────────────────────────── */
svk_result_t svk_draw(sigma_u32 vertex_count)
{
    if (!g_initialised) return SVK_ERR_NOT_READY;

    svk_result_t rc = svk_wait_idle();
    if (rc != SVK_OK) return rc;

    HAL_MMIO_W32(GPU_REG_DRAW_CNT, vertex_count);
    HAL_MMIO_W32(GPU_REG_CTRL, GPU_CTRL_DRAW | GPU_CTRL_CLK_EN);

    sigma_log_info("[SVK] Draw: %u vertices", vertex_count);
    return SVK_OK;
}
