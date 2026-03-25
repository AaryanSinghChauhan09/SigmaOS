# SigmaOS — Suggestions & Unimplemented Features Log

> **Purpose:** Track features that are planned, partially implemented, broken, or not working as intended. Updated continuously.

---

## 🔴 Critical — Not Working As Intended

| # | Feature | File/Module | Issue | Priority |
| --- | --------- | ------------- | ------- | ---------- |
| 1 | GPU Compositing Kernel | `kernel/gpu_compositor.cpp` | Stub only — no real WebGL/Vulkan bridge | CRITICAL |
| 2 | Bootloader Chain | `SigmaBootloader.asm`, `kernel/boot.asm` | ASM compiles but no real MBR/UEFI chain. No signed boot artifact produced. | CRITICAL |
| 3 | Real Process Scheduler | `kernel/sovereign_scheduler.c` | Round-robin implemented in C but not proven in a real bare-metal context | HIGH |
| 4 | Kernel Self-Healing | `SigmaKernelSelfHealing.cpp` | Logic present but hooks to actual kernel state are missing | HIGH |
| 5 | Network Stack End-to-End | `kernel/network_stack.c` | TCP/IP stack written in C but no packet-level test harness | HIGH |
| 6 | Virtual Machine Node (sigma_vm.html) | `userland/apps/sigma_vm.html` | Stub iframe — no real WASM-based QEMU or KVM bridge | HIGH |
| 7 | Forensics / Data Lab | `userland/apps/sigma_data_lab.html` | Very basic stub — needs hex editor, disk image analysis, ML pipeline | HIGH |
| 8 | IDE (sigma_ide.html) | `userland/apps/sigma_ide.html` | Basic textarea — needs AST, syntax highlight, compile & run via WASM | HIGH |
| 9 | Draw.io Module (sigma_draw.html) | `userland/apps/sigma_draw.html` | Minimal stub — no canvas-based diagram editor | MEDIUM |
| 10 | NetCut Network Tool | `userland/apps/sigma_netcut.html` | UI-only stub — needs Web Networking API integration | MEDIUM |
| 11 | Bare-Metal UEFI Boot | `kernel/efi_main.c` | EFI stub exists but lacks full secure boot signature and protocol loading | CRITICAL |
| 12 | Hardware Driver Layer | `kernel/drivers/*` | Stubs for actual PCI/USB device enumeration and handling | CRITICAL |

---

## 🟡 Partially Working — Needs Improvement

| # | Feature | File/Module | Current State | Suggestion |
| --- | --------- | ------------- | --------------- | ------------ |
| 1 | Omni-Search Legal Refs | `index.html` SigmaWebKernel | Only 5 hardcoded legal refs | Add dynamic lookup from full BNS/BNSS JSON database |
| 2 | System Monitor (sigma_monitor.html) | `userland/apps/sigma_monitor.html` | Shows simulated CPU/RAM | Use `performance.memory`, `requestAnimationFrame`, Worker API for real metrics |
| 3 | Terminal (sigma_terminal.html) | `userland/apps/sigma_terminal.html` | Basic command echo — no real shell engine | Add built-in shell parser with VFS commands (`ls`, `cat`, `cd`, `mkdir`, `rm`, process management) |
| 4 | File Manager (sigma_files.html) | `userland/apps/sigma_files.html` | Uses localStorage VFS | Add OPFS (Origin Private File System) API for real file persistence |
| 5 | Music Player (sigma_music.html) | `userland/apps/sigma_music.html` | Functional but no equalizer | Add Web Audio API equalizer, visualizer, playlist queue |
| 6 | Gallery (sigma_gallery.html) | `userland/apps/sigma_gallery.html` | Works for images | Add EXIF metadata reader, slideshow, zoom, WASM image processing |
| 7 | Settings (sigma_settings.html) | `userland/apps/sigma_settings.html` | Comprehensive but many toggles non-persistent | Persist all settings to OPFS/localStorage with live apply |
| 8 | Clock (sigma_clock.html) | `userland/apps/sigma_clock.html` | Timer/stopwatch works | Add world clock, sunrise/sunset API (offline, formula-based) |
| 9 | Calculator (sigma_calc.html) | `userland/apps/sigma_calc.html` | Scientific calc works | Add matrix solver, equation plotter, unit converter integration |
| 10 | Writer (sigma_writer.html) | `userland/apps/sigma_writer.html` | Basic rich text | Add markdown export, spell check (WASM), table insertion, word count |
| 11 | Package Manager | `userland/system_api/package_manager/` | Stub directory | Implement SIGMA-PKG protocol: declare, fetch, verify, install app packages from manifest |
| 12 | Window Snap | `index.html` | Drag-drop works | Add keyboard snap (Win+←/→), quarter-tile snap zones |
| 13 | Notification Center | `index.html` | NC panel opens | Add persistent store, dismiss all, categorization, sound notification |
| 14 | App Drawer Categories | `index.html` | Single flat list | Add categories: System, Productivity, Finance, Legal, Games, Education |
| 15 | Wallpaper Engine | `index.html` cycleWallpaper | 4 color palettes | Add animated WebGL wallpapers, user-uploadable background images |
| 16 | AppArmor / Seccomp Policy | `kernel/security.c` | Design complete | Implement actual syscall filtering and filesystem access control lists |

