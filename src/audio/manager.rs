//! SigmaOS Audio Manager
//!
//! Inspired by: Omarchy audio controls (omancy-audio-*, wireplumber, pipewire)
//! Following principles: SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY,
//! LAYERED DESIGN, KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING,
//! SECURE_DEFAULTS, DIRECT_DEVICE_CONTROL, LOW_LEVEL_CONTROL, LOW_LATENCY,
//! ZERO_COPY, EFFICIENT_SCHEDULING, SAFE_CONCURRENCY, RACE_FREE DESIGN.

use std::collections::HashMap;

/// Audio device type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AudioDeviceType {
    Input,
    Output,
}

/// Audio device
#[derive(Debug, Clone)]
pub struct AudioDevice {
    pub id: String,
    pub name: String,
    pub device_type: AudioDeviceType,
    pub is_available: bool,
    pub is_default: bool,
    pub volume: f32,
    pub is_muted: bool,
    pub supported_formats: Vec<String>,
}

/// Audio sink
#[derive(Debug, Clone)]
pub struct AudioSink {
    pub id: String,
    pub name: String,
    pub volume: f32,
    pub is_muted: bool,
    pub is_default: bool,
}

/// Audio source
#[derive(Debug, Clone)]
pub struct AudioSource {
    pub id: String,
    pub name: String,
    pub volume: f32,
    pub is_muted: bool,
    pub is_default: bool,
}

/// Audio manager
pub struct AudioManager {
    pub sinks: HashMap<String, AudioSink>,
    pub sources: HashMap<String, AudioSource>,
    pub active_sink: Option<String>,
    pub active_source: Option<String>,
    pub master_volume: f32,
    pub is_muted: bool,
}

impl AudioManager {
    pub fn new() -> Self {
        Self {
            sinks: HashMap::new(),
            sources: HashMap::new(),
            active_sink: None,
            active_source: None,
            master_volume: 1.0,
            is_muted: false,
        }
    }

    /// Add an audio sink
    pub fn add_sink(&mut self, sink: AudioSink) {
        let id = sink.id.clone();
        self.sinks.insert(id, sink);
        if self.active_sink.is_none() {
            self.active_sink = self.sinks.keys().next().cloned();
        }
    }

    /// Add an audio source
    pub fn add_source(&mut self, source: AudioSource) {
        let id = source.id.clone();
        self.sources.insert(id, source);
        if self.active_source.is_none() {
            self.active_source = self.sources.keys().next().cloned();
        }
    }

    /// Set default sink
    pub fn set_default_sink(&mut self, id: &str) -> Result<(), String> {
        if !self.sinks.contains_key(id) {
            return Err(format!("Sink {} not found", id));
        }
        self.active_sink = Some(id.to_string());
        Ok(())
    }

    /// Set default source
    pub fn set_default_source(&mut self, id: &str) -> Result<(), String> {
        if !self.sources.contains_key(id) {
            return Err(format!("Source {} not found", id));
        }
        self.active_source = Some(id.to_string());
        Ok(())
    }

    /// Set sink volume
    pub fn set_sink_volume(&mut self, id: &str, volume: f32) -> Result<(), String> {
        if volume < 0.0 || volume > 2.0 {
            return Err("Volume must be between 0.0 and 2.0".to_string());
        }
        let sink = self
            .sinks
            .get_mut(id)
            .ok_or_else(|| format!("Sink {} not found", id))?;
        sink.volume = volume;
        Ok(())
    }

    /// Set source volume
    pub fn set_source_volume(&mut self, id: &str, volume: f32) -> Result<(), String> {
        if volume < 0.0 || volume > 2.0 {
            return Err("Volume must be between 0.0 and 2.0".to_string());
        }
        let source = self
            .sources
            .get_mut(id)
            .ok_or_else(|| format!("Source {} not found", id))?;
        source.volume = volume;
        Ok(())
    }

    /// Toggle mute for sink
    pub fn toggle_sink_mute(&mut self, id: &str) -> Result<(), String> {
        let sink = self
            .sinks
            .get_mut(id)
            .ok_or_else(|| format!("Sink {} not found", id))?;
        sink.is_muted = !sink.is_muted;
        Ok(sink.is_muted)
    }

    /// Toggle mute for source
    pub fn toggle_source_mute(&mut self, id: &str) -> Result<(), String> {
        let source = self
            .sources
            .get_mut(id)
            .ok_or_else(|| format!("Source {} not found", id))?;
        source.is_muted = !source.is_muted;
        Ok(source.is_muted)
    }

