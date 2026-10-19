# SigmaOS Audio Manager Components

## Overview
SigmaOS audio manager providing audio device management with volume and mute control, competitive with Omarchy audio controls (omancy-audio-*, wireplumber, pipewire).

## Features
- **Audio sink management** - Output devices with volume and mute
- **Audio source management** - Input devices with volume and mute
- **Default device selection** - Set default sink and source
- **Master volume** - Global volume control
- **Master mute** - Global mute toggle
- **Per-device volume** - Individual device volume control
- **Per-device mute** - Individual device mute toggle

## Structures

| Structure | Description |
|-----------|-------------|
| **AudioManager** | Audio manager with sinks and sources |
| **AudioDevice** | Audio device info |
| **AudioSink** | Audio output device |
| **AudioSource** | Audio input device |
| **AudioDeviceType** | Input, Output |

## Usage Examples

```rust
// Create audio manager
let mut mgr = AudioManager::new();

// Add a sink
mgr.add_sink(AudioSink {
    id: "sink0".to_string(),
    name: "Built-in Audio".to_string(),
    volume: 0.8,
    is_muted: false,
    is_default: true,
});

// Set default sink
mgr.set_default_sink("sink0").unwrap();

// Set volume
mgr.set_sink_volume("sink0", 0.5).unwrap();

// Toggle mute
mgr.toggle_sink_mute("sink0").unwrap();

// Set master volume
mgr.set_master_volume(0.7).unwrap();

// Toggle master mute
mgr.toggle_master_mute();
```

## Status
- **Implementation**: Complete
- **Dependencies**: None (stdlib only)
- **Roadmap Phase**: Multimedia (P1)

## Related Files
- `src/audio/manager.rs` - Audio manager implementation
- `wiki/Audio-Manager.md` - Wiki documentation

## Principles Followed
SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY, LAYERED DESIGN,
KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING, SECURE_DEFAULTS,
DIRECT_DEVICE_CONTROL, LOW_LEVEL_CONTROL, LOW_LATENCY, ZERO_COPY,
EFFICIENT_SCHEDULING, SAFE_CONCURRENCY, RACE_FREE DESIGN