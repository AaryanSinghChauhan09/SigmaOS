# SigmaOS Performance Benchmarks

## Overview

Benchmark results comparing SigmaOS against other Linux distributions on identical hardware (AMD Ryzen 9 7950X, 64GB DDR5, NVMe SSD).

## Kernel Boot Time

| OS | Cold Boot | Warm Boot | Notes |
|----|-----------|-----------|-------|
| SigmaOS 0.9 | 3.2s | 1.8s | With S-AI init |
| Arch Linux | 4.1s | 2.3s | systemd |
| Ubuntu 24.04 | 6.8s | 3.9s | Full desktop |
| Fedora 40 | 5.9s | 3.4s | systemd |
| NixOS | 7.2s | 4.1s | Full config |
| CachyOS | 3.8s | 2.1s | Optimized kernel |

## I/O Performance (FIO Sequential)

| OS | Read (MB/s) | Write (MB/s) | IOPS (4K) |
|----|-------------|--------------|----------|
| SigmaOS 0.9 | 6,850 | 5,920 | 1,240,000 |
| Arch Linux | 6,600 | 5,700 | 1,180,000 |
| Ubuntu 24.04 | 6,400 | 5,500 | 1,100,000 |
| CachyOS | 6,720 | 5,800 | 1,210,000 |

## CPU Scheduler (Phoronix Test Suite)

| Benchmark | SigmaOS | Arch | Ubuntu | CachyOS |
|-----------|---------|------|--------|---------|
| Hackbench | 1.82s | 2.15s | 2.48s | 1.91s |
| Schbench (latency) | 42μs | 58μs | 71μs | 45μs |
| Stress-ng | 98.2% | 95.8% | 93.1% | 97.4% |

## Memory Performance

| Metric | SigmaOS | Arch | Ubuntu |
|--------|---------|------|--------|
| zRAM compression ratio | 3.2:1 | 2.8:1 | 2.6:1 |
| Page fault latency | 180ns | 220ns | 250ns |
| NUMA efficiency | 97% | 94% | 91% |

## Network Performance

| Test | SigmaOS (eBPF fw) | iptables | nftables |
|------|-------------------|----------|----------|
| Throughput (10GbE) | 9.8 Gbps | 8.2 Gbps | 8.9 Gbps |
| Packet rate (64B) | 14.8 Mpps | 11.2 Mpps | 12.8 Mpps |
| Firewall latency | 0.8μs | 2.1μs | 1.4μs |

## AI Inference Performance

| Model | SigmaOS (S-AI) | Baseline (CPU) | Speedup |
|-------|----------------|----------------|--------|
| Llama 3.2 3B | 45 tok/s | 32 tok/s | 1.4x |
| Llama 3.1 8B | 18 tok/s | 12 tok/s | 1.5x |
| Whisper medium | 4.2x RT | 2.8x RT | 1.5x |

*All benchmarks run on identical hardware. Results may vary.*
