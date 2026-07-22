// ALSA-style Audio Stack - Linux-style Advanced Linux Sound Architecture
// Supports PCM devices, mixers, and audio stream management

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioDirection {
    Playback,
    Capture,
    Both,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioFormat {
    U8,
    S16Le,
    S16Be,
    S24Le,
    S24Be,
    S32Le,
    S32Be,
    Float32Le,
    Float32Be,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SampleRate {
    Rate8000,
    Rate11025,
    Rate16000,
    Rate22050,
    Rate32000,
    Rate44100,
    Rate48000,
    Rate64000,
    Rate88200,
    Rate96000,
    Rate176400,
    Rate192000,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelConfig {
    Mono,
    Stereo,
    Surround51, // 5.1 surround
    Surround71, // 7.1 surround
}

#[derive(Debug, Clone)]
pub struct PcmStream {
    pub id: u32,
    pub name: String,
    pub direction: AudioDirection,
    pub format: AudioFormat,
    pub sample_rate: SampleRate,
    pub channels: ChannelConfig,
    pub buffer_size: usize,
    pub period_size: usize,
    pub opened: bool,
    pub running: bool,
}

#[derive(Debug, Clone)]
pub struct MixerControl {
    pub id: u32,
    pub name: String,
    pub min_value: i32,
    pub max_value: i32,
    pub current_value: i32,
    pub is_muted: bool,
}

pub struct AlsaAudioStack {
    pcm_streams: BTreeMap<u32, PcmStream>,
    mixer_controls: BTreeMap<u32, MixerControl>,
    next_stream_id: u32,
    next_mixer_id: u32,
}

impl AlsaAudioStack {
    pub fn new() -> Self {
        Self {
            pcm_streams: BTreeMap::new(),
            mixer_controls: BTreeMap::new(),
            next_stream_id: 0,
            next_mixer_id: 0,
        }
    }

    /// Create a new PCM stream
    pub fn create_pcm_stream(
        &mut self,
        name: String,
        direction: AudioDirection,
        format: AudioFormat,
        sample_rate: SampleRate,
        channels: ChannelConfig,
        buffer_size: usize,
        period_size: usize,
    ) -> Result<u32, &'static str> {
        let id = self.next_stream_id;
        self.next_stream_id += 1;

        let stream = PcmStream {
            id,
            name,
            direction,
            format,
            sample_rate,
            channels,
            buffer_size,
            period_size,
            opened: false,
            running: false,
        };

        self.pcm_streams.insert(id, stream);
        Ok(id)
    }

    /// Open a PCM stream
    pub fn open_pcm_stream(&mut self, id: u32) -> Result<(), &'static str> {
        let stream = self
            .pcm_streams
            .get_mut(&id)
            .ok_or("PCM stream not found")?;

        stream.opened = true;
        Ok(())
    }

    /// Close a PCM stream
    pub fn close_pcm_stream(&mut self, id: u32) -> Result<(), &'static str> {
        let stream = self
            .pcm_streams
            .get_mut(&id)
            .ok_or("PCM stream not found")?;

        stream.opened = false;
        stream.running = false;
        Ok(())
    }

    /// Start a PCM stream
    pub fn start_pcm_stream(&mut self, id: u32) -> Result<(), &'static str> {
        let stream = self
            .pcm_streams
            .get_mut(&id)
            .ok_or("PCM stream not found")?;

        if !stream.opened {
            return Err("PCM stream not opened");
        }

        stream.running = true;
        Ok(())
    }

    /// Stop a PCM stream
    pub fn stop_pcm_stream(&mut self, id: u32) -> Result<(), &'static str> {
        let stream = self
            .pcm_streams
            .get_mut(&id)
            .ok_or("PCM stream not found")?;

        stream.running = false;
        Ok(())
    }

    /// Write audio data to a playback stream
    pub fn write_pcm(&mut self, id: u32, data: &[u8]) -> Result<usize, &'static str> {
        let stream = self
            .pcm_streams
            .get_mut(&id)
            .ok_or("PCM stream not found")?;

        if !stream.opened {
            return Err("PCM stream not opened");
        }

        if !stream.running {
            return Err("PCM stream not running");
        }

        if stream.direction != AudioDirection::Playback && stream.direction != AudioDirection::Both
        {
            return Err("Stream is not a playback stream");
        }

        // In a real implementation, this would write to the audio hardware
        Ok(data.len())
    }

    /// Read audio data from a capture stream
    pub fn read_pcm(&mut self, id: u32, buffer: &mut [u8]) -> Result<usize, &'static str> {
        let stream = self.pcm_streams.get_mut(&id)
            .ok_or("PCM stream not found")?;

        if !stream.opened {
            return Err("PCM stream not opened");
        }

        if !stream.running {
            return Err("PCM stream not running");
        }

        if stream.direction != AudioDirection::Capture && stream.direction != AudioDirection::Both {
            return Err("Stream is not a capture stream");
        }

        // In a real implementation, this would read from the audio hardware
        Ok(buffer.len())
    }

    /// Create a mixer control
    pub fn create_mixer_control(
        &mut self,
        name: String,
        min_value: i32,
        max_value: i32,
        default_value: i32,
    ) -> Result<u32, &'static str> {
        let id = self.next_mixer_id;
        self.next_mixer_id += 1;

        let control = MixerControl {
            id,
            name,
            min_value,
            max_value,
            current_value: default_value,
            is_muted: false,
        };

        self.mixer_controls.insert(id, control);
        Ok(id)
    }

    /// Set mixer control value
    pub fn set_mixer_value(&mut self, id: u32, value: i32) -> Result<(), &'static str> {
        let control = self
            .mixer_controls
            .get_mut(&id)
            .ok_or("Mixer control not found")?;

        if value < control.min_value || value > control.max_value {
            return Err("Value out of range");
        }

        control.current_value = value;
        Ok(())
    }

    /// Get mixer control value
    pub fn get_mixer_value(&self, id: u32) -> Result<i32, &'static str> {
        let control = self
            .mixer_controls
            .get(&id)
            .ok_or("Mixer control not found")?;

        Ok(control.current_value)
    }

    /// Mute/unmute mixer control
    pub fn set_mute(&mut self, id: u32, muted: bool) -> Result<(), &'static str> {
        let control = self
            .mixer_controls
            .get_mut(&id)
            .ok_or("Mixer control not found")?;

        control.is_muted = muted;
        Ok(())
    }

    /// Get PCM stream by ID
    pub fn get_pcm_stream(&self, id: u32) -> Option<&PcmStream> {
        self.pcm_streams.get(&id)
    }

    /// Get mixer control by ID
    pub fn get_mixer_control(&self, id: u32) -> Option<&MixerControl> {
        self.mixer_controls.get(&id)
    }

    /// List all PCM streams
    pub fn list_pcm_streams(&self) -> Vec<&PcmStream> {
        self.pcm_streams.values().collect()
    }

    /// List all mixer controls
    pub fn list_mixer_controls(&self) -> Vec<&MixerControl> {
        self.mixer_controls.values().collect()
    }

    /// Get PCM stream count
    pub fn pcm_stream_count(&self) -> usize {
        self.pcm_streams.len()
    }

    /// Get mixer control count
    pub fn mixer_control_count(&self) -> usize {
        self.mixer_controls.len()
    }
}

