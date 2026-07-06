# SigmaOS Distribution Formats

SigmaOS is distributed in 50+ formats to cover every deployment scenario — from bare-metal OS to mobile app, from RTOS to cloud microservice.

---

## How to Get SigmaOS

```bash

# Interactive format selector

sigma-pkg formats list

# Download a specific format

sigma-pkg download --format appimage
sigma-pkg download --format rtos-microkernel
sigma-pkg download --format cloud-oci
```

Or visit: https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/DOWNLOAD.md

---

## 📦 Application Formats

Run SigmaOS tools and apps on any host OS.

| Format | Command | Description |
|--------|---------|-------------|
| **Native (sigpkg)** | `sigma-pkg install <app>` | SigmaOS-native signed package |
| **AppImage** | `./SigmaApp.AppImage` | Self-contained Linux bundle |
| **Flatpak** | `flatpak run io.sigmaos.*` | Sandboxed Linux app |
| **Snap** | `snap install sigma-tools` | Canonical store format |
| **Electron** | `./sigma-app` | Cross-platform desktop app |
| **Java JAR** | `java -jar sigma.jar` | JVM-compatible tools |
| **.NET** | `dotnet sigma.dll` | .NET / Mono runtime |
| **Python** | `pip install sigmaos-sdk` | Python SDK and tools |
| **WASM** | Browser or wasmtime | WebAssembly bundle |
| **Mobile (APK)** | Play Store / sideload | Android application |
| **Mobile (IPA)** | App Store / TestFlight | iOS application |
| **ELF** | `./sigma-binary` | Native Linux/SigmaOS binary |

---

## 🖥️ Standalone OS Formats

Full SigmaOS as a standalone operating system.

| Format | Download | Use Case |
|--------|----------|---------|
| **Standalone ISO** | `sigmaos-zenith.iso` | Install to bare metal |
| **AppImage** | `sigmaos.AppImage` | Run without installing |
| **Portable EXE** | `sigmaos-compat.exe` | Windows compatibility layer |
| **Electron Desktop** | `sigmaos-electron.zip` | Demo on any host |
| **Python Bundle** | `sigmaos-py.zip` | Scripting + automation |
| **WASM Bundle** | `sigmaos.wasm` | Browser demo |

---

## ⚙️ RTOS Formats

Hard real-time variants for industrial, embedded, and safety-critical use.

| Variant | IRQ Latency | Description |
|---------|-------------|-------------|
| **Monolithic RTOS** | < 50 µs | Single binary, all drivers built-in |
| **Microkernel RTOS** | < 10 µs | Minimal kernel, ring-3 drivers |
| **Layered RTOS** | < 25 µs | HAL → BSP → RTOS layers |
| **Exokernel** | < 5 µs | Libos model, minimal abstractions |
| **POSIX RTOS** | < 100 µs | POSIX thread/timer API |
| **Bare-Metal** | < 1 µs | No OS, just sigma-hal + app |

```bash

# Build RTOS variant

make iso PROFILE=rtos SCHED=edf IRQ_LATENCY_TARGET=10
```

---

## 📱 Mobile Formats

SigmaOS on smartphones, tablets, and embedded displays.

| Format | Platform | Status |
|--------|----------|--------|
| **Native APK** | Android (ARM64) | Phase D |
| **Native IPA** | iOS (ARM64) | Phase D |
| **Hybrid HTML/JS/CSS** | Any WebView | Available |
| **Cross-Platform** | React Native / Flutter | Phase D |
| **PWA** | Any browser | Available |
| **Mobile Game Engine** | Unity / Godot bridge | Phase E |

---

## 🧩 Microkernel Variants

Different microkernel architectures for research and production.

| Variant | Size | IPC | Use Case |
|---------|------|-----|---------|
| **Pure** | < 64 KB | Message-passing only | High-assurance |
| **Hybrid** | < 256 KB | sigma-bus IPC | Production default |
| **Modular** | < 512 KB | Loadable modules | Desktop |
| **Exokernel** | < 32 KB | Direct hardware | Research |
| **POSIX Layer** | < 1 MB | POSIX sockets | Linux compat |

---

## 💻 Dual-Boot Formats

Run SigmaOS alongside existing OS installations.

| Format | Host | Method |
|--------|------|--------|
| **Traditional Partition** | Any | UEFI boot entry, separate partition |
| **Separate Disk** | Any | Dedicated SSD/NVMe |
| **Nested/Chainload** | Linux | GRUB chainload sigma-boot.efi |
| **Virtualized** | Windows/macOS/Linux | QEMU/Hyper-V/VMware |
| **Live OS** | Any | Boot from USB, no install |

