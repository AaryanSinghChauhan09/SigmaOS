# Filesystem Internals

SigmaOS provides a robust, zero-allocation filesystem layer spanning from the unified cache to the block-level verification systems.

## Unified Buffer Cache (UBC)
Implemented in `sigma_ubc.rs`.
Caches block data in memory to dramatically accelerate I/O. Uses a Clock (second-chance) eviction algorithm, and maintains full telemetry for cache hits and misses without any dynamic memory allocations.

## Pre-Emptive Read-Ahead
Implemented in `sigma_readahead.rs`.
An adaptive engine that detects sequential file accesses and pre-fetches contiguous disk blocks into the UBC. It scales the prefetch window exponentially for sequential reads and falls back to a minimal window for random I/O.

## Asynchronous I/O (`io_uring`)
Implemented in `sigma_uring.rs`.
SigmaOS adopts the modern `io_uring` architectural pattern. Applications submit work to a Submission Queue (SQ) ring buffer and read results from a Completion Queue (CQ) ring buffer, eliminating synchronous syscall blocking.

## dm-verity Block Verification
Implemented in `sigma_dmverity.rs`.
Validates read-only rootfilesystems on a per-block basis. Uses a custom, sovereign SHA-256 implementation to hash data blocks as they are read and compares them against a Merkle tree. If tampering is detected, the block read fails, securing the supply chain.

## SigmaFS
Implemented in `sigma_mkfs.rs`.
The native filesystem format. Supports a simple superblock layout with fixed inode tables and a data bitmap, designed explicitly for high-performance `no_std` environments without complex journal overhead.
