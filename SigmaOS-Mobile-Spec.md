# SigmaOS Mobile — Architecture & Implementation Spec

> **Status**: Planning | **Target**: v0.5-mobile | **Architecture**: ARM64, RISC-V

---

## Vision

SigmaOS Mobile is a sovereign, security-first mobile operating system built on the same kernel as SigmaOS Desktop. It does **not** depend on Android, iOS, or Linux mobile stacks. Every component — from the telephony stack to the UI compositor — is sovereign.

---

## Target Hardware

### Primary Targets (Phase 1)

| Device Class | SoC | RAM | Storage | Status |
|-------------|-----|-----|---------|--------|
| Reference Phone | Qualcomm SM8650 | 12 GB | 256 GB UFS 3.1 | Planned |
| Reference Tablet | MediaTek MT8195 | 8 GB | 128 GB UFS 3.1 | Planned |
| Dev Board | Raspberry Pi 5 | 8 GB | 64 GB microSD | In progress |
| Pinebook Pro | RK3399 | 4 GB | 128 GB eMMC | In progress |

### Architecture Requirements

- **ARM64** (AArch64) with ARMv8.2-A minimum
- **RISC-V RV64GC** support (secondary target)
- Hardware memory tagging (MTE) for memory safety
- TrustZone for secure enclave integration

---

## Architecture Overview

```
┌──────────────────────────────────────────────────────────┐
│                    Applications                           │
│  Phone │ Messages │ Camera │ Maps │ Browser │ App Store  │
├──────────────────────────────────────────────────────────┤
│              Zenith Mobile Compositor                    │
│  Touch Input │ Window Manager │ Animations │ IME        │
├──────────────────────────────────────────────────────────┤
│              Mobile System Services                      │
│  Telephony │ Sensor Hub │ Location │ Notifications       │
├──────────────────────────────────────────────────────────┤
│              SigmaOS Kernel (ARM64)                      │
│  Scheduler │ Memory │ IPC │ Security │ Power             │
├──────────────────────────────────────────────────────────┤
│              Hardware Abstraction                        │
│  Camera HAL │ Audio HAL │ Modem HAL │ Sensor HAL        │
├──────────────────────────────────────────────────────────┤
│              Hardware                                    │
└──────────────────────────────────────────────────────────┘
```

---

## Core Subsystems

### 1. Telephony Stack (sigma-tel)

```rust
// kernel/mobile/telephony/mod.rs
pub trait ModemInterface {
    fn send_at_command(&mut self, cmd: &str) -> Result<String, ModemError>;
    fn register_network(&mut self, plmn: &str) -> Result<(), ModemError>;
    fn initiate_call(&mut self, number: &str) -> Result<CallHandle, ModemError>;
    fn send_sms(&mut self, dest: &str, text: &str) -> Result<(), ModemError>;
    fn get_signal_strength(&self) -> SignalStrength;
    fn get_network_type(&self) -> NetworkType; // 2G/3G/4G/5G
}

pub enum NetworkType { Gsm, Umts, Lte, Nr5G }
```

Features:
- 4G LTE and 5G NR support via AT command abstraction
- VoLTE (Voice over LTE) call support
- Emergency calling (E911/E112) compliance
- SIM card management (single/dual SIM)

### 2. Touch Input Processing (sigma-touch)

Multi-touch support with:
- Up to 10 simultaneous touch points
- Pressure and stylus (Apple Pencil / USI 2.0 compatible)
- Hover detection (5mm range)
- Palm rejection ML model (TFLite inference)

```rust
pub struct TouchEvent {
    pub id: u8,
    pub x: f32,
    pub y: f32,
    pub pressure: f32,
    pub major: f32,  // contact ellipse major axis
    pub minor: f32,  // contact ellipse minor axis
    pub orientation: f32,
    pub event_type: TouchEventType,
}

pub enum TouchEventType { Down, Move, Up, Cancel }
```

### 3. Camera HAL (sigma-cam)

```rust
pub trait CameraDevice {
    fn open(&mut self, camera_id: u8) -> Result<(), CameraError>;
    fn configure(&mut self, config: CameraConfig) -> Result<(), CameraError>;
    fn capture_image(&mut self) -> Result<ImageBuffer, CameraError>;
    fn start_preview(&mut self, surface: &mut dyn RenderSurface) -> Result<(), CameraError>;
    fn start_recording(&mut self, output: &mut dyn VideoSink) -> Result<(), CameraError>;
}

pub struct CameraConfig {
    pub resolution: (u32, u32),
    pub fps: u8,
    pub format: PixelFormat,  // RAW10, JPEG, YUV420
    pub hdr: bool,
    pub ois: bool,  // Optical Image Stabilization
}
```

### 4. Power Management (Mobile Profile)

Mobile-specific extensions to the thermal daemon:

| State | CPU Max | GPU | Modem | Display | Trigger |
|-------|---------|-----|-------|---------|---------|
| Active | 100% | Full | Active | Full | Screen on |
| Doze | 10% | Off | Idle | Off | Screen off 10min |
| DeepSleep | 0% | Off | Minimal | Off | Screen off 1hr |
| Emergency | 5% | Off | Emergency only | Off | Battery < 5% |

### 5. Sensor Hub (sigma-sensors)

| Sensor | Interface | Sample Rate |
|--------|-----------|------------|
| Accelerometer | I2C/SPI | 200 Hz |
| Gyroscope | I2C/SPI | 200 Hz |
| Magnetometer | I2C | 100 Hz |
| Barometer | I2C | 25 Hz |
| Proximity | I2C | 50 Hz |
| Ambient Light | I2C | 10 Hz |
| Fingerprint | SPI | Event-driven |

---

## Security Model

Mobile adds additional security layers:
- **Secure Enclave**: ARM TrustZone integration for biometric keys, payment tokens
- **App Sandboxing**: Each app in its own cgroup + namespace + MAC profile
- **Permission Model**: Fine-grained runtime permissions (camera, mic, location, contacts)
- **Baseband Isolation**: Modem in hardware-isolated compartment (IOMMU-enforced)
- **Verified Boot**: Boot chain integrity check at every stage (sigma-boot → kernel → system)

---

## Mobile App Model

Apps ship as `.spkg` files with a `[mobile]` section:

```toml
[package]
name = "sigma-camera"
version = "1.0.0"
targets = ["arm64", "riscv64"]

[mobile]
min_sdk = "sigma-mobile-0.5"
permissions = ["camera", "microphone", "storage.write"]
background_modes = ["location", "audio"]
activity_main = "camera.SigmaCameraActivity"
```

---

## Roadmap

| Milestone | Target | Description |
|-----------|--------|-------------|
| M1 | 2027 Q3 | ARM64 kernel boots on dev board |
| M2 | 2027 Q4 | Touch input + basic display |
| M3 | 2028 Q1 | Telephony stack (calls + SMS) |
| M4 | 2028 Q2 | Camera HAL + basic camera app |
| M5 | 2028 Q3 | App model + app store |
| M6 | 2028 Q4 | Security hardening + biometrics |
| M7 | 2029 Q1 | Public developer preview |
