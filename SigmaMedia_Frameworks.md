# Media Frameworks in SigmaOS

> **Status**: 🔄 Active | **Subsystem**: `SigmaMedia`

## 1. Executive Summary

Legacy Linux multimedia has suffered from intense fragmentation (ALSA → PulseAudio → JACK → PipeWire, alongside GStreamer vs FFmpeg). SigmaOS introduces **SigmaMedia**, a strictly unified framework that absorbs the best concepts of PipeWire and GStreamer, layering AI-driven processing and declarative routing on top.

---

## 2. Absorbed Distro Capabilities

| Linux Tech | Inspiration | SigmaMedia Capability |
| :--- | :--- | :--- |
| **PulseAudio/PipeWire** | Unified audio/video routing | Zero-latency, graph-based routing for professional audio and video. |
| **GStreamer** | Modular pipelines | Graph-based multimedia pipelines for arbitrary streams. |
| **JACK** | Pro-audio latency | Hard real-time scheduling for audio nodes natively within the kernel. |

---

## 3. SigmaOS Innovations

### 3.1 Unified Framework (SigmaMedia)

SigmaMedia treats all streams — audio, video, Webcams, SDRs (Software Defined Radios), and generic sensor data — as nodes in a real-time, zero-copy processing graph. It bypasses userspace daemon bottlenecks by utilizing eBPF for data transfers between hardware and application boundaries.

### 3.2 AI-Enhanced Codecs

SigmaMedia integrates hardware-accelerated AI models directly into the pipeline graph.
- **Adaptive Compression**: Automatically adjusting bitrate based on network latency and screen content.
- **On-the-fly Denoising**: Native RNNoise integration removes background noise before it even reaches the application (e.g., Discord or Zoom).
- **Video Upscaling**: Real-time AI upscaling for legacy video streams.

### 3.3 Declarative Routing

Multimedia routing is no longer hidden in opaque graphical tools. SigmaOS uses declarative configurations to map endpoints:

```yaml
# /etc/sigma/media_routes.yaml
routes:
  - id: "podcasting_setup"
    source: "hw:usb_mic_0"
    filters:
      - ai_denoise
      - eq_compressor
    sink: "app:obs_studio"
```
