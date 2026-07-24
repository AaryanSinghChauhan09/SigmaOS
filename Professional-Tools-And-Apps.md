# SigmaOS Professional Tools & Applications

> Ecosystem layer: the tools and apps that make SigmaOS usable every day.
> Status reflects v15.0 baseline. Target versions in brackets.

---

## 🛠️ Developer Suite

| Tool | Description | Status | Build Target |
|------|-------------|--------|-------------|
| **sigma-sdk** | Compiler toolchain (Clang/LLVM sovereign build), headers, linker scripts | 🔄 [v1.0] | All profiles |
| **sigma-gdb** | GDB port with shard-aware stack unwinder | ⬜ [v1.0] | standalone |
| **sigma-perf** | CPU/memory profiler, flamegraph output | ⬜ [v1.0] | standalone |
| **sigma-pkg** | Sovereign package manager — install/remove/update/search | 🔄 [v0.1] | All |
| **sigma-build** | Multi-format build pipeline (ELF→AppImage→APK→WASM from one spec) | 🔄 [v1.0] | CI/CD |
| **sigma-container** | sigma-pod — OCI-compatible container runtime | ✅ [v15.0] | cloud |
| **sigma-ide-plugin** | VS Code extension — shard lattice explorer, sigma-pkg GUI, kernel symbol lookup | ⬜ [v1.0] | dev |
| **sigma-ci** | GitHub Actions workflow templates for SigmaOS projects | ✅ [v15.0] | CI |

---

## 🖥️ System Utilities

| Tool | Description | Status |
|------|-------------|--------|
| **sigma-monitor** | Process + resource monitor (CPU, RAM, I/O, shard health) — like htop/btop | 🔄 [v0.1] |
| **sigma-disks** | Disk partitioner + filesystem formatter (GUI + CLI) | ⬜ [v1.0] |
| **sigma-logs** | Structured log viewer with shard filtering + severity levels | ⬜ [v1.0] |
| **sigma-firewall** | Stateful firewall manager — rules GUI + CLI | ✅ [v15.0] |
| **sigma-update** | Signed rolling/stable update manager with A/B rollback | 🔄 [v0.1] |
| **sigma-backup** | PQC-signed incremental snapshot manager | ⬜ [v1.0] |
| **sigma-doctor** | Self-diagnostics: shard health, memory, boot integrity | 🔄 [v15.0] |
| **sigma-virt** | Virtualization manager — create/manage sigma-pod VMs | 🔄 [v15.0] |
| **sigma-boot-manager** | EFI boot entry manager + dual-boot configurator | ⬜ [v1.0] |

---

## 🌐 Networking Tools

| Tool | Description | Status |
|------|-------------|--------|
| **sigma-ssh** | Sovereign SSH client + server (Kyber-1024 key exchange) | ⬜ [v1.0] |
| **sigma-vpn** | WireGuard-based VPN manager with GUI | 🔄 [v15.0] |
| **sigma-netctl** | Network interface manager (DHCP, static IP, Wi-Fi, DNS) | 🔄 [v15.0] |
| **sigma-curl** | HTTP/HTTPS client (PQC TLS 1.3 by default) | ⬜ [v0.1] |
| **sigma-nmap** | Network scanner port (profiling shard) | ⬜ [v1.0] |
| **sigma-wireshark** | Packet analyser — sovereign shard wrapper | ⬜ [v1.0] |

---

## 🔒 Security Suite

| Tool | Description | Status |
|------|-------------|--------|
| **sigma-vault** | Password manager + secrets store (TPM2-backed) | ✅ [v15.0] |
| **sigma-encrypt** | File/disk encryption (LUKS2 + Kyber-derived keys) | 🔄 [v15.0] |
| **sigma-sandbox** | WASM-isolated sandbox for untrusted apps | ✅ [v15.0] |
| **sigma-audit** | Syscall monitor + compliance logger (SELinux-style output) | ✅ [v15.0] |
| **sigma-2fa** | TOTP/FIDO2 authenticator | ⬜ [v1.0] |
| **sigma-trustd** | TPM2 remote attestation daemon | 🔄 [v15.0] |
| **sigma-hardened** | Security hardening wizard (applies pledge/unveil policies) | ⬜ [v1.0] |