impl Default for AlsaAudioStack {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_pcm_stream() {
        let mut stack = AlsaAudioStack::new();

        let id = stack
            .create_pcm_stream(
                "default".to_string(),
                AudioDirection::Playback,
                AudioFormat::S16Le,
                SampleRate::Rate48000,
                ChannelConfig::Stereo,
                4096,
                1024,
            )
            .unwrap();

        assert_eq!(stack.pcm_stream_count(), 1);

        let stream = stack.get_pcm_stream(id).unwrap();
        assert_eq!(stream.name, "default");
    }

    #[test]
    fn test_open_pcm_stream() {
        let mut stack = AlsaAudioStack::new();

        let id = stack
            .create_pcm_stream(
                "test".to_string(),
                AudioDirection::Playback,
                AudioFormat::S16Le,
                SampleRate::Rate44100,
                ChannelConfig::Mono,
                4096,
                1024,
            )
            .unwrap();

        stack.open_pcm_stream(id).unwrap();

        let stream = stack.get_pcm_stream(id).unwrap();
        assert!(stream.opened);
    }

    #[test]
    fn test_start_stop_pcm_stream() {
        let mut stack = AlsaAudioStack::new();

        let id = stack
            .create_pcm_stream(
                "test".to_string(),
                AudioDirection::Playback,
                AudioFormat::S16Le,
                SampleRate::Rate48000,
                ChannelConfig::Stereo,
                4096,
                1024,
            )
            .unwrap();

        stack.open_pcm_stream(id).unwrap();
        stack.start_pcm_stream(id).unwrap();

        let stream = stack.get_pcm_stream(id).unwrap();
        assert!(stream.running);

        stack.stop_pcm_stream(id).unwrap();

        let stream = stack.get_pcm_stream(id).unwrap();
        assert!(!stream.running);
    }

