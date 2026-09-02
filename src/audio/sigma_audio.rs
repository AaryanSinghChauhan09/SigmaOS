//! Audio/Media System (PipeWire/Jack2 Inspiration)
//! Professional audio graph with low-latency processing and device management
extern crate alloc;



use alloc::vec::Vec;
use alloc::string::{String, ToString};

/// Audio node type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioNodeType {
    Source,
    Sink,
    Filter,
    Mixer,
    Router,
}

/// Audio format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioFormat {
    S16LE,
    S24LE,
    S32LE,
    F32LE,
    F64LE,
}

/// Audio node
#[derive(Debug, Clone)]
pub struct AudioNode {
    pub id: String,
    pub name: String,
    pub node_type: AudioNodeType,
    pub format: AudioFormat,
    pub sample_rate: u32,
    pub channels: u32,
    pub state: NodeState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeState {
    Idle,
    Running,
    Suspended,
    Error,
}

impl AudioNode {
    pub fn new(name: &str, node_type: AudioNodeType, format: AudioFormat) -> Self {
        Self {
            id: Self::generate_id(),
            name: name.to_string(),
            node_type,
            format,
            sample_rate: 48000,
            channels: 2,
            state: NodeState::Idle,
        }
    }

    fn generate_id() -> String {
        "node_abcdef1234567890".to_string()
    }

    pub fn set_sample_rate(&mut self, rate: u32) {
        self.sample_rate = rate;
    }

    pub fn set_channels(&mut self, channels: u32) {
        self.channels = channels;
    }

    pub fn start(&mut self) -> Result<(), AudioError> {
        self.state = NodeState::Running;
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), AudioError> {
        self.state = NodeState::Idle;
        Ok(())
    }

    pub fn suspend(&mut self) -> Result<(), AudioError> {
        self.state = NodeState::Suspended;
        Ok(())
    }
}

/// Audio link
#[derive(Debug, Clone)]
pub struct AudioLink {
    pub id: String,
    pub source_node: String,
    pub sink_node: String,
    pub state: LinkState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkState {
    Active,
    Inactive,
    Error,
}

impl AudioLink {
    pub fn new(source: &str, sink: &str) -> Self {
        Self {
            id: Self::generate_id(),
            source_node: source.to_string(),
            sink_node: sink.to_string(),
            state: LinkState::Active,
        }
    }

    fn generate_id() -> String {
        "link_abcdef1234567890".to_string()
    }

    pub fn activate(&mut self) {
        self.state = LinkState::Active;
    }

    pub fn deactivate(&mut self) {
        self.state = LinkState::Inactive;
    }
}

/// Audio graph state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphState {
    Running,
    Suspended,
    Error,
}

/// Audio graph
pub struct AudioGraph {
    pub nodes: Vec<AudioNode>,
    pub links: Vec<AudioLink>,
    pub graph_state: GraphState,
}

impl AudioGraph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            links: Vec::new(),
            graph_state: GraphState::Suspended,
        }
    }

    pub fn add_node(&mut self, node: AudioNode) {
        self.nodes.push(node);
    }

    pub fn get_node(&mut self, id: &str) -> Option<&mut AudioNode> {
        self.nodes.iter_mut().find(|n| n.id == id || n.name == id)
    }

    pub fn add_link(&mut self, link: AudioLink) {
        self.links.push(link);
    }

    pub fn remove_link(&mut self, id: &str) -> Result<(), AudioError> {
        self.links.retain(|l| l.id != id);
        Ok(())
    }

    pub fn start(&mut self) -> Result<(), AudioError> {
        for node in &mut self.nodes {
            node.start()?;
        }
        self.graph_state = GraphState::Running;
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), AudioError> {
        for node in &mut self.nodes {
            node.stop()?;
        }
        self.graph_state = GraphState::Suspended;
        Ok(())
    }

    pub fn suspend(&mut self) -> Result<(), AudioError> {
        for node in &mut self.nodes {
            node.suspend()?;
        }
        self.graph_state = GraphState::Suspended;
        Ok(())
    }
}

/// Audio device
#[derive(Debug, Clone)]
pub struct AudioDevice {
    pub id: String,
    pub name: String,
    pub device_type: DeviceType,
    pub profiles: Vec<AudioProfile>,
    pub active_profile: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    Playback,
    Capture,
    Duplex,
}

#[derive(Debug, Clone)]
pub struct AudioProfile {
    pub name: String,
    pub format: AudioFormat,
    pub sample_rate: u32,
    pub channels: u32,
}

impl AudioDevice {
    pub fn new(name: &str, device_type: DeviceType) -> Self {
        Self {
            id: Self::generate_id(),
            name: name.to_string(),
            device_type,
            profiles: Vec::new(),
            active_profile: None,
        }
    }

