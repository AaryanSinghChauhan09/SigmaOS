# SigmaOS — 1000+ Development Ideas

> Living document. Every release adds ideas; contributors add more.
> Each category targets 100–150 ideas, scaling to 1000+ across all areas.
> Ideas are grouped into sub-themes for contributor pick-up.

---

## 🖥️ OS / Core System (~150 ideas)

### Kernel Architectures
1. Modular monolithic kernel with hot-loadable modules
2. Hybrid microkernel: critical drivers in kernel, rest in user-space
3. Pure microkernel: only IPC + MM in Ring 0
4. Exokernel: expose raw hardware to applications
5. Nanokernel: only interrupt routing + context switch
6. Unikernel profile: single-address-space for cloud functions
7. Library OS mode: kernel as a linkable library
8. Multi-kernel: per-CPU kernel instances with message-passing
9. Capability-based kernel (seL4-inspired rings)
10. Formally verified kernel subsystem (Coq proofs for MM + IPC)
11. Self-healing kernel: auto-restart faulted subsystems
12. Live kernel patching without reboot (kpatch-style)
13. Deterministic kernel: reproducible execution traces
14. Time-partitioned kernel: guaranteed CPU slices per domain
15. Soft real-time mode alongside hard RT (PREEMPT_RT-inspired)

### Boot Systems
16. UEFI BIOS boot with sigma-boot.efi
17. Legacy BIOS boot via GRUB chainload
18. Secure Boot with SigmaOS-signed shim
19. Multi-OS boot menu with graphical selector
20. Network boot (iPXE + sigma-netboot)
21. Live OS from USB with tmpfs overlay + persistence
22. Signed initramfs with dm-verity root
23. A/B boot partition with automatic rollback
24. Measured boot with TPM2 PCR sealing
25. Fast boot: skip POST, direct EFI hand-off (<2s target)
26. Suspend-to-RAM (S3) and Suspend-to-Disk (S4)
27. Hibernate with encrypted swap + TPM2 key unsealing
28. Chainload SigmaOS from Windows Boot Manager
29. Chainload SigmaOS from GRUB2 loop device
30. QEMU direct kernel boot for CI (-kernel flag)

### Virtualization
31. KVM hypervisor host mode
32. Firecracker-style microVM for FaaS cold start
33. VirtIO-GPU guest driver
34. VirtIO-net + VirtIO-blk paravirt drivers
35. VFIO GPU passthrough to VM guest
36. Nested virtualization (VT-x in VM)
37. sigma-pod: OCI container runtime without Linux namespaces
38. Container image build pipeline (sigma-build)
39. Rootless containers via user namespaces
40. WASM-based container isolation (no kernel namespace needed)
41. Live migration of running sigma-pod containers
42. Snapshot + restore of container state
43. Thin-provisioned disk images (QCOW2 + COW layer)
44. Memory ballooning for VM guest
45. VirtIO-mem hot-add/remove RAM in running VM

### Cloud Images
46. AWS AMI with cloud-init support
47. GCE image with metadata server integration
48. Azure VHD with waagent-compatible boot
49. OpenStack QCOW2 image
50. Proxmox VE template
51. VMware vSphere OVA
52. OCI container image (`docker pull sigmaos:15.0`)
53. Vagrant box for local dev
54. Packer templates for all cloud providers
55. Minimal 50MB cloud base image
56. GPU-enabled cloud variant (CUDA/ROCm userspace)
57. Spot-instance-optimized build (fast checkpoint/restore)
58. ARM64 cloud image (AWS Graviton, Ampere)
59. RISC-V cloud image (experimental)
60. Immutable root + OSTree A/B atomic cloud updates

