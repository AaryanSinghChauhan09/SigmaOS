//! SigmaOS Indicator Sound
//!
//! Panel sound-indicator applet: exposes volume state, mute toggle,
//! and device selection. Wraps PipeWire/WirePlumber via an in-memory
//! model for testing; real integration bridges to PipeWire D-Bus.
//! Upstream: GNOME volume indicator, waybar/pulseaudio module

/// Audio sink (output device).
#[derive(Debug, Clone)]
pub struct AudioSink {
    pub id:          u32,
    pub name:        String,
    pub description: String,
    pub volume:      f32,   // 0.0 – 1.0
    pub muted:       bool,
    pub is_default:  bool,
}

impl AudioSink {
    pub fn new(id: u32, name: &str, description: &str) -> Self {
        Self {
            id,
            name:        name.to_owned(),
            description: description.to_owned(),
            volume:      0.8,
            muted:       false,
            is_default:  false,
        }
    }

    /// Clamp volume to [0.0, 1.5] (PipeWire allows >100% boost).
    pub fn set_volume(&mut self, v: f32) {
        self.volume = v.max(0.0).min(1.5);
    }

    /// Icon name appropriate to current state.
    pub fn icon(&self) -> &'static str {
        if self.muted || self.volume < 0.01 {
            return "audio-volume-muted-symbolic";
        }
        if self.volume < 0.34 { "audio-volume-low-symbolic"    }
        else if self.volume < 0.67 { "audio-volume-medium-symbolic" }
        else { "audio-volume-high-symbolic" }
    }

    /// Label shown in the panel indicator (e.g. "80%").
    pub fn label(&self) -> String {
        if self.muted { "🔇 Muted".to_owned() }
        else { format!("🔊 {}%", (self.volume * 100.0).round() as u32) }
    }
}

/// The in-memory audio mixer state (PipeWire adapter stub).
pub struct SoundIndicator {
    sinks:          Vec<AudioSink>,
    default_sink_id: Option<u32>,
}

impl SoundIndicator {
    pub fn new() -> Self {
        let mut indicator = Self { sinks: Vec::new(), default_sink_id: None };
        // Populate with two common devices for scaffold demos
        let mut builtin = AudioSink::new(1, "alsa_output.pci-0000_00_1f.3", "Built-in Audio");
        builtin.is_default = true;
        indicator.sinks.push(builtin);
        indicator.sinks.push(AudioSink::new(2, "bluez_sink.AA_BB_CC", "Bluetooth Headphones"));
        indicator.default_sink_id = Some(1);
        indicator
    }

    fn default_mut(&mut self) -> Option<&mut AudioSink> {
        let id = self.default_sink_id?;
        self.sinks.iter_mut().find(|s| s.id == id)
    }

    pub fn default_sink(&self) -> Option<&AudioSink> {
        let id = self.default_sink_id?;
        self.sinks.iter().find(|s| s.id == id)
    }

    pub fn set_volume(&mut self, v: f32) -> bool {
        match self.default_mut() {
            Some(s) => { s.set_volume(v); true }
            None    => false,
        }
    }

    pub fn adjust_volume(&mut self, delta: f32) -> bool {
        match self.default_mut() {
            Some(s) => { let v = s.volume + delta; s.set_volume(v); true }
            None    => false,
        }
    }

    pub fn toggle_mute(&mut self) -> bool {
        match self.default_mut() {
            Some(s) => { s.muted = !s.muted; true }
            None    => false,
        }
    }

    pub fn set_default(&mut self, id: u32) -> bool {
        if self.sinks.iter().any(|s| s.id == id) {
            for s in &mut self.sinks { s.is_default = s.id == id; }
            self.default_sink_id = Some(id);
            true
        } else {
            false
        }
    }

    pub fn sinks(&self) -> &[AudioSink] { &self.sinks }

    /// Panel badge label.
    pub fn badge(&self) -> String {
        self.default_sink().map(|s| s.label()).unwrap_or_else(|| "🔇".to_owned())
    }
}

impl Default for SoundIndicator {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_label() {
        let ind = SoundIndicator::new();
        let lbl = ind.badge();
        assert!(lbl.contains("80%"), "expected 80% in label, got: {lbl}");
    }

    #[test]
    fn set_volume_clamped() {
        let mut ind = SoundIndicator::new();
        ind.set_volume(2.0);
        assert!((ind.default_sink().unwrap().volume - 1.5).abs() < f32::EPSILON);
        ind.set_volume(-1.0);
        assert!((ind.default_sink().unwrap().volume).abs() < f32::EPSILON);
    }

    #[test]
    fn mute_toggle() {
        let mut ind = SoundIndicator::new();
        assert!(!ind.default_sink().unwrap().muted);
        ind.toggle_mute();
        assert!(ind.default_sink().unwrap().muted);
        assert!(ind.badge().contains("Muted"));
        ind.toggle_mute();
        assert!(!ind.default_sink().unwrap().muted);
    }

    #[test]
    fn set_default_sink() {
        let mut ind = SoundIndicator::new();
        assert!(ind.set_default(2));
        assert_eq!(ind.default_sink().unwrap().id, 2);
        assert!(!ind.set_default(99));
    }

    #[test]
    fn icon_names() {
        let mut sink = AudioSink::new(1, "test", "Test");
        sink.volume = 0.0;  assert_eq!(sink.icon(), "audio-volume-muted-symbolic");
        sink.volume = 0.2;  assert_eq!(sink.icon(), "audio-volume-low-symbolic");
        sink.volume = 0.5;  assert_eq!(sink.icon(), "audio-volume-medium-symbolic");
        sink.volume = 0.9;  assert_eq!(sink.icon(), "audio-volume-high-symbolic");
    }
}
