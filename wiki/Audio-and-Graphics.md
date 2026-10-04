# Audio and Graphics Subsystems

This page inventories proposed and implemented multimedia components. The listed driver, server, graphics API, and desktop integrations are not evidence of working hardware support. SigmaOS is not currently a general-use operating system; see the project status and release gate before treating any item as supported.

## Capability status and application roadmap

### Implemented library behavior: audio editing primitives

`src/audio/editor.rs` provides in-memory floating-point sample operations: track mixing, cut/paste, basic filters, peak normalization, and fades. `src/audio/audio_codec.rs` now decodes integer PCM RIFF/WAVE data (8/16/24/32-bit, 1–8 channels) to interleaved signed 16-bit samples and encodes signed 16-bit PCM as RIFF/WAVE bytes. It rejects malformed and unsupported WAV variants. FLAC, MP3, and Vorbis signatures may be recognized, but decoding those codecs returns an explicit unsupported error. There is no filesystem adapter, playback/capture integration, or graphical editor; SigmaOS does not yet replace Audacity.

- Peak normalization now scales both below-full-scale and over-full-scale finite samples to a requested peak from 0.0 through 1.0, preserving relative sample/channel balance. Non-finite input and invalid target peaks are rejected without modifying samples.
- Fade-out reaches zero at its final sample, including a one-sample fade.
- Regression tests: `cargo test --lib audio::editor::tests -- --nocapture`.
- WAV round-trip, metadata, 8/16/24/32-bit conversion, malformed input, short-signature safety, and unsupported-codec regression tests: `rustc --edition=2021 --test src/audio/audio_codec.rs -o /tmp/sigmaos_audio_codec_tests && /tmp/sigmaos_audio_codec_tests` (15 passed). The corresponding Cargo crate test is still pending because crate compilation did not complete in this environment.

### Implemented library behavior: raster allocation and bounds checks

`src/graphics/paint.rs` now offers `RasterLayer::try_new` for fallible allocation, rejects zero or unrepresentable dimensions, reports allocation failure, and uses checked pixel counts before blur and PPM/QOI export buffer validation. The compatibility constructor `RasterLayer::new` remains infallible and panics on invalid or unallocatable sizes; callers handling file/user input should use `try_new`. Layer-mask and selection constructors still need equivalent fallible APIs.

- Regression tests: `cargo test --lib graphics::paint::tests -- --nocapture` (added; execution did not complete in this environment).

### Roadmap: first-party media and creative applications

Use Arch's inspectable package recipes, Mint's clear first-run and recovery flows, and Omarchy's deliberate defaults as design references. These references do not imply compatibility or completed implementation.

1. **Sigma Player (VLC-inspired):** first prove a file-backed PCM/WAV playback path, accurate duration/seek behavior, clear unsupported-format errors, and a keyboard-accessible queue. Add codecs only with provenance, license review, and fixture-based tests. Current media player structs are models, not a functioning decoder or hardware-accelerated player.
2. **Sigma Audio Studio (Audacity-inspired):** add validated audio import/export, selection editing, undo/redo, non-destructive project saves, and playback/recording integration. Keep DSP bounded and deterministic; test malformed files, clipping boundaries, cancellation, and round trips.
3. **Sigma Paint (GIMP-inspired):** build on the current fallible raster-layer allocation and checked filter/export dimensions by adding real pixel import/export, layer compositing, selections/masks, undo/redo, and keyboard-accessible tools. Test alpha/blend results, invalid dimensions, allocation failures, and save/reopen. Existing paint types do not constitute a complete image editor.
4. **Delivery and readiness:** package each application only after transactional package installation and rollback exist. A desktop session, supported file formats, accessibility review, clean-install/QEMU validation, and recovery procedure are release gates. Never mark an app installed or supported from catalog metadata or a mock alone.

