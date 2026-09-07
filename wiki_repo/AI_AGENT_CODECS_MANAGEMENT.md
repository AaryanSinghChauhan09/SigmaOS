# AI Agent Codecs Management in SigmaOS

## Overview
SigmaOS incorporates an autonomous Codec Management Subsystem governed by AI Agents (**Bolt** ⚡, **Sentinel** 🛡️, **Palette** 🎨). This document defines operational protocols, hardware acceleration rules, compression codec policies, and Bluetooth audio codec negotiation interfaces for AI agents supervising media and data codecs across SigmaOS.

AI agents interact directly with `src/compression/archive.rs` (`CompressionCodec`), `src/productivity/screen_recorder.rs` (`ScreenRecorderBackend`), and `src/compatibility/fedora.rs` (`PipeWire` Bluetooth codec engine).

---

## 1. Codec Subsystems & Architecture

### 1.1 Data Compression Codecs (`CompressionCodec`)
Implemented in `src/compression/archive.rs`. Supports high-throughput data compression and decompression codecs:
* **Gzip / Bzip2 / Xz**: Standard POSIX archive compression algorithms.
* **Zstd (Zstat / Zstandard)**: High-speed, SIMD-vectorized multi-threaded compression codec used for package payload extraction and fast RAM/swap compression.

### 1.2 Hardware Accelerated Video & Screen Capture Codecs
Implemented in `src/productivity/screen_recorder.rs`. Automatically selects GPU hardware encoder codecs based on PCI vendor ID (`select_best_gpu_codec`):
* **NVIDIA (0x10DE)**: `NVENC` (NVIDIA H.264 / HEVC hardware encoder).
* **AMD (0x1002)**: `AMF / VCE` (Advanced Media Framework H.264 / AV1).
* **Intel (0x8086)**: `QuickSync / VAAPI` (Intel QuickSync Video).
* **Fallback Software Codec**: `x264 / VP9` CPU software encoding.

### 1.3 PipeWire Bluetooth Audio Codec Negotiation
Implemented in `src/compatibility/fedora.rs`. Dynamically negotiates high-fidelity Bluetooth audio codecs (`SBC`, `AAC`, `aptX`, `aptX-HD`, `LDAC`, `LC3`).

---

## 2. AI Agent Operational Directives & Protocols

### 2.1 Hardware Codec Auto-Selection Protocol
1. **GPU Vendor Detection**:
   AI agents query host GPU vendor ID during screen recording or media encoding sessions (`select_best_gpu_codec`).
2. **Compression Ratio Optimization**:
   **Bolt** ⚡ selects `CompressionCodec::Zstd` for package archives to achieve up to 3x faster decompression speeds during `sigpkg` installations.

### 2.2 PipeWire Bluetooth Codec Negotiation
* **Codec Switching (`set_bluetooth_codec`)**:
  When a high-bitrate Bluetooth audio device connects, agents auto-negotiate `LDAC` or `aptX-HD` for maximum audio fidelity, falling back to `SBC` for power-saving profiles.

---

## 3. Sample Agent Commands & CLI Interactions

```bash
# Query active hardware video encoding codec
sigma-codec hw-encoder status

# Set active PipeWire Bluetooth audio codec
sigma-codec bt-codec --set LDAC

# Test compression codec decompression throughput
sigma-codec bench-zstd --file /var/cache/sigma/pkg.zst
```
