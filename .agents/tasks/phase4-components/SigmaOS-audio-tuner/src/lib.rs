//! SigmaOS audio tuner.
//!
//! Provides per-application audio effects, filtering, and hardware tuning
//! built on a pipeline model.

#![warn(missing_docs)]

/// A single equalizer band.
#[derive(Debug, Clone, Copy)]
pub struct Band {
    /// Center frequency in Hz.
    pub freq: f32,
    /// Gain in dB.
    pub gain: f32,
    /// Quality factor.
    pub q: f32,
}

/// A collection of bands forming an equalizer preset.
#[derive(Debug, Clone)]
pub struct Preset {
    /// Name of the preset.
    pub name: String,
    /// Ordered list of bands.
    pub bands: Vec<Band>,
}

/// Trait for pipelines that apply audio effects.
pub trait EffectsPipeline {
    /// Apply the given preset to the pipeline.
    fn apply_preset(&mut self, preset: &Preset) -> Result<(), TuneError>;
    /// Mute effects bypassing the pipeline.
    fn bypass(&mut self);
}

/// Errors returned by the tuner.
#[derive(Debug)]
pub enum TuneError {
    /// A band frequency was invalid.
    InvalidFrequency,
    /// The audio server rejected the change.
    PipelineError(String),
}

impl std::fmt::Display for TuneError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TuneError::InvalidFrequency => write!(f, "invalid frequency"),
            TuneError::PipelineError(msg) => write!(f, "pipeline error: {msg}"),
        }
    }
}

impl std::error::Error for TuneError {}
