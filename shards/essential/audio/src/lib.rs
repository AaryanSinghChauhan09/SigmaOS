// Sigma Audio Driver - Audio Subsystem Prototype
// Implements audio codec initialization and PCM playback
// No external dependencies - implementing from first principles

use std::fmt;

/// Audio device type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioDeviceType {
    Playback,
    Capture,
    Duplex,
}

impl AudioDeviceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            AudioDeviceType::Playback => "Playback",
            AudioDeviceType::Capture => "Capture",
            AudioDeviceType::Duplex => "Duplex",
        }
    }
}

/// Sample format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SampleFormat {
    U8,
    S16LE,
    S16BE,
    S24LE,
    S24BE,
    S32LE,
    S32BE,
    Float32LE,
    Float32BE,
}

impl SampleFormat {
    pub fn bytes_per_sample(&self) -> usize {
        match self {
            SampleFormat::U8 => 1,
            SampleFormat::S16LE | SampleFormat::S16BE => 2,
            SampleFormat::S24LE | SampleFormat::S24BE => 3,
            SampleFormat::S32LE | SampleFormat::S32BE => 4,
            SampleFormat::Float32LE | SampleFormat::Float32BE => 4,
        }
    }
    
    pub fn as_str(&self) -> &'static str {
        match self {
            SampleFormat::U8 => "U8",
            SampleFormat::S16LE => "S16_LE",
            SampleFormat::S16BE => "S16_BE",
            SampleFormat::S24LE => "S24_LE",
            SampleFormat::S24BE => "S24_BE",
            SampleFormat::S32LE => "S32_LE",
            SampleFormat::S32BE => "S32_BE",
            SampleFormat::Float32LE => "FLOAT_LE",
            SampleFormat::Float32BE => "FLOAT_BE",
        }
    }
}

/// Sample rate
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SampleRate {
    Hz8000,
    Hz11025,
    Hz16000,
    Hz22050,
    Hz44100,
    Hz48000,
    Hz96000,
    Hz192000,
}

impl SampleRate {
    pub fn as_u32(&self) -> u32 {
        match self {
            SampleRate::Hz8000 => 8000,
            SampleRate::Hz11025 => 11025,
            SampleRate::Hz16000 => 16000,
            SampleRate::Hz22050 => 22050,
            SampleRate::Hz44100 => 44100,
            SampleRate::Hz48000 => 48000,
            SampleRate::Hz96000 => 96000,
            SampleRate::Hz192000 => 192000,
        }
    }
    
    pub fn as_str(&self) -> &'static str {
        match self {
            SampleRate::Hz8000 => "8000 Hz",
            SampleRate::Hz11025 => "11025 Hz",
            SampleRate::Hz16000 => "16000 Hz",
            SampleRate::Hz22050 => "22050 Hz",
            SampleRate::Hz44100 => "44100 Hz",
            SampleRate::Hz48000 => "48000 Hz",
            SampleRate::Hz96000 => "96000 Hz",
            SampleRate::Hz192000 => "192000 Hz",
        }
    }
}

/// Channel count
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelCount {
    Mono,
    Stereo,
    Surround51,
    Surround71,
}

impl ChannelCount {
    pub fn as_u32(&self) -> u32 {
        match self {
            ChannelCount::Mono => 1,
            ChannelCount::Stereo => 2,
            ChannelCount::Surround51 => 6,
            ChannelCount::Surround71 => 8,
        }
    }
    
    pub fn as_str(&self) -> &'static str {
        match self {
            ChannelCount::Mono => "Mono",
            ChannelCount::Stereo => "Stereo",
            ChannelCount::Surround51 => "5.1 Surround",
            ChannelCount::Surround71 => "7.1 Surround",
        }
    }
}

/// Audio configuration
#[derive(Debug, Clone)]
pub struct AudioConfig {
    pub sample_format: SampleFormat,
    pub sample_rate: SampleRate,
    pub channels: ChannelCount,
}

impl AudioConfig {
    pub fn new(sample_format: SampleFormat, sample_rate: SampleRate, channels: ChannelCount) -> Self {
        AudioConfig {
            sample_format,
            sample_rate,
            channels,
        }
    }
    
    pub fn bytes_per_second(&self) -> u32 {
        self.sample_format.bytes_per_sample() as u32 * self.sample_rate.as_u32() * self.channels.as_u32()
    }
    