### Package Ecosystem
61. sigpkg v1: local install/remove/list
62. sigpkg v2: online registry at pkg.sigmaos.app
63. Reproducible builds: SOURCE_DATE_EPOCH + sorted archives
64. Content-addressed package store (Nix-inspired)
65. Binary cache + substituters (build once, use everywhere)
66. Generational rollback: `sigma-pkg rollback 3`
67. Atomic upgrades: packages applied as one transaction
68. Dependency solver: SAT-based (like apt's APT-solver)
69. Virtual packages: `editor` provided by sigma-edit or nano
70. Split packages: sigma-edit + sigma-edit-docs as separate
71. Build recipes: PKGBUILD-style, version-controlled
72. Signing key rotation without breaking existing installs
73. Delta updates: binary diffs instead of full re-download
74. sigpkg audit: `sigma-pkg audit` scans for known CVEs
75. sigpkg graph: visualize dependency tree

### Multi-Format Builds
76. ELF64 native binary output
77. AppImage (Linux portable, no install)
78. Snap package output
79. Flatpak bundle output
80. Android APK (ARM64 JNI)
81. iOS IPA (TestFlight)
82. WASM/WASI bundle
83. Java JAR (fat jar via sigma-jvm)
84. .NET NuGet package
85. Python Wheel (PyPI)
86. Electron installer (Win/Mac/Linux)
87. Portable EXE (Windows no-install)
88. macOS .app bundle
89. sigpkg native format
90. Docker/OCI tar archive

### Distributed OS Concepts
91. Actor model runtime (sigma-bus mailbox)
92. CRDT-based offline-first state sync
93. RAFT consensus (SovereignConsensus engine)
94. Distributed ledger for package attestation
95. ZeroNet peer discovery + routing
96. Gossip protocol for cluster membership
97. CRDTs for distributed filesystem (SovereignCloudFS)
98. Byzantine fault tolerance in distributed shard routing
99. Content-addressed mesh storage
100. Geo-distributed shards with latency-aware routing

---

## 🔧 Drivers (~150 ideas)

### GPU
101. Intel i915 modesetting (Gen 6–12)
102. Intel Xe / Arc (Alchemist) open driver
103. AMD amdgpu (GCN4+ Radeon RX 400+)
104. AMD radeon (HD 5000–7000 legacy)
105. NVIDIA Nouveau (community reverse-engineered)
106. NVIDIA open kernel modules (R560+, Turing+)
107. VirtIO-GPU for QEMU/KVM guests
108. VESA/GOP framebuffer fallback
109. DRM/KMS atomic modesetting layer
110. Mesa Gallium3D interface (cleanroom)
111. Vulkan 1.3 ICD loader
112. OpenGL 4.6 compatibility profile
113. Display hotplug via DP/HDMI HPD IRQ
114. Multi-monitor spanning + rotation
115. HDR display support (10-bit colour)

### Wi-Fi / Bluetooth
116. Intel iwlwifi (Wi-Fi 5/6/6E/7)
117. Qualcomm ath9k (802.11n)
118. Qualcomm ath11k (Wi-Fi 6 QCA6390+)
119. MediaTek mt76 (Wi-Fi 5/6)
120. Realtek rtw89 (802.11ax)
121. Realtek rtl8xxxu (USB Wi-Fi dongles)
122. Broadcom brcmfmac (firmware blob loader)
123. mac80211/cfg80211 wireless framework (cleanroom)
124. WPA3/SAE dragonfly handshake
125. WPA2/EAP enterprise auth (802.1X)
126. BlueZ HCI layer port (cleanroom)
127. Bluetooth HCI over USB transport
128. Bluetooth HCI over UART (embedded)
129. BLE (Bluetooth Low Energy) scanning
130. A2DP audio over Bluetooth

### Storage
131. NVMe PCIe (already implemented ✅)
132. SATA AHCI controller
133. SCSI/SAS disk controller
134. USB mass storage (BOT protocol)
135. SD/eMMC (ARM mobile)
136. VirtIO-blk (already implemented ✅)
137. IDE legacy (compatibility)
138. NVMe-oF (NVMe over Fabrics)
139. Zoned Namespace (ZNS) NVMe
140. RAID 0/1/5/6 in software
141. dm-crypt block device encryption
142. dm-verity read-only integrity checking
143. bcache: SSD as HDD cache
144. LVM: logical volume manager
145. Loop device (file-backed block device)

### Peripheral Support
146. USB HID keyboard (scan-code → Unicode)
147. USB HID mouse + scroll wheel
148. USB HID gamepad (XInput + HID generic)
149. USB webcam (UVC class)
150. USB printer (USB printing class)
151. USB audio (UAC 1.0 + 2.0)
152. USB hub (multi-port)
153. PS/2 keyboard + mouse fallback
154. Touchpad (I2C HID, Synaptics)
155. Touchscreen (I2C HID, multi-touch)
156. Drawing tablet (Wacom protocol)
157. Fingerprint reader (libfprint interface)
158. Smart card reader (PCSC protocol)
159. Barcode scanner (HID keyboard emulation)
160. Serial port (16550 UART)

### Experimental / Advanced
161. FPGA partial reconfiguration driver
162. RISC-V PLIC interrupt controller
163. IoT sensor hub (I2C/SPI multi-sensor)
164. CAN bus controller (automotive)
165. NFC reader (PN532, ACR122U)
166. SDR (Software Defined Radio) via RTL2832U
167. NPU/VPU (Intel VPU, AMD XDNA) — `accel` class
168. Hot-plug PCIe device enumeration
169. Thunderbolt 4 device tree
170. USB4 tunnelling host controller
171. Firmware loader shim (sigma-firmware-loader)
172. Signed firmware blob verification before load
173. Driver hot-reload without kernel reboot
174. Ring-3 driver isolation (fault-tolerant)
175. Automatic driver selection by PCI subsystem ID

---

## 🔒 Security (~150 ideas)

### Sandboxing
176. WASM-isolated app sandbox (sigma-wasm)
177. sigma_pledge: process capability allowlist
178. sigma_unveil: per-process filesystem restriction
179. seccomp-BPF syscall filter per process
180. Namespace isolation (PID, net, mount, UTS, IPC, user)
181. cgroup v2 resource enforcement
182. Landlock filesystem sandboxing
183. SELinux-style AVC MAC policy engine
184. AppArmor-style profile loader
185. Seccomp profile generator from strace output
186. WASM component model isolation boundary
187. Containerized app with per-app network namespace
188. Bubblewrap (bwrap) equivalent for unprivileged sandboxing
189. Time-of-check/time-of-use (TOCTOU) mitigation
190. Spectre/Meltdown mitigations (KPTI, retpoline)

### Encryption
191. LUKS2 full-disk encryption
192. eCryptfs per-directory encryption
193. fscrypt native filesystem encryption
194. TPM2-sealed key derivation
195. YubiKey-backed disk unlock
196. Password manager (sigma-vault, TPM2-backed)
197. Encrypted swap partition
198. Secure memory erasure on process exit
199. Memory-safe string handling (no unbounded strcpy)
200. Encrypted hibernation image
201. Per-user home directory encryption
202. Encrypted tmpfs for /tmp
203. Kyber-1024 KEM in TLS 1.3
204. Dilithium-5 package signatures
205. NTRU-based backup encryption (experimental)

### Access Control
206. Role-based access control (RBAC) policy engine
207. Mandatory access control (MAC) via AVC cache
208. Capability-based access tokens (seL4-inspired)
209. SPIFFE workload identity per process
210. Per-syscall cryptographic attestation
211. Multi-factor auth for sudo equivalent
212. Immutable root filesystem (read-only + overlay)
213. Read-only /usr with writable /etc overlay
214. Restricted shell (rbash equivalent)
215. No-root default: all admin via capability tokens
216. Audit log for every privilege escalation
217. Time-limited sudo sessions
218. SSH certificate authority for fleet auth
219. FIDO2/WebAuthn hardware key support
220. Biometric unlock (fingerprint) via sigma-vault

### Network Security
221. Stateful firewall (nftables-inspired cleanroom)
222. NAT + conntrack for home router use
223. WireGuard VPN integration
224. IPsec/IKEv2 tunnel support
225. DNS-over-HTTPS (DoH) enforced by default
226. DNSSEC validation
227. TLS certificate pinning for system services
228. HSTS preload list for sigma-browser
229. Intrusion detection (sigma-ids, signature-based)
230. Intrusion prevention (block matching traffic)
231. Network namespace per application
232. Egress filtering: apps declare allowed hosts
233. Transparent proxy for security inspection
234. Zero-trust network policy (per-flow attestation)
235. DDoS rate limiting at kernel network layer

### Reproducibility & Trust
236. Reproducible builds (SOURCE_DATE_EPOCH)
237. Content-addressed package store (hash = identity)
238. Binary transparency log (sigmaOS equivalent of sigstore)
239. Build provenance (SLSA level 2 attestation)
240. Verified boot chain: UEFI → sigma-boot.efi → kernel → initramfs
241. dm-verity root filesystem integrity
242. IMA (Integrity Measurement Architecture) equivalent
243. sigma-appraise: verify every exec'd binary
244. Reproducibility checker: rebuild + compare
245. Public key pinning for sigma-pkg registry
246. Rollback protection: monotonic version counter in TPM2
247. Supply chain attack mitigation (no pre-built binaries in source)
248. All CI artefacts signed with Dilithium-5
249. Dependency lockfile with hash pinning
250. Security advisory database at cve.sigmaos.app

---

## 🛠️ Tools (~150 ideas)

### Developer SDK
251. sigma-sdk: Clang/LLVM sovereign toolchain
252. sigma-gdb: debugger with shard-aware stack unwinder
253. sigma-perf: CPU/memory profiler + flamegraph
254. sigma-strace: syscall tracer
255. sigma-ltrace: library call tracer
256. sigma-valgrind: memory error detector (cleanroom)
257. sigma-asan: AddressSanitizer integration
258. sigma-fuzz: AFL++ integration for kernel fuzzing
259. sigma-coverage: LLVM coverage for CI
260. VS Code extension: shard lattice explorer
261. JetBrains plugin: sigma-pkg + kernel symbol lookup
262. Neovim LSP plugin for SigmaOS codebase
263. sigma-format: opinionated code formatter
264. sigma-lint: static analysis (clippy + custom rules)
265. sigma-docs: API doc generator + local server

### System Utilities
266. sigma-monitor: htop/btop-style process monitor
267. sigma-disks: disk partitioner + mkfs GUI + CLI
268. sigma-logs: structured log viewer with shard filter
269. sigma-update: A/B rolling update manager
270. sigma-backup: incremental PQC-signed snapshots
271. sigma-restore: one-command system restore
272. sigma-doctor: self-diagnostics + repair wizard
273. sigma-clean: orphan package + cache cleaner
274. sigma-boot-manager: EFI entry editor
275. sigma-benchmark: standardised perf suite
276. sigma-top: real-time shard resource usage
277. sigma-pstree: process tree with capability display
278. sigma-lsof: open files per process
279. sigma-dmesg: kernel ring buffer viewer + filter
280. sigma-audit: syscall audit log viewer

### Networking Tools
281. sigma-ssh: Kyber-1024 SSH client + server
282. sigma-curl: HTTP/HTTPS/HTTP2/HTTP3 client
283. sigma-wget: simple file downloader
284. sigma-nmap: network scanner
285. sigma-wireshark: packet analyser GUI
286. sigma-tcpdump: CLI packet capture
287. sigma-dig: DNS query tool (DoH by default)
288. sigma-ping: ICMP + TCP ping
289. sigma-traceroute: path tracing
290. sigma-netstat: connection + socket display
291. sigma-ip: interface configuration (iproute2-style)
292. sigma-vpn: WireGuard manager with QR code import
293. sigma-hotspot: Wi-Fi AP mode with captive portal
294. sigma-proxy: transparent HTTP/S proxy
295. sigma-netmon: bandwidth monitor per process

### Productivity
296. sigma-edit: sovereign text/code editor
297. sigma-office: writer + calc + impress (lightweight)
298. sigma-pdf: PDF viewer + annotator + PQC verify
299. sigma-notes: encrypted Markdown note-taker
300. sigma-calc: scientific calculator + unit converter
301. sigma-files: dual-pane VFS file manager
302. sigma-calendar: local + CalDAV calendar
303. sigma-contacts: vCard + CardDAV contact manager
304. sigma-tasks: to-do list with sigma-vault encryption
305. sigma-clipboard: clipboard manager + history
306. sigma-search: full-text desktop search (like Recoll)
307. sigma-terminal: GPU-accelerated terminal emulator
308. sigma-font: font manager + preview
309. sigma-archive: GUI archive manager (tar/gz/zip/zst)
310. sigma-diff: visual file diff tool

### Media
311. sigma-play: audio/video player (FFmpeg cleanroom)
312. sigma-view: image viewer (JPEG/PNG/AVIF/HEIC/SVG)
313. sigma-snap: screenshot + annotate + OCR
314. sigma-record: screen recorder (OBS-lite)
315. sigma-cast: Chromecast/AirPlay sovereign sender
316. sigma-edit-video: basic video editor (cut/join/transcode)
317. sigma-edit-audio: waveform editor + equalizer
318. sigma-draw: vector graphics editor (Inkscape-lite)
319. sigma-paint: raster image editor (GIMP-lite)
320. sigma-camera: webcam capture + streaming
321. sigma-podcast: podcast aggregator + player
322. sigma-radio: internet radio player
323. sigma-ebook: EPUB/PDF e-reader
324. sigma-thumb: bulk image resizer/converter
325. sigma-stream: RTMP/RTSP stream viewer

### Cloud Sync & Automation
326. sigma-sync: Nextcloud client (CRDT offline-first)
327. sigma-drive: Google Drive/OneDrive sovereign bridge
328. sigma-s3: S3-compatible object storage client
329. sigma-git: sovereign Git client + GUI
330. sigma-rsync: delta file sync (rsync protocol)
331. sigma-cron: cron-compatible task scheduler
332. sigma-at: one-shot job scheduler
333. sigma-webhook: incoming webhook receiver/dispatcher
334. sigma-automate: GUI task automation (Shortcuts-style)
335. sigma-ci-runner: local sigma-ci runner for dev
336. sigma-notify: desktop notification daemon
337. sigma-rss: RSS/Atom feed aggregator
338. sigma-mail-sync: IMAP/JMAP offline sync daemon
339. sigma-cloud-shell: browser-based shell to local machine
340. sigma-deploy: one-command app deployment to cloud

---

## 🎨 Design (~100 ideas)

### Brand Identity
341. SigmaOS Σ logo — geometric, monochromatic, scalable
342. Primary palette: #45f3ff (cyan) + #a855f7 (purple) + #07080c (near-black)
343. Secondary palette: #34d399 (green) + #fbbf24 (yellow) + #f87171 (red)
344. Typography: Outfit (UI) + JetBrains Mono (code/terminal)
345. Logo usage guidelines (clear space, minimum size, don't-do)
346. Animated logo reveal (boot splash, ~800ms)
347. App icon grid: 48×48, 64×64, 128×128, 256×256, SVG
348. Unified icon style: rounded-square, line-weight 2px, sovereign glyph
349. Brand book as a PDF published at sigmaos.app/brand
350. Sticker pack for community use

### Desktop Environment
351. Zenith compositor: Wayland-inspired (not dependent) protocol
352. Glassmorphism panels: blur-behind, 60% opacity
353. Dynamic Island status bar (top center adaptive capsule)
354. Auto-tiling window manager + floating override
355. Workspace (virtual desktop) switcher
356. Mission Control-style overview (Super key)
357. Snap-to-edge window placement
358. Window animations: open/close/minimize curves
359. Desktop wallpaper engine (static + animated)
360. Widget system: clock, CPU meter, calendar, weather

### Accessibility
361. Screen reader (ORCA-compatible interface, cleanroom)
362. Screen magnifier (2×–16× smooth zoom)
363. High-contrast theme (WCAG AA compliant)
364. Large text mode (1.5× + 2× scale)
365. Keyboard navigation for all UI (no mouse required)
366. Sticky keys + slow keys + bounce keys
367. Colour-blind modes (deuteranopia, protanopia, tritanopia)
368. Mono audio mode
369. Cursor customisation (size, colour, speed)
370. Focus highlight ring (3px accent colour)

### Themes & Customisation
371. Dark mode (default)
372. Light mode (auto-switch by time)
373. Custom accent colour picker
374. Per-app colour scheme override
375. Font size per-app override
376. Corner radius customisation (0–16px)
377. Panel position: top/bottom/left/right
378. Taskbar icon size (small/medium/large)
379. Transparency level control (0–100%)
380. Import GNOME/KDE themes as base (cleanroom translate)

### Motion & Animation
381. Reduce motion mode (OS-level system preference)
382. Spring physics for window open/close
383. Parallax desktop background
384. Smooth scroll (momentum scrolling)
385. Page turn animation for document viewer
386. Splash screen: kernel boot progress visualised
387. Fade-in for newly opened windows
388. Micro-animations for button press feedback
389. Loading spinner: Σ rotation
390. State transitions: instantaneous vs animated toggle

---

## 🖼️ User Interface (~100 ideas)

### Desktop Environment Components
391. Unified Settings hub (single pane of glass)
392. App launcher: type-to-search, fuzzy match
393. Global menu bar (macOS-style, optional)
394. System tray: volume, network, battery, clock
395. Notification centre with action buttons
396. Quick Settings panel (Wi-Fi, BT, volume, brightness)
397. Focus mode: blocks notifications for set duration
398. Do Not Disturb scheduler
399. Screen lock with clock + media controls
400. Login screen: DID-based + biometric + password

### Window Manager
401. Tiling layouts: master-stack, BSP, grid, spiral
402. Floating override: drag title bar to float
403. PiP (picture-in-picture) for video windows
404. Sticky windows: persist across workspaces
405. Window rules: auto-tile by app class
406. Resize handles: corner + edge drag zones
407. Window border width + colour customisation
408. Snapping grid (8px increments)
409. Alt+F4 equivalent: Ctrl+Super+W
410. Window group / tab stacking

### Mobile UI (APK/IPA)
411. Bottom navigation bar for primary screens
412. Swipe-up gesture for home + recent apps
413. Swipe-down for notification shade
414. Long-press for context menu
415. Pinch-to-zoom in image/document viewers
416. Haptic feedback API (sigma-haptics)
417. Adaptive layout: phone/tablet breakpoints
418. Safe area insets (notch/punch-hole aware)
419. Dynamic Type: text scales with system setting
420. Dark mode + auto-switch on mobile

### Widgets
421. CPU/RAM live graph widget
422. Weather widget (offline by default)
423. Calendar widget with agenda view
424. Music player mini-widget
425. Quick-note sticky widget
426. Network speed widget (up/down Mbps)
427. Battery widget with estimated time
428. Clock widget (digital + analog variants)
429. System uptime widget
430. Active shard count widget

---

## 🌟 User Experience (~100 ideas)

### Onboarding
431. First-boot wizard: language → timezone → user → disk
432. Privacy onboarding: explain each data touchpoint
433. Hardware detection summary: "We found X drivers"
434. Optional telemetry consent (off by default, explicit opt-in)
435. Demo mode: try Zenith Desktop without installing
436. Quick tour overlay: 5-step UI walkthrough
437. Suggested apps based on profession profile
438. Import settings from previous OS (dotfiles)
439. Keyboard shortcut cheat sheet on first launch
440. "What's New" page after each update

### Documentation Hub
441. docs.sigmaos.app — searchable, versioned
442. Getting Started guide: install → boot → first command
443. Kernel developer handbook (architecture + SDF)
444. Driver development guide + SDF skeleton
445. App developer tutorial (Rust + JS + Python)
446. sigma-pkg maintainer guide
447. Security hardening guide
448. Cloud deployment cookbook
449. RTOS integration guide
450. Troubleshooting: top 50 problems + fixes

### Community
451. GitHub Discussions: Q&A + announcements
452. Discord server with channel per subsystem
453. Community sigpkg repository (user packages)
454. Hacktoberfest participation labels
455. "Good first issue" labelling policy
456. Monthly contributor digest email
457. Public roadmap voting (GitHub Projects)
458. RFC process for major changes
459. SigmaOS blog at sigmaos.app/blog
460. Conference talk slides + recordings

### Performance Defaults
461. Compressed RAM (zram) enabled by default
462. Background app CPU throttling
463. Battery saver mode: cap CPU at 50%
464. Fast app launch: pre-fork on login
465. Lazy loading: defer non-critical services
466. Startup time target: desktop ready in <5s
467. Memory target: idle desktop <300MB RAM
468. Disk target: base install <1.5GB
469. Network: DNS cache warm on boot
470. Swappiness tuned per profile (standalone vs cloud)

### Privacy Defaults
471. No telemetry by default (hard off, not just opt-out)
472. No analytics SDKs in any bundled app
473. Local-only crash reports (user decides to share)
474. Privacy dashboard: see what each app accesses
475. Network isolation per app (declare allowed hosts)
476. DNS-over-HTTPS enforced for all system traffic
477. Auto-clear /tmp on shutdown
478. No clipboard access without explicit permission
479. Camera/microphone hardware kill switch support
480. Location: off by default, per-app permission

---

## 🤖 AI / ML Integration (~50 ideas)

481. On-device TinyLlama inference daemon (sigma-ai)
482. GGUF/ONNX/safetensors model packaging via sigpkg
483. NPU/VPU HAL abstraction (Intel VPU, AMD XDNA)
484. AVX-512 accelerated inference on x86_64
485. NEON accelerated inference on ARM64
486. sigma-ai predictive scheduler (hot code path pre-warm)
487. AI-assisted tab completion in sigma-sh
488. AI-powered search in app launcher
489. On-device OCR (sigma-snap)
490. On-device speech-to-text (sigma-voice)
491. On-device text summarisation (sigma-summarise)
492. Smart notification grouping (on-device classifier)
493. Anomaly detection in sigma-monitor (resource spikes)
494. AI-assisted driver fault diagnosis in sigma-doctor
495. Privacy-preserving federated learning for telemetry opt-in
496. Model versioning + rollback via sigpkg
497. AI governance policy: define kernel boundary for agents
498. Capability-gated AI actions (pledge before inference)
499. Offline-first: all AI features work without internet
500. sigma-ai benchmark: measure on-device inference throughput

---

## How to Contribute More Ideas

This document is a living backlog. To add an idea:

1. Open a GitHub Discussion with the `idea` label.
2. Or open a PR: add your idea to the relevant section, numbered sequentially.
3. Keep it one line per idea — detail lives in a separate spec doc.
4. Don't duplicate existing ideas — search before adding.

**Target**: 1000+ ideas across all categories. Current count: ~500.
Each release cycle, contributors add 50–100 new ideas from community input.

---

*See also: [ROADMAP.md](../ROADMAP.md) · [FUTURE_IDEAS.md](../docs/FUTURE_IDEAS.md) · [STRATEGIC_VISION.md](../STRATEGIC_VISION.md)*