    fn generate_id() -> String {
        "device_abcdef1234567890".to_string()
    }

    pub fn add_profile(&mut self, profile: AudioProfile) {
        self.profiles.push(profile);
    }

    pub fn set_profile(&mut self, profile_name: &str) -> Result<(), AudioError> {
        if self.profiles.iter().any(|p| p.name == profile_name) {
            self.active_profile = Some(profile_name.to_string());
            Ok(())
        } else {
            Err(AudioError::ProfileNotFound)
        }
    }
}

/// Ubuntu `indicator-sound` Sound Theme Spec Event Types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoundEventType {
    SystemBoot,
    SystemLogout,
    DevicePlug,
    DeviceUnplug,
    VolumeChangeFeedback,
    NotificationAlert,
    BatteryLow,
}

/// Sound Event Feedback Theme Dispatcher
pub struct SoundEventThemeDispatcher {
    pub current_theme: String,
    pub feedback_enabled: bool,
}

impl SoundEventThemeDispatcher {
    pub fn new(theme_name: &str) -> Self {
        Self {
            current_theme: theme_name.to_string(),
            feedback_enabled: true,
        }
    }

    pub fn get_sound_file_uri(&self, event: SoundEventType) -> String {
        let file_name = match event {
            SoundEventType::SystemBoot => "desktop-login.ogg",
            SoundEventType::SystemLogout => "desktop-logout.ogg",
            SoundEventType::DevicePlug => "device-added.ogg",
            SoundEventType::DeviceUnplug => "device-removed.ogg",
            SoundEventType::VolumeChangeFeedback => "audio-volume-change.ogg",
            SoundEventType::NotificationAlert => "message-new-instant.ogg",
            SoundEventType::BatteryLow => "battery-low.ogg",
        };
        format!("/usr/share/sounds/{}/stereo/{}", self.current_theme, file_name)
    }
}

/// Ubuntu `indicator-sound` Canonical Sound Indicator Controller
pub struct IndicatorSoundController {
    pub master_volume_percent: u8,
    pub master_muted: bool,
    pub allow_overamplification: bool, // Up to 150% volume boost
}

impl IndicatorSoundController {
    pub fn new() -> Self {
        Self {
            master_volume_percent: 80,
            master_muted: false,
            allow_overamplification: true,
        }
    }

    pub fn set_master_volume(&mut self, volume: u8) {
        let max_vol = if self.allow_overamplification { 150 } else { 100 };
        self.master_volume_percent = volume.min(max_vol);
    }

    pub fn toggle_mute(&mut self) {
        self.master_muted = !self.master_muted;
    }
}

/// Per-App Volume & Mute Overrides (PulseAudio / PipeWire pavucontrol)
#[derive(Debug, Clone)]
pub struct AppVolumeControl {
    pub app_name: String,
    pub volume_percent: u8,
    pub is_muted: bool,
}

impl AppVolumeControl {
    pub fn new(app_name: &str) -> Self {
        Self {
            app_name: app_name.to_string(),
            volume_percent: 100,
            is_muted: false,
        }
    }
}

/// PulseAudio / PipeWire Peak Meter & Over-Amplification Warning
pub struct AudioPeakMeter {
    pub current_peak_db: f32,
    pub clipping_warning: bool,
}

impl AudioPeakMeter {
    pub fn new() -> Self {
        Self {
            current_peak_db: -60.0,
            clipping_warning: false,
        }
    }

    pub fn update_peak(&mut self, peak_db: f32) {
        self.current_peak_db = peak_db;
        self.clipping_warning = peak_db > 0.0; // Above 0 dB causes digital clipping
    }
}

/// Audio session
#[derive(Debug, Clone)]
pub struct AudioSession {
    pub id: String,
    pub name: String,
    pub client: String,
    pub state: SessionState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    Active,
    Inactive,
    Suspended,
}

impl AudioSession {
    pub fn new(name: &str, client: &str) -> Self {
        Self {
            id: Self::generate_id(),
            name: name.to_string(),
            client: client.to_string(),
            state: SessionState::Active,
        }
    }

    fn generate_id() -> String {
        "session_abcdef1234567890".to_string()
    }

    pub fn activate(&mut self) {
        self.state = SessionState::Active;
    }

    pub fn suspend(&mut self) {
        self.state = SessionState::Suspended;
    }
}

/// SigmaAudio - Professional Audio System
pub struct SigmaAudio {
    pub graph: AudioGraph,
    pub devices: Vec<AudioDevice>,
    pub sessions: Vec<AudioSession>,
}