---

## 🟢 New Feature Suggestions (Not Yet Started)

| # | Feature | Rationale | Implementation Path |
| --- | --------- | ----------- | --------------------- |
| 1 | **SigmaStore (App Store)** | Competitors: Mac App Store, Snap, Flatpak | JSON-based app catalogue, iframe-installable app manifests, ratings, search |
| 2 | **Sigma Cloud Drive** | Competitors: iCloud, Google Drive, OneDrive | IndexedDB + optional WebDAV sync to self-hosted backend |
| 3 | **Sigma AI Assistant (Aether)** | Competitors: Copilot, Siri, Gemini | Local WebLLM/ONNX inference for offline AI queries |
| 4 | **Split-Screen Multitasking** | Competitors: Windows 11 Snap Layouts | Drag window to screen edge — auto-tile in halves/quarters |
| 5 | **Sigma Clipboard History** | Competitors: Windows Clipboard History (Win+V) | Ring-buffer of last 50 clipboard items with search and pin |
| 6 | **Virtual Desktops** | Competitors: macOS Spaces, Windows Task View | Multiple desktop contexts switcheable via Ctrl+D or gesture |
| 7 | **Sigma Shell Scripting** | Competitors: Bash, PowerShell, Zsh | `.sigma` script format: sequential commands, variables, loops — interpreted in JS |
| 8 | **Encrypted Vault** | Competitors: VeraCrypt, BitLocker | AES-256 in-browser vault for sensitive documents using WebCrypto API |
| 9 | **WASM C/C++ Runtime** | Competitors: Windows Subsystem for Linux | WASM-compiled POSIX layer: run `.c` files compiled via Emscripten in browser |
| 10 | **Live Collaboration** | Competitors: Google Docs real-time | WebRTC data channel for two-user simultaneous document editing |
| 11 | **Thermal & Battery Monitor** | Competitors: HWMonitor, iStat Menus | `getBattery()` API + `performance.now()` heat estimation |
| 12 | **SigmaOS PWA install** | Competitors: ChromeOS | Service Worker + Web App Manifest for offline-installable PWA |
| 13 | **Sigma Package Builder** | Competitors: `dpkg`, `rpm`, `makepkg` | GUI: select JS/HTML files → bundle into `.spkg` manifest → upload to SigmaStore |
| 14 | **Quantum BI Enhanced Charts** | Competitors: Tableau, PowerBI | Radar chart, heatmap, treemap, Sankey diagram, geospatial pins added to Sigma BI |
| 15 | **Sigma Kernel Benchmark Suite** | Competitors: Geekbench, sysbench | In-browser: CPU single/multi-core score, memory bandwidth, JS JIT speed, storage IOPS |
| 16 | **NCERT Simulation Hub** | Competitors: PhET, CK-12 | Unified launcher for all NCERT simulation apps with progress tracking |
| 17 | **Sigma DevOps Console** | Competitors: Jenkins, GitHub Actions | YAML-based pipeline definition, local runner via Service Worker, step log viewer |
| 18 | **Sigma Legal Document Builder** | Competitors: LegalZoom, Vakil Desk | Template-based: NDA, Employment Agreement, MOU, MOA with Indian law defaults |
| 19 | **SigmaOS Theme Marketplace** | Competitors: GNOME Look, KDE Store | JSON-based theme manifests: colors, fonts, icon packs — live preview + apply |
| 20 | **Gesture Navigation** | Competitors: macOS trackpad, GNOME 3 | 3-finger swipe: switch virtual desktops; 4-finger: toggle omni-search |
| 21 | **Sigma Screen Recorder** | Competitors: OBS, ShareX | MediaRecorder API: capture full screen to WebM, download or save to Sigma Cloud |
| 22 | **Sigma VoIP** | Competitors: Signal, Teams | WebRTC audio/video call between two SigmaOS users via mesh P2P |
| 23 | **Sigma Bootable ISO Builder** | Competitors: Rufus, Etcher | Generate a valid `.iso` file with the current SigmaOS state using BIOS boot sector |
| 24 | **Sigma Hardware Abstraction Test** | Competitors: hwinfo, inxi | WebSerial, WebHID, WebUSB: enumerate and communicate with attached hardware |
| 25 | **Sigma BPELX Workflow Engine** | Competitors: Camunda, n8n | Drag-and-drop microservice pipeline editor with event triggers and conditions |
| 26 | **Sigma Container Broker** | Competitors: Docker, Podman | Local WebContainers layer to run pure Linux images natively inside SigmaOS |
| 27 | **Network Sniffer / PCAP Analyzer** | Competitors: Wireshark | WebSockets raw port capture with local PCAP parsing, protocol decoding |

