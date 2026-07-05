# SigmaOS Roadmap

## Phase 0 (0-3 months)
- Finalize architecture RFC (microkernel vs hybrid, async syscalls)
- Set kernel coding standards (Rust, unsafe usage rules)
- Implement minimal reproducible build pipeline & cross-toolchain
- Build boot-to-userspace demo:
  - Boot kernel
  - Spawn userspace process
  - Demonstrate simple async syscall & IPC
- Create driver SDK prototypes (WASM + Rust host ABI)
- Configure CI with cross-compilation, unit tests, nightly benchmarks
- Begin outreach to 3 hardware vendors for driver partnerships

## Phase 1 (3-9 months)
- Kernel v0: memory management, basic process model, IPC, async syscall interface
- Basic userspace: shell, minimal filesystem, package manager skeleton
- Linux-compat prototype (container runtime or syscall shim)

## Phase 2 (9-18 months)
- Stable driver model (WASM host)
- NIC and block drivers (bare minimum)
- Scheduler tuning
- Basic security features (secure boot)
- Mature developer tooling

## Phase 3 (18-36 months)
- Full userspace stack
- Production-grade filesystems
- GPU/graphics stack
- NVMe performance optimizations
- Enterprise-grade CI
- Fuzzing everywhere
- Formal verification for critical modules

```
Phase F  ████████████████████  100% ✅  (KMS, cgroup, pkg registry)
Phase G  ████████████████████  100% ✅  (kernel boot — COMPLETE)
Phase H  ████████████████░░░░  50% 🔄  (India Stack — ACTIVE)
  - sigma-health (ABDM FHIR): ✅
  - sigma-accounts (GST IRN): ✅
  - sigma-pay (UPI/NPCI): ✅
  - sigma-aadhaar (QR Auth): ✅
Phase I  ████████████████████  100% ✅  (Desktop & Drivers — COMPLETE)
  - Zenith Desktop Compositor: ✅
  - Auto-tiling Window Manager: ✅
  - Application Launcher: ✅
  - System Tray: ✅
  - Accessibility Features: ✅
  - i915 GPU Driver: ✅
  - iwlwifi Wi-Fi Driver: ✅
  - AMD amdgpu Driver: ✅
  - HDA Audio Driver: ✅
  - sigma-ai Daemon: ✅
  - 10 Bundled Applications: ✅
Stage 0 ████████████████████  100% ✅  (Bootable Foundation — COMPLETE)
  - Kernel Scheduler (MLFQ/CFS/EDF): ✅
  - Memory Manager (Buddy/Slab/ASLR): ✅
  - Interrupt Controller (APIC/PIC): ✅
  - Virtual Memory (4-level page tables): ✅
  - Syscall Gate (30 syscalls): ✅
  - UEFI Bootloader (sigma-boot.zig): ✅
  - Bootable ISO (build-iso.sh): ✅
Phase 5B ████████████████████  100% ✅  (Desktop Dominance — COMPLETE)
  - Window Manager (auto-tiling): ✅
  - Application Launcher (fuzzy search): ✅
  - System Tray (time/battery/network): ✅
  - Accessibility (WCAG AAA): ✅
  - sigma-edit (text editor): ✅
  - sigma-files (file manager): ✅
  - sigma-terminal (terminal): ✅
  - sigma-browser (web stub): ✅
  - sigma-mail (email client): ✅
  - sigma-calc (calculator): ✅
  - sigma-calendar (calendar): ✅
  - sigma-notes (note app): ✅
  - sigma-clock (system clock): ✅
  - sigma-settings (settings panel): ✅
Education ████████████████████  100% ✅  (CBSE & Professional — COMPLETE)
  - Virtual Lab (physics/chemistry/biology): ✅
  - Data Visualization (graphing/plotting): ✅
  - Symbolic Math Engine (algebra/calculus): ✅
  - Adaptive Practice (CBSE syllabus): ✅
  - Math Proof Assistant (step-by-step solver): ✅
  - AI Exam Paper Generator (NCERT aligned): ✅
  - Coding Playground (Python/C++/Java): ✅
  - Curriculum Projects (IT practicals): ✅
  - Multilingual Support (Hindi/Gujarati/Tamil/Bengali): ✅
  - Exam Prep (UPSC/SSC/GATE/NET): ✅
  - Math Visualization (GeoGebra style): ✅
  - Scientific Computing (Scilab/Octave): ✅
  - E-Learning Platform (Moodle): ✅
  - Digital Whiteboard (OpenBoard): ✅
  - Library Management (Koha): ✅
  - ERP System (ERPNext): ✅
Privacy ████████████████████  100% ✅  (Security & Anonymity — COMPLETE)
  - GnuPG Integration (encryption/signatures): ✅
  - Tor Integration (anonymous networking): ✅
Security ████████████████████  100% ✅  (IT Training — COMPLETE)
  - Cybersecurity Sandbox (malware/firewalls): ✅
  - Audit Trail Visualizer (logs/monitoring): ✅
  - Security Policy Advisor (AI best practices): ✅
  - Networking Simulator (TCP/IP/routing): ✅
Law & Governance ████████████████████  100% ✅  (Legal Professionals — COMPLETE)
  - Labour Code Explorer (Labour Law/OSH/Social Security): ✅
  - Case Law Database (Indian judgments): ✅
  - Legal Drafting Assistant (petitions/contracts): ✅
  - Policy Simulation (workplace law testing): ✅
Professional Tools ████████████████████  100% ✅  (Indian Sectors — COMPLETE)
  - Healthcare (medical data analysis): ✅
  - Engineering (CAD/circuit simulators): ✅
  - Finance (GST/TDS/TCS compliance): ✅
  - Agriculture (crop yield/soil health): ✅
  - Multilingual Office Suite (9 Indian languages): ✅
Sector Applications ████████████████████  100% ✅  (Enterprise Tools — COMPLETE)
  - Healthcare Records (OpenMRS): ✅
  - Engineering CAD (FreeCAD): ✅
  - GIS (QGIS): ✅
  - Accounting (GNUCash): ✅
  - Project Management (OpenProject): ✅
Integration ████████████████████  100% ✅  (Open-Source Tools — COMPLETE)
  - Package Manager (Nixpkgs/Flatpak/Homebrew): ✅
  - AI Coding Assistant (StarCoder/CodeGen): ✅
  - Version Control (Git integration): ✅
  - Security Tools (OpenSSL/KeePassXC): ✅
  - Office Suite (LibreOffice integration): ✅
```

