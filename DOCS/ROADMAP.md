# SigmaOS Sovereign Lattice Roadmap & Triage

## 🚨 Phase 1: Critical Must-Haves (Immediate Focus)
These are essential for SigmaOS to be usable and competitive.
*   **Driver Support & Hardware Compatibility:** `drivers/` - GPU, Wi-Fi, printers, peripherals.
*   **Security & Trust:** `core/security/` - Secure Boot, TPM integration, WASM Sandboxing, Sovereign Vault encryption.
*   **App Ecosystem (WASM):** `plugins/wasm/` - The decentralized shard repository to replace APT/Snaps.
*   **UI Polish & Accessibility:** `ui/window_manager/` - High-DPI scaling, multi-monitor support, centered glassmorphic taskbar, integrated readers.
*   **Networking Stack:** `networking/` - VPN (WireGuard), enterprise networking.
*   **Update & Backup:** `services/` - Atomic OTA updates and timeline restores.
*   **Core Productivity:** Browser integration and essential WASM applications.

## ⚡ Phase 2: Medium-Term Imperatives (The Polish Phase)
These make SigmaOS universally appealing but aren’t boot-level blockers.
*   **Cloud Integration:** Native synchronization for network drives.
*   **Dual-Boot & Virtualization:** Cross-compatibility for KVM/Proton to ensure Adobe/Gaming functionality.
*   **Enterprise Tooling:** Provisioning and policy locking via `core/security/`.
*   **Cross-Architecture Builds:** Ensure the core boots flawlessly on ARM and RISC-V.
*   **Monitoring & Power:** Task orchestration, battery/ACPI management, and hybrid GPU switching.

## 🌱 Phase 3: Long-Term Vision (Market Dominance)
*   **Shard Ecosystem:** A robust developer marketplace for community plugins.
*   **Enterprise Support & Certifications:** Proving the hardware/software lattice for enterprise adoption.
*   **Advanced Ergonomics:** Voice, gestures, and fluid macOS-tier aesthetics across all user interactions.
*   **Deep Cloud-Native Layers:** Built-in orchestration to replace Kubernetes bloat.
