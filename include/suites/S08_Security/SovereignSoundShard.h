/* S SIGMAOS: SOVEREIGN SOUND SHARD HEADER */
#ifndef SOVEREIGN_SOUND_SHARD_H
#define SOVEREIGN_SOUND_SHARD_H
#include "sigma_types.h"

typedef enum { SND_STREAM_PCM, SND_STREAM_MIDI, SND_STREAM_COMP } SigmaSndType_t;

sigma_err_t sigma_snd_open (const char* client, SigmaSndType_t type, sigma_u32 rate, sigma_u16 ch, sigma_u16 lat);
void        sigma_snd_render_block (void);
void        SovereignSoundShard_Init (void);
void        SovereignSound_Audit      (void);

#endif
