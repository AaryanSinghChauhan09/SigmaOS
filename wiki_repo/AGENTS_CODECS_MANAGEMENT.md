# AI Agent Guidelines: Codecs Management in SigmaOS

## Overview
This document defines guidelines for AI agents working on **Codecs Management**, multimedia audio/video stream decoding, container demuxing, hardware-accelerated video decoding (VA-API, NVDEC), and High Definition Audio (HDA) hardware codec initialization in SigmaOS.

SigmaOS provides zero-dependency `#![no_std]` multimedia codec abstractions for high-fidelity audio playback, low-latency video decoding, and native hardware audio chip supervision.

---

## 1. Codec Architecture & Subsystems

AI agents interacting with multimedia codecs in SigmaOS must interface with the following core subsystems:

| Subsystem / Engine | Location | Description |
| :--- | :--- | :--- |
| **Audio Stream Codec (`AudioCodec`)** | `src/audio/audio_codec.rs` | Auto-detects and decodes audio formats (FLAC, MP3, WAV, Ogg Vorbis) from magic byte headers; handles audio resampling. |
| **HDA Hardware Audio Codec (`HdaCodec`)** | `src/driver/audio_codec_hda.rs` | Driver layer supervising Intel HDA, Realtek ALC, and Conexant hardware audio codecs, stream routing, and volume control. |
| **Sovereign Video Player (`SovereignVideoPlayer`)** | `src/media/sovereign_video_player.rs` | Demuxes and decodes video streams (`CodecType`: AV1, H.264, VP9, HEVC, ProRes) with audio/video clock drift sync (`AvClockSynchronizer`). |
| **Hardware Video Decoder (`HardwareVideoDecoder`)** | `src/media/sovereign_video_player.rs` | Interfaces with hardware acceleration APIs (`VaApi`, `NvDec`, `V4l2Codecs`). |

---

## 2. Audio Stream Codec & Magic Header Detection

`AudioCodec` identifies audio container streams by inspecting leading byte signatures:

| Format | Magic Bytes / Header Signature | Action |
| :--- | :--- | :--- |
| **FLAC** | `fLaC` (`b"fLaC"`) | Stream uncompressed PCM frames; check sample rate and bit depth. |
| **MP3** | `ID3` or `\xFF\xFB` frame sync | Parse ID3v2 tags, decode MPEG-1 Audio Layer III frames. |
| **WAV** | `RIFF` ... `WAVE` | Parse RIFF header chunks (`fmt `, `data`) for raw LPCM bytes. |
| **Ogg Vorbis** | `OggS` | Demux Ogg pages and Vorbis comment header headers. |

```rust
// Standard audio format magic detection in SigmaOS
let format = AudioCodec::detect_format(&input_bytes);
let audio_pcm = codec.decode(&input_bytes)?;
```

---

## 3. Hardware Video Acceleration Protocols

When working with video playback or editing pipelines:
1. **API Fallback Order:** Prefer `VaApi` on Intel/AMD GPUs, `NvDec` on NVIDIA GPUs, and fallback to software decoding (`SoftwareFallback`) when hardware acceleration is unavailable.
2. **Buffer Alignment:** Ensure zero-copy video frame ring buffers maintain 64-byte alignment for SIMD/AVX vectorized color conversion (`YUV420p` → `NV12` → `ARGB`).
3. **Audio-Video Clock Synchronization:** Sync video frame presentation to the audio master clock (`AvClockSynchronizer`). If video lags by `> 40ms`, drop lagging frames to maintain real-time sync.

---

## 4. HDA Hardware Codec Initialization Protocols

When initializing hardware audio codecs (`HdaCodec`):
1. **Probe Sequence:** Issue broadcast probe commands (`PROBE_CMD`) to discover attached codec addresses (`0..15`).
2. **GETPARAM Discovery:** Read vendor IDs, revision IDs, and AFUNC node parameters before configuring output stream channels (`Stereo`, `Surround51`, `Surround71`).
3. **Volume & Mute:** Apply volume levels using log-scale attenuation steps; verify mute bit state transitions.

---

## 5. AI Agent Self-Assessment Checklist

Before finalizing changes to audio or video codec modules:

- [ ] Does `AudioCodec::detect_format` correctly handle unknown or truncated byte slices?
- [ ] Are audio/video stream timestamps (PTS/DTS) aligned with master clock sync?
- [ ] Do HDA codec drivers validate stream index bounds before applying volume or stream controls?
- [ ] Has `./run_sigma_tests.sh` been executed and confirmed passing with 0 failures?
