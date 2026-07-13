# SigmaOS Gaming Layer — Vulkan & DirectX Compatibility Spec

> **Status**: Planning | **Target**: v0.6-gaming | **Codename**: Sigma Forge

---

## Vision

The SigmaOS Gaming Layer provides first-class gaming support through:
- **Native Vulkan**: Direct Vulkan 1.3 implementation using the sovereign GPU HAL
- **DirectX Compatibility (sigma-dx)**: DirectX 11/12 → Vulkan translation layer
- **Performance Runtime**: Low-latency audio, high-priority GPU scheduling, frame pacing
- **Game Store Integration**: Native `.spkg` game distribution with asset streaming

---

## Architecture

```
┌────────────────────────────────────────────────────────────┐
│                     Games / Applications                    │
│  Native Vulkan │ DirectX (via sigma-dx) │ OpenGL (via ANGLE)│
├────────────────────────────────────────────────────────────┤
│                    Sigma Forge Runtime                      │
│  Frame Pacer │ Input Latency │ Audio Engine │ Shader Cache  │
├────────────────────────────────────────────────────────────┤
│                  sigma-dx (Translation Layer)               │
│   D3D12 → Vulkan │ D3D11 → Vulkan │ DXGI Surface Bridge   │
├────────────────────────────────────────────────────────────┤
│                  Sovereign Vulkan ICD                       │
│   Command Buffers │ Descriptor Sets │ Render Passes        │
├────────────────────────────────────────────────────────────┤
│                   GPU HAL (Ring 1 Driver)                   │
│   NVIDIA (NV50+) │ AMD RDNA │ Intel Xe │ ARM Mali          │
├────────────────────────────────────────────────────────────┤
│                         Hardware                            │
└────────────────────────────────────────────────────────────┘
```

---

## Vulkan Implementation

### Supported Extensions (Phase 1 Target)

| Extension | Priority | Notes |
|-----------|----------|-------|
| `VK_KHR_swapchain` | Critical | Display output |
| `VK_KHR_synchronization2` | Critical | Modern sync primitives |
| `VK_KHR_dynamic_rendering` | High | Renderpass simplification |
| `VK_KHR_ray_tracing_pipeline` | Medium | Ray tracing support |
| `VK_EXT_mesh_shader` | Medium | Next-gen geometry pipeline |
| `VK_KHR_shader_float16_int8` | High | FP16 compute |
| `VK_EXT_descriptor_indexing` | High | Bindless rendering |
| `VK_EXT_memory_budget` | High | VRAM management |

### Performance Features

- **Vulkan Memory Allocator (VMA)** equivalent: `sigma_vma.rs`
- **Pipeline Cache** with disk persistence across boots
- **Async Compute** queues for parallel GPU/CPU work
- **Variable Rate Shading (VRS)** where supported

---

## DirectX Compatibility (sigma-dx)

sigma-dx is a translation layer that converts DirectX API calls to Vulkan, inspired by DXVK but designed from scratch for SigmaOS.

### Supported DirectX Versions

| API | Status | Games Coverage |
|-----|--------|---------------|
| Direct3D 9 | Planned | Legacy (D9→Vulkan via ANGLE) |
| Direct3D 11 | Planned | ~80% of PC game library |
| Direct3D 12 | Planned | Modern titles |
| DXGI 1.6 | Planned | Display management |
| XAudio2 | Planned | Audio (→ sigma-audio) |
| XInput | Planned | Gamepad input |

### D3D12 → Vulkan Mapping

| D3D12 Concept | Vulkan Equivalent |
|---------------|-------------------|
| `ID3D12Device` | `VkDevice` |
| `ID3D12CommandList` | `VkCommandBuffer` |
| `ID3D12Fence` | `VkSemaphore` + `VkFence` |
| `ID3D12DescriptorHeap` | `VkDescriptorPool` |
| `D3D12_RESOURCE_BARRIER` | `VkMemoryBarrier2` |
| DXGI SwapChain | `VkSwapchainKHR` |

---

## Performance Runtime (Sigma Forge)

