// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
// sigma_voice_control.h — Voice navigation using local sigma-ai (Whisper.cpp)
#include <sigma_kernel_types.h>

namespace sigma::a11y {

class VoiceControl {
public:
    bool  enabled;
    bool  wake_word_enabled;
    char  wake_word[32];

    // Statistics
    sigma_u64 utterance_count;
    sigma_u64 commands_dispatched;
    sigma_u64 unrecognised_count;

    VoiceControl();
    void enable();
    void disable();
    void set_wake_word(const char *word);

    /** Process a transcribed text utterance, dispatch matched command. */
    int process_utterance(const char *text);

    /**
     * Transcribe raw PCM audio via sigma-ai Whisper.cpp endpoint.
     * pcm_samples: 16-bit mono PCM at 16 kHz, converted to float32.
     */
    int transcribe_audio(const float *pcm_samples, size_t n_samples,
                          char *text_out, size_t max_len);

    /** List all built-in voice commands. */
    void list_commands(void (*cb)(const char *phrase, void *ctx), void *ctx);
};

} // namespace sigma::a11y
