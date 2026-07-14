# OSS Absorption: PipeWire & PulseAudio

## Overview

PulseAudio (and historically JACK) managed audio routing in Linux until PipeWire absorbed and replaced both. They act as multimedia routing daemons, allowing multiple applications to play sound simultaneously and routing them to ALSA hardware drivers.

## Key Principles Absorbed

### Low-Latency Routing (`sigma_audio`)

- SigmaOS displaces PipeWire and PulseAudio with `sigma_audio::AudioRouter`.
- Rather than passing audio through complex IPC sockets and intermediate daemons, SigmaOS leverages native lock-free `RingBuffer` implementations.

### Zero-Copy Transfers

- Applications write float32 audio samples directly into shared ring buffers (`sigma_audio::AudioStream`).
- The `sigma_audio` daemon simply mixes these buffers and pushes them to the kernel's hardware ALSA endpoints.
- By enforcing memory safety in Rust, we avoid the buffer underrun/overrun issues common in legacy C-based audio systems.

## Displaced Technologies

| Technology | SigmaOS Replacement |
| --- | --- |
| PipeWire | `sigma_audio::AudioRouter` |
| PulseAudio | `sigma_audio::AudioRouter` |
| JACK | Low-latency ring buffers |

## Status

**Core Absorbed** — The `sigma_audio` scaffolding and zero-copy `RingBuffer` concepts are integrated natively in `userland/sigma_audio`.
