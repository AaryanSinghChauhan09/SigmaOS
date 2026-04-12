/* Σ SIGMAOS: SOVEREIGN GAMING SHARD HEADER */
#ifndef SOVEREIGN_GAMING_SHARD_H
#define SOVEREIGN_GAMING_SHARD_H
#include "sigma_types.h"
typedef enum { GAMING_MODE_OFF, GAMING_MODE_BALANCED,
               GAMING_MODE_PERFORMANCE, GAMING_MODE_COMPETITIVE } SigmaGamingMode_t;
sigma_err_t sigma_gaming_launch     (const char* title, sigma_u32 pid,
                                      SigmaGamingMode_t mode, sigma_u32 fps);
void        sigma_gaming_frame_tick  (sigma_u32 pid);
void        sigma_gaming_stop        (sigma_u32 pid);
void        SovereignGamingShard_Init(void);
void        SovereignGaming_Audit    (void);
#endif
