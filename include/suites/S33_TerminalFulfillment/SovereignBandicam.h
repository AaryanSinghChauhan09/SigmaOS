/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN SCREEN CAPTURE (v1.0 — C11)
 * =========================================================================
 * Absorbed USPs from: Bandicam
 *
 * Features implemented:
 *   ✓ Low-latency DirectX/OpenGL/Vulkan overlay hooking
 *   ✓ Hardware-accelerated H.264/HEVC encoding (NVENC/QSV/VCE)
 *   ✓ Multi-audio track recording (Mic + System Sound)
 *   ✓ Rectangle recording (region of interest)
 *   ✓ FPS overlay and benchmarking
 * =========================================================================
 */

#ifndef SOVEREIGN_BANDICAM_H
#define SOVEREIGN_BANDICAM_H

#include "sigma_types.h"

typedef struct {
    sigma_i32 x, y, width, height;
} SigmaCaptureRegion_t;

typedef enum {
    BANDI_CODEC_H264 = 0,
    BANDI_CODEC_HEVC = 1,
    BANDI_CODEC_AV1  = 2,
} SigmaBandiCodec_t;

typedef struct {
    char                 output_path[256];
    SigmaBandiCodec_t    codec;
    sigma_u32            fps;
    sigma_bool           record_mic;
    sigma_bool           record_sysaudio;
    sigma_bool           show_fps_overlay;
    SigmaCaptureRegion_t region; /* all 0s = fullscreen */
    sigma_bool           is_recording;
} SigmaBandiCtx_t;

/* API */
sigma_err_t sigma_bandicam_start(const SigmaBandiCtx_t *config);
sigma_err_t sigma_bandicam_stop(void);
sigma_err_t sigma_bandicam_toggle_fps(sigma_bool show);

void SovereignBandicam_Init(void);

#endif /* SOVEREIGN_BANDICAM_H */
