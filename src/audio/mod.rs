// SigmaOS Audio Subsystem Module

pub mod driver;
pub mod editor;
pub mod podcast;
pub mod sigma_audio;

pub use editor::{
    AmplifyEffect, AudioEditor, AudioEffect, AudioTrack as EditorAudioTrack,
    AutomationEnvelope, CenterChannelExtractor, DistortionEffect, DynamicRangeCompressor,
    EchoEffect, EnvelopePoint, FlangerEffect, GraphicEqualizer, HighPassFilter, LowPassFilter,
    MultiTrackSession, NoiseGateEffect, PitchShifter, ReverbEffect, SignalGenerator,
    SpectralNoiseSuppressionEffect, StereoMixdown, WaveformType,
};
pub use podcast::{
    AudioClip, AudioMasteringEffect, AudioTrack, PodcastEpisode, PodcastFeed, PodcastRecorder,
};
pub use sigma_audio::{
    AudioDevice, AudioError, AudioFormat, AudioGraph, AudioLink, AudioNode, AudioNodeType,
    AudioProfile, AudioSession, AudioStats, DeviceType, GraphState, LinkState, NodeState,
    SessionState, SigmaAudio,
};
