use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

// =========================================================================
// 1. EXISTING GSTREAMER / PULSEAUDIO PIPELINE TYPES
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioSinkBackend {
    PipeWire,
    PulseAudio,
    AlsaDirect,
    Ossv4BSD,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GstHardwareDecoder {
    Vaapi,
    Nvdec,
    V4l2Codecs,
    SoftwareFallback,
}

#[derive(Debug, Clone)]
pub struct MediaStreamPipeline {
    pub pipeline_id: u64,
    pub uri: String,
    pub sink_backend: AudioSinkBackend,
    pub hw_decoder: GstHardwareDecoder,
    pub is_playing: bool,
    pub buffer_level_percent: u32,
    pub latency_ms: u32,
}

pub struct GStreamerPulseAudioPipeline {
    pub active_pipelines: BTreeMap<u64, MediaStreamPipeline>,
    pub default_audio_sink: AudioSinkBackend,
    pub next_pipeline_id: u64,
}

impl GStreamerPulseAudioPipeline {
    pub fn new(default_sink: AudioSinkBackend) -> Self {
        Self {
            active_pipelines: BTreeMap::new(),
            default_audio_sink: default_sink,
            next_pipeline_id: 1,
        }
    }

    pub fn play_uri(&mut self, uri: &str, decoder: GstHardwareDecoder) -> u64 {
        let id = self.next_pipeline_id;
        self.next_pipeline_id += 1;

        let pipeline = MediaStreamPipeline {
            pipeline_id: id,
            uri: uri.to_string(),
            sink_backend: self.default_audio_sink,
            hw_decoder: decoder,
            is_playing: true,
            buffer_level_percent: 100,
            latency_ms: 12,
        };

        self.active_pipelines.insert(id, pipeline);
        id
    }

    pub fn set_audio_sink(
        &mut self,
        pipeline_id: u64,
        sink: AudioSinkBackend,
    ) -> Result<(), String> {
        let pipe = self
            .active_pipelines
            .get_mut(&pipeline_id)
            .ok_or_else(|| format!("Pipeline ID {} not found", pipeline_id))?;
        pipe.sink_backend = sink;
        Ok(())
    }

    pub fn stop_pipeline(&mut self, pipeline_id: u64) -> Result<(), String> {
        let pipe = self
            .active_pipelines
            .get_mut(&pipeline_id)
            .ok_or_else(|| format!("Pipeline ID {} not found", pipeline_id))?;
        pipe.is_playing = false;
        Ok(())
    }
}

impl Default for GStreamerPulseAudioPipeline {
    fn default() -> Self {
        Self::new(AudioSinkBackend::PipeWire)
    }
}

// =========================================================================
// 2. EXISTING MPV & BSD SNDIO ENGINE TYPES
// =========================================================================

#[derive(Debug, Clone)]
pub struct MpvAudioTrack {
    pub track_id: usize,
    pub language: String,
    pub codec: String,
    pub channels: u32,
    pub sample_rate_hz: u32,
}

pub struct MpvFreeBsdSndioEngine {
    pub sndio_device_name: String,
    pub audio_tracks: Vec<MpvAudioTrack>,
    pub selected_track_id: Option<usize>,
    pub volume_level: u32, // 0 to 100
    pub is_sndio_connected: bool,
}

impl MpvFreeBsdSndioEngine {
    pub fn new(sndio_device: &str) -> Self {
        Self {
            sndio_device_name: sndio_device.to_string(),
            audio_tracks: Vec::new(),
            selected_track_id: None,
            volume_level: 80,
            is_sndio_connected: true,
        }
    }

    pub fn add_audio_track(&mut self, lang: &str, codec: &str, channels: u32, rate: u32) -> usize {
        let id = self.audio_tracks.len() + 1;
        let track = MpvAudioTrack {
            track_id: id,
            language: lang.to_string(),
            codec: codec.to_string(),
            channels,
            sample_rate_hz: rate,
        };
        self.audio_tracks.push(track);
        if self.selected_track_id.is_none() {
            self.selected_track_id = Some(id);
        }
        id
    }

    pub fn select_track(&mut self, track_id: usize) -> Result<(), String> {
        if self.audio_tracks.iter().any(|t| t.track_id == track_id) {
            self.selected_track_id = Some(track_id);
            Ok(())
        } else {
            Err(format!("Audio track ID {} not found", track_id))
        }
    }

    pub fn set_sndio_volume(&mut self, vol: u32) {
        self.volume_level = vol.min(100);
    }
}

impl Default for MpvFreeBsdSndioEngine {
    fn default() -> Self {
        Self::new("snd/0")
    }
}

// =========================================================================
// 3. EXISTING FFMPEG ENCODER TYPES
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FfmpegHwEncoderBackend {
    VaapiIntelGpu,
    NvencNvidia,
    QuickSyncVideo,
    SoftwareLibx264,
}

#[derive(Debug, Clone)]
pub struct EncodingSessionSpec {
    pub session_id: u64,
    pub input_format: String,
    pub output_codec: String,
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub bitrate_kbps: u32,
    pub backend: FfmpegHwEncoderBackend,
    pub frames_encoded: u64,
}

pub struct FfmpegZeroCopyEncoder {
    pub active_sessions: BTreeMap<u64, EncodingSessionSpec>,
    pub next_session_id: u64,
}

impl FfmpegZeroCopyEncoder {
    pub fn new() -> Self {
        Self {
            active_sessions: BTreeMap::new(),
            next_session_id: 1,
        }
    }

    pub fn create_session(
        &mut self,
        input_fmt: &str,
        output_codec: &str,
        w: u32,
        h: u32,
        fps: u32,
        bitrate: u32,
        backend: FfmpegHwEncoderBackend,
    ) -> u64 {
        let id = self.next_session_id;
        self.next_session_id += 1;

        let session = EncodingSessionSpec {
            session_id: id,
            input_format: input_fmt.to_string(),
            output_codec: output_codec.to_string(),
            width: w,
            height: h,
            fps,
            bitrate_kbps: bitrate,
            backend,
            frames_encoded: 0,
        };

        self.active_sessions.insert(id, session);
        id
    }

    pub fn encode_frame_batch(&mut self, session_id: u64, frame_count: u64) -> Result<u64, String> {
        let session = self
            .active_sessions
            .get_mut(&session_id)
            .ok_or_else(|| format!("Session ID {} not found", session_id))?;
        session.frames_encoded += frame_count;
        Ok(session.frames_encoded)
    }
}

impl Default for FfmpegZeroCopyEncoder {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 4. EXISTING VLC SUBTITLE MANAGER TYPES
// =========================================================================

#[derive(Debug, Clone)]
pub struct SubtitleTrackEntry {
    pub track_id: usize,
    pub language: String,
    pub title: String,
    pub format_kind: String, // e.g. "SSA/ASS", "SubRip SRT", "WebVTT"
    pub sync_delay_ms: i32,
}

pub struct VlcSubtitleManager {
    pub subtitle_tracks: Vec<SubtitleTrackEntry>,
    pub active_subtitle_id: Option<usize>,
}

impl VlcSubtitleManager {
    pub fn new() -> Self {
        Self {
            subtitle_tracks: Vec::new(),
            active_subtitle_id: None,
        }
    }

    pub fn register_subtitle_track(&mut self, lang: &str, title: &str, format_kind: &str) -> usize {
        let id = self.subtitle_tracks.len() + 1;
        let track = SubtitleTrackEntry {
            track_id: id,
            language: lang.to_string(),
            title: title.to_string(),
            format_kind: format_kind.to_string(),
            sync_delay_ms: 0,
        };
        self.subtitle_tracks.push(track);
        if self.active_subtitle_id.is_none() {
            self.active_subtitle_id = Some(id);
        }
        id
    }

    pub fn set_subtitle_sync_delay(
        &mut self,
        track_id: usize,
        delay_ms: i32,
    ) -> Result<(), String> {
        let track = self
            .subtitle_tracks
            .iter_mut()
            .find(|t| t.track_id == track_id)
            .ok_or_else(|| format!("Subtitle track {} not found", track_id))?;
        track.sync_delay_ms = delay_ms;
        Ok(())
    }
}

impl Default for VlcSubtitleManager {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 5. ALSA SOUND ENGINE (ADVANCED LINUX SOUND ARCHITECTURE)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlsaPcmState {
    Open,
    Prepared,
    Running,
    Xrun,
    Draining,
    Suspended,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlsaSampleFormat {
    S16Le,
    S243Le,
    S32Le,
    FloatLe,
}

#[derive(Debug, Clone)]
pub struct SndPcmHwParams {
    pub format: AlsaSampleFormat,
    pub channels: u32,
    pub rate_hz: u32,
    pub period_size: u32,
    pub buffer_size: u32,
}

#[derive(Debug, Clone)]
pub struct AlsaMixerElement {
    pub name: String,
    pub volume_db: f32,
    pub muted: bool,
    pub min_db: f32,
    pub max_db: f32,
}

pub struct AlsaSoundEngine {
    pub card_name: String,
    pub pcm_state: AlsaPcmState,
    pub hw_params: SndPcmHwParams,
    pub mixer_elements: BTreeMap<String, AlsaMixerElement>,
    pub active_ucm_verb: String,
}

impl AlsaSoundEngine {
    pub fn new(card: &str) -> Self {
        let mut mixer = BTreeMap::new();
        mixer.insert(
            "Master".to_string(),
            AlsaMixerElement {
                name: "Master".to_string(),
                volume_db: 0.0,
                muted: false,
                min_db: -64.0,
                max_db: 6.0,
            },
        );
        mixer.insert(
            "PCM".to_string(),
            AlsaMixerElement {
                name: "PCM".to_string(),
                volume_db: 0.0,
                muted: false,
                min_db: -50.0,
                max_db: 0.0,
            },
        );

        Self {
            card_name: card.to_string(),
            pcm_state: AlsaPcmState::Open,
            hw_params: SndPcmHwParams {
                format: AlsaSampleFormat::S16Le,
                channels: 2,
                rate_hz: 48000,
                period_size: 1024,
                buffer_size: 4096,
            },
            mixer_elements: mixer,
            active_ucm_verb: "HiFi".to_string(),
        }
    }

    pub fn prepare_pcm(&mut self, params: SndPcmHwParams) {
        self.hw_params = params;
        self.pcm_state = AlsaPcmState::Prepared;
    }

    pub fn start_pcm(&mut self) -> Result<(), String> {
        if self.pcm_state == AlsaPcmState::Prepared {
            self.pcm_state = AlsaPcmState::Running;
            Ok(())
        } else {
            Err(format!("PCM cannot start from state {:?}", self.pcm_state))
        }
    }

    pub fn set_mixer_volume(&mut self, element: &str, db: f32) -> Result<(), String> {
        let elem = self
            .mixer_elements
            .get_mut(element)
            .ok_or_else(|| format!("Mixer element {} not found", element))?;
        elem.volume_db = db.clamp(elem.min_db, elem.max_db);
        Ok(())
    }
}

impl Default for AlsaSoundEngine {
    fn default() -> Self {
        Self::new("hw:0")
    }
}

// =========================================================================
// 6. OSS V4 / BSD SOUND ENGINE (OPEN SOUND SYSTEM)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OssIoctlCmd {
    SetFormat,
    SetChannels,
    SetSpeed,
    GetOSpace,
    SetTrigger,
    Sync,
}

#[derive(Debug, Clone)]
pub struct OssDspDevice {
    pub path: String,
    pub format_bitmask: u32,
    pub channels: u32,
    pub speed_hz: u32,
    pub is_triggered: bool,
    pub buffer_space_bytes: u32,
}

pub struct OssSoundEngine {
    pub devices: BTreeMap<String, OssDspDevice>,
    pub master_volume: u32, // 0..100
}

impl OssSoundEngine {
    pub fn new() -> Self {
        let mut dev_map = BTreeMap::new();
        dev_map.insert(
            "/dev/dsp0".to_string(),
            OssDspDevice {
                path: "/dev/dsp0".to_string(),
                format_bitmask: 0x00000010, // AFMT_S16_LE
                channels: 2,
                speed_hz: 44100,
                is_triggered: false,
                buffer_space_bytes: 65536,
            },
        );
        Self {
            devices: dev_map,
            master_volume: 80,
        }
    }

    pub fn ioctl(&mut self, dev_path: &str, cmd: OssIoctlCmd, val: u32) -> Result<u32, String> {
        let dev = self
            .devices
            .get_mut(dev_path)
            .ok_or_else(|| format!("OSS device {} not found", dev_path))?;
        match cmd {
            OssIoctlCmd::SetFormat => {
                dev.format_bitmask = val;
                Ok(dev.format_bitmask)
            }
            OssIoctlCmd::SetChannels => {
                dev.channels = val.clamp(1, 8);
                Ok(dev.channels)
            }
            OssIoctlCmd::SetSpeed => {
                dev.speed_hz = val;
                Ok(dev.speed_hz)
            }
            OssIoctlCmd::SetTrigger => {
                dev.is_triggered = val != 0;
                Ok(val)
            }
            OssIoctlCmd::GetOSpace => Ok(dev.buffer_space_bytes),
            OssIoctlCmd::Sync => Ok(0),
        }
    }
}

impl Default for OssSoundEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 7. PULSEAUDIO ENGINE
// =========================================================================

#[derive(Debug, Clone)]
pub struct PulseAudioSink {
    pub sink_id: u32,
    pub name: String,
    pub channel_map: String, // e.g. "front-left,front-right"
    pub volume_percent: u32,
    pub is_muted: bool,
}

pub struct PulseAudioEngine {
    pub sinks: Vec<PulseAudioSink>,
    pub loaded_modules: Vec<String>,
    pub default_sink_name: String,
}

impl PulseAudioEngine {
    pub fn new() -> Self {
        let main_sink = PulseAudioSink {
            sink_id: 1,
            name: "alsa_output.pci-0000_00_1f.3.analog-stereo".to_string(),
            channel_map: "front-left,front-right".to_string(),
            volume_percent: 100,
            is_muted: false,
        };
        Self {
            sinks: vec![main_sink],
            loaded_modules: vec![
                "module-device-restore".to_string(),
                "module-stream-restore".to_string(),
                "module-always-sink".to_string(),
            ],
            default_sink_name: "alsa_output.pci-0000_00_1f.3.analog-stereo".to_string(),
        }
    }

    pub fn load_module(&mut self, module_name: &str) -> usize {
        self.loaded_modules.push(module_name.to_string());
        self.loaded_modules.len()
    }

    pub fn set_sink_volume(&mut self, sink_id: u32, vol: u32) -> Result<(), String> {
        let sink = self
            .sinks
            .iter_mut()
            .find(|s| s.sink_id == sink_id)
            .ok_or_else(|| format!("PA Sink {} not found", sink_id))?;
        sink.volume_percent = vol.min(150); // PA allows volume boost up to 150%
        Ok(())
    }
}

impl Default for PulseAudioEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 8. PIPEWIRE & WIREPLUMBER ENGINE
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipeWireNodeState {
    Unconnected,
    Connecting,
    Idle,
    Running,
    Suspended,
    Error,
}

pub struct PipeWireEngine {
    pub sample_rate_hz: u32,
    pub quantum_frames: u32,
    pub node_state: PipeWireNodeState,
    pub wireplumber_policy_route: String,
    pub active_spa_ports_count: u32,
}

impl PipeWireEngine {
    pub fn new() -> Self {
        Self {
            sample_rate_hz: 48000,
            quantum_frames: 512,
            node_state: PipeWireNodeState::Idle,
            wireplumber_policy_route: "HeadphonesFirstFallbackSpeaker".to_string(),
            active_spa_ports_count: 8,
        }
    }

    pub fn switch_sample_rate(&mut self, target_rate: u32) {
        self.sample_rate_hz = target_rate;
        self.node_state = PipeWireNodeState::Running;
    }

    pub fn set_quantum(&mut self, quantum: u32) {
        self.quantum_frames = quantum;
    }
}

impl Default for PipeWireEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 9. JACK PRO-AUDIO LOW-LATENCY GRAPH ENGINE
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JackTransportState {
    Stopped,
    Rolling,
    Starting,
}

pub struct JackProAudioEngine {
    pub client_name: String,
    pub buffer_size_frames: u32,
    pub sample_rate_hz: u32,
    pub transport_state: JackTransportState,
    pub frame_position: u64,
    pub bpm: f32,
    pub connected_ports: Vec<(String, String)>,
}

impl JackProAudioEngine {
    pub fn new(client: &str) -> Self {
        Self {
            client_name: client.to_string(),
            buffer_size_frames: 128,
            sample_rate_hz: 96000,
            transport_state: JackTransportState::Stopped,
            frame_position: 0,
            bpm: 120.0,
            connected_ports: Vec::new(),
        }
    }

    pub fn connect_ports(&mut self, out_port: &str, in_port: &str) {
        self.connected_ports
            .push((out_port.to_string(), in_port.to_string()));
    }

    pub fn start_transport(&mut self) {
        self.transport_state = JackTransportState::Rolling;
    }

    pub fn stop_transport(&mut self) {
        self.transport_state = JackTransportState::Stopped;
    }

    pub fn process_quantum(&mut self) -> u64 {
        if self.transport_state == JackTransportState::Rolling {
            self.frame_position += self.buffer_size_frames as u64;
        }
        self.frame_position
    }
}

impl Default for JackProAudioEngine {
    fn default() -> Self {
        Self::new("SigmaAudioJackClient")
    }
}

// =========================================================================
// 10. SNDIO BSD AUDIO DAEMON ENGINE
// =========================================================================

pub struct SndioEngine {
    pub device_spec: String,
    pub channels: u32,
    pub sample_rate_hz: u32,
    pub is_started: bool,
    pub volume: u32, // 0..127
}

impl SndioEngine {
    pub fn new(spec: &str) -> Self {
        Self {
            device_spec: spec.to_string(),
            channels: 2,
            sample_rate_hz: 48000,
            is_started: false,
            volume: 100,
        }
    }

    pub fn sio_setpar(&mut self, channels: u32, rate: u32) {
        self.channels = channels;
        self.sample_rate_hz = rate;
    }

    pub fn sio_start(&mut self) {
        self.is_started = true;
    }

    pub fn sio_stop(&mut self) {
        self.is_started = false;
    }

    pub fn sio_onvol(&mut self, vol: u32) {
        self.volume = vol.min(127);
    }
}

impl Default for SndioEngine {
    fn default() -> Self {
        Self::new("snd/0")
    }
}

// =========================================================================
// 11. GSTREAMER MULTIMEDIA FRAMEWORK ENGINE
// =========================================================================

pub struct GStreamerEngine {
    pub pipeline_str: String,
    pub caps_negotiated: String,
    pub is_bus_eos: bool,
}

impl GStreamerEngine {
    pub fn new() -> Self {
        Self {
            pipeline_str: String::new(),
            caps_negotiated: "video/x-raw,format=NV12,width=1920,height=1080,framerate=60/1"
                .to_string(),
            is_bus_eos: false,
        }
    }

    pub fn parse_launch_pipeline(&mut self, pipeline: &str) {
        self.pipeline_str = pipeline.to_string();
        self.is_bus_eos = false;
    }
}

impl Default for GStreamerEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 12. FFMPEG & LIBAV ACCELERATED TRANSCODER ENGINE
// =========================================================================

pub struct FfmpegEngine {
    pub codec_context: String,
    pub hw_accel_device: String,
    pub filtergraph_str: String,
}

impl FfmpegEngine {
    pub fn new() -> Self {
        Self {
            codec_context: "h264_vaapi".to_string(),
            hw_accel_device: "/dev/dri/renderD128".to_string(),
            filtergraph_str: "scale=1920:1080,fps=60,format=nv12".to_string(),
        }
    }

    pub fn set_filtergraph(&mut self, filters: &str) {
        self.filtergraph_str = filters.to_string();
    }
}

impl Default for FfmpegEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 13. V4L2 VIDEO4LINUX2 CAPTURE ENGINE
// =========================================================================

#[derive(Debug, Clone)]
pub struct V4l2DeviceControl {
    pub id: u32,
    pub name: String,
    pub value: i32,
    pub min_val: i32,
    pub max_val: i32,
}

pub struct V4l2CaptureEngine {
    pub dev_path: String,
    pub pixel_format: String, // e.g. "YUYV", "NV12", "MJPEG"
    pub width: u32,
    pub height: u32,
    pub buffer_queue_count: u32,
    pub controls: BTreeMap<u32, V4l2DeviceControl>,
}

impl V4l2CaptureEngine {
    pub fn new(path: &str) -> Self {
        let mut ctrl_map = BTreeMap::new();
        ctrl_map.insert(
            0x00980900, // V4L2_CID_BRIGHTNESS
            V4l2DeviceControl {
                id: 0x00980900,
                name: "Brightness".to_string(),
                value: 128,
                min_val: 0,
                max_val: 255,
            },
        );
        ctrl_map.insert(
            0x00980901, // V4L2_CID_CONTRAST
            V4l2DeviceControl {
                id: 0x00980901,
                name: "Contrast".to_string(),
                value: 32,
                min_val: 0,
                max_val: 128,
            },
        );

        Self {
            dev_path: path.to_string(),
            pixel_format: "NV12".to_string(),
            width: 1920,
            height: 1080,
            buffer_queue_count: 4,
            controls: ctrl_map,
        }
    }

    pub fn request_buffers(&mut self, count: u32) -> u32 {
        self.buffer_queue_count = count;
        self.buffer_queue_count
    }

    pub fn set_control(&mut self, ctrl_id: u32, val: i32) -> Result<(), String> {
        let ctrl = self
            .controls
            .get_mut(&ctrl_id)
            .ok_or_else(|| format!("Control ID {:#x} not found", ctrl_id))?;
        ctrl.value = val.clamp(ctrl.min_val, ctrl.max_val);
        Ok(())
    }
}

impl Default for V4l2CaptureEngine {
    fn default() -> Self {
        Self::new("/dev/video0")
    }
}

// =========================================================================
// 14. VLC MEDIA PLAYER ENGINE
// =========================================================================

pub struct VlcPlayerEngine {
    pub media_uri: String,
    pub is_playing: bool,
    pub rate: f32,
    pub time_ms: u64,
    pub audio_delay_us: i64,
}

impl VlcPlayerEngine {
    pub fn new() -> Self {
        Self {
            media_uri: String::new(),
            is_playing: false,
            rate: 1.0,
            time_ms: 0,
            audio_delay_us: 0,
        }
    }

    pub fn play(&mut self, uri: &str) {
        self.media_uri = uri.to_string();
        self.is_playing = true;
    }

    pub fn pause(&mut self) {
        self.is_playing = false;
    }

    pub fn set_audio_delay(&mut self, delay_us: i64) {
        self.audio_delay_us = delay_us;
    }
}

impl Default for VlcPlayerEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 15. MPV PLAYER ENGINE
// =========================================================================

pub struct MpvPlayerEngine {
    pub ipc_socket_path: String,
    pub hwdec_mode: String,
    pub active_gpu_shader: String,
}

impl MpvPlayerEngine {
    pub fn new() -> Self {
        Self {
            ipc_socket_path: "/tmp/mpvsocket".to_string(),
            hwdec_mode: "auto-safe".to_string(),
            active_gpu_shader: "FSRCNNX_x2_16-0-4-1.glsl".to_string(),
        }
    }

    pub fn send_ipc_command(&self, prop_name: &str, val: &str) -> String {
        format!(
            "{{\"command\": [\"set_property\", \"{}\", \"{}\"]}}",
            prop_name, val
        )
    }
}

impl Default for MpvPlayerEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 16. AUDIO & VIDEO PULL REQUEST GATEWAY ENGINE
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioVideoPrStatus {
    Proposed,
    CiValidationPassed,
    Gated,
    Merged,
    Rejected,
}

#[derive(Debug, Clone)]
pub struct AudioVideoPrSubmission {
    pub pr_id: u32,
    pub title: String,
    pub target_subsystem: String, // e.g. "PipeWire", "ALSA", "V4L2", "FFmpeg"
    pub author_pqc_signature: String,
    pub status: AudioVideoPrStatus,
}

pub struct LinuxBsdAudioVideoPrGatewayEngine {
    pub submissions: BTreeMap<u32, AudioVideoPrSubmission>,
    pub next_pr_id: u32,
}

impl LinuxBsdAudioVideoPrGatewayEngine {
    pub fn new() -> Self {
        Self {
            submissions: BTreeMap::new(),
            next_pr_id: 1,
        }
    }

    pub fn submit_pr(&mut self, title: &str, subsystem: &str, pqc_sig: &str) -> u32 {
        let pr_id = self.next_pr_id;
        self.next_pr_id += 1;

        let pr = AudioVideoPrSubmission {
            pr_id,
            title: title.to_string(),
            target_subsystem: subsystem.to_string(),
            author_pqc_signature: pqc_sig.to_string(),
            status: AudioVideoPrStatus::Proposed,
        };

        self.submissions.insert(pr_id, pr);
        pr_id
    }

    pub fn run_ci_gate(&mut self, pr_id: u32) -> Result<AudioVideoPrStatus, String> {
        let pr = self
            .submissions
            .get_mut(&pr_id)
            .ok_or_else(|| format!("PR ID {} not found", pr_id))?;
        if !pr.author_pqc_signature.is_empty() {
            pr.status = AudioVideoPrStatus::CiValidationPassed;
            Ok(pr.status)
        } else {
            pr.status = AudioVideoPrStatus::Rejected;
            Err("PQC signature missing".to_string())
        }
    }

    pub fn merge_pr(&mut self, pr_id: u32) -> Result<AudioVideoPrStatus, String> {
        let pr = self
            .submissions
            .get_mut(&pr_id)
            .ok_or_else(|| format!("PR ID {} not found", pr_id))?;
        if pr.status == AudioVideoPrStatus::CiValidationPassed {
            pr.status = AudioVideoPrStatus::Merged;
            Ok(pr.status)
        } else {
            Err(format!("PR {} not ready for merge", pr_id))
        }
    }
}

impl Default for LinuxBsdAudioVideoPrGatewayEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// MASTER COORDINATOR: UNIVERSAL DISTRO MEDIA SUITE
// =========================================================================

pub struct LinuxBsdDistroMediaSuite {
    pub gstreamer_pipeline: GStreamerPulseAudioPipeline,
    pub mpv_bsd_engine: MpvFreeBsdSndioEngine,
    pub ffmpeg_encoder: FfmpegZeroCopyEncoder,
    pub vlc_subtitles: VlcSubtitleManager,
    pub alsa_engine: AlsaSoundEngine,
    pub oss_engine: OssSoundEngine,
    pub pulseaudio_engine: PulseAudioEngine,
    pub pipewire_engine: PipeWireEngine,
    pub jack_engine: JackProAudioEngine,
    pub sndio_engine: SndioEngine,
    pub gstreamer_engine: GStreamerEngine,
    pub ffmpeg_engine: FfmpegEngine,
    pub v4l2_engine: V4l2CaptureEngine,
    pub vlc_engine: VlcPlayerEngine,
    pub mpv_engine: MpvPlayerEngine,
    pub pr_gateway: LinuxBsdAudioVideoPrGatewayEngine,
}

impl LinuxBsdDistroMediaSuite {
    pub fn new() -> Self {
        Self {
            gstreamer_pipeline: GStreamerPulseAudioPipeline::new(AudioSinkBackend::PipeWire),
            mpv_bsd_engine: MpvFreeBsdSndioEngine::new("snd/0"),
            ffmpeg_encoder: FfmpegZeroCopyEncoder::new(),
            vlc_subtitles: VlcSubtitleManager::new(),
            alsa_engine: AlsaSoundEngine::new("hw:0"),
            oss_engine: OssSoundEngine::new(),
            pulseaudio_engine: PulseAudioEngine::new(),
            pipewire_engine: PipeWireEngine::new(),
            jack_engine: JackProAudioEngine::new("SigmaAudioJackClient"),
            sndio_engine: SndioEngine::new("snd/0"),
            gstreamer_engine: GStreamerEngine::new(),
            ffmpeg_engine: FfmpegEngine::new(),
            v4l2_engine: V4l2CaptureEngine::new("/dev/video0"),
            vlc_engine: VlcPlayerEngine::new(),
            mpv_engine: MpvPlayerEngine::new(),
            pr_gateway: LinuxBsdAudioVideoPrGatewayEngine::new(),
        }
    }
}

impl Default for LinuxBsdDistroMediaSuite {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gstreamer_pulseaudio_pipeline() {
        let mut gst = GStreamerPulseAudioPipeline::new(AudioSinkBackend::PipeWire);
        let pipe_id = gst.play_uri(
            "https://media.sigmaos.org/demo.mp4",
            GstHardwareDecoder::Vaapi,
        );
        assert_eq!(pipe_id, 1);
        assert!(gst
            .set_audio_sink(pipe_id, AudioSinkBackend::PulseAudio)
            .is_ok());
        assert_eq!(
            gst.active_pipelines.get(&pipe_id).unwrap().sink_backend,
            AudioSinkBackend::PulseAudio
        );
        assert!(gst.stop_pipeline(pipe_id).is_ok());
        assert!(!gst.active_pipelines.get(&pipe_id).unwrap().is_playing);
    }

    #[test]
    fn test_mpv_freebsd_sndio_engine() {
        let mut mpv = MpvFreeBsdSndioEngine::new("snd/0");
        let t1 = mpv.add_audio_track("en", "FLAC", 2, 48000);
        let t2 = mpv.add_audio_track("jp", "AAC", 6, 96000);
        assert_eq!(mpv.selected_track_id, Some(t1));

        assert!(mpv.select_track(t2).is_ok());
        assert_eq!(mpv.selected_track_id, Some(t2));

        mpv.set_sndio_volume(95);
        assert_eq!(mpv.volume_level, 95);
    }

    #[test]
    fn test_ffmpeg_zero_copy_encoder() {
        let mut enc = FfmpegZeroCopyEncoder::new();
        let sid = enc.create_session(
            "rawvideo",
            "hevc",
            3840,
            2160,
            60,
            12000,
            FfmpegHwEncoderBackend::VaapiIntelGpu,
        );
        assert_eq!(sid, 1);

        let frames = enc.encode_frame_batch(sid, 120).unwrap();
        assert_eq!(frames, 120);
    }

    #[test]
    fn test_vlc_subtitle_manager() {
        let mut sub = VlcSubtitleManager::new();
        let trk = sub.register_subtitle_track("eng", "English [Full]", "SSA/ASS");
        assert_eq!(trk, 1);
        assert!(sub.set_subtitle_sync_delay(trk, -250).is_ok());
        assert_eq!(sub.subtitle_tracks[0].sync_delay_ms, -250);
    }

    #[test]
    fn test_alsa_sound_engine() {
        let mut alsa = AlsaSoundEngine::new("hw:0");
        alsa.prepare_pcm(SndPcmHwParams {
            format: AlsaSampleFormat::S32Le,
            channels: 2,
            rate_hz: 96000,
            period_size: 512,
            buffer_size: 2048,
        });
        assert_eq!(alsa.pcm_state, AlsaPcmState::Prepared);
        assert!(alsa.start_pcm().is_ok());
        assert_eq!(alsa.pcm_state, AlsaPcmState::Running);
        assert!(alsa.set_mixer_volume("Master", -6.0).is_ok());
    }

    #[test]
    fn test_oss_sound_engine() {
        let mut oss = OssSoundEngine::new();
        let format = oss
            .ioctl("/dev/dsp0", OssIoctlCmd::SetFormat, 0x00000020)
            .unwrap();
        assert_eq!(format, 0x00000020);
        let space = oss.ioctl("/dev/dsp0", OssIoctlCmd::GetOSpace, 0).unwrap();
        assert_eq!(space, 65536);
    }

    #[test]
    fn test_pulseaudio_engine() {
        let mut pa = PulseAudioEngine::new();
        let mid = pa.load_module("module-null-sink");
        assert_eq!(mid, 4);
        assert!(pa.set_sink_volume(1, 120).is_ok());
        assert_eq!(pa.sinks[0].volume_percent, 120);
    }

    #[test]
    fn test_pipewire_engine() {
        let mut pw = PipeWireEngine::new();
        pw.switch_sample_rate(96000);
        assert_eq!(pw.sample_rate_hz, 96000);
        assert_eq!(pw.node_state, PipeWireNodeState::Running);
    }

    #[test]
    fn test_jack_pro_audio_engine() {
        let mut jack = JackProAudioEngine::new("JackTestClient");
        jack.connect_ports("system:capture_1", "JackTestClient:input_1");
        jack.start_transport();
        assert_eq!(jack.transport_state, JackTransportState::Rolling);
        let frame = jack.process_quantum();
        assert_eq!(frame, 128);
    }

    #[test]
    fn test_sndio_engine() {
        let mut sndio = SndioEngine::new("snd/0");
        sndio.sio_setpar(2, 96000);
        sndio.sio_start();
        assert!(sndio.is_started);
        sndio.sio_onvol(110);
        assert_eq!(sndio.volume, 110);
    }

    #[test]
    fn test_v4l2_capture_engine() {
        let mut v4l2 = V4l2CaptureEngine::new("/dev/video0");
        let bufs = v4l2.request_buffers(8);
        assert_eq!(bufs, 8);
        assert!(v4l2.set_control(0x00980900, 150).is_ok());
    }

    #[test]
    fn test_audio_video_pr_gateway() {
        let mut gateway = LinuxBsdAudioVideoPrGatewayEngine::new();
        let pr_id = gateway.submit_pr(
            "Add PipeWire SPA Buffer Zero-Copy Ring",
            "PipeWire",
            "pqc_dilithium5_sig_valid",
        );
        assert_eq!(pr_id, 1);
        let status = gateway.run_ci_gate(pr_id).unwrap();
        assert_eq!(status, AudioVideoPrStatus::CiValidationPassed);
        let merged_status = gateway.merge_pr(pr_id).unwrap();
        assert_eq!(merged_status, AudioVideoPrStatus::Merged);
    }
}