    /// Set master volume
    pub fn set_master_volume(&mut self, volume: f32) -> Result<(), String> {
        if volume < 0.0 || volume > 2.0 {
            return Err("Master volume must be between 0.0 and 2.0".to_string());
        }
        self.master_volume = volume;
        Ok(())
    }

    /// Toggle master mute
    pub fn toggle_master_mute(&mut self) -> bool {
        self.is_muted = !self.is_muted;
        self.is_muted
    }

    /// Get active sink
    pub fn get_active_sink(&self) -> Option<&AudioSink> {
        self.active_sink.as_ref().and_then(|id| self.sinks.get(id))
    }

    /// Get active source
    pub fn get_active_source(&self) -> Option<&AudioSource> {
        self.active_source.as_ref().and_then(|id| self.sources.get(id))
    }

    /// List sinks
    pub fn list_sinks(&self) -> Vec<&AudioSink> {
        self.sinks.values().collect()
    }

    /// List sources
    pub fn list_sources(&self) -> Vec<&AudioSource> {
        self.sources.values().collect()
    }

    /// Sink count
    pub fn sink_count(&self) -> usize {
        self.sinks.len()
    }

    /// Source count
    pub fn source_count(&self) -> usize {
        self.sources.len()
    }
}

impl Default for AudioManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_sink() {
        let mut mgr = AudioManager::new();
        let sink = AudioSink {
            id: "sink0".to_string(),
            name: "Built-in Audio".to_string(),
            volume: 0.8,
            is_muted: false,
            is_default: true,
        };
        mgr.add_sink(sink);
        assert_eq!(mgr.sink_count(), 1);
        assert!(mgr.get_active_sink().is_some());
    }

    #[test]
    fn test_add_source() {
        let mut mgr = AudioManager::new();
        let source = AudioSource {
            id: "source0".to_string(),
            name: "Microphone".to_string(),
            volume: 0.5,
            is_muted: false,
            is_default: true,
        };
        mgr.add_source(source);
        assert_eq!(mgr.source_count(), 1);
    }

    #[test]
    fn test_set_default_sink() {
        let mut mgr = AudioManager::new();
        mgr.add_sink(AudioSink {
            id: "sink0".to_string(),
            name: "Sink 0".to_string(),
            volume: 0.8,
            is_muted: false,
            is_default: false,
        });
        mgr.add_sink(AudioSink {
            id: "sink1".to_string(),
            name: "Sink 1".to_string(),
            volume: 0.6,
            is_muted: false,
            is_default: false,
        });
        assert!(mgr.set_default_sink("sink1").is_ok());
        assert_eq!(mgr.get_active_sink().unwrap().id, "sink1");
    }

    #[test]
    fn test_set_volume() {
        let mut mgr = AudioManager::new();
        mgr.add_sink(AudioSink {
            id: "sink0".to_string(),
            name: "Sink 0".to_string(),
            volume: 0.8,
            is_muted: false,
            is_default: true,
        });
        assert!(mgr.set_sink_volume("sink0", 0.5).is_ok());
        assert_eq!(mgr.get_active_sink().unwrap().volume, 0.5);
    }

    #[test]
    fn test_volume_out_of_range() {
        let mut mgr = AudioManager::new();
        mgr.add_sink(AudioSink {
            id: "sink0".to_string(),
            name: "Sink 0".to_string(),
            volume: 0.8,
            is_muted: false,
            is_default: true,
        });
        assert!(mgr.set_sink_volume("sink0", 2.5).is_err());
    }

    #[test]
    fn test_toggle_mute() {
        let mut mgr = AudioManager::new();
        mgr.add_sink(AudioSink {
            id: "sink0".to_string(),
            name: "Sink 0".to_string(),
            volume: 0.8,
            is_muted: false,
            is_default: true,
        });
        assert!(mgr.toggle_sink_mute("sink0").unwrap());
        assert!(mgr.get_active_sink().unwrap().is_muted);
    }

    #[test]
    fn test_master_volume() {
        let mut mgr = AudioManager::new();
        assert!(mgr.set_master_volume(0.5).is_ok());
        assert_eq!(mgr.master_volume, 0.5);
        assert!(mgr.set_master_volume(3.0).is_err());
    }

    #[test]
    fn test_master_mute() {
        let mut mgr = AudioManager::new();
        assert!(!mgr.is_muted);
        assert!(mgr.toggle_master_mute());
        assert!(mgr.is_muted);
    }
}