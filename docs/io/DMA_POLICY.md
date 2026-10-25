# SigmaOS DMA Management Policy

## Overview
The DMA Manager (`DmaManager`) governs physical buffer scatter-gather mapping, cache coherence, ring descriptors, and in-flight tracking.

## Requirements & Enforcements
- **Race-Free Mapping**: Synchronized allocation and release of DMA descriptors.
- **Double-DMA Protection**: Prevents double-mapping or double-starting an active buffer (`DmaError::DoubleDmaPrevented`).
- **Coherence & Scatter-Gather**: Validates scatter-gather list addresses and lengths before issuing transfers (`DmaError::InvalidScatterGatherList`).
- **Queue Depth Bounds**: Enforces maximum queue depth (`DmaError::QueueFull`).
- **Resource Recovery**: Reclaims descriptors and physical mappings immediately upon completion or cancelation.