    pub fn frame_size(&self) -> usize {
        self.sample_format.bytes_per_sample() * self.channels.as_u32() as usize
    }
}

impl fmt::Display for AudioConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AudioConfig\n\
             Format: {}\n\
             Rate: {}\n\
             Channels: {}\n\
             Bytes/Second: {}\n\
             Frame Size: {}",
            self.sample_format.as_str(),
            self.sample_rate.as_str(),
            self.channels.as_str(),
            self.bytes_per_second(),
            self.frame_size()
        )
    }
}

/// Audio device
#[derive(Debug, Clone)]
pub struct AudioDevice {
    pub device_id: [u8; 32],
    pub name: String,
    pub device_type: AudioDeviceType,
    pub config: Option<AudioConfig>,
    pub initialized: bool,
    pub volume: u8,
}

impl AudioDevice {
    pub fn new(name: String, device_type: AudioDeviceType) -> Self {
        let device_id = Self::generate_device_id(&name, &device_type);
        
        AudioDevice {
            device_id,
            name,
            device_type,
            config: None,
            initialized: false,
            volume: 100,
        }
    }
    
    fn generate_device_id(name: &str, device_type: &AudioDeviceType) -> [u8; 32] {
        // Placeholder for actual hardware ID
        let mut hash = [0u8; 32];
        let name_bytes = name.as_bytes();
        for (i, &byte) in name_bytes.iter().enumerate() {
            hash[i % 32] = hash[i % 32].wrapping_add(byte);
        }
        let type_bytes = device_type.as_str().as_bytes();
        for (i, &byte) in type_bytes.iter().enumerate() {
            hash[(i + 16) % 32] = hash[(i + 16) % 32].wrapping_add(byte);
        }
        hash
    }
    
    pub fn get_device_id(&self) -> String {
        self.device_id.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join("")
    }
    
    pub fn initialize(&mut self, config: AudioConfig) -> Result<(), String> {
        if self.initialized {
            return Err("Device already initialized".to_string());
        }
        
        self.config = Some(config);
        self.initialized = true;
        
        Ok(())
    }
    
    pub fn set_volume(&mut self, volume: u8) -> Result<(), String> {
        if volume > 100 {
            return Err("Volume must be between 0 and 100".to_string());
        }
        
        self.volume = volume;
        Ok(())
    }
    
    pub fn get_volume(&self) -> u8 {
        self.volume
    }
    
    pub fn play_pcm(&self, data: &[u8]) -> Result<(), String> {
        if !self.initialized {
            return Err("Device not initialized".to_string());
        }
        
        if let Some(ref config) = self.config {
            let frame_size = config.frame_size();
            if data.len() % frame_size != 0 {
                return Err("Data size must be a multiple of frame size".to_string());
            }
        }
        
        // Simulate PCM playback
        Ok(())
    }
    
    pub fn capture_pcm(&self, buffer: &mut [u8]) -> Result<(), String> {
        if !self.initialized {
            return Err("Device not initialized".to_string());
        }
        
        if self.device_type == AudioDeviceType::Playback {
            return Err("Device does not support capture".to_string());
        }
        
        // Simulate PCM capture
        for byte in buffer.iter_mut() {
            *byte = 0;
        }
        
        Ok(())
    }
    
    pub fn get_info(&self) -> AudioDeviceInfo {
        AudioDeviceInfo {
            device_id: self.get_device_id(),
            name: self.name.clone(),
            device_type: self.device_type,
            config: self.config.clone(),
            initialized: self.initialized,
            volume: self.volume,
        }
    }
}

/// Audio device information
#[derive(Debug, Clone)]
pub struct AudioDeviceInfo {
    pub device_id: String,
    pub name: String,
    pub device_type: AudioDeviceType,
    pub config: Option<AudioConfig>,
    pub initialized: bool,
    pub volume: u8,
}

impl fmt::Display for AudioDeviceInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Audio Device Information\n\
             Device ID: {}\n\
             Name: {}\n\
             Type: {}\n\
             Initialized: {}\n\
             Volume: {}%",
            self.device_id,
            self.name,
            self.device_type.as_str(),
            self.initialized,
            self.volume
        )?;
        
        if let Some(ref config) = self.config {
            write!(f, "\n{}", config)?;
        }
        
        Ok(())
    }
}

/// Audio driver
pub struct AudioDriver {
    devices: Vec<AudioDevice>,
}

