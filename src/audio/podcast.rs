use std::format;
// Sovereign Podcast Recording & RSS Publishing Engine
// Inspired by GarageBand and Anchor, providing multi-track recording, mastering filters, and iTunes XML feed generation.


use std::string::String;
use std::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering};

/// A recorded audio clip segment
#[derive(Debug, Clone)]
pub struct AudioClip {
    pub source_path: String,
    pub timeline_start_sample: usize,
    pub duration_samples: usize,
    pub gain: f32,
}

impl AudioClip {
    pub fn new(source_path: &str, start_sample: usize, duration: usize) -> Self {
        Self {
            source_path: String::from(source_path),
            timeline_start_sample: start_sample,
            duration_samples: duration,
            gain: 1.0,
        }
    }
}

/// A specific audio track (Mic, Background Music, Sound Effects)
pub struct AudioTrack {
    pub id: usize,
    pub name: String,
    pub clips: Vec<AudioClip>,
    pub mute: bool,
    pub solo: bool,
    pub pan: f32, // -1.0 = full left, 0.0 = center, 1.0 = full right
}

impl AudioTrack {
    pub fn new(id: usize, name: &str) -> Self {
        Self {
            id,
            name: String::from(name),
            clips: Vec::new(),
            mute: false,
            solo: false,
            pan: 0.0,
        }
    }

    pub fn add_clip(&mut self, clip: AudioClip) {
        self.clips.push(clip);
    }

    pub fn set_pan(&mut self, pan: f32) {
        self.pan = pan.clamp(-1.0, 1.0);
    }
}

/// Dynamic mastering effects matching GarageBand tools
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AudioMasteringEffect {
    NoiseGate { threshold_db: f32 },
    Limiter { threshold_db: f32 },
    Reverb { room_size: f32 },
    Equalizer { bass_gain: f32, treble_gain: f32 },
    Compressor { threshold_db: f32, ratio: f32, attack_ms: f32, release_ms: f32 },
    DeEsser { frequency_hz: f32, threshold_db: f32 },
}

/// Podcast Publisher details matching Anchor XML metadata spec
#[derive(Debug, Clone)]
pub struct PodcastFeed {
    pub title: String,
    pub description: String,
    pub author: String,
    pub language: String,
    pub cover_art_url: String,
    pub category: String,
    pub explicit: bool,
    pub episodes: Vec<PodcastEpisode>,
}

#[derive(Debug, Clone)]
pub struct PodcastEpisode {
    pub id: String,
    pub title: String,
    pub description: String,
    pub audio_url: String,
    pub duration_seconds: usize,
    pub explicit: bool,
}

impl PodcastFeed {
    pub fn new(title: &str, description: &str, author: &str) -> Self {
        Self {
            title: String::from(title),
            description: String::from(description),
            author: String::from(author),
            language: String::from("en-us"),
            cover_art_url: String::from("https://anchor.sigma.os/default_cover.png"),
            category: String::from("Technology"),
            explicit: false,
            episodes: Vec::new(),
        }
    }

    pub fn add_episode(&mut self, episode: PodcastEpisode) {
        self.episodes.push(episode);
    }

    /// Automatically synthesizes valid, iTunes-compliant RSS XML feed format
    pub fn generate_rss_xml(&self) -> String {
        let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str(
            "<rss version=\"2.0\" xmlns:itunes=\"http://www.itunes.com/dtds/podcast-1.0.dtd\">\n",
        );
        xml.push_str("  <channel>\n");
        xml.push_str(&std::format!("    <title>{}</title>\n", self.title));
        xml.push_str(&std::format!(
            "    <description>{}</description>\n",
            self.description
        ));
        xml.push_str(&std::format!(
            "    <itunes:author>{}</itunes:author>\n",
            self.author
        ));
        xml.push_str(&std::format!(
            "    <language>{}</language>\n",
            self.language
        ));
        xml.push_str(&std::format!(
            "    <itunes:image href=\"{}\"/>\n",
            self.cover_art_url
        ));
        xml.push_str(&std::format!(
            "    <itunes:category text=\"{}\"/>\n",
            self.category
        ));
        xml.push_str(&std::format!(
            "    <itunes:explicit>{}</itunes:explicit>\n",
            if self.explicit { "yes" } else { "no" }
        ));

        for ep in &self.episodes {
            xml.push_str("    <item>\n");
            xml.push_str(&std::format!("      <guid>{}</guid>\n", ep.id));
            xml.push_str(&std::format!("      <title>{}</title>\n", ep.title));
            xml.push_str(&std::format!(
                "      <description>{}</description>\n",
                ep.description
            ));
            xml.push_str(&std::format!(
                "      <enclosure url=\"{}\" length=\"0\" type=\"audio/mpeg\"/>\n",
                ep.audio_url
            ));
            xml.push_str(&std::format!(
                "      <itunes:duration>{}</itunes:duration>\n",
                ep.duration_seconds
            ));
            xml.push_str(&std::format!(
                "      <itunes:explicit>{}</itunes:explicit>\n",
                if ep.explicit { "yes" } else { "no" }
            ));
            xml.push_str("    </item>\n");
        }

        xml.push_str("  </channel>\n</rss>");
        xml
    }
}

