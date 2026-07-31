# SigmaOS Distro Absorption: Void Linux (Micro-second Init & XBPS Engine)

## 1. Overview
SigmaOS absorbs Void Linux's runit init speed and non-blocking service supervision to deliver boot times under 5ms (`siginit`).

## 2. Technical Design
- **Non-blocking Event Loop**: Asynchronous service dependency resolution.
- **Micro-Daemon Supervision**: Minimal memory footprint per active service (< 100 KB).
- **Fast Package Indexing**: Binary package delta updates with sub-millisecond execution.