---

## 🌐 Distributed Formats

Multi-node deployment for clusters and distributed computing.

| Format | Protocol | Description |
|--------|----------|-------------|
| **Client-Server** | gRPC (sigma.proto) | Traditional client-server |
| **Peer-to-Peer** | ZeroNet mesh | Decentralized nodes |
| **Clustered** | CRDT sync | Replicated state across nodes |
| **Grid** | sigma-pod orchestration | Workload distribution |
| **Cloud-Oriented** | REST/gRPC | Cloud-native microservices |
| **Distributed Ledger** | Dilithium-5 signed | Append-only audit chain |
| **Actor Model** | sigma-bus IPC | Message-passing actors |

---

## ☁️ Cloud Formats

Deployment targets for cloud infrastructure.

| Format | Registry | Description |
|--------|----------|-------------|
| **OCI Container** | ghcr.io/sigmaos | Docker-compatible container |
| **VM Image (QCOW2)** | GitHub Releases | KVM/QEMU virtual machine |
| **AWS AMI** | AWS Marketplace | Amazon Machine Image |
| **Azure VM Image** | Azure Marketplace | Microsoft Azure VM |
| **GCP Image** | GCP Marketplace | Google Cloud VM |
| **Kubernetes** | Helm chart | sigma-pod K8s deployment |
| **Serverless (FaaS)** | sigma-lambda | Function-as-a-Service |
| **IaaS** | Terraform modules | Infrastructure-as-Code |
| **PaaS** | sigma-deploy | Platform-as-a-Service |
| **SaaS** | sigma-cloud.io | Hosted SigmaOS service |

```bash

# Deploy to cloud

sigma-deploy aws --region ap-south-1 --profile cloud
sigma-deploy k8s --cluster my-cluster
sigma-deploy container --registry ghcr.io/sigmaos
```

---

## 🌍 Browser Formats

Run SigmaOS directly in a web browser.

| Format | Technology | Description |
|--------|------------|-------------|
| **Desktop Demo** | WASM + WebGL | Full Zenith desktop in browser |
| **Mobile Browser** | PWA | Progressive Web App |
| **Embedded WebView** | Electron / Tauri | Embedded in another app |
| **Headless** | Node.js WASM | CI/CD automation |
| **Minimalist** | ~1 MB WASM | Lightweight browser OS |

---

## 🖥️ Kernel Format Variants

Different kernel architectures compiled from one codebase.

| Format | Size | Features | Profile Flag |
|--------|------|---------|-------------|
| **Monolithic** | ~4 MB | All drivers in kernel | `PROFILE=monolithic` |
| **Microkernel** | < 512 KB | Ring-3 drivers via IPC | `PROFILE=microkernel` |
| **Hybrid** | ~2 MB | Core in kernel, opt drivers ring-3 | `PROFILE=standalone` |
| **Exokernel** | < 128 KB | Libos model, no abstractions | `PROFILE=exo` |
| **Nanokernel** | < 64 KB | Only scheduling + IPC | `PROFILE=nano` |
| **Modular** | Variable | `insmod`/`rmmod` driver loading | `PROFILE=modular` |
| **Monolithic + Modular** | ~3 MB | Core monolithic + loadable extras | `PROFILE=hybrid_mod` |

---

## Building All Formats

```bash

# Build everything

make all-formats

# Build specific format

make iso PROFILE=standalone
make iso PROFILE=rtos
make iso PROFILE=cloud
make wasm PROFILE=browser
make apk PROFILE=mobile

# CI: build matrix

# See .github/workflows/build.yml

```

---

## Release Schedule

| Format Group | Current Status | Next Milestone |
|---|---|---|
| Standalone ISO | ✅ v15.0.0 | v15.1 — boot + desktop |
| RTOS | 🔄 v15.0.0 partial | v16.0 — < 10 µs IRQ |
| Cloud OCI | 🔄 v15.0.0 partial | v15.1 — production image |
| Mobile | ⬜ Phase D | v17.0 |
| Browser WASM | 🔄 demo | v15.1 — full Zenith |
| Distributed | ⬜ Phase E | v17.0 Sovereign |

---

*See also: [PROFILES.md](../PROFILES.md) · [Building from Source](../wiki_repo/Building-from-Source.md) · [Release Notes](../RELEASE_NOTES.md)*