impl AudioDriver {
    pub fn new() -> Self {
        AudioDriver {
            devices: Vec::new(),
        }
    }
    
    /// Detect audio devices
    pub fn detect_devices(&mut self) {
        // Simulate device detection
        let playback = AudioDevice::new("default".to_string(), AudioDeviceType::Playback);
        let capture = AudioDevice::new("default_capture".to_string(), AudioDeviceType::Capture);
        
        self.devices.push(playback);
        self.devices.push(capture);
    }
    
    /// Get device by ID
    pub fn get_device(&self, device_id: &str) -> Option<&AudioDevice> {
        self.devices
            .iter()
            .find(|d| d.get_device_id() == device_id)
    }
    
    /// Get device by ID (mutable)
    pub fn get_device_mut(&mut self, device_id: &str) -> Option<&mut AudioDevice> {
        self.devices
            .iter_mut()
            .find(|d| d.get_device_id() == device_id)
    }
    
    /// Initialize device
    pub fn initialize_device(&mut self, device_id: &str, config: AudioConfig) -> Result<(), String> {
        let device = self.get_device_mut(device_id)
            .ok_or_else(|| "Device not found".to_string())?;
        
        device.initialize(config)
    }
    
    /// Set volume
    pub fn set_volume(&mut self, device_id: &str, volume: u8) -> Result<(), String> {
        let device = self.get_device_mut(device_id)
            .ok_or_else(|| "Device not found".to_string())?;
        
        device.set_volume(volume)
    }
    
    /// Play PCM data
    pub fn play_pcm(&self, device_id: &str, data: &[u8]) -> Result<(), String> {
        let device = self.get_device(device_id)
            .ok_or_else(|| "Device not found".to_string())?;
        
        device.play_pcm(data)
    }
    
    /// Capture PCM data
    pub fn capture_pcm(&self, device_id: &str, buffer: &mut [u8]) -> Result<(), String> {
        let device = self.get_device(device_id)
            .ok_or_else(|| "Device not found".to_string())?;
        
        device.capture_pcm(buffer)
    }
    
    /// List all devices
    pub fn list_devices(&self) -> Vec<&AudioDevice> {
        self.devices.iter().collect()
    }
    
    /// Get device count
    pub fn device_count(&self) -> usize {
        self.devices.len()
    }
}

impl Default for AudioDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_audio_device_creation() {
        let device = AudioDevice::new("default".to_string(), AudioDeviceType::Playback);
        assert_eq!(device.name, "default");
        assert_eq!(device.device_type, AudioDeviceType::Playback);
        assert!(!device.initialized);
    }
    
    #[test]
    fn test_audio_config() {
        let config = AudioConfig::new(SampleFormat::S16LE, SampleRate::Hz44100, ChannelCount::Stereo);
        assert_eq!(config.sample_format, SampleFormat::S16LE);
        assert_eq!(config.sample_rate, SampleRate::Hz44100);
        assert_eq!(config.channels, ChannelCount::Stereo);
    }
    
    #[test]
    fn test_device_initialization() {
        let mut device = AudioDevice::new("default".to_string(), AudioDeviceType::Playback);
        let config = AudioConfig::new(SampleFormat::S16LE, SampleRate::Hz44100, ChannelCount::Stereo);
        
        assert!(device.initialize(config).is_ok());
        assert!(device.initialized);
    }
    
    #[test]
    fn test_volume_control() {
        let mut device = AudioDevice::new("default".to_string(), AudioDeviceType::Playback);
        
        assert!(device.set_volume(50).is_ok());
        assert_eq!(device.get_volume(), 50);
        
        assert!(device.set_volume(150).is_err());
    }
    
    #[test]
    fn test_pcm_playback() {
        let mut device = AudioDevice::new("default".to_string(), AudioDeviceType::Playback);
        let config = AudioConfig::new(SampleFormat::S16LE, SampleRate::Hz44100, ChannelCount::Stereo);
        device.initialize(config).unwrap();
        
        let data = vec![0u8; 4096];
        assert!(device.play_pcm(&data).is_ok());
    }
    
    #[test]
    fn test_audio_driver() {
        let mut driver = AudioDriver::new();
        driver.detect_devices();
        
        assert_eq!(driver.device_count(), 2);
        
        let devices = driver.list_devices();
        assert_eq!(devices.len(), 2);
    }
}
