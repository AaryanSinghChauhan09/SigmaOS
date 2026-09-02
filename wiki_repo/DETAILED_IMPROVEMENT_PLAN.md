# SigmaOS Detailed Improvement Plan

This document outlines the systematic, zero-dependency improvement plan for SigmaOS across kernel, userspace, package management, security, and desktop compositing subsystems.

## 1. Storage & Memory Architecture

*   Buddy allocator page checkpointing and recovery.
*   Merkle CoW tree filesystem (`SigmaFS`) with CRC32C signed commit blocks.
*   Multi-tier extent migration (`bcachefs` style).

## 2. Process Control & Supervision

*   Decoupled process watchdogs (`S-INIT` / `S6` / `Dinit` style).
*   OpenBSD `pledge` and `unveil` process isolation.
*   FreeBSD `Jails` and `Capsicum` descriptor capability delegation.

## 3. Package Management & Distro Parity

*   Content-Addressed Storage (`CAS`) package store.
*   DPLL SAT dependency constraint solver.
*   Universal package translation adapters (`.deb`, `.rpm`, `.pkg.tar.zst`, `.apk`, `.xbps`, `.pkg`).

## 4. Zenith Desktop Compositor & Rendering Engine

*   Direct-to-framebuffer DMA-BUF direct scanout blitting for zero-copy low-latency rendering.
*   HiDPI fractional scaling and Variable Refresh Rate (VRR / FreeSync) adaptive frame pacing.
*   Tiling window manager layout matrices and workspace transitions.
*   Multi-monitor virtual desktop bounds and gesture hot corners.