---

## 📄 Productivity Applications

| App | Description | Status |
|-----|-------------|--------|
| **sigma-edit** | Sovereign text/code editor (nano + micro inspired, < 1 MB) | ⬜ [v0.1] |
| **sigma-office** | Lightweight office suite: writer, spreadsheet, presentation | ⬜ [v1.0] |
| **sigma-pdf** | PDF viewer + annotator (PQC signature verification) | 🔄 [v15.0] |
| **sigma-notes** | Markdown note-taker with encrypted sync | ⬜ [v1.0] |
| **sigma-calc** | Scientific calculator with unit converter | ⬜ [v0.1] |
| **sigma-files** | Sovereign VFS file manager — dual-pane, bookmark, search | ⬜ [v1.0] |

---

## 🌍 Web & Communication

| App | Description | Status |
|-----|-------------|--------|
| **sigma-browser** | Custom Chromium fork with `navigator.sigmaos.*` API | 🔄 [v1.0] |
| **sigma-mail** | Email client (JMAP/IMAP with PQC-signed messages) | ⬜ [v1.0] |
| **sigma-chat** | Matrix/XMPP client + optional IRC | ⬜ [v1.0] |
| **sigma-meet** | VoIP/video call (WebRTC + Kyber signaling) | ⬜ [v2.0] |
| **sigma-feed** | RSS/Atom feed reader | ⬜ [v1.0] |

---

## 🎵 Media

| App | Description | Status |
|-----|-------------|--------|
| **sigma-play** | Audio/video player (FFmpeg port) | ⬜ [v1.0] |
| **sigma-view** | Image viewer (JPEG, PNG, AVIF, HEIC, SVG) | ⬜ [v1.0] |
| **sigma-snap** | Screenshot + screen recorder | ⬜ [v1.0] |
| **sigma-cast** | Media casting (Chromecast / AirPlay sovereign) | ⬜ [v2.0] |

---

## ☁️ Cloud Sync

| App | Description | Status |
|-----|-------------|--------|
| **sigma-sync** | Nextcloud client (CRDT offline-first) | ⬜ [v1.0] |
| **sigma-drive** | OneDrive / Google Drive sovereign bridge | ⬜ [v2.0] |
| **sigma-cloudsync** | Native sovereign storage daemon | ✅ [v15.0] |
| **sigma-s3** | S3-compatible object storage client | ⬜ [v2.0] |

---

## 🎓 Educational Mode

| Feature | Description | Status |
|---------|-------------|--------|
| **sigma-learn** | Interactive OS tutorials built into the shell | ⬜ [v1.0] |
| **sigma-sim** | Kernel simulation mode — run scheduler/MM as user-space sim | ⬜ [v1.0] |
| **Syllabus integration** | 15 CS syllabi already in wiki (`Syllabus-*.md`) | ✅ [v15.0] |
| **sigma-sandbox (edu)** | Isolated student environments with sub-ms reset | ⬜ [v1.0] |

---

## 🧪 SigmaOS Labs (Experimental)

| Project | Description | Status |
|---------|-------------|--------|
| **SigmaOS RTOS Lab** | Bare-metal, POSIX-layer, exokernel RTOS variants | 🔄 [v2.0] |
| **SigmaOS Distributed Lab** | Blockchain-based distributed OS, actor model runtime | 🔄 [v2.0] |
| **SigmaOS Mobile Lab** | APK/IPA builds, hybrid Capacitor apps | 🔄 [v2.0] |
| **SigmaOS Quantum Lab** | PQC research: Kyber v2, NTRU experiments | 🔄 [v15.0] |
| **SigmaOS AI Lab** | On-device LLM (TinyLlama), sigma-ai scheduler | 🔄 [v15.0] |

---

*See also: [sigpkg-Spec](sigpkg-Spec.md) · [SDK-Guide](SDK-Guide.md) · [Component-Integration](Component-Integration.md) · [ROADMAP](Roadmap.md)*
