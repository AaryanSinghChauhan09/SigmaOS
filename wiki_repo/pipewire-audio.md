# PipeWire Audio Integration

## Overview

SigmaOS runs [PipeWire](https://pipewire.org/) (LGPL-2.1) as a **userland audio shard** (`sigma-audiod`). PipeWire handles low-latency audio and video routing with a graph-based pipeline model. A thin Rust wrapper (`sigma-audio-api`) provides the application-facing API without exposing raw libpipewire C bindings.

---

## Architecture

```
sigma-play / sigma-edit / browser
        │  sigma-audio-api (Rust)
        │  (FFI over libpipewire)
        ▼
  sigma-audiod (PipeWire daemon shard)
        │  PipeWire graph: source → effect → sink
        ▼
  ALSA / hardware audio device (via PipeWire ALSA plugin)
```

---

## File Layout

```
userland/audio/
├── README.md
├── sigma_audiod.rs      # PipeWire daemon wrapper shard

├── sigma_audio_api.rs   # Rust API wrapper for apps

└── Cargo.toml
```

---

## sigma-audiod: PipeWire Daemon Shard

`userland/audio/sigma_audiod.rs`:

```rust
//! sigma-audiod: PipeWire audio daemon shard for SigmaOS.
//! Launched by sigma-init on desktop and cloud profiles.
//! Registers as a sigma-bus shard with capability "audio".

use std::process::{Command, Stdio};

pub struct SigmaAudiod {
    pw_process: Option<std::process::Child>,
}

impl SigmaAudiod {
    pub fn new() -> Self { Self { pw_process: None } }

    /// Start the PipeWire daemon.
    pub fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let child = Command::new("pipewire")
            .env("PIPEWIRE_RUNTIME_DIR", "/run/sigma/audio")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
        self.pw_process = Some(child);
        // Also start pipewire-pulse for PulseAudio compatibility
        Command::new("pipewire-pulse")
            .env("PIPEWIRE_RUNTIME_DIR", "/run/sigma/audio")
            .spawn()?;
        Ok(())
    }

    pub fn stop(&mut self) {
        if let Some(ref mut p) = self.pw_process {
            let _ = p.kill();
        }
    }
}
```

---

## sigma-audio-api: Rust Wrapper

`userland/audio/sigma_audio_api.rs`:

```rust
//! sigma-audio-api: safe Rust wrapper over libpipewire for SigmaOS apps.

use std::ffi::CString;

/// Represents an audio source (microphone, file, generator).
pub struct AudioSource {
    name: String,
    sample_rate: u32,
    channels: u8,
}

/// Represents a playback sink (speaker, file output).
pub struct AudioSink {
    name: String,
}

/// A simple audio pipeline: source → optional effect → sink.
pub struct AudioPipeline {
    pub source: AudioSource,
    pub sink: AudioSink,
    pub gain_db: f32,
}

impl AudioPipeline {
    pub fn new(source_name: &str, sink_name: &str) -> Self {
        Self {
            source: AudioSource {
                name: source_name.to_string(),
                sample_rate: 48_000,
                channels: 2,
            },
            sink: AudioSink { name: sink_name.to_string() },
            gain_db: 0.0,
        }
    }

    /// Connect the pipeline in PipeWire.
    /// In real impl: use libspa + pw_stream via pw-sys FFI bindings.
    pub fn connect(&self) -> Result<(), AudioError> {
        // TODO: pw_stream_new, pw_stream_connect
        println!(
            "AudioPipeline: {} → (gain {:.1}dB) → {}",
            self.source.name, self.gain_db, self.sink.name
        );
        Ok(())
    }

    /// Play a WAV file through the pipeline.
    pub fn play_wav(&self, wav_path: &str) -> Result<(), AudioError> {
        std::process::Command::new("pw-play")
            .arg(wav_path)
            .env("PIPEWIRE_RUNTIME_DIR", "/run/sigma/audio")
            .status()
            .map_err(|_| AudioError::PlaybackFailed)?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum AudioError {
    ConnectionFailed,
    PlaybackFailed,
    DeviceNotFound,
}
```

---

## Audio Graph Example

```
Microphone (AudioSource, 48kHz, stereo)
        │
        ▼ [noise-reduce effect node]
        │
        ▼ [volume node: -3dB]
        │
        ▼
Speakers (AudioSink, hw:0,0)
```

---

## sigma-play CLI

```bash

# Play a WAV file

sigma-play /home/user/music/sample.wav

# Record 5 seconds from default mic

sigma-record --duration 5s --output /tmp/rec.wav

# List audio devices

sigma-audio list
```

---

## Exit Criteria

- `sigma-play /usr/share/sounds/sigma-boot.wav` plays audio through PipeWire on real hardware.

- `sigma-audio list` enumerates audio sources and sinks from the PipeWire graph.

- sigma-audiod registers as a sigma-bus shard and responds to `capability: audio` queries.
