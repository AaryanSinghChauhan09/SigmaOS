# OSS Absorption: PipeWire — Unified Audio/Video Pipeline

> **Status**: 🔄 Active | **Source Project**: PipeWire 1.x (Wim Taymans / Red Hat) | **Target Shard**: `Zenith Audio + Video Router`

---

## 1. Executive Summary

PipeWire is the modern Linux audio and video server that unifies PulseAudio (consumer audio), JACK (pro audio), and GStreamer camera routing into a single graph-based media pipeline. It achieves:

- **Sub-millisecond latency** for professional audio workflows (matching JACK)
- **Full PulseAudio API compatibility** (applications don't need changes)
- **Camera/video routing** for screen sharing, video conferencing, and OBS Studio
- **Per-application volume control, routing, and effects processing**

SigmaOS implements `sigma-audio` as a PipeWire-compatible media server with the Sovereign Lattice's capability-based access model for camera/microphone permissions.

---

## 2. Architecture

```
┌──────────────────────────────────────────────────────────────────┐
│                    SIGMA AUDIO/VIDEO PIPELINE                    │
│                                                                  │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │              APPLICATION LAYER                          │    │
│  │  Firefox ──┐                                            │    │
│  │  Discord ──┼── PulseAudio API (compat)                 │    │
│  │  Spotify ──┘                                            │    │
│  │                                                         │    │
│  │  Ardour  ──── JACK API (compat, pro-audio)             │    │
│  │  OBS     ──── PipeWire native (screen capture)         │    │
│  └───────────────────────┬─────────────────────────────────┘    │
│                          │                                       │
│  ┌───────────────────────▼─────────────────────────────────┐    │
│  │              SIGMA-AUDIO GRAPH ENGINE                   │    │
│  │                                                         │    │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐             │    │
│  │  │  Source   │→ │  Filter  │→ │   Sink   │             │    │
│  │  │ (mic,app)│  │ (volume, │  │ (speaker,│             │    │
│  │  │          │  │  echo-   │  │  headset,│             │    │
│  │  │          │  │  cancel) │  │  BT)     │             │    │
│  │  └──────────┘  └──────────┘  └──────────┘             │    │
│  │                                                         │    │
│  │  Graph is dynamic — nodes added/removed at runtime      │    │
│  └───────────────────────┬─────────────────────────────────┘    │
│                          │                                       │
│  ┌───────────────────────▼─────────────────────────────────┐    │
│  │              HARDWARE ABSTRACTION                       │    │
│  │  ALSA Driver │ Bluetooth (A2DP/LC3) │ USB Audio        │    │
│  └─────────────────────────────────────────────────────────┘    │
└──────────────────────────────────────────────────────────────────┘
```

---

## 3. Key Features

### 3.1 Graph-Based Audio Routing

```rust
// userland/audio/graph.rs
// SPDX-License-Identifier: MIT

pub struct AudioGraph {
    nodes: Vec<AudioNode>,
    links: Vec<AudioLink>,
}

pub enum AudioNode {
    Source(SourceNode),     // Microphone, app output, file playback
    Filter(FilterNode),     // Volume, EQ, echo cancellation, compressor
    Sink(SinkNode),         // Speaker, headphones, Bluetooth, network
}

pub struct AudioLink {
    pub from: (NodeId, PortId),
    pub to:   (NodeId, PortId),
    pub format: AudioFormat,   // S16LE, S32LE, F32LE
    pub rate:   u32,           // 44100, 48000, 96000, 192000
    pub channels: u8,          // 1 (mono), 2 (stereo), 6 (5.1), 8 (7.1)
}

impl AudioGraph {
    /// Route Firefox audio through EQ filter to speakers
    pub fn route_app_to_sink(
        &mut self,
        app: &str,
        filter: Option<FilterNode>,
        sink: &str,
    ) -> Result<()> {
        let app_node = self.find_source(app)?;
        let sink_node = self.find_sink(sink)?;

        if let Some(f) = filter {
            let filter_id = self.add_node(AudioNode::Filter(f));
            self.link(app_node, filter_id)?;
            self.link(filter_id, sink_node)?;
        } else {
            self.link(app_node, sink_node)?;
        }
        Ok(())
    }
}
```

### 3.2 Per-Application Audio Control

```bash
$ sigma audio list
Σ [AUDIO] Active streams:
  Firefox           ████████░░  80%  → Speakers (Built-in)
  Discord (voice)   ██████░░░░  60%  → Headset (USB)
  Spotify           █████░░░░░  50%  → Speakers (Built-in)
  Microphone        ██████████ 100%  → Discord (voice input)

# Change volume for a specific app
$ sigma audio set firefox 40%

# Route Discord to headset, everything else to speakers
$ sigma audio route discord --sink "USB Headset"
$ sigma audio route --default --sink "Built-in Speakers"

# Apply noise cancellation to microphone
$ sigma audio filter mic --add noise-cancel
Σ [AUDIO] Noise cancellation enabled on microphone (RNNoise model)
```

### 3.3 Pro Audio / JACK Compatibility

```bash
# Enable low-latency mode for music production
$ sigma audio set-profile pro-audio
Σ [AUDIO] Switched to Pro Audio profile:
  Sample rate : 96kHz
  Buffer size : 64 samples
  Latency     : 1.3ms (round-trip)
  JACK API    : Active (compatible with Ardour, Bitwig, Carla)

# Connect MIDI keyboard to DAW
$ sigma audio midi list
  Input:  Arturia KeyLab 88   (USB MIDI)
  Input:  Sigma Virtual MIDI  (software)

$ sigma audio midi route "Arturia KeyLab 88" ardour
```

### 3.4 Camera/Screen Sharing with Portal Integration

```bash
# Screen sharing (via sigma-portals — no direct access to framebuffer)
$ sigma audio screen-share start
Σ [PORTAL] Screen sharing requested:
  [1] Entire screen (HDMI-1 — 2560x1440)
  [2] Window: Firefox
  [3] Window: Terminal

  Select: 2
Σ [AUDIO] Sharing "Firefox" window → PipeWire video stream
  Consumer: Discord (screen share)

# Camera access (requires portal permission)
$ sigma audio camera list
  Webcam: Logitech C920 (USB, 1080p30)
  Status: ❌ No app has camera access

$ sigma audio camera grant discord
Σ [PORTAL] Grant camera access to Discord? [y/N] y
Σ [AUDIO] Camera → Discord (1080p @ 30fps)
```

---

## 4. Latency Comparison

| Audio System | Typical Latency | Pro-Audio Capable |
|:------------|:---------------|:------------------|
| PulseAudio | ~40ms | ❌ |
| JACK | ~2ms | ✅ |
| PipeWire (Linux) | ~3ms | ✅ |
| sigma-audio | ~1.3ms | ✅ |

---

## 5. References & Standards

- PipeWire — `pipewire.org` (MIT)
- WirePlumber (session/policy manager) — `pipewire.pages.freedesktop.org/wireplumber` (MIT)
- JACK Audio Connection Kit — `jackaudio.org` (GPL/LGPL)
- ALSA — `alsa-project.org` (GPL/LGPL)
- RNNoise — `jmvalin.ca/demo/rnnoise` (BSD-3-Clause)
