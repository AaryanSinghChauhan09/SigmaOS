# SigmaOS Future Development Roadmap

With the foundational kernel subsystems, `no_std` coreutils, bespoke package manager (`sigpkg`), and security hooks implemented, SigmaOS is transitioning from foundational OS architecture to high-level integration and user-facing features. 

The following phases outline the next 12-18 months of development for SigmaOS.

## Phase 8: Networking & Connectivity
**Goal:** Establish a robust networking stack without relying on the standard library.
- **TCP/IP Stack Integration:** Wrap and integrate `smoltcp` into the kernel to provide `no_std`, `no_alloc` networking.
- **Network Interface Drivers:** Implement e1000e (Intel Gigabit) and virtio-net drivers in the Sigma Driver Framework (SDF).
- **Socket IPC:** Bridge the kernel network stack with userland via a POSIX-like, zero-copy socket API.

## Phase 9: Post-Quantum Cryptography (PQC)
**Goal:** Solidify SigmaOS as a quantum-resistant platform.
- **Sigpkg PQC:** Replace the Ed25519 stubs in `sigma_pkg_repo.rs` with true Kyber/Dilithium implementations for package signing.
- **Kernel Keystore:** Create a secure, memory-isolated keystore for PQC keys.
- **Verified Boot:** Implement a post-quantum verifiable bootloader chain linking into `sigma_dmverity.rs`.

## Phase 10: AI LLM Integration
**Goal:** Bring the `sigma-agent` CLI to life with a local, offline LLM.
- **llama.cpp Port:** Port `llama.cpp` to run on top of SigmaOS's `no_std` APIs.
- **Agent IPC:** Connect the `sigma_ai_agent.rs` stub to the local LLM daemon via `sigma-bus`.
- **System Telemetry RAG:** Pipe kernel telemetry and `sigma_audit.rs` logs into a localized vector database for real-time, context-aware AI debugging.

## Phase 11: The Zenith Compositor & GUI
**Goal:** Deliver a modern, hardware-accelerated desktop experience.
- **KMS / DRM Subsystem:** Implement the Kernel Mode Setting and Direct Rendering Manager APIs.
- **GPU Drivers:** Provide basic frame-buffer and virtio-gpu drivers.
- **Wayland-Compatible Compositor:** Develop the Zenith Compositor using `no_alloc` paradigms, targeting 60FPS fluid UI with micro-animations.

## Phase 12: Hardware & Driver Ecosystem
**Goal:** Expand compatibility with bare-metal hardware.
- **USB Subsystem:** Implement xHCI controllers for USB 3.0 support.
- **Storage Controllers:** NVMe and AHCI (SATA) driver development.
- **Driver Sandboxing:** Enforce strict MAC policies (`sigma_mac.rs`) on third-party hardware drivers.
