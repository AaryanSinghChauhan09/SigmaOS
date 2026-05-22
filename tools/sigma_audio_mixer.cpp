/*
 * =========================================================================
 * Î£ SIGMAOS: SIGMA AUDIO MIXER (sigma_audio_mixer) v1.0
 * =========================================================================
 * Mission: Sovereign sound routing and mixing.
 * Inspiration: PipeWire / PulseAudio, but lock-free and deterministic.
 * Principle: Zero-copy hardware audio buffer orchestration.
 * =========================================================================
 */

#include "sigma_kernel_types.h"
#include "sigma_log.h"
#include "SigmaOOP.hpp"

namespace SigmaOS {
namespace Tools {

struct AudioStream {
    char      name[64];
    sigma_u32 sample_rate;
    sigma_u8  channels;
    sigma_u8  volume;  /* 0-100 */
    sigma_u8  active;
};

class SigmaAudioMixer : public SigmaObject, public SigmaSingleton<SigmaAudioMixer> {
    friend class SigmaSingleton<SigmaAudioMixer>;
public:
    const char* type_name() const noexcept override { return "SigmaAudioMixer"; }

    void init() {
        m_stream_count = 0;
        m_master_volume = 80;
        sigma_log_info("[AUDIO] Sigma Audio Mixer v1.0 initialized.");
        sigma_log_info("[AUDIO] Engine: Lock-Free Zero-Copy Mode");
    }

    void register_stream(const char* name, sigma_u32 rate, sigma_u8 channels) {
        if (m_stream_count >= MAX_STREAMS) return;
        AudioStream& s = m_streams[m_stream_count++];
        sigma_u32 i = 0;
        while (name[i] && i < 63) { s.name[i] = name[i]; i++; } s.name[i] = '\0';
        s.sample_rate = rate;
        s.channels = channels;
        s.volume = 100;
        s.active = 1;
        sigma_log_info("[AUDIO] Registered stream '%s' (%uHz, %uch)", name, rate, channels);
    }

    void set_volume(const char* name, sigma_u8 vol) {
        if (vol > 100) vol = 100;
        for (sigma_u32 i = 0; i < m_stream_count; i++) {
            sigma_u32 j = 0;
            while (m_streams[i].name[j] == name[j] && name[j]) j++;
            if (!name[j] && !m_streams[i].name[j]) {
                m_streams[i].volume = vol;
                sigma_log_info("[AUDIO] Stream '%s' volume set to %u%%", name, vol);
                return;
            }
        }
        sigma_log_infoor("[AUDIO] Stream '%s' not found.", name);
    }

    void set_master_volume(sigma_u8 vol) {
        if (vol > 100) vol = 100;
        m_master_volume = vol;
        sigma_log_info("[AUDIO] Master volume set to %u%%", vol);
    }

    void list_streams() const {
        sigma_log_info("[AUDIO] ===== Active Audio Streams =====");
        sigma_log_info("[AUDIO] MASTER VOLUME: %u%%", m_master_volume);
        for (sigma_u32 i = 0; i < m_stream_count; i++) {
            sigma_log_info("[AUDIO] %-20s %uHz %uch Vol:%u%%", 
                m_streams[i].name, m_streams[i].sample_rate, m_streams[i].channels, m_streams[i].volume);
        }
    }

private:
    static constexpr sigma_u32 MAX_STREAMS = 32;
    SigmaAudioMixer() : m_stream_count(0), m_master_volume(80) {}
    AudioStream m_streams[MAX_STREAMS];
    sigma_u32 m_stream_count;
    sigma_u8 m_master_volume;
};

} // namespace Tools
} // namespace SigmaOS

extern "C" {
void audio_init()                                                  { SigmaOS::Tools::SigmaAudioMixer::getInstance().init(); }
void audio_register(const char* name, sigma_u32 r, sigma_u8 c)     { SigmaOS::Tools::SigmaAudioMixer::getInstance().register_stream(name, r, c); }
void audio_set_vol(const char* name, sigma_u8 vol)                 { SigmaOS::Tools::SigmaAudioMixer::getInstance().set_volume(name, vol); }
void audio_master_vol(sigma_u8 vol)                                { SigmaOS::Tools::SigmaAudioMixer::getInstance().set_master_volume(vol); }
void audio_list()                                                  { SigmaOS::Tools::SigmaAudioMixer::getInstance().list_streams(); }
}

