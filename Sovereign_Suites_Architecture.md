# Σ SIGMAOS: Sovereign Suites Architecture

This document defines the modular architecture of SigmaOS, organized into **Integrated Sovereign Suites**.

## 1. Sovereign Memory Suite
**Location**: `kernel/modules/core/SovereignMemorySuite.c`
**Components**:
- **PMM**: Physical Memory Manager (Frame allocator)
- **VMM**: Virtual Memory Manager (Demand paging, Page faults)
- **VMA**: Virtual Memory Area (Address space tracking)
- **AddressSpace**: Per-process shadow table management

## 2. Sovereign Cryptography Suite
**Location**: `kernel/modules/security/SovereignCryptoSuite.c`
**Components**:
- **Hashing**: SHA-256 and HMAC-SHA256 implementations
- **Ciphers**: ChaCha20 Stream Cipher
- **Entropy**: CSPRNG (Cryptographically Secure Pseudo-Random Number Generator)
- **KDF**: PBKDF2 (Password-Based Key Derivation Function)

## 3. Sovereign Application Management Suite
**Location**: `kernel/modules/core/SovereignAppManagement.c`
**Components**:
- **Packaging**: Universal shard deployment matrix
- **Containment**: Flatpak-inspired sandboxing engine

## 4. Sovereign Service Control Suite
**Location**: `kernel/modules/core/SovereignServiceControl.c`
**Components**:
- **Lattice**: PID-1 Init system and service lattice
- **LSD**: Large-Scale Service Daemon / Unit supervisor

## 5. Sovereign Intelligence Suite
**Location**: `kernel/modules/core/SovereignIntelligenceSuite.c`
**Components**:
- **AI Kernel**: Native LLM/ML acceleration shard
- **Neural Bridge**: Synaptic logic mapping
- **Tensor Core**: Hardware-accelerated matrix operations

## 6. Sovereign Frontend Suite
**Location**: `kernel/modules/core/SovereignFrontendSuite.c`
**Components**:
- **Display**: Window Manager and Compositor
- **Sound**: Audio Engine and Neural Synth
- **UI**: Hyprland-inspired Sovereign GUI

## 7. Sovereign Ecosystem Suite
**Location**: `kernel/modules/core/SovereignEcosystemSuite.c`
**Components**:
- **Legacy Absorbers**: XNU, Android Binder, Haiku, and Wine compatibility
- **Interoperability**: Binary translation and ABI bridging

## 8. Sovereign CLI Suite
**Location**: `kernel/modules/core/cli/`
**Components**:
- **Core Dispatcher**: O(1) Command Hash Dispatcher
- **Essential Commands**: Base OS utility suite (ls, cat, ps, etc.)

---
*Created by Antigravity AI on 2026-04-12*
