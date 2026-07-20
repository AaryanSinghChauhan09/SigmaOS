// SigmaOS Audio Module
// Audio processing, codec support, and audio operations

pub mod audio_codec;
pub mod driver;

pub use audio_codec::{
    AudioChannels, AudioCodec, AudioFormat, AudioMetadata, AudioSampleRate, DecodedAudio,
};
pub use driver::{AudioDriver, AudioDriverError, AudioDriverResult};