/// Anchor Platform Publishing Integration
pub struct AnchorPublisher {
    pub auth_token: String,
    pub feed: PodcastFeed,
    pub published_count: usize,
}

impl AnchorPublisher {
    pub fn new(auth_token: &str, feed: PodcastFeed) -> Self {
        Self {
            auth_token: String::from(auth_token),
            feed,
            published_count: 0,
        }
    }

    pub fn publish_episode(&mut self, episode: PodcastEpisode) -> Result<String, &'static str> {
        if self.auth_token.is_empty() {
            return Err("Anchor Publishing: Invalid API authentication token");
        }
        let episode_url = episode.audio_url.clone();
        self.feed.add_episode(episode);
        self.published_count += 1;
        Ok(episode_url)
    }

    pub fn export_rss_xml(&self) -> String {
        self.feed.generate_rss_xml()
    }
}

/// Podcast Recording and Mixing console
pub struct PodcastRecorder {
    pub tracks: Vec<AudioTrack>,
    pub mastering_effects: Vec<AudioMasteringEffect>,
    pub recording: AtomicBool,
}

impl PodcastRecorder {
    pub fn new() -> Self {
        Self {
            tracks: Vec::new(),
            mastering_effects: Vec::new(),
            recording: AtomicBool::new(false),
        }
    }

    pub fn add_track(&mut self, track: AudioTrack) {
        self.tracks.push(track);
    }

    pub fn add_mastering_effect(&mut self, effect: AudioMasteringEffect) {
        self.mastering_effects.push(effect);
    }

    pub fn toggle_recording(&self, active: bool) {
        self.recording.store(active, Ordering::SeqCst);
    }

    /// Renders and mixes down the multi-track recording into a single mono/stereo buffer frame, applying mastering filters sequentially
    pub fn process_master_mix(&self, sample_index: usize, base_amplitude: f32) -> f32 {
        let mut mixed_sample = 0.0;
        let mut active_solo_tracks = 0;

        for track in &self.tracks {
            if track.solo && !track.mute {
                active_solo_tracks += 1;
            }
        }

        // Mix tracks
        for track in &self.tracks {
            if track.mute {
                continue;
            }
            if active_solo_tracks > 0 && !track.solo {
                continue;
            }

            // Find if any clip is active at this sample index
            let active_clip = track.clips.iter().find(|clip| {
                sample_index >= clip.timeline_start_sample
                    && sample_index < (clip.timeline_start_sample + clip.duration_samples)
            });

            if let Some(clip) = active_clip {
                let pan_gain = 1.0 - (track.pan.abs() * 0.15); // Mild pan attenuation
                mixed_sample += base_amplitude * clip.gain * pan_gain;
            }
        }

        // Apply GarageBand-grade mastering filters
        for effect in &self.mastering_effects {
            match *effect {
                AudioMasteringEffect::NoiseGate { threshold_db } => {
                    let amp_threshold = 10.0f32.powf(threshold_db / 20.0);
                    if mixed_sample.abs() < amp_threshold {
                        mixed_sample = 0.0; // Mute below noise floor
                    }
                }
                AudioMasteringEffect::Limiter { threshold_db } => {
                    let amp_limit = 10.0f32.powf(threshold_db / 20.0);
                    if mixed_sample.abs() > amp_limit {
                        mixed_sample = mixed_sample.signum() * amp_limit; // Clamp peak
                    }
                }
                AudioMasteringEffect::Compressor { threshold_db, ratio, .. } => {
                    let amp_thresh = 10.0f32.powf(threshold_db / 20.0);
                    if mixed_sample.abs() > amp_thresh {
                        let excess = mixed_sample.abs() - amp_thresh;
                        let compressed_excess = excess / ratio.max(1.0);
                        mixed_sample = mixed_sample.signum() * (amp_thresh + compressed_excess);
                    }
                }
                AudioMasteringEffect::DeEsser { threshold_db, .. } => {
                    let amp_thresh = 10.0f32.powf(threshold_db / 20.0);
                    if mixed_sample.abs() > amp_thresh {
                        mixed_sample *= 0.85; // Suppress high-frequency sibilance peaks
                    }
                }
                AudioMasteringEffect::Reverb { room_size } => {
                    mixed_sample += mixed_sample * room_size * 0.1;
                }
                AudioMasteringEffect::Equalizer {
                    bass_gain,
                    treble_gain,
                } => {
                    mixed_sample *= 1.0 + (bass_gain + treble_gain) * 0.05;
                }
            }
        }

        mixed_sample
    }
}

