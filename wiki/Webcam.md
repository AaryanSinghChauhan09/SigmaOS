# SigmaOS Webcam Components

## Overview
SigmaOS webcam manager providing webcam device management with resolution and FPS control, competitive with Omarchy webcam (omancy-capture-webcam-list, omancy-capture-webcam-resize).

## Features
- **Device discovery** - List available webcams
- **Resolution control** - Set and get camera resolution
- **FPS control** - Set and get camera FPS
- **Active device** - Set and get active webcam
- **Device management** - Add and remove devices
- **Feed resize** - Resize webcam feed for overlay use

## Structures

| Structure | Description |
|-----------|-------------|
| **WebcamManager** | Webcam manager with device registry |
| **WebcamDevice** | Individual webcam device |

## Usage Examples

```rust
// Create webcam manager
let mut mgr = WebcamManager::new();

// Add a device
let device = WebcamDevice {
    id: "cam0".to_string(),
    name: "Integrated Camera".to_string(),
    path: "/dev/video0".to_string(),
    is_available: true,
    current_resolution: (1920, 1080),
    supported_resolutions: vec![(640, 480), (1280, 720), (1920, 1080)],
    current_fps: 30,
    supported_fps: vec![15, 30, 60],
};
mgr.add_device(device);

// Set active device
mgr.set_active_device("cam0").unwrap();

// Set resolution
mgr.set_resolution("cam0", 1280, 720).unwrap();

// Resize feed
mgr.resize_feed("cam0", 320, 240).unwrap();
```

## Status
- **Implementation**: Complete
- **Dependencies**: None (stdlib only)
- **Roadmap Phase**: Multimedia (P1)

## Related Files
- `src/webcam/manager.rs` - Webcam manager implementation
- `wiki/Webcam.md` - Wiki documentation

## Principles Followed
SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY, LAYERED DESIGN,
KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING, FAIL_FAST_SYSTEM,
DIRECT_DEVICE_CONTROL, LOW_LEVEL_CONTROL, MEMORY_SAFETY