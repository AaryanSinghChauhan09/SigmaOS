// SigmaOS Audio Subsystem Module

pub mod driver;
pub mod podcast;
pub mod pipewire;
pub mod editor;

pub use editor::{
    AudioEditor, AudioEffect, AudioTrack as EditorAudioTrack,
    EchoEffect, LowPassFilter, MultiTrackSession, NoiseGateEffect,
    SpectralNoiseSuppressionEffect,
};
pub use podcast::{
    AudioClip, AudioMasteringEffect, AudioTrack, PodcastEpisode, PodcastFeed, PodcastRecorder,
};
pub use pipewire::{AudioGraph, AudioLink, AudioNode, GraphState, NodeType};
