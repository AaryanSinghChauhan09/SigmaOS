/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN AUDIO ENGINE (SAE)
 * =========================================================================
 * Mission: Silicon-native, zero-latency waveform orchestration.
 * =========================================================================
 */

#ifndef SIGMA_AUDIO_H
#define SIGMA_AUDIO_H

#include "../include/core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    sigma_u32 sample_rate;
    sigma_u8 channels;
    sigma_u8 bit_depth;
} sigma_audio_config_t;

/* --- Audio Primitives --- */
void audio_init(sigma_audio_config_t* config);
void audio_play_shard_tone(uint32_t shard_id, sigma_u32 frequency, sigma_u32 duration_ms);
void audio_flush(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_AUDIO_H */
