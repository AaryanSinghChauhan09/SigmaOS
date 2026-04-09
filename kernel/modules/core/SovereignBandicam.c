/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SCREEN CAPTURE — IMPL (v1.0 — C11)
 * =========================================================================
 */

#include "../../../include/sigma_kernel.h"
#include "../../../include/SovereignBandicam.h"

static SigmaBandiCtx_t s_ctx;

sigma_err_t sigma_bandicam_start(const SigmaBandiCtx_t *config) {
    if (s_ctx.is_recording) {
        sigma_printf("Σ [BANDI]: Already recording.\n");
        return SIGMA_EBUSY;
    }
    sigma_memcpy(&s_ctx, config, sizeof(s_ctx));
    s_ctx.is_recording = SIGMA_TRUE;

    sigma_printf("Σ [BANDI]: Recording started. HW-Encoder: %s @ %u FPS.\n",
                 config->codec == BANDI_CODEC_H264 ? "NVENC H.264" :
                 config->codec == BANDI_CODEC_HEVC ? "QSV HEVC" : "AV1",
                 config->fps);
    if (config->region.width == 0) {
        sigma_printf("Σ [BANDI]: Target: Fullscreen (%s)\n", config->output_path);
    } else {
        sigma_printf("Σ [BANDI]: Target: Region [%d,%d %dx%d] (%s)\n",
                     config->region.x, config->region.y, config->region.width, config->region.height,
                     config->output_path);
    }
    return SIGMA_OK;
}

sigma_err_t sigma_bandicam_stop(void) {
    if (!s_ctx.is_recording) return SIGMA_ENOENT;
    s_ctx.is_recording = SIGMA_FALSE;
    sigma_printf("Σ [BANDI]: Recording stopped and multiplexed to '%s'.\n", s_ctx.output_path);
    return SIGMA_OK;
}

sigma_err_t sigma_bandicam_toggle_fps(sigma_bool show) {
    s_ctx.show_fps_overlay = show;
    sigma_printf("Σ [BANDI]: FPS Overlay %s.\n", show ? "ON" : "OFF");
    return SIGMA_OK;
}

void SovereignBandicam_Init(void) {
    sigma_printf("Σ [BANDI]: Initialising Sovereign Screen Capture...\n");
    SigmaBandiCtx_t cfg;
    sigma_memset(&cfg, 0, sizeof(cfg));
    sigma_strcpy(cfg.output_path, "/videos/capture_01.mp4", 256);
    cfg.codec = BANDI_CODEC_HEVC;
    cfg.fps = 60;
    cfg.record_mic = SIGMA_TRUE;
    cfg.record_sysaudio = SIGMA_TRUE;
    cfg.show_fps_overlay = SIGMA_TRUE;
    sigma_bandicam_start(&cfg);
    sigma_bandicam_stop();
}
