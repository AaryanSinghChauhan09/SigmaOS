/* Σ SIGMAOS: SOVEREIGN VOICE SHARD HEADER */
#ifndef SOVEREIGN_VOICE_SHARD_H
#define SOVEREIGN_VOICE_SHARD_H
#include "sigma_types.h"

void sigma_voice_listen    (void);
void sigma_voice_intent    (const char* phrase);
void SovereignVoiceShard_Init   (void);
void SovereignVoice_Audit       (void);

#endif
