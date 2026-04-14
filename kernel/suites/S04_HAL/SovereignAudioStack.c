// SigmaOS Sovereign Audio Driver Stack
// Absorbs ALSA (Linux), CoreAudio (macOS), WASAPI (Windows) paradigms.
// Zero-dependency, hardware-mapped C11 shard.

#include "sigma_types.h"


#define SIGMA_AUDIO_SAMPLE_RATE_48K   48000
#define SIGMA_AUDIO_SAMPLE_RATE_192K 192000
#define SIGMA_AUDIO_MAX_CHANNELS      8

typedef struct {
    uint32_t sample_rate;
    uint8_t  channels;
    uint8_t  bit_depth;
    bool     low_latency_mode; // Absorbs WASAPI Exclusive Mode
} SigmaAudioConfig;

// Initialize the hardware audio subsystem (I2S/HDA/USB audio)
void audio_init_hardware(SigmaAudioConfig* cfg);

// Render a PCM buffer to the DAC in the hardware — direct ring buffer model
void audio_render_pcm(const int16_t* samples, uint32_t frame_count);

// Capture PCM from microphone ADC into a kernel ring buffer
void audio_capture_pcm(int16_t* out_buffer, uint32_t frame_count);

// Route audio between shards via the sovereign IPC layer (zero-copy)
void audio_route_shards(uint32_t src_port, uint32_t dst_port);

// Apply hardware EQ filters using GPU compute shaders (WASAPI APO equivalent)
void audio_apply_hw_equalizer(float* band_gains, uint8_t band_count);