impl SigmaAudio {
    pub fn new() -> Self {
        Self {
            graph: AudioGraph::new(),
            devices: Vec::new(),
            sessions: Vec::new(),
        }
    }

    pub fn add_device(&mut self, device: AudioDevice) {
        self.devices.push(device);
    }

    pub fn get_device(&mut self, id: &str) -> Option<&mut AudioDevice> {
        self.devices.iter_mut().find(|d| d.id == id || d.name == id)
    }

    pub fn add_session(&mut self, session: AudioSession) {
        self.sessions.push(session);
    }

    pub fn get_session(&mut self, id: &str) -> Option<&mut AudioSession> {
        self.sessions.iter_mut().find(|s| s.id == id || s.name == id)
    }

    pub fn start_audio(&mut self) -> Result<(), AudioError> {
        self.graph.start()
    }

    pub fn stop_audio(&mut self) -> Result<(), AudioError> {
        self.graph.stop()
    }

    pub fn suspend_audio(&mut self) -> Result<(), AudioError> {
        self.graph.suspend()
    }

    pub fn get_audio_stats(&self) -> AudioStats {
        AudioStats {
            total_nodes: self.graph.nodes.len(),
            active_nodes: self.graph.nodes.iter().filter(|n| n.state == NodeState::Running).count(),
            total_links: self.graph.links.len(),
            active_links: self.graph.links.iter().filter(|l| l.state == LinkState::Active).count(),
            total_devices: self.devices.len(),
            total_sessions: self.sessions.len(),
            active_sessions: self.sessions.iter().filter(|s| s.state == SessionState::Active).count(),
        }
    }

    pub fn list_devices(&self) -> Vec<&AudioDevice> {
        self.devices.iter().collect()
    }
}

#[derive(Debug, Clone)]
pub struct AudioStats {
    pub total_nodes: usize,
    pub active_nodes: usize,
    pub total_links: usize,
    pub active_links: usize,
    pub total_devices: usize,
    pub total_sessions: usize,
    pub active_sessions: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AudioError {
    NodeNotFound,
    LinkNotFound,
    DeviceNotFound,
    SessionNotFound,
    ProfileNotFound,
    StartFailed,
    StopFailed,
}

impl Default for SigmaAudio {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_node() {
        let node = AudioNode::new("test-node", AudioNodeType::Source, AudioFormat::S16LE);
        assert_eq!(node.name, "test-node");
        assert_eq!(node.node_type, AudioNodeType::Source);
    }

    #[test]
    fn test_audio_link() {
        let link = AudioLink::new("source", "sink");
        assert_eq!(link.source_node, "source");
        assert_eq!(link.sink_node, "sink");
    }

    #[test]
    fn test_audio_graph() {
        let mut graph = AudioGraph::new();
        let node = AudioNode::new("test", AudioNodeType::Source, AudioFormat::S16LE);
        graph.add_node(node);
        assert_eq!(graph.nodes.len(), 1);
    }

    #[test]
    fn test_audio_device() {
        let device = AudioDevice::new("test-device", DeviceType::Playback);
        assert_eq!(device.name, "test-device");
    }

    #[test]
    fn test_audio_session() {
        let session = AudioSession::new("test-session", "test-client");
        assert_eq!(session.name, "test-session");
    }

    #[test]
    fn test_sigmaaudio() {
        let mut audio = SigmaAudio::new();
        let device = AudioDevice::new("test-device", DeviceType::Playback);
        audio.add_device(device);
        assert_eq!(audio.list_devices().len(), 1);
    }

    #[test]
    fn test_indicator_sound_controller() {
        let mut ctrl = IndicatorSoundController::new();
        ctrl.set_master_volume(120);
        assert_eq!(ctrl.master_volume_percent, 120);
        ctrl.toggle_mute();
        assert!(ctrl.master_muted);
    }

    #[test]
    fn test_sound_event_theme_dispatcher() {
        let dispatcher = SoundEventThemeDispatcher::new("ubuntu");
        let uri = dispatcher.get_sound_file_uri(SoundEventType::SystemBoot);
        assert_eq!(uri, "/usr/share/sounds/ubuntu/stereo/desktop-login.ogg");
    }

    #[test]
    fn test_app_volume_control() {
        let app_vol = AppVolumeControl::new("VLC");
        assert_eq!(app_vol.app_name, "VLC");
        assert_eq!(app_vol.volume_percent, 100);
    }

    #[test]
    fn test_audio_peak_meter() {
        let mut meter = AudioPeakMeter::new();
        assert!(!meter.clipping_warning);
        meter.update_peak(3.5);
        assert!(meter.clipping_warning);
    }
}