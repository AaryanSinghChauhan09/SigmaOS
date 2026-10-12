# DMA Manager Policy

## Overview
The DMA Manager governs physical memory scatter-gather buffer mapping, total byte accounting, in-flight transfer tracking, and completion reclamation.

## Requirements
- DMA enqueue/dequeue is race-free.
- DMA reuse follows a defined completion policy.
- DMA metadata is bounded.
- Lost-DMA prevention is enforced via descriptor lookup.
- Double-DMA prevention is enforced per buffer ID (`DmaError::DoubleDmaPrevented`).
- Empty or invalid scatter-gather lists return `DmaError::InvalidScatterGatherList`.
