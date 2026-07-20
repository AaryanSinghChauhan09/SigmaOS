// SigmaOS Audio Module
// Audio processing, codec support, and audio operations

pub mod alsa;
pub mod audio_codec;
pub mod driver;

pub use alsa::{
    AlsaAudioStack, AudioDirection, AudioFormat as AlsaFormat, ChannelConfig, MixerControl, PcmStream, SampleRate,
};
pub use audio_codec::{
    AudioChannels, AudioCodec, AudioFormat, AudioMetadata, AudioSampleRate, DecodedAudio,
};
pub use driver::{AudioDriver, AudioDriverError, AudioDriverResult};