---

## 🔵 Low-Level Language Improvement Suggestions

| # | Component | Suggestion |
| --- | ----------- | ------------ |
| 1 | `SigmaLibC.c` | Implement `mmap()`-based allocator, `brk()`/`sbrk()` syscall shims, `memcpy_sse2()` with SIMD |
| 2 | `SigmaRustCore.rs` | Add `#[no_std]` compliant ring buffer, lock-free MPSC channel, MMAP file reader |
| 3 | `SigmaBootloader.asm` | Produce valid 512-byte MBR; add UEFI stub loader in C calling Assembly |
| 4 | `kernel/process_scheduler.c` | Implement CFS-like virtual runtime fairness; add `SCHED_DEADLINE` policy |
| 5 | `kernel/synchronization.c` | Futex-based locking, test-and-set spinlocks in inline asm, ticket lock |
| 6 | `kernel/network_stack.c` | Implement ARP, ICMP echo, DNS resolver, add checksum in assembly |
| 7 | `kernel/mmu_core.c` | Implement 4-level page table walk, TLB flush in asm, huge-page support |
| 8 | `kernel/interrupt_handler.c` | IDT entries wired to ISR stubs in asm, EOI to APIC, spurious IRQ handling |
| 9 | `SigmaConcurrencyZenith.cpp` | Use `std::atomic`, CAS loops, implement a custom read-copy-update (RCU) |
| 10 | `SigmaOOP.hpp` | Add multiple-inheritance diamond resolution, CRTP patterns, concept-based polymorphism |
| 11 | `kernel/block_io.c` | Implement elevator algorithm, asynchronous request queueing, NVMe polling |
| 12 | `kernel/fs/ext4.c` | Native EXT4 read/write support, journaling parsing, inode metadata cache |

---

## 🏆 Competitor USPs to Absorb

| Competitor | USP | SigmaOS Target |
| ----------- | ----- | ---------------- |
| macOS | Mission Control, Spotlight, Continuity | Virtualized desktops + Omni-Search + PWA handoff |
| Windows 11 | Snap Layouts, WSL2, DirectStorage | Snap zones + WASM Linux + IndexedDB fast IO |
| Ubuntu | APT package mgmt, LTS reliability, GNOME | SigmaStore SPKG + 5-year compat policy + Morphic Desktop |
| Arch Linux | AUR, rolling release, pacman | Sigma AUR-like community manifests + rolling app updates |
| NixOS | Declarative config, reproducible builds | `.sigma` config format, deterministic app bundles |
| ChromeOS | Browser-native, PWA-first, edu focus | SigmaOS is already browser-native; add Classroom integration |
| Android | Notification tray, app widgets, Share API | Notification Center + Widget board + Web Share Target API |
| Tails OS | Amnesic, privacy-by-design, Tor | Sigma Privacy Mode: no localStorage, all in-memory, Tor proxy option |
| QubesOS | VM-based compartmentalization | Sigma Silo: per-app sandboxed iframes with CSP isolation |
| HaikuOS | Lightweight, responsive, single-user | Sigma Minimalist Mode: strip to 5 essential apps, 2MB footprint |

---

Last Updated: *2026-03-25 — by SigmaOS Sovereign Dev Engine*
