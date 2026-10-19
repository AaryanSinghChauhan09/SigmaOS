# SigmaOS Screen Recording Components

## Overview
SigmaOS screen recording system providing low-latency screen capture with webcam overlay, competitive with Omarchy screen recording (omancy-capture-screenrecording, omancy-capture-screenrecording-with-webcam).

## Features
- **Screen recording** with configurable FPS and resolution
- **Webcam overlay** support with positioning
- **Audio capture** toggle
- **Multiple formats**: MP4, WebM, AVI, MOV
- **Pause/resume** recording
- **Session tracking** with frame count and duration
- **Statistics** (total sessions, frames, duration, size)

## Structures

| Structure | Description |
|-----------|-------------|
| **ScreenRecorder** | Screen recorder with session management |
| **RecordingSession** | Individual recording session |
| **RecordingConfig** | Recording configuration |
| **RecordingFormat** | Mp4, Webm, Avi, Mov |
| **RecordingStatus** | Idle, Recording, Paused, Stopping, Error |
| **WebcamPosition** | TopLeft, TopRight, BottomLeft, BottomRight, Center |

## Usage Examples

```rust
// Create screen recorder
let mut recorder = ScreenRecorder::new();

// Configure recording
let config = RecordingConfig {
    format: RecordingFormat::Mp4,
    fps: 30,
    width: 1920,
    height: 1080,
    audio_enabled: true,
    webcam_enabled: true,
    webcam_position: WebcamPosition::BottomRight,
    webcam_size: 160,
    bitrate: 5000,
    show_cursor: true,
    show_clicks: false,
};

// Start recording
let id = recorder.start("/tmp/recording.mp4".to_string(), config).unwrap();

// Pause/resume
recorder.pause().unwrap();
recorder.resume().unwrap();

// Stop
recorder.stop().unwrap();

// Get stats
let stats = recorder.stats();
```

## Status
- **Implementation**: Complete
- **Dependencies**: None (stdlib only)
- **Roadmap Phase**: Multimedia (P1)

## Related Files
- `src/screen/recorder.rs` - Screen recorder implementation
- `wiki/Screen-Recording.md` - Wiki documentation

## Principles Followed
SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY, LAYERED DESIGN,
KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING, FAIL_FAST_SYSTEM,
SECURE_DEFAULTS, DIRECT_DEVICE_CONTROL, LOW_LEVEL_CONTROL, MEMORY_SAFETY,
ZERO_COPY, EFFICIENT_SCHEDULING, SAFE_CONCURRENCY