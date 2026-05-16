/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN VOICE ORCHESTRATOR (S-VOICE)
 * =========================================================================
 * Mission: A completely offline, zero-latency voice recognition and 
 * command engine for true hands-free automation.
 * =========================================================================
 */

#ifndef SIGMA_VOICE_H
#define SIGMA_VOICE_H

#include "./sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Voice Orchestrator Primitives --- */
void voice_init(void);
void voice_process_audio_stream(const void* audio_buffer, uint32_t size);
void voice_execute_intent(const char* intent);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_VOICE_H */