    #[test]
    fn test_write_pcm() {
        let mut stack = AlsaAudioStack::new();

        let id = stack
            .create_pcm_stream(
                "playback".to_string(),
                AudioDirection::Playback,
                AudioFormat::S16Le,
                SampleRate::Rate48000,
                ChannelConfig::Stereo,
                4096,
                1024,
            )
            .unwrap();

        stack.open_pcm_stream(id).unwrap();
        stack.start_pcm_stream(id).unwrap();

        let data = vec![0u8; 512];
        let written = stack.write_pcm(id, &data).unwrap();

        assert_eq!(written, 512);
    }

    #[test]
    fn test_create_mixer_control() {
        let mut stack = AlsaAudioStack::new();

        let id = stack
            .create_mixer_control("Master".to_string(), 0, 100, 50)
            .unwrap();
        assert_eq!(stack.mixer_control_count(), 1);

        let control = stack.get_mixer_control(id).unwrap();
        assert_eq!(control.name, "Master");
        assert_eq!(control.current_value, 50);
    }

    #[test]
    fn test_set_mixer_value() {
        let mut stack = AlsaAudioStack::new();

        let id = stack
            .create_mixer_control("Volume".to_string(), 0, 100, 50)
            .unwrap();
        stack.set_mixer_value(id, 75).unwrap();

        let value = stack.get_mixer_value(id).unwrap();
        assert_eq!(value, 75);
    }

    #[test]
    fn test_set_mute() {
        let mut stack = AlsaAudioStack::new();

        let id = stack
            .create_mixer_control("Master".to_string(), 0, 100, 50)
            .unwrap();
        stack.set_mute(id, true).unwrap();

        let control = stack.get_mixer_control(id).unwrap();
        assert!(control.is_muted);
    }

    #[test]
    fn test_list_streams() {
        let mut stack = AlsaAudioStack::new();

        stack
            .create_pcm_stream(
                "stream1".to_string(),
                AudioDirection::Playback,
                AudioFormat::S16Le,
                SampleRate::Rate48000,
                ChannelConfig::Stereo,
                4096,
                1024,
            )
            .unwrap();
        stack
            .create_pcm_stream(
                "stream2".to_string(),
                AudioDirection::Capture,
                AudioFormat::S16Le,
                SampleRate::Rate44100,
                ChannelConfig::Mono,
                4096,
                1024,
            )
            .unwrap();

        let streams = stack.list_pcm_streams();
        assert_eq!(streams.len(), 2);
    }

    #[test]
    fn test_invalid_mixer_value() {
        let mut stack = AlsaAudioStack::new();

        let id = stack
            .create_mixer_control("Volume".to_string(), 0, 100, 50)
            .unwrap();
        let result = stack.set_mixer_value(id, 150);

        assert!(result.is_err());
    }
}