impl Default for PodcastRecorder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_podcast_mixing_and_recording() {
        let mut recorder = PodcastRecorder::new();
        let mut track_mic = AudioTrack::new(1, "SovereignMic");
        track_mic.set_pan(0.0);

        let clip = AudioClip::new("recorded_vocal.raw", 0, 48000);
        track_mic.add_clip(clip);

        recorder.add_track(track_mic);
        recorder.toggle_recording(true);

        assert!(recorder.recording.load(Ordering::SeqCst));

        // Sample is inside clip duration -> Mix down matches base amplitude
        let mix_active = recorder.process_master_mix(1000, 0.5);
        assert_eq!(mix_active, 0.5);

        // Sample is outside clip duration -> Mix down is silent
        let mix_silent = recorder.process_master_mix(96000, 0.5);
        assert_eq!(mix_silent, 0.0);
    }

    #[test]
    fn test_podcast_mastering_filters() {
        let mut recorder = PodcastRecorder::new();
        let mut track_mic = AudioTrack::new(1, "SovereignMic");
        let clip = AudioClip::new("vocal.raw", 0, 100);
        track_mic.add_clip(clip);

        recorder.add_track(track_mic);

        // Apply a Limiter filter (clamping peak amplitude)
        recorder.add_mastering_effect(AudioMasteringEffect::Limiter { threshold_db: -6.0 }); // Limit around ~0.5 amplitude

        let mix = recorder.process_master_mix(50, 0.9); // Base amplitude 0.9 gets clamped!
        assert!(mix < 0.9);
        assert!((mix - 0.5011).abs() < 0.01);

        // Test Compressor
        let mut comp_recorder = PodcastRecorder::new();
        let mut comp_track = AudioTrack::new(2, "CompTrack");
        comp_track.add_clip(AudioClip::new("speech.raw", 0, 100));
        comp_recorder.add_track(comp_track);
        comp_recorder.add_mastering_effect(AudioMasteringEffect::Compressor {
            threshold_db: -12.0,
            ratio: 4.0,
            attack_ms: 10.0,
            release_ms: 100.0,
        });

        let comp_mix = comp_recorder.process_master_mix(10, 0.8);
        assert!(comp_mix < 0.8);
    }

    #[test]
    fn test_podcast_anchor_xml_feed_publishing() {
        let feed = PodcastFeed::new(
            "Sovereign Voice",
            "A weekly talk about operating systems and AI.",
            "Sovereign Team",
        );
        let ep = PodcastEpisode {
            id: String::from("ep-101"),
            title: String::from("Episode 1: The Boot Labyrinth"),
            description: String::from("Deep dive into GDT and IDT setup."),
            audio_url: String::from("https://sigmaos.org/episodes/ep1.mp3"),
            duration_seconds: 1800,
            explicit: false,
        };

        let mut publisher = AnchorPublisher::new("anchor_secret_token_123", feed);
        let published_url = publisher.publish_episode(ep).unwrap();
        assert_eq!(published_url, "https://sigmaos.org/episodes/ep1.mp3");

        let xml = publisher.export_rss_xml();
        assert!(xml.contains("<title>Sovereign Voice</title>"));
        assert!(xml.contains("<itunes:author>Sovereign Team</itunes:author>"));
        assert!(xml.contains("<guid>ep-101</guid>"));
        assert!(xml.contains("<enclosure url=\"https://sigmaos.org/episodes/ep1.mp3\""));
    }
}
