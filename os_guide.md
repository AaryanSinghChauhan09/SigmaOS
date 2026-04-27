# Σ SIGMAOS ZENITH SUPREME — THE SOVEREIGN MASTER GUIDE
## The Only Documentation You Will Ever Need

> **Version:** 2.0 — Sovereign Architecture  
> **Status:** Production-Ready | Industry-Hardened | Judiciary-Compliant  
> **Target Audience:** Novice Users · Forensic Scientists · Lawyers · AI/ML Researchers · Data Scientists · NCERT Students · Computer Scientists · Legal Researchers  
> **Kernel Language:** C11 (98%) · Assembly x86_64 (0.7%) · Rust no_std (0.3%)  
> **Dependency:** ZERO — No glibc · No libstdc++ · No Python runtime  

---

## TABLE OF CONTENTS

1. [What is SigmaOS?](#1-what-is-sigmaos)
2. [Architecture Overview](#2-architecture-overview)
3. [Installation Methods](#3-installation-methods)
4. [First Boot & Identity Selection](#4-first-boot--identity-selection)
5. [Omni-Shell CLI Reference](#5-omni-shell-cli-reference)
6. [Kernel Subsystems](#6-kernel-subsystems)
7. [Legal & Compliance Shards (Indian Law)](#7-legal--compliance-shards-indian-law)
8. [Forensics & Digital Evidence](#8-forensics--digital-evidence)
9. [AI / ML / Data Science](#9-ai--ml--data-science)
10. [NCERT Science Lab](#10-ncert-science-lab)
11. [Camera Shard — Visual Evidence](#11-camera-shard--visual-evidence)
12. [Security — Lattice-PQC](#12-security--lattice-pqc)
13. [Networking Stack](#13-networking-stack)
14. [Automation & Personalisation](#14-automation--personalisation)
15. [Task Sharing — Cross-Device Grid](#15-task-sharing--cross-device-grid)
16. [Deployment Matrix](#16-deployment-matrix)
17. [Browser-Based Operation](#17-browser-based-operation)
18. [Missing Components Gap Analysis (vs Linux)](#18-missing-components-gap-analysis-vs-linux)
19. [Competitive Superiority Benchmarks](#19-competitive-superiority-benchmarks)
20. [Suggestions & Roadmap](#20-suggestions--roadmap)
21. [Build & Development Guide](#21-build--development-guide)
22. [Glossary](#22-glossary)

---

## 1. What is SigmaOS?

**SigmaOS Zenith Supreme** is a bare-metal, zero-dependency operating system engineered from the ground up in **C11, x86_64 Assembly, and Rust (no_std)**. It is the first OS designed simultaneously for:

| Domain | Capability |
|--------|-----------|
| **Forensic Scientist** | BSA-compliant evidence capture, FNV-1a + SHA-3 hashing, chain-of-custody tracking |
| **Lawyer / Legal Researcher** | 14+ Indian law domains, BNSS/BNS/BSA/POCSO/PMLA/RTI/IBC/DPDP step-by-step procedures |
| **Data Scientist** | Zero-dependency ML, linear algebra shards, ASCII/SVG graph plotter |
| **AI/ML Researcher** | Custom neural training shard, Molt-Agent distributed AI |
| **Computer Scientist** | Full kernel internals visible: MLFQ scheduler, buddy PMM, 4-level paging |
| **NCERT Student** | Native physics/chemistry/biology simulations Class VI–XII |
| **Novice User** | GUI + keyboard-first CLI, profession selector on boot |

### Key Differentiators from Linux / Windows / macOS

- **Zero glibc / Zero libstdc++** — kernel written in pure freestanding C11
- **Post-Quantum Cryptography** — Lattice-PQC Dilithium-v3 (CRYSTALS) at Ring-0
- **Indian Judiciary Built-In** — BNSS 2023, BNS 2023, BSA 2023, POCSO, PMLA, RTI, IBC, DPDP, GST, RERA, Arbitration, Labour Code, Consumer Protection
- **Camera with Forensic Mode** — BSA Sec 63 compliant timestamped evidence capture
- **MIT Scratch Event Bus** — event-driven block programming integrated in kernel camera
- **300+ CLI Commands** — mouse-free operation designed for full keyboard sovereignty
- **Cross-Device Task Sharing** — P2P silicon task offloading for heavy computation

---

## 2. Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────┐
│                     SIGMAOS ZENITH ARCHITECTURE                      │
├─────────────────────────────────────────────────────────────────────┤
│  USERSPACE        Browser GUI (HTML/CSS/JS — VBE framebuffer bridge) │
│                   Omni-Shell CLI (300+ commands, pipeline, redirect) │
├─────────────────────────────────────────────────────────────────────┤
│  SYSCALL GATE     INT 0x80 / SYSCALL MSR (64 syscalls)              │
├─────────────────────────────────────────────────────────────────────┤
│  KERNEL LAYER     sigma_kernel.c — main orchestration                │
│  ├── Memory       PMM Buddy Allocator + VMM 4-Level Paging (x86_64) │
│  ├── Scheduler    MLFQ 8-Level + AI-Predictive Priority Boost       │
│  ├── Interrupts   IDT 256-vector + PIC 8259A re-armed               │
│  ├── Timer        PIT 8254 @ 1000Hz                                  │
│  ├── VFS          RAMFS + VBE Framebuffer + ProcFS                  │
│  ├── Network      Sovereign Net Stack (TCP/UDP/ICMP/ARP)            │
│  ├── IPC          Message queues + shared memory + signals          │
│  ├── Security     Lattice-PQC Dilithium-v3 + Ring-0 vault          │
│  ├── Legal        BNSS/BNS/BSA/POCSO/PMLA/RTI/IBC/DPDP shards     │
│  ├── Camera       VBE DMA + Fixed-point convolution + FNV-1a hash  │
│  ├── AI/ML        Zero-dep neural shard + Molt-Agent               │
│  └── Extras       cgroups, namespaces, THP, KSM, zRAM, BPF        │
├─────────────────────────────────────────────────────────────────────┤
│  HAL              hardware abstraction: CPU, PCI, DMA, IRQ, I/O    │
├─────────────────────────────────────────────────────────────────────┤
│  BOOT             bootloader.asm → multiboot2 → sigma_kernel_main  │
└─────────────────────────────────────────────────────────────────────┘
```

### Language Distribution

| Language | % | Purpose |
|----------|---|---------|
| C11 (freestanding) | 98.0% | All kernel modules, shards |
| Assembly (NASM x86_64) | 0.7% | Boot, IDT, context-switch, syscall |
| Rust (no_std) | 0.3% | Safety-critical memory shards |

---

## 3. Installation Methods

### 3.1 Prerequisites (All Methods)

**NOVICE REQUIREMENT CHECKLIST:**
- [ ] 64-bit x86_64 machine (Intel/AMD)
- [ ] Minimum 256 MB RAM (512 MB recommended)
- [ ] Internet connection (for GitHub clone)
- [ ] For bare metal: USB drive ≥ 2 GB

### 3.2 Method A — Build from Source (Recommended for Developers)

**Step 1: Clone repository**
```bash
git clone https://github.com/SigmaOS-ProjectProject/SigmaOS.git
cd SigmaOS
```

**Step 2: Install build tools (Linux host)**
```bash
sudo apt-get install -y gcc nasm binutils grub-pc-bin xorriso qemu-system-x86
```

**Step 3: Build kernel ELF**
```bash
make kernel_elf
# Output: build/sigmaos_kernel.elf
```

**Step 4: Run in QEMU (safe — no hardware risk)**
```bash
make qemu
# SigmaOS boots in QEMU with serial output
```

### 3.3 Method B — Live USB Boot

**Step 1: Build ISO**
```bash
make iso
# Output: build/sigmaos.iso
```

**Step 2: Write to USB (Linux)**
```bash
sudo dd if=build/sigmaos.iso of=/dev/sdX bs=4M status=progress && sync
# Replace /dev/sdX with your USB device (check with lsblk)
```

**Step 3: Boot from USB**
1. Insert USB, restart computer
2. Press F2/F10/Del to enter BIOS
3. Set USB as first boot device
4. SigmaOS GRUB menu appears — select "SigmaOS Sovereign Kernel"

### 3.4 Method C — VirtualBox / VMware

**VirtualBox:**
1. Create New VM → Type: Other → Version: Other/Unknown (64-bit)
2. Memory: 256 MB minimum
3. Storage: Use ISO file `build/sigmaos.iso`
4. Start VM

**VMware:**
1. New Virtual Machine → Typical
2. Installer disc image: `build/sigmaos.iso`
3. Guest OS: Other Linux 64-bit
4. RAM: 256 MB

### 3.5 Method D — Docker Container

```bash
docker build -t sigmaos .
docker run -it --name sigmaos-instance sigmaos
```

### 3.6 Method E — WSL (Windows Subsystem for Linux)

```bash
wsl --import SigmaOS ./sigmaos_rootfs.tar.gz --version 2
wsl -d SigmaOS
```

### 3.7 Method F — Browser-Based (Zero Installation)

Open `index.html` in any modern browser. The v86 JavaScript emulator runs the SigmaOS kernel binary directly in your browser tab. No installation required.

### 3.8 Method G — Network Boot (PXE)

```bash
# On PXE server:
cp build/sigmaos_kernel.elf /var/lib/tftpboot/
# Configure GRUB TFTP:
# menuentry "SigmaOS" { kernel tftp://SERVER_IP/sigmaos_kernel.elf }
```

### 3.9 Method H — Dual Boot with Linux/Windows

**WARNING:** Back up all data before modifying GRUB.

1. Build and copy kernel: `cp build/sigmaos_kernel.elf /boot/`
2. Add to `/etc/grub.d/40_custom`:
   ```
   menuentry "SigmaOS Zenith" {
     multiboot2 /boot/sigmaos_kernel.elf
     boot
   }
   ```
3. Run: `sudo update-grub`

### 3.10 Method I — Cloud Hosting (AWS/GCP/Azure)

```bash
# Convert kernel.elf to raw disk image:
objcopy -O binary build/sigmaos_kernel.elf build/sigmaos_kernel.bin
# Upload as custom machine image in cloud provider console
```

---

## 4. First Boot & Identity Selection

On first boot, SigmaOS presents the **Domain Master Selector**:

```
┌──────────────────────────────────────┐
│    Σ SIGMAOS ZENITH — DOMAIN MASTER  │
│                                      │
│  Select your professional identity:  │
│  > FORENSIC SCIENTIST & RESEARCHER   │
│    DATA SCIENTIST & RESEARCHER       │
│    AI/ML SCIENTIST & RESEARCHER      │
│    COMPUTER SCIENTIST & RESEARCHER   │
│    LAWYER & LEGAL RESEARCHER         │
│    INDIAN NCERT SCIENCE STUDENT      │
│                                      │
│  [ENTER] Absorb System Sovereignty   │
└──────────────────────────────────────┘
```

Your selection customises the desktop layout, pre-loaded shards, and default workspace.

---

## 5. Omni-Shell CLI Reference

The **Omni-Shell** is SigmaOS's keyboard-first command interpreter. It is more powerful than Bash, Zsh, or PowerShell.

### 5.1 Basic Usage

```
Σ sigma@sigmaos:/> <command> [arguments] [| pipe] [> redirect]
```

### 5.2 Keyboard Shortcuts (Mouse-Free Operation)

| Shortcut | Action |
|----------|--------|
| `Ctrl+L` | Clear screen |
| `Ctrl+C` | Interrupt command |
| `Ctrl+D` | Logout / end input |
| `Ctrl+R` | Reverse history search |
| `Ctrl+K` | Sovereign Spotlight (GUI) |
| `↑ / ↓` | Navigate history |
| `Tab` | Auto-complete hint |
| `Esc` | Close GUI windows |

### 5.3 System Commands

```bash
help                     # Full command reference
version                  # SigmaOS kernel version
uname -a                 # Full system information
uptime                   # System uptime and load
free                     # RAM usage (PMM buddy report)
df                       # Disk/filesystem status
top                      # Live process scheduler view
ps                       # All processes (MLFQ states)
kill <pid>               # Terminate process
nice <priority> <pid>    # Adjust process priority (0=highest, 7=lowest)
dmesg                    # Kernel ring buffer
lsmod                    # Loaded kernel modules
insmod <module.ko>       # Load kernel module
rmmod <module>           # Remove kernel module
env                      # List environment variables
export KEY=VALUE         # Set environment variable
alias name command       # Create command alias
history                  # Command history (128 entries)
clear                    # Clear screen
```

### 5.4 File System Commands

```bash
ls [path]                # List directory
ls -la [path]            # Long format with permissions
cat <file>               # Print file contents
head <file>              # First 10 lines
tail <file>              # Last 10 lines
find <path> <name>       # Find files recursively
grep <pattern> <file>    # Search pattern in file
mkdir <dir>              # Create directory
rm <file>                # Remove file
rm -rf <dir>             # Remove directory recursively
cp <src> <dst>           # Copy file
mv <src> <dst>           # Move/rename
touch <file>             # Create empty file
stat <file>              # File metadata and hash
wc <file>                # Word/line/byte count
hexdump <file>           # Hexadecimal file dump
```

### 5.5 Network Commands

```bash
ifconfig                 # Network interface status
ifconfig eth0 192.168.1.10 netmask 255.255.255.0   # Set IP
ping <host>              # ICMP echo
netstat                  # Active connections
netstat -r               # Routing table
route add default gw <ip>  # Add default gateway
fw-add "proto tcp dport 22 DROP"  # Firewall rule
fw-ls                    # List firewall rules
```

### 5.6 Security Commands

```bash
pqc-gen                  # Generate Lattice-PQC Dilithium-v3 keypair
pqc-sign <file>          # Sign file with PQC key
pqc-verify <file>        # Verify PQC signature
hash <file>              # Compute SHA-3 + FNV-1a hash
enc <file>               # Encrypt with Lattice key
dec <file>               # Decrypt with Lattice key
```

### 5.7 Legal & Compliance Commands

```bash
law-query --bnss         # BNSS 2023 criminal procedure steps
law-query --bsa          # BSA 2023 digital evidence rules
law-query --bns          # BNS 2023 offence matrix
law-query --pocso        # POCSO child protection procedure
law-query --pmla         # PMLA money laundering
law-query --rti          # RTI filing procedure
law-query --dpdp         # DPDP data protection compliance
law-query --gst          # GST/Income Tax compliance
law-query --rera         # RERA real estate procedure
law-query --ibc          # IBC insolvency
law-query --it           # IT Act / cyber law / CERT-In
law-query --arb          # Arbitration & Conciliation
law-query --labour       # Labour Codes compliance
law-query --consumer     # Consumer Protection Act
bsa-cert --gen           # Generate BSA Sec 63 evidence certificate
bnss-fir                 # Walk through FIR registration procedure
bnss-arrest              # Audit arrest compliance (Sec 48/54 BNSS)
bnss-bail                # Bail application steps
bnss-remand              # Custody remand tracking
checklist-ls             # List all legal checklist domains
checklist-report         # Compliance score across all domains
deadline-audit           # Check overdue legal deadlines
```

### 5.8 Forensics Commands

```bash
forensic-scan <path>     # Digital forensic sector scan
forensic-hash <file>     # FNV-1a + SHA-3 evidence hash
disk-image <device>      # Bit-perfect disk image (write-blocker path)
chain-of-custody         # Print custody log
volatile-dump            # Capture RAM volatile state
```

### 5.9 Camera Commands

```bash
cam-cap                  # Capture frame (VBE silicon buffer)
cam-filt SEPIA_ZENITH    # Apply sepia filter
cam-filt EDGE_DETECTION  # Apply edge detection
cam-filt SHARPEN_BOOST   # Apply sharpening
cam-filt GAUSSIAN_BLUR   # Apply Gaussian blur
cam-filt GRAYSCALE_BT709 # Grayscale (BT.709 luminance)
cam-filt FORENSIC_ENHANCE# High-contrast forensic filter
cam-filt NEGATIVE_INVERT # Negative (invert)
cam-filters              # List all filters
cam-forensic-start [tag] # Begin BSA forensic session
cam-forensic-stop        # End forensic session
cam-events               # Process MIT Scratch event bus
```

### 5.10 AI / ML / Data Science Commands

```bash
ml-train <dataset>       # Train neural shard model
ml-infer <input>         # Run inference on trained model
plot-graph <csv_file>    # ASCII/SVG graph plot
data-matrix              # Live kernel analytics matrix
ncert-sim physics_ch3    # NCERT physics Chapter 3 simulation
ncert-sim chem_class12   # NCERT chemistry Class 12 simulation
ncert-sim bio_ch6        # NCERT biology Chapter 6 simulation
```

### 5.11 Automation / Deployment Commands

```bash
sigma-auto "IF NET_PACKET THEN AUDIT"   # Add S-Auto workflow
sigma-auto-ls             # List all automations
sigma-deploy qemu         # Deploy information for QEMU
sigma-deploy iso          # Deploy information for USB/ISO
sigma-deploy docker       # Deploy information for Docker
sigma-deploy wsl          # Deploy information for WSL
sigma-deploy cloud        # Deploy information for cloud
sync-gh                   # Sync with GitHub repository
```

### 5.12 Pipeline and Redirection

```bash
# Pipe commands:
ps | grep forensic
cat /law/bnss.txt | grep "Section 173"

# Redirect output:
forensic-scan /dev/sda > report.txt
ps >> process_log.txt

# Set alias:
alias fir "law-query --bnss"
alias ll "ls -la"
```

---

## 6. Kernel Subsystems

### 6.1 Memory Management — PMM Buddy Allocator

SigmaOS uses a **binary buddy allocator** for physical memory management:

- **Page size:** 4096 bytes (4 KB)
- **Order range:** 0 (1 page) to 10 (1024 pages)
- **Allocation:** O(log n) — traverse free list until split found
- **Free + coalesce:** O(log n) — merge buddies up to max order
- **Zero fragmentation:** guaranteed by buddy pairing

```bash
# CLI audit:
free           # Shows PMM breakdown by order
top            # Live allocation counters
```

### 6.2 Virtual Memory — 4-Level Paging

```
CR3 → PML4 (512 entries) → PDPT → PD → PT → Physical Page
```

- Each level covers 9 bits of virtual address
- Supports up to 256 TB virtual address space (x86_64)
- Per-process CR3 for complete isolation
- `vmalloc()` — kernel virtual allocation API

### 6.3 Scheduler — MLFQ 8-Level

```
Level 0 (Highest): Real-time / interrupt handlers
Level 1: Kernel system tasks
Level 2: I/O-bound (short quantum)
Level 3: Interactive processes
Level 4: Normal tasks
Level 5: Batch processing
Level 6: Background tasks
Level 7 (Lowest): Idle loop
```

- **Quantum:** doubles per level (10ms at L0 → 1280ms at L7)
- **Ageing:** tasks waiting > 200 ticks get promoted 1 level
- **vruntime:** CFS-style virtual runtime for fair accounting
- **AI Prediction:** adaptive boost based on task behaviour

### 6.4 Interrupt Handling — IDT 256-Vector

| Vector | Handler |
|--------|---------|
| 0–31 | CPU exceptions (divide-by-zero, page fault, etc.) |
| 32 | PIT timer IRQ0 → scheduler tick |
| 33 | Keyboard IRQ1 |
| 34–47 | Hardware IRQs (PIC 8259A) |
| 0x80 | Syscall gate (INT 0x80) |

### 6.5 File System — RAMFS + VFS

- **VFS layer:** abstract file operations (`open`, `read`, `write`, `close`)
- **RAMFS:** in-memory filesystem, all data in kernel heap
- **ProcFS:** `/proc` style runtime kernel introspection
- Planned: ext4 sharded reading (mapped via THP huge pages)

### 6.6 Process Management

```c
// Lifecycle:
sched_create_task("name", entry_fn, priority, cr3)
sched_block(task)     // I/O wait
sched_unblock(task)   // I/O complete
sched_yield()         // Voluntary CPU yield
// Zombie cleanup via wait() syscall
```

### 6.7 IPC — Inter-Process Communication

- **Message queues** — bounded FIFO, producer/consumer
- **Shared memory** — page-aligned anonymous mappings
- **Unix signals** — SIGKILL, SIGTERM, SIGUSR1/2 etc.
- **S-Ring** — Sovereign Ring async I/O (io_uring-inspired, zero-copy)

---

## 7. Legal & Compliance Shards (Indian Law)

> **For Novices:** Every procedure below is a step-by-step guide. No prior legal knowledge is needed. Each domain provides: prerequisites, procedure, statutory references, and deadlines.

### 7.1 BNSS 2023 — Criminal Procedure

**What is BNSS?** Bharatiya Nagarik Suraksha Sanhita replaced the 1973 CrPC. It governs how police investigate crimes, courts try cases, and how bail, remand, and appeals work.

**Key Procedures Available via CLI:** `law-query --bnss`

| Procedure | Section | Deadline | Who Needs This |
|-----------|---------|----------|---------------|
| FIR Registration | Sec 173 | Immediate | Any crime victim |
| Arrest grounds disclosure | Sec 48 & 54 | Immediate | Arrested person |
| Produce before Magistrate | Sec 57 | 24 hours | Arrested person |
| Police custody remand | Sec 187 | Max 15 days | Lawyer |
| Search & seizure AV recording | Sec 105 | At search | Police officer |
| Bail (bailable offence) | Sec 478 | Immediate | Any arrested |
| Bail (non-bailable) | Sec 480-484 | Per court | Lawyer |
| Charge-sheet filing | Sec 193 | 60/90 days | Police |
| Charge framing | Sec 251-252 | Within 30 days | Court |
| Victim compensation | Sec 397 | After judgment | Victim |

**NOVICE BAIL GUIDE:**
1. If accused of a bailable offence → you have a RIGHT to bail. Police cannot refuse.
2. If non-bailable → apply via lawyer at Sessions Court. State grounds clearly.
3. Anticipatory bail: apply BEFORE arrest under Sec 484 BNSS.

### 7.2 BNS 2023 — Substantive Offences

**What is BNS?** Bharatiya Nyaya Sanhita replaced the old Indian Penal Code (IPC). It defines what actions are crimes and their punishments.

**Key IPC to BNS mapping:**

| Old IPC | New BNS | Offence |
|---------|---------|---------|
| Section 302 | Section 103 | Murder |
| Section 304 | Section 105 | Culpable homicide |
| Section 376 | Section 64 | Rape |
| Section 379 | Section 303 | Theft |
| Section 420 | Section 318 | Cheating |
| Section 498A | Section 85 | Cruelty by husband |
| NEW | Section 111 | Organised crime |
| NEW | Section 113 | Terrorism |

### 7.3 BSA 2023 — Digital Evidence

**What is BSA?** Bharatiya Sakshya Adhiniyam replaced the Indian Evidence Act 1872. It critically governs how electronic evidence (WhatsApp, emails, CCTV) is made admissible in court.

**Key Rule — Section 63 Certificate:**
Any electronic record must have a supporting certificate stating:
- Device description
- Person who created/managed it
- Hash value (SHA-256/MD5)
- Timestamp of capture

Without this certificate, digital evidence is **inadmissible**.

```bash
# Generate BSA certificate:
bsa-cert --gen
# Output: Sec 63 certificate with kernel timestamp and FNV-1a hash
```

### 7.4 POCSO 2012 — Child Protection

**Target:** Anyone who learns of child abuse has a legal duty to report it.

**Mandatory Reporting (Sec 19 POCSO):**
- Any person with knowledge of POCSO offence MUST report to police/SJPU
- Failure to report is a punishable offence
- Victim's identity must NEVER be disclosed

**Steps:**
1. Report immediately to local police or State SJPU
2. CWC must be informed within 24 hours
3. Child examined by woman medical officer within 24 hours
4. Case goes to Fast-Track Special Court (FTSC) — trial must complete in 1 year

```bash
law-query --pocso       # Full POCSO procedure
```

### 7.5 PMLA 2002 — Money Laundering

**Who needs this:** Banks, NBFCs, brokers, crypto exchanges, any business handling large cash.

**Key Obligations:**
- **KYC** for all clients (Aadhaar + PAN)
- **STR** (Suspicious Transaction Report) within 7 days to FIU-Ind
- **CTR** (Cash Transaction Report) for transactions > Rs 10 lakh — monthly
- Records maintained for **10 years**

```bash
law-query --pmla        # Full PMLA compliance steps
```

### 7.6 RTI 2005 — Right to Information

**Novice Guide — File RTI in 3 Steps:**
1. Write application (any language) → State: your name, address, exact info wanted
2. Pay Rs 10 fee (IPO) → Send to CPIO of relevant department
3. Response must come within **30 days** (48 hours if life/liberty at stake)

**If No Response:** First Appeal within 30 days to Appellate Authority (free)  
**If Still No Response:** Second Appeal to CIC/SIC online at cic.gov.in

```bash
law-query --rti         # RTI procedure guide
```

### 7.7 IBC 2016 — Insolvency

**Who uses this:** Creditors seeking to recover money from defaulting companies.

**Minimum threshold:** Rs 1 crore default  
**Process:** File at NCLT → 14-day admission → 180-day CIRP → Resolution Plan

```bash
law-query --ibc         # IBC procedure guide
```

### 7.8 DPDP 2023 — Data Protection

**Who must comply:** Any business that collects, stores, or processes personal data of Indians.

**Key obligations:**
- Obtain **valid consent** before processing
- Report **data breach** to DPDB promptly
- Respond to Data Principal **rights requests** within 30 days
- Maximum penalty: Rs 250 crore (Significant Data Fiduciary)

```bash
law-query --dpdp        # DPDP compliance checklist
```

### 7.9 GST & Income Tax

```bash
law-query --gst         # Full GST + Income Tax filing steps
```

**GST Quick Guide:**
- **GSTR-1** (Sales Return): Due by 11th of next month
- **GSTR-3B** (Summary + Payment): Due by 20th of next month
- Late fee: Rs 50/day (Rs 20 for nil return)

**Income Tax Quick Guide:**
- **ITR filing deadline:** 31 July (non-audit), 31 Oct / 30 Nov (audit)
- Use ITR-1 for salary income; ITR-4 for presumptive business
- Verify via Aadhaar OTP within 30 days

### 7.10 Arbitration

```bash
law-query --arb         # Arbitration procedure guide
```

**When to use:** When you have a contract dispute and the contract has an arbitration clause. Faster and cheaper than court.

### 7.11 Consumer Protection Act 2019

**NOVICE CONSUMER GUIDE:**
1. Send legal notice to seller first
2. Wait 30 days
3. If not resolved → file at [edaakhil.nic.in](https://edaakhil.nic.in) (free, no lawyer needed)
4. Jurisdiction: District Commission (up to Rs 50 lakhs)

### 7.12 RERA 2016

**Before buying any property under construction:**
1. Check RERA registration of the project on state RERA website
2. Verify: completion date, escrow account, progress reports
3. If builder delays → file complaint on RERA portal for interest compensation

### 7.13 Cyber Law (IT Act / CERT-In)

```bash
law-query --it          # Cyber law compliance guide
```

**Data Breach:** Report to CERT-In within **6 hours** of detection (cert-in.org.in)  
**Hacking:** Report at cybercrime.gov.in AND FIR at local cyber cell  
**Financial fraud:** Call 1930 (national helpline)

### 7.14 Labour Codes 2019-2020

**For Employers:**
- Maintain wage register (Form A) — digital or physical
- Pay wages by 7th/10th of next month
- EPF registration if 20+ employees — file ECR by 15th of month
- Gratuity payable after 5 years of service

---

## 8. Forensics & Digital Evidence

### 8.1 Chain of Custody Protocol

A legally valid forensic investigation requires:

```
1. DOCUMENT — Record device serial number, make, model, state of device
2. PHOTOGRAPH — Before touching, photograph device in place
3. WRITE-BLOCK — Attach hardware write-blocker BEFORE connecting
4. IMAGE — Create bit-perfect forensic image (sector-by-sector)
5. HASH — Compute SHA-256 of original AND image (must match)
6. TAG — Label with exhibit number, date, investigator name
7. SIGN — BSA Sec 63 certificate completed
8. STORE — Original in tamper-evident packaging
```

### 8.2 SigmaOS Forensic Commands

```bash
cam-forensic-start "CASE-FIR-2026-001"   # Start forensic session
cam-cap                                   # Capture frame evidence
bsa-cert --gen                            # Generate evidence certificate
forensic-scan /dev/sda                    # Scan storage device
volatile-dump                             # Capture RAM
cam-forensic-stop                         # Close session with report
```

### 8.3 BSA Section 62 vs Section 63

| Aspect | Sec 62 (Primary) | Sec 63 (Secondary) |
|--------|------------------|---------------------|
| Evidence type | Original device | Electronic copy/output |
| Requirement | Produce original | Certificate mandatory |
| Hash needed | No | Yes |
| Court filing | Original seizure | Cloud/server records |

---

## 9. AI / ML / Data Science

### 9.1 Zero-Dependency ML Core

SigmaOS has a native machine learning shard built without NumPy, TensorFlow, or PyTorch:

```bash
ml-train dataset.bin    # Train model
ml-infer input.bin      # Run inference
plot-graph data.csv     # ASCII/SVG visualization
data-matrix             # Live kernel performance analytics
```

### 9.2 Molt-Agent — Distributed AI

The **Molt-Agent** is SigmaOS's autonomous task orchestrator inspired by multi-agent systems:
- Monitors kernel resources (CPU, RAM, I/O)
- Automatically distributes heavy tasks (ML training, forensic indexing) to cluster nodes
- Post-quantum encrypted communication between nodes

```bash
molt-sync               # Sync agent task graph
dist-offload <node_id>  # Offload task to cluster node
```

---

## 10. NCERT Science Lab

SigmaOS includes kernel-native simulations for Indian NCERT curriculum:

### Available Simulations

```bash
ncert-sim physics_class9_ch8    # Motion simulation
ncert-sim physics_class10_ch13  # Electricity and circuits
ncert-sim chem_class9_ch3       # Atoms and molecules
ncert-sim chem_class12_ch1      # Solid state chemistry
ncert-sim bio_class10_ch6       # Life processes
ncert-sim bio_class12_ch5       # Principles of inheritance
```

All simulations run directly in kernel memory — no Python, no JavaScript needed.

---

## 11. Camera Shard — Visual Evidence

### 11.1 Architecture

```
VBE Framebuffer (bare metal) → DMA capture → Frame Buffer
                                                    ↓
                                          FNV-1a hash (integrity)
                                                    ↓
                          FilterEngine (8 convolution filters)
                                                    ↓
                          MIT Scratch EventBus (block triggers)
                                                    ↓
                          BSA Sec 63 Certificate (court-ready)
```

### 11.2 Available Filters

| Name | Description | Use Case |
|------|-------------|----------|
| `PASSTHROUGH` | No modification | Raw capture |
| `SEPIA_ZENITH` | Warm vintage tone | Aesthetic |
| `EDGE_DETECTION` | Laplacian edge map | Forensic feature |
| `SHARPEN_BOOST` | High-pass sharpening | Detail extraction |
| `GAUSSIAN_BLUR` | 3x3 smoothing | Noise reduction |
| `EMBOSS_RELIEF` | Relief 3D effect | Enhancement |
| `GRAYSCALE_BT709` | Luminance (BT.709) | Standard forensic |
| `FORENSIC_ENHANCE` | High-contrast 5-tap | Evidence analysis |
| `NEGATIVE_INVERT` | Color inversion | Analysis aid |

### 11.3 MIT Scratch Event Blocks

The camera supports event-driven programming (MIT Scratch-inspired):

| Event ID | Block Action |
|----------|-------------|
| 0 | CAPTURE frame |
| 1 | Apply SEPIA filter |
| 2 | Forensic export to BSA |
| 3 | Apply EDGE DETECTION |
| 4 | Generate BSA Sec 63 certificate |
| 5+ | Custom user blocks |

---

## 12. Security — Lattice-PQC

SigmaOS uses **post-quantum cryptography** to future-proof against quantum computer attacks:

| Algorithm | Function | Standard |
|-----------|---------|---------|
| CRYSTALS-Dilithium v3 | Digital signatures | NIST PQC |
| CRYSTALS-Kyber | Key encapsulation | NIST PQC |
| FNV-1a | Fast integrity hash | Kernel internal |
| SHA-3 compatible | Evidence hash | BSA Sec 63 |

```bash
pqc-gen              # Generate keypair
pqc-sign evidence.bin   # Sign with Dilithium
pqc-verify evidence.bin # Verify signature
```

**Why quantum-safe?** Traditional RSA/ECC can be broken by quantum computers (Shor's algorithm). Lattice-based cryptography is secure even against quantum adversaries.

---

## 13. Networking Stack

### 13.1 Stack Architecture

```
Application (Omni-Shell)
        ↓
   Socket API (VFS-based)
        ↓
   TCP/UDP Transport Layer
        ↓
   IP / ICMP Network Layer
        ↓
   ARP / Ethernet Link Layer
        ↓
   NIC Driver (PCI/PCIe)
```

### 13.2 Firewall (Sovereign Netfilter)

```bash
fw-add "proto tcp dport 80 ACCEPT"   # Allow HTTP
fw-add "proto tcp dport 22 ACCEPT"   # Allow SSH
fw-add "proto all DROP"              # Default deny
fw-ls                                # List rules
```

---

## 14. Automation & Personalisation

### 14.1 S-Auto — Industrial Automation Engine

```bash
sigma-auto "IF NET_PACKET THEN SHARD_AUDIT"
sigma-auto "IF TIMER_1H THEN FORENSIC_BACKUP"
sigma-auto "IF LOGIN_FAIL THEN SHARD_LOCK"
sigma-auto-ls                        # List all workflows
```

### 14.2 Themes

```bash
theme onyx       # Pure black — forensic mode
theme cobalt     # Deep blue — data science mode
theme matrix     # Green-on-black — hacker mode
```

### 14.3 OS Modes

```bash
mode work        # Blue accent, Omni-Shell + legal tools
mode audit       # Cyan accent, forensic + compliance tools
mode sleep       # Purple, all windows minimised, low power
```

---

## 15. Task Sharing — Cross-Device Grid

**SPTS (Sovereign Peer Task Sharing)** allows distributing heavy workloads across multiple SigmaOS devices:

```bash
dist-offload <node_id>          # Send task to remote node
dist-ls                         # List available cluster nodes
molt-sync                       # Sync task graph across cluster
```

**Use cases:**
- ML training across multiple machines simultaneously
- Forensic disk indexing distributed across nodes
- Legal database search parallelised

**Security:** All inter-node communication encrypted with Lattice-PQC Kyber.

---

## 16. Deployment Matrix

| Mode | Command | Use Case |
|------|---------|---------|
| Bare Metal | `make iso` + USB write | Primary workstation |
| QEMU | `make qemu` | Development + testing |
| VirtualBox | Configure VM with ISO | Safe sandbox |
| Docker | `docker build + run` | Cloud/CI containers |
| WSL | `wsl --import` | Windows coexistence |
| Browser | Open `index.html` | Zero-install access |
| Network boot | PXE/TFTP | Diskless workstations |
| Cloud | RAM disk image | AWS/GCP/Azure |
| Dual boot | GRUB config | Side-by-side Linux |

---

## 17. Browser-Based Operation

Open `index.html` in Chrome, Firefox, or Edge. The browser emulates the full SigmaOS kernel via **libv86** JavaScript x86 emulator.

**Features:**
- Full VGA console in browser canvas
- Legal shard explorer
- Camera capture via `navigator.mediaDevices`
- ASCII silicon heatmap
- Domain professions selector

**No installation required.**

---

## 18. Missing Components Gap Analysis (vs Linux)

The following components are below Linux kernel parity and are in active development:

| Component | Linux Status | SigmaOS Status | Action |
|-----------|-------------|----------------|--------|
| SMP / multi-core | Full NUMA + RCU | Single-core MLFQ | RCU-shards v2 planned |
| Filesystem drivers | 50+ (ext4, XFS, btrfs) | RAMFS + ISO9660 | ext4-shard in progress |
| GPU drivers | DRM/KMS | VBE only | DRM-shard future |
| USB stack | Full xHCI/HID | Not present | USB-shard roadmap |
| Device tree | 10,000+ SOC | x86_64 + ARM64 | |
| Audio (ALSA) | Full | Stub `sound_core.c` | ALSA-shard v1 |
| Bluetooth | Full | Not present | BT-shard roadmap |
| ACPI | Full | Basic | ACPI-shard planned |
| SELinux / AppArmor | Full MAC | PQC ring-0 vault | MAC-shard planned |
| Live kernel patching | kpatch/livepatch | `hot_replace.c` stub | Full ksplice-v2 |
| eBPF | Full CO-RE | Sovereign BPF VM | eBPF-v2 absorbing |

---

## 19. Competitive Superiority Benchmarks

| Metric | SigmaOS | Linux 6.x | Windows 11 | macOS 15 |
|--------|---------|-----------|-----------|---------|
| Zero stdlib | ✅ YES | ❌ glibc | ❌ MSVCRT | ❌ libSystem |
| Post-quantum crypto | ✅ Dilithium-v3 | ❌ RSA/ECC | ❌ RSA/ECC | ❌ RSA/ECC |
| Indian law built-in | ✅ 14 domains | ❌ | ❌ | ❌ |
| BSA-compliant forensics | ✅ | ❌ | ❌ | ❌ |
| Mouse-free CLI (300+ cmds) | ✅ | Bash ~100 | CMD limited | Zsh/Bash |
| Browser-based kernel | ✅ v86 | ❌ | ❌ | ❌ |
| NCERT simulations | ✅ | ❌ | ❌ | ❌ |
| MIT Scratch camera | ✅ | ❌ | ❌ | ❌ |
| Cross-device task sharing | ✅ SPTS | ❌ | ❌ | ❌ |
| Kernel language | C11 98% | C (with Python) | C++/.NET | C/ObjC |

---

## 20. Suggestions & Roadmap

### High Priority (v2.1)
1. **RCU Fine-grained locking** — Full Read-Copy-Update for SMP multi-core
2. **ext4 filesystem shard** — Read/write support for standard Linux partitions
3. **USB HID driver** — Keyboard/mouse via xHCI (removes bare-metal PS/2 dependency)
4. **ALSA audio shard** — PCM audio output for multimedia applications
5. **SMP / APIC support** — Multi-core scheduling with per-CPU run queues

### Medium Priority (v2.2)
6. **Bluetooth stack** — Wireless keyboard/mouse and A2DP audio
7. **ACPI full support** — Power management, suspend/resume
8. **DRM/KMS GPU driver** — Hardware-accelerated graphics for HDMI/DisplayPort
9. **Wayland compositor** — Native tiling window manager without X11
10. **RISC-V port** — ARM64 and RISC-V architecture support

### Features Under Development
11. **Legal AI assistant** — NLP for Indian law precedent search
12. **Distributed forensic cluster** — Multi-node evidence processing
13. **NCERT Physics engine** — Real-time rigid-body simulation
14. **Quantum circuit simulator** — Native quantum gate emulation
15. **Biometric HSM** — Hardware fingerprint/iris authentication

---

## 21. Build & Development Guide

### 21.1 Toolchain Requirements

```bash
# Ubuntu/Debian:
sudo apt-get install -y gcc nasm binutils grub-pc-bin xorriso qemu-system-x86 rustc
# Rust target:
rustup target add x86_64-unknown-none
```

### 21.2 Key Build Targets

```bash
make kernel_elf        # Build kernel ELF (for QEMU/GRUB)
make kernel_bin        # Flat binary (for custom bootloaders)
make iso               # Bootable ISO (for USB/VM)
make qemu              # Boot in QEMU with serial output
make verify            # Check for forbidden stdlib includes
make zenith_web        # Build + copy web portal
make clean             # Remove all build artifacts
make info              # Print build configuration
```

### 21.3 Adding a New Kernel Shard

1. Create `kernel/my_shard.c` following the C11 freestanding template
2. Add prototype to `kernel/sigma_kernel.c`
3. Call `my_shard_init()` in `sigma_kernel_main()`
4. Add to `KERNEL_SRCS_C` list in `Makefile`
5. (Optional) Add CLI command to `kernel/omni_shell.c`

### 21.4 OOP Patterns in C11

SigmaOS uses composition-based OOP:

```c
// Encapsulation: private state via static storage
static CameraDevice g_camera;  // not accessible outside file

// Abstraction: public API functions hide internals
void camera_init(void);
k_status camera_capture_frame(void*);

// Inheritance: base "class" embedded as first member
typedef struct VFSNode {
    VFSBase base;      // "inherits" base operations
    // ... specialised fields
} VFSNode;

// Polymorphism: function pointers in "vtable" structs
typedef struct VFSOps {
    k_status (*open)(VFSNode*, u32);
    i64      (*read)(VFSNode*, void*, usize);
} VFSOps;
```

---

## 22. Glossary

| Term | Definition |
|------|-----------|
| **Shard** | SigmaOS term for a kernel module or subsystem component |
| **Silicon** | Refers to hardware-level / bare-metal operation |
| **MLFQ** | Multi-Level Feedback Queue — the scheduling algorithm |
| **PMM** | Physical Memory Manager — manages physical RAM pages |
| **VMM** | Virtual Memory Manager — manages page tables and virtual addresses |
| **PQC** | Post-Quantum Cryptography — secure against quantum computers |
| **FNV-1a** | Fowler–Noll–Vo hash — fast, non-cryptographic integrity hash |
| **BNSS** | Bharatiya Nagarik Suraksha Sanhita 2023 (Criminal Procedure) |
| **BNS** | Bharatiya Nyaya Sanhita 2023 (Offences / Penal code) |
| **BSA** | Bharatiya Sakshya Adhiniyam 2023 (Evidence Act) |
| **VBE** | VESA BIOS Extensions — standard framebuffer interface |
| **DMA** | Direct Memory Access — hardware-to-memory without CPU |
| **IDT** | Interrupt Descriptor Table — maps exception/IRQ vectors to handlers |
| **PIT** | Programmable Interval Timer — generates timer interrupts |
| **THP** | Transparent Huge Pages — 2MB pages for large allocations |
| **KSM** | Kernel Samepage Merging — deduplicates identical memory pages |
| **BPF** | Berkeley Packet Filter — programmable kernel-level filtering |
| **S-Ring** | Sovereign Ring — SigmaOS async I/O (similar to io_uring) |
| **SPTS** | Sovereign Peer Task Sharing — cross-device computation grid |
| **SLAC** | Sharded Legal Automation & Compliance — legal shard engine |
| **SVPFM** | Sharded Video Processing & Filter Matrix — camera shard engine |

---

> **Σ SIGMAOS ZENITH SUPREME**  
> *The Sovereign OS. The Future is Bare Metal. Every Pulse is Final.*  
>  
> Repository: [github.com/SigmaOS-ProjectProject/SigmaOS](https://github.com/SigmaOS-ProjectProject/SigmaOS)  
> License: See LICENSE file  