### Frame Pacing

- **Target**: < 1 ms frame timing variance at 60/120/144 Hz
- Adaptive sync support (GSYNC, FreeSync, HDMI VRR)
- Frame time prediction using exponential moving average

### Low-Latency Audio

```rust
pub struct AudioEngineConfig {
    pub sample_rate: u32,       // 48000 or 44100 Hz
    pub buffer_frames: u16,     // Target: 64–256 frames (1.3–5.3 ms at 48 kHz)
    pub channels: u8,           // 2 (stereo) to 7.1 (8 channels)
    pub format: AudioFormat,    // F32, S16, S24
    pub exclusive_mode: bool,   // Bypass OS mixer for minimal latency
}
```

Target latency: **< 10 ms** end-to-end (input → GPU → display)

### Input Pipeline

| Input Source | Maximum Latency Target |
|-------------|----------------------|
| Mouse/Keyboard | < 1 ms |
| Gamepad (USB) | < 2 ms |
| Gamepad (Bluetooth) | < 8 ms |
| Touch (Mobile) | < 4 ms |

---

## Gamepad Support

### Supported Protocols

| Controller | Protocol | Notes |
|-----------|----------|-------|
| Xbox Series X/S | USB + BT | Native XInput |
| PlayStation DualSense | USB + BT | Native HID |
| Nintendo Switch Pro | USB + BT | Nintendo HID |
| Steam Controller | USB + BT | Valve protocol |
| Generic HID | USB | Standard gamepad profile |

### sigma-gamepad API

```rust
pub trait Gamepad {
    fn id(&self) -> GamepadId;
    fn name(&self) -> &str;
    fn state(&self) -> GamepadState;
    fn rumble(&mut self, low: f32, high: f32, duration_ms: u32) -> Result<(), ()>;
    fn set_lightbar(&mut self, r: u8, g: u8, b: u8) -> Result<(), ()>;  // DualSense
    fn adaptive_trigger(&mut self, trigger: Trigger, effect: TriggerEffect) -> Result<(), ()>;
}
```

---

## Gaming Performance Profiles

The thermal daemon supports a `gaming` profile:

| Setting | Value |
|---------|-------|
| CPU max frequency | 5.2 GHz (all-core boost) |
| GPU priority | Elevated (RT scheduling class) |
| RAM prefetch | Aggressive |
| I/O scheduler | Deadline (low latency) |
| Thermal limit | 95°C CPU / 83°C GPU |
| Fan mode | Aggressive auto |

```bash
sigma power set-profile gaming
sigma forge start <game-executable>
```

---

## Game Distribution

Games distribute through the Sigma Store as `.spkg` packages:

```toml
[package]
name = "my-game"
version = "1.0.0"
type = "game"

[game]
engine = "unreal5"              # or "unity", "godot", "native"
dx_version = "d3d12"            # Triggers sigma-dx translation
vulkan_native = false           # True = bypasses sigma-dx
requires_gpu = "vulkan_1.3"
vram_minimum_mb = 4096
```

---

## Shader Compilation

- **Pre-compiled shaders** via SPIR-V in `.spkg` game package
- **JIT compilation** for shaders not pre-compiled (sigma-shader-compiler)
- **Shader cache** stored at `/var/cache/sigma/shaders/<game-hash>/`
- **Online shader cache sharing** (opt-in): users contribute compiled shaders to improve experience for others

---

## Implementation Roadmap

| Milestone | Target | Description |
|-----------|--------|-------------|
| M1 | 2027 Q1 | Vulkan ICD skeleton + NV/AMD GPU HAL |
| M2 | 2027 Q2 | D3D11→Vulkan translation (core) |
| M3 | 2027 Q3 | D3D12→Vulkan (core) |
| M4 | 2027 Q4 | Frame pacing + low-latency audio |
| M5 | 2028 Q1 | Gamepad support (Xbox/PS) |
| M6 | 2028 Q2 | Gaming performance profile |
| M7 | 2028 Q3 | Public beta with first compatible titles |