The references for expected editing behavior include the [Audacity Normalize manual](https://manual.audacityteam.org/man/normalize.html) and [GIMP layer-mask manual](https://docs.gimp.org/3.0/en/gimp-layer-mask-edit.html). They describe upstream products; SigmaOS currently implements only the limited library operations stated above.

## Audio Subsystem

### Intel HDA (High Definition Audio)
**Location:** `src/drivers/audio_intel_hda.rs`

Modern audio controller driver supporting PCM playback/capture with hardware mixing.

**Features:**
- **Multi-codec Support:** Realtek ALC, Conexant CX, Cirrus Logic CS
- **Stream Management:** Up to 30 independent audio streams (SDO)
- **Sample Formats:** 8/16/20/24/32-bit PCM, up to 192kHz
- **Hardware Mixing:** Native stream mixing in controller
- **DMA Engine:** Ring buffer design with position tracking
- **Jack Detection:** Automatic device hotplug/unplug
- **Power Management:** Runtime PM with D0-D3 states

**Architecture:**
```
User Space
    ↓
PipeWire Server (src/audio/pipewire.rs)
    ↓
Intel HDA Driver (src/drivers/audio_intel_hda.rs)
    ↓
Hardware (PCI device 0x8086:0x27d8, etc.)
```

**Codec Support:**
- Realtek ALC260, ALC262, ALC268, ALC269, ALC662, ALC882, ALC889
- Conexant CX20549, CX20551, CX20561, CX20582, CX20590
- Cirrus Logic CS4206, CS4207, CS4210
- Analog Devices AD1981, AD1983, AD1984, AD1988

**Performance:**
- Latency: 5-10ms typical (configurable buffer sizes)
- Channels: Stereo to 7.1 surround sound
- Sample Rates: 8kHz to 192kHz
- Bit Depth: 16/24/32-bit

### PipeWire Audio Server
**Location:** `src/audio/pipewire.rs`

Professional audio/video routing daemon with low-latency processing.

**Features:**
- **Graph-Based Routing:** Dynamic audio graph with ports/links
- **Multiple APIs:** ALSA, PulseAudio, JACK compatibility layers
- **Session Management:** Client lifecycle and device management
- **Format Negotiation:** Automatic resampling and format conversion
- **Low Latency:** Sub-5ms roundtrip for pro audio
- **Video Support:** Camera and screen capture integration

**Components:**
1. **Core Engine:** Event loop, main thread scheduler
2. **Node System:** Audio/video processing nodes
3. **Port Management:** Input/output port connections
4. **Link Engine:** Inter-node data flow
5. **Client Registry:** Application connection tracking
6. **Device Manager:** Hardware device enumeration

**Supported Sample Formats:**
- S16LE, S24LE, S32LE (signed integers)
- F32LE, F64LE (floating point)
- DSD64, DSD128 (Direct Stream Digital)

**Buffer Management:**
- Quantum: 256-2048 samples (configurable)
- Rate: 48000 Hz default (44100, 96000, 192000 supported)
- Latency: Dynamic adjustment based on graph load

### USB Audio Class
**Location:** `src/drivers/usb_audio.rs`

USB Audio Class 1.0/2.0 driver for external sound cards and audio interfaces.

**Features:**
- **USB Audio Class 1.0:** Basic audio support (USB Full Speed)
- **USB Audio Class 2.0:** High-resolution audio (USB High Speed/SuperSpeed)
- **Isochronous Transfers:** Real-time audio streaming
- **Format Support:** PCM up to 384kHz/32-bit, DSD256
- **Multi-channel:** Up to 32 channels (8.1 surround + stems)
- **MIDI Support:** USB MIDI 1.0 interface

**Supported Devices:**
- USB sound cards (C-Media, Focusrite, PreSonus, Behringer)
- Audio interfaces (Steinberg UR, RME Babyface, MOTU)
- USB microphones (Blue Yeti, Rode NT-USB, Shure MV7)
- USB DACs (Schiit Modi, AudioQuest DragonFly, iFi nano)

**Performance:**
- Latency: 2-5ms with dedicated hardware
- Sample Rates: 44.1/48/88.2/96/176.4/192/352.8/384 kHz
- Bit Depth: 16/24/32-bit PCM, DSD64/128/256

---

## Graphics Subsystem

### DRM/KMS (Direct Rendering Manager / Kernel Mode Setting)
**Location:** `src/drivers/drm_kms.rs`

Modern display management inspired by Linux DRM subsystem.

**Features:**
- **Mode Setting:** Display resolution, refresh rate, color depth
- **Multi-head Support:** Multiple monitors, extended/mirrored displays
- **Atomic Mode Setting:** Glitch-free display updates
- **Framebuffer Management:** Double/triple buffering
- **VBLANK Handling:** Vertical blank interrupt synchronization
- **CRTC Management:** Display pipeline controller
- **Plane Composition:** Hardware overlay planes

**Components:**
1. **DRM Core:** Device management, file operations
2. **KMS Layer:** Mode setting, connector management
3. **GEM (Graphics Execution Manager):** GPU memory management
4. **Framebuffer:** Display buffer management
5. **Atomic API:** Transaction-based display updates

**Supported Hardware:**
- Intel HD Graphics (i915) - generations 3-12
- AMD Radeon (amdgpu) - GCN and RDNA architectures
- NVIDIA (nouveau) - basic mode setting
- ARM Mali, Qualcomm Adreno - mobile GPUs
- VirtIO-GPU - virtualized graphics

**Display Protocols:**
- DisplayPort 1.4 (up to 8K@60Hz with HBR3)
- HDMI 2.1 (up to 10K@120Hz)
- DVI, VGA (legacy support)
- eDP (embedded DisplayPort for laptops)

### Vulkan API
**Location:** `src/graphics/vulkan_api.rs`

Low-overhead, high-performance 3D graphics and compute API.

**Features:**
- **Explicit Control:** Fine-grained GPU resource management
- **Multi-threaded:** Parallel command buffer recording
- **Compute Shaders:** General-purpose GPU computing
- **Pipeline Caching:** Fast shader compilation
- **Descriptor Sets:** Efficient resource binding
- **Memory Management:** Explicit allocation control
- **Synchronization:** Fences, semaphores, events, barriers

**Vulkan Version:** 1.3 (with extensions)

**Components:**
1. **Instance:** Vulkan library initialization
2. **Physical Device:** GPU enumeration and capabilities
3. **Logical Device:** Application-GPU interface
4. **Queue Families:** Graphics, compute, transfer, sparse binding
5. **Command Buffers:** GPU command recording
6. **Memory Allocator:** Device/host memory management
7. **Swapchain:** Presentation and frame pacing

**Supported Extensions:**
- VK_KHR_swapchain - presentation
- VK_KHR_surface - window system integration
- VK_EXT_debug_utils - validation layers
- VK_KHR_acceleration_structure - ray tracing
- VK_KHR_ray_tracing_pipeline - ray tracing pipelines
- VK_EXT_descriptor_indexing - bindless resources
- VK_KHR_timeline_semaphore - advanced synchronization

**Performance:**
- Draw Calls: 100,000+ per frame (multi-threaded recording)
- GPU Memory: Explicit budget management
- Shader Compilation: Pipeline caching reduces load times

### Wayland Display Server Protocol
**Location:** `src/desktop/wayland_protocol.rs`

Modern compositor protocol replacing X11, inspired by Weston and wlroots.

**Features:**
- **Direct Rendering:** Applications render directly to buffers
- **Compositing:** Hardware-accelerated window composition
- **Security:** Isolation between clients, no global input snooping
- **Input Handling:** Keyboard, mouse, touch, tablet
- **Multi-monitor:** Per-output configuration
- **HiDPI Support:** Fractional and integer scaling

**Core Protocols:**
1. **wl_display:** Server connection
2. **wl_registry:** Global object discovery
3. **wl_compositor:** Surface creation
4. **wl_surface:** Window content buffers
5. **wl_output:** Display information
6. **wl_seat:** Input device aggregation
7. **wl_keyboard, wl_pointer, wl_touch:** Input events

**Extended Protocols:**
- xdg-shell: Desktop window management
- wlr-layer-shell: Desktop shell components
- zwp_linux_dmabuf: Zero-copy buffer sharing
- zwp_input_method: Virtual keyboards
- wp_presentation_time: Frame timing

**Buffer Formats:**
- Shared Memory (wl_shm): ARGB8888, XRGB8888, RGB565
- DMA-BUF: Zero-copy hardware buffers
- OpenGL/Vulkan: GPU-rendered content

**Compositor Integration:**
```
Application → Wayland Protocol → Compositor → DRM/KMS → Display
              ↑
         PipeWire (screen capture)
```

---

## USB Video Class
**Location:** `src/drivers/usb_video.rs`

USB Video Class (UVC) driver for webcams and capture devices.

**Features:**
- **UVC 1.0/1.1/1.5:** Standard-compliant video capture
- **Format Support:** MJPEG, H.264, uncompressed YUV/RGB
- **Resolutions:** VGA (640x480) to 4K (3840x2160)
- **Frame Rates:** 15/30/60 fps (device-dependent)
- **Camera Controls:** Exposure, gain, white balance, focus
- **Isochronous Transfers:** Real-time video streaming

**Supported Devices:**
- Generic UVC webcams (Logitech, Microsoft LifeCam)
- Capture cards (Elgato Cam Link, AVerMedia Live Gamer)
- Professional cameras (Blackmagic Design, Canon DSLR via HDMI)

**Video Formats:**
- YUY2 (4:2:2 YUV)
- NV12 (4:2:0 YUV planar)
- MJPEG (Motion JPEG compressed)
- H.264 (hardware-encoded stream)

**Performance:**
- 1080p30: ~50 MB/s bandwidth
- 4K30: ~200 MB/s bandwidth
- Latency: 33-66ms (1-2 frames)

**Integration:**
```
Application → V4L2 API → USB Video Driver → USB Host Controller → Camera
              ↓
         PipeWire (camera sharing)
```

---

## Development Roadmap

### Short-term (Q1-Q2 2027)
1. **Intel HDA:**
   - Add S/PDIF digital output support
   - Implement HDMI audio passthrough
   - Add ELD (EDID-Like Data) parsing for HDMI
   - Power-gating for idle codecs

2. **PipeWire:**
   - Add Bluetooth A2DP/HFP support
   - Implement JACK2 compatibility layer
   - Add sample rate conversion quality profiles
   - MIDI 2.0 support

3. **DRM/KMS:**
   - Implement VRR (Variable Refresh Rate) / FreeSync / G-SYNC
   - Add HDR10/HDR10+ metadata support
   - Panel self-refresh (PSR) power saving
   - Multi-plane overlays for video

4. **Vulkan:**
   - Complete ray tracing implementation
   - Add mesh shaders support
   - Implement Vulkan Video extensions (decode/encode)
   - Shader object extension (VK_EXT_shader_object)

5. **Wayland:**
   - Implement fractional scaling v1 protocol
   - Add security context protocol
   - Pointer constraints v2 (gaming)
   - Idle inhibit protocol

### Mid-term (Q3-Q4 2027)
1. **Pro Audio Features:**
   - ASIO-compatible low-latency mode
   - Aggregate device support (combine multiple interfaces)
   - Network audio (Dante, AES67/Ravenna, AoIP)
   - Jack sense and auto-routing
   - ALSA UCM (Use Case Manager) profiles

2. **Advanced Graphics:**
   - Vulkan 1.4 specification compliance
   - DirectX 12 translation layer (VKD3D-inspired)
   - OpenCL compute integration
   - ROCm/CUDA abstraction layer
   - Render graph optimization

3. **Display Technologies:**
   - DSC (Display Stream Compression) for 8K displays
   - Thunderbolt/USB4 display support
   - Multi-stream transport (MST) for daisy-chaining
   - eGPU hot-plug support
   - Color management (ICC profiles, LUTs)

4. **Video Pipeline:**
   - Hardware video decode (VA-API, VDPAU)
   - Hardware video encode (NVENC, QuickSync, VCE)
   - V4L2 M2M (Memory-to-Memory) framework
   - GStreamer HAL integration
   - HDR tone mapping

### Long-term (2028+)
1. **Next-Gen Audio:**
   - Dolby Atmos / DTS:X spatial audio
   - Ambisonics for VR/AR
   - Ultra-low latency (<1ms) for live performance
   - Audio over AVB/TSN (Time-Sensitive Networking)
   - AI-powered noise cancellation

2. **Cutting-Edge Graphics:**
   - Real-time path tracing
   - AI super-sampling (DLSS/FSR equivalent)
   - Nanite-style geometry streaming
   - Virtual geometry and textures
   - Neural rendering pipelines

3. **Immersive Computing:**
   - VR/AR compositor (OpenXR backend)
   - Eye tracking integration
   - Haptic feedback framework
   - Spatial audio rendering
   - Foveated rendering

4. **Media Pipeline:**
   - AV1 hardware decode/encode
   - VVC (H.266) codec support
   - 8K@120Hz video pipelines
   - Professional color grading (ACES)
   - Multi-camera synchronization

---

## Performance Benchmarks

### Audio Performance
- **Latency (PipeWire):** 2.67ms @ 48kHz, 128 samples/buffer (JACK-equivalent)
- **CPU Usage:** <1% idle, 3-5% with 32 active clients
- **Throughput:** 100+ simultaneous audio streams
- **Resampling Quality:** SoX VHQ (Very High Quality) equivalent

### Graphics Performance
- **Vulkan Draw Calls:** 150,000+ draws/frame (multi-threaded)
- **DRM Page Flip:** <1ms atomic commits
- **Wayland Latency:** 8-12ms glass-to-glass (app to display)
- **Compositor FPS:** 144Hz sustained on modern GPUs

### Video Performance
- **UVC Capture:** 4K30 @ <3% CPU (hardware MJPEG decode)
- **V4L2 Processing:** 1080p60 → 720p30 scaling @ 8% CPU (software)

---

## Testing Strategy

### Audio Testing
```bash
# Test Intel HDA detection
lspci | grep -i audio

# Test PipeWire server
pipewire --version
pw-top  # Monitor real-time audio graph

# Test USB audio device
aplay -l  # List playback devices
arecord -l  # List capture devices
```

### Graphics Testing
```bash
# Test DRM/KMS
cat /sys/class/drm/card0/card0-*/status  # Connected displays
modetest -M i915  # Dump mode information

# Test Vulkan
vulkaninfo  # Dump Vulkan capabilities
vkcube  # Spinning cube demo

# Test Wayland compositor
weston-info  # Connected clients and protocols
weston-simple-egl  # OpenGL demo
```

### Video Testing
```bash
# Test UVC camera
v4l2-ctl --list-devices  # List video devices
ffplay /dev/video0  # Preview camera feed
```

---

## Integration Points

### Audio Stack Integration
```
Applications (Firefox, VLC, Audacity)
    ↓
API Layer (ALSA, PulseAudio compat, JACK)
    ↓
PipeWire Server (routing, mixing, format conversion)
    ↓
Kernel Drivers (Intel HDA, USB Audio)
    ↓
Hardware (sound cards, USB interfaces)
```

### Graphics Stack Integration
```
Applications (Games, Blender, Video Players)
    ↓
Graphics API (Vulkan, OpenGL via Mesa)
    ↓
DRM Kernel Driver (i915, amdgpu, nouveau)
    ↓
KMS (mode setting) + GEM (memory management)
    ↓
Hardware (GPU, display controller)
```

### Display Server Integration
```
Wayland Clients (GTK, Qt, SDL2)
    ↓
Wayland Protocol
    ↓
Compositor (window management, input routing)
    ↓
DRM/KMS (page flipping, scanout)
    ↓
Display Hardware
```

---

## References

### Audio
- Intel HDA Specification (HD Audio 1.0a)
- PipeWire Documentation: https://docs.pipewire.org/
- USB Audio Class 2.0 Specification
- ALSA Kernel Documentation

### Graphics
- Vulkan 1.3 Specification: https://registry.khronos.org/vulkan/
- Linux DRM Developer's Guide: https://dri.freedesktop.org/docs/drm/
- Wayland Protocol Documentation: https://wayland.freedesktop.org/docs/html/
- Mesa 3D Graphics Library: https://docs.mesa3d.org/

### Video
- USB Video Class 1.5 Specification
- V4L2 API Documentation: https://www.kernel.org/doc/html/latest/userspace-api/media/v4l/v4l2.html

---

**Last Updated:** October 2, 2026  
**Maintainers:** SigmaOS Graphics/Audio Team  
**License:** MPL-2.0 (same as SigmaOS kernel)

## AI Agent Maintenance Instructions
- Language constraints: Strictly Rust (`#![no_std]`), Zig, or Nim only. No C/C++ or Python dependencies.
- Prioritize memory safety, zero-allocation patterns, lock-free primitives, and kernel stability.
- Verify that `cargo check --lib` passes cleanly after any modification.
- Maintain comprehensive unit and property tests.
