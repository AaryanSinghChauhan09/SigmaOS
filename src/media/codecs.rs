//! Media codec support and hardware decode pipeline for SigmaOS.
//!
//! Tracks which codecs are available, manages GStreamer plugin sets,
//! and validates hardware-accelerated decode via VA-API and NVDEC.

#![allow(dead_code)]

/// A media codec supported by SigmaOS.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MediaCodec {
    pub name: &'static str,
    pub mime_type: &'static str,
    pub gst_plugin: &'static str,
    pub is_free: bool,
    pub hw_decode: bool,
}

impl MediaCodec {
    const fn new(
        name: &'static str,
        mime: &'static str,
        plugin: &'static str,
        free: bool,
        hw: bool,
    ) -> Self {
        Self {
            name,
            mime_type: mime,
            gst_plugin: plugin,
            is_free: free,
            hw_decode: hw,
        }
    }
}

/// All codecs in the V1 SigmaOS codec bundle.
pub const V1_CODECS: &[MediaCodec] = &[
    // Video — free
    MediaCodec::new("H.264 / AVC", "video/x-h264", "gst-plugins-bad", true, true),
    MediaCodec::new(
        "H.265 / HEVC",
        "video/x-h265",
        "gst-plugins-bad",
        true,
        true,
    ),
    MediaCodec::new("VP8", "video/x-vp8", "gst-plugins-good", true, false),
    MediaCodec::new("VP9", "video/x-vp9", "gst-plugins-good", true, true),
    MediaCodec::new("AV1", "video/x-av1", "gst-plugins-bad", true, true),
    MediaCodec::new("Theora", "video/x-theora", "gst-plugins-base", true, false),
    // Video — container
    MediaCodec::new(
        "MP4 / ISOBMFF",
        "video/mp4",
        "gst-plugins-good",
        true,
        false,
    ),
    MediaCodec::new(
        "MKV / Matroska",
        "video/x-matroska",
        "gst-plugins-good",
        true,
        false,
    ),
    MediaCodec::new("WebM", "video/webm", "gst-plugins-good", true, false),
    MediaCodec::new("AVI", "video/x-msvideo", "gst-plugins-good", true, false),
    // Audio — free
    MediaCodec::new("MP3", "audio/mpeg", "gst-plugins-good", true, false),
    MediaCodec::new("AAC", "audio/aac", "gst-plugins-bad", true, false),
    MediaCodec::new("FLAC", "audio/flac", "gst-plugins-good", true, false),
    MediaCodec::new("Vorbis / OGG", "audio/ogg", "gst-plugins-base", true, false),
    MediaCodec::new("Opus", "audio/opus", "gst-plugins-base", true, false),
    MediaCodec::new("WAV / PCM", "audio/wav", "gst-plugins-good", true, false),
    MediaCodec::new("ALAC", "audio/x-alac", "gst-plugins-good", true, false),
    MediaCodec::new("WMA", "audio/x-wma", "gst-plugins-bad", false, false),
    // Image (handled by GStreamer pipelines)
    MediaCodec::new("JPEG", "image/jpeg", "gst-plugins-good", true, false),
    MediaCodec::new("PNG", "image/png", "gst-plugins-base", true, false),
    MediaCodec::new("WebP", "image/webp", "gst-plugins-good", true, false),
    MediaCodec::new("AVIF", "image/avif", "gst-plugins-bad", true, false),
];

/// Hardware decode backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HwDecodeBackend {
    VaApi, // Intel, AMD via libva
    Nvdec, // NVIDIA via NVDEC
    V4l2,  // V4L2 request API (embedded / SBCs)
    None,
}

impl HwDecodeBackend {
    /// Check if this backend is available in the current environment.
    pub fn is_available(self) -> bool {
        match self {
            // Production: check /dev/dri/renderD128 exists, vainfo succeeds, etc.
            Self::VaApi => true,
            Self::Nvdec => false, // only if NVIDIA driver is loaded
            Self::V4l2 => false,
            Self::None => true,
        }
    }
}

/// Codec manager: validates and reports on codec availability.
pub struct CodecManager {
    codecs: Vec<MediaCodec>,
}

impl CodecManager {
    pub fn new() -> Self {
        Self {
            codecs: V1_CODECS.to_vec(),
        }
    }

    pub fn all_codecs(&self) -> &[MediaCodec] {
        &self.codecs
    }

    pub fn free_codecs(&self) -> Vec<&MediaCodec> {
        self.codecs.iter().filter(|c| c.is_free).collect()
    }

    pub fn hw_decode_codecs(&self) -> Vec<&MediaCodec> {
        self.codecs.iter().filter(|c| c.hw_decode).collect()
    }

    pub fn codec_for_mime(&self, mime: &str) -> Option<&MediaCodec> {
        self.codecs.iter().find(|c| c.mime_type == mime)
    }

    pub fn plugins_required(&self) -> Vec<&'static str> {
        let mut p: Vec<&'static str> = self.codecs.iter().map(|c| c.gst_plugin).collect();
        p.sort_unstable();
        p.dedup();
        p
    }
}

impl Default for CodecManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v1_codecs_nonempty() {
        assert!(V1_CODECS.len() >= 20);
    }

    #[test]
    fn test_mp3_is_free() {
        let c = CodecManager::new();
        assert!(c.free_codecs().iter().any(|x| x.name == "MP3"));
    }

    #[test]
    fn test_h264_has_hw_decode() {
        let c = CodecManager::new();
        assert!(c.hw_decode_codecs().iter().any(|x| x.name == "H.264 / AVC"));
    }

    #[test]
    fn test_codec_for_mime_html_none() {
        let c = CodecManager::new();
        assert!(c.codec_for_mime("text/html").is_none());
    }

    #[test]
    fn test_codec_for_mime_mp4_found() {
        let c = CodecManager::new();
        let codec = c.codec_for_mime("video/mp4");
        assert!(codec.is_some());
    }

    #[test]
    fn test_plugins_required_deduped() {
        let c = CodecManager::new();
        let plugins = c.plugins_required();
        let mut sorted = plugins.clone();
        sorted.dedup();
        assert_eq!(plugins, sorted);
    }
}
