/* Σ SIGMAOS: SOVEREIGN MULTIMEDIA SHARD HEADER */
#ifndef SOVEREIGN_MULTIMEDIA_SHARD_H
#define SOVEREIGN_MULTIMEDIA_SHARD_H
#include "sigma_types.h"
typedef enum { STREAM_PCM_AUDIO, STREAM_COMPRESSED_AUDIO,
               STREAM_VIDEO_RAW, STREAM_VIDEO_ENCODED } SigmaStreamType_t;
sigma_err_t sigma_mm_open_stream    (const char* client, SigmaStreamType_t type,
                                      sigma_u32 rate, sigma_u32 ch, sigma_u32 bits,
                                      sigma_u32 lat_ms, sigma_bool hw_accel);
void        sigma_mm_process         (sigma_u32 stream_id);
void        sigma_mm_set_volume      (sigma_u32 vol);
void        sigma_mm_suspend_stream  (sigma_u32 stream_id);
void        SovereignMultimediaShard_Init (void);
void        SovereignMultimedia_Audit     (void);
#endif