## The Critical Path

Everything depends on `kernel-exp` shipping Phase 0:

1. `kernel-exp` → bootable kernel
2. `drivers-dev` → GPU + Wi-Fi drivers
3. `fs-dev` → VFS + SigmaFS
4. All `release/*` profiles become functional

## Quick Links

- [CURRENT_PROBLEMS_MANIFEST.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/CURRENT_PROBLEMS_MANIFEST.md)
- [FEATURE_MATRIX.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/FEATURE_MATRIX.md)
- [CONTRIBUTOR_ROADMAP.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/CONTRIBUTOR_ROADMAP.md)
- [GitHub Issues](https://github.com/AaryanSinghChauhan09/SigmaOS/issues)

## Recent Module Enhancements (July 2026)

### Core Sovereignty Tools
#### ZFS Filesystem (sigma_zfs.rs)
- Advanced filesystem with snapshots, compression, deduplication
- Pool management with VDev support and health monitoring
- Dataset management with property configuration
- Snapshot creation, cloning, and rollback functionality
- Scrub operations for data integrity verification

#### AppArmor/SELinux MAC (sigma_mac.rs)
- Mandatory access control with profile-based security
- File rules with permissions (read, write, execute, create, delete)
- Network rules with protocol and port filtering
- Capability management for Linux capabilities
- Enforcement modes: enforce, complain, kill, unconfined
- Security context management for processes

#### ClamAV Antivirus (sigma_clamav.rs)
- Virus signature scanning with multiple threat levels
- Heuristic analysis for suspicious patterns
- Quarantine management with restore/delete operations
- Real-time protection and auto-quarantine options
- Signature updates and exclusion rules
- Scan reports with threat details

#### Fail2Ban Intrusion Prevention (sigma_fail2ban.rs)
- Log monitoring with regex pattern matching
- IP banning with configurable ban duration
- Jail management for different services (SSH, HTTP, FTP)
- Whitelist and blacklist support
- Automatic ban expiration clearing
- Statistics and audit logging

#### KeePassXC Password Manager (sigma_keepass.rs)
- Secure credential storage with database encryption
- Password generation with customizable options
- Group organization for entries
- Search functionality across entries
- Import/export in JSON and CSV formats
- Recycle bin for deleted entries

### Networking
#### NetworkManager (sigma_networkmanager.rs)
- Unified network configuration for all devices
- WiFi management with scanning and connection
- Ethernet and device management
- IP configuration (auto, manual, link-local)
- Connection profiles with auto-connect
- DNS server configuration

#### dnsmasq/Unbound DNS (sigma_dnsmasq.rs)
- DNS resolver with caching for performance
- DHCP server with lease management
- Local DNS record management (A, AAAA, MX, TXT, etc.)
- Upstream DNS server configuration
- Domain blocking for ad/tracker filtering
- Domain forwarding to specific servers

### Developer Tools
#### Git Integration (sigma_git.rs)
- Version control with repository management
- Branch creation, checkout, and merging
- Staging and committing changes
- Remote repository management
- Commit log viewing
- Global configuration management

#### Podman/Docker Containerization (sigma_podman.rs)
- Container lifecycle management (create, start, stop, remove)
- Image management with pull operations
- Pod orchestration for grouped containers
- Network management (bridge, host, none)
- Volume mounting and port mapping
- Environment variable configuration
- Container statistics and logs

### Package Management
#### Flatpak/Sandbox (sigma_flatpak.rs)
- Sandboxed application distribution
- Runtime management for application dependencies
- Remote repository management (Flathub)
- Permission management (network, X11, Wayland, etc.)
- Sandbox level configuration (full, host, shared)
- Portal integration for file access, printing, screenshots
- Application search and installation

### Previous Enhancements
#### Office Suite (sigma_office.rs)
- Enhanced formula evaluation with support for +, -, *, /, ^ operators
- Added SUM function for spreadsheet calculations
- Implemented column sum/average calculations
- Added document import/export functionality for file persistence

#### Core Utilities (sigma_coreutils.rs) v1.1
- Added recursive directory removal with `rm -r` flag
- Added parent directory creation with `mkdir -p` flag
- Added line numbering with `cat -n` flag
- Added `pwd` command for printing working directory
- Added `echo` command for text output

#### Text Editor (sigma_edit.rs)
- Implemented undo/redo functionality with 50-state history
- Added search/replace with result highlighting
- Added goto line command for navigation
- Added word count and character count statistics
- Enhanced CLI with new commands: undo, redo, search, clear, replace, goto, stats, saveas

#### Service Manager (sigma_service.rs)
- Added restart count tracking for monitoring service stability
- Added memory limit configuration per service (in MB)
- Added CPU limit configuration per service (percentage)
- Added environment variable support for service configuration
- Added methods: set_memory_limit(), set_cpu_limit(), set_env_var()

## Phase 4 (36+ months)
- Certifications
- Vendor partnerships
- Mainstream device driver coverage
- Migration tools
- Scale community and support offerings
