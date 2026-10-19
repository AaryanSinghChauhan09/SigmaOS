# Microphone and Audio Capture Components

## Overview
The Microphone and Audio Capture subsystem manages low-latency hardware audio capture, noise suppression (RNNoise), echo cancellation, and PipeWire stream routing.

## Features
- Direct ALSA / Intel HDA / USB Audio Class 2.0 microphone capture
- Real-time AI-based background noise cancellation (RNNoise)
- Automatic microphone volume normalization and peak clipping prevention
- Per-application hardware permission gate with visual indicator OSD

## Structures
- **AudioCaptureDevice** - Device configuration and state
- **MicrophoneManager** - Audio capture management
- **AudioFormat** - Supported audio formats
- **AudioSource** - Audio source types

## Status
- **Implementation**: Planned
- **Dependencies**: Hardware access, capability model
- **Roadmap Phase**: Multimedai & audio (P3)

## Related Files
- `src/microphone/capture_device.rs` - Device structure
- `src/microphone/manager.rs` - Manager implementation
- `docs/AUDIO_SUBSYSTEM_COMPONENT_GUIDE.md` - Guide document
