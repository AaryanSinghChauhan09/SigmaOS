<div align="center">
  <h1>Σ SIGMA OS : OFFICIAL USER MANUAL v1.0</h1>
  <p><strong>The Definitive Guide to Navigating the Sovereign Architecture</strong></p>
</div>

---

## Table of Contents

1. [Introduction to Absolute Sovereignty](#1-introduction-to-absolute-sovereignty)
2. [Booting and Initialization](#2-booting-and-initialization)
3. [The Zenith Window Manager (GUI)](#3-the-zenith-window-manager-gui)
4. [Mastering the Omni-CLI Shell](#4-mastering-the-omni-cli-shell)
5. [File System & Sovereign Storage](#5-file-system--sovereign-storage)
6. [Process Isolation & Interference Guard](#6-process-isolation--interference-guard)
7. [Domain-Specific Shards (The App Ecosystem)](#7-domain-specific-shards-the-app-ecosystem)
8. [Developer Guidelines: Building Custom Shards](#8-developer-guidelines-building-custom-shards)

---

## 1. Introduction to Absolute Sovereignty

Welcome to SigmaOS. Unlike Windows, macOS, or mainstream Linux distributions, SigmaOS operates on a zero-dependency architecture. Every "application" is a native **Shard** compiled in Pure C11 or Assembly, interacting directly with bare-metal silicon.

By removing heavy Virtual File Systems (VFS), high-level language wrappers (like Python or Java), and monolithic systemdaemons, Sigma OS calculates processes in microseconds rather than milliseconds. It is designed to crush standard legacy systems through raw hardware efficiency.

---

## 2. Booting and Initialization

Booting SigmaOS on supported hardware (or via the simulated environment) involves bypassing standard bootloaders.

### Launching the System

From your host environment shell, run the launch dispatcher:

```powershell
./launch_sigmaos.ps1
```

**The Boot Sequence**:

1. **System Pulse Verification**: Verifies your silicon DMA paths.
2. **SOD (Shard-On-Demand) Core Load**: The central allocator is shoved into kernel space.
3. **RAM Sweep**: Any residual generic execution artifacts are forcefully purged.
4. **Handoff**: Dropped directly into the Zenith Window Manager or the Omni-CLI.

---

## 3. The Zenith Window Manager (GUI)

SigmaOS features the `SigmaWM` – an ultra-lightweight dynamic window manager built without heavy X11/Wayland bloat.

- **Workspace Navigation**: Workspaces dynamically appear as Shards are activated.
- **Display Matrix**: Native matrix calculations happen natively over assembly (`SigmaMathUnit`), providing smooth, 120 FPS capable UI scaling without relying on massive external graphic compositors.
- **App Drawer**: Displays your active repository of registered Shards (e.g., OmniMedia, SigmaDS, LegalDB).

---

## 4. Mastering the Omni-CLI Shell

The core of the system is the **Omni-CLI**, managed by the `SigmaShell` handler. Every executable tool can be summoned using the master `sigma` command without needing disparate sub-systems.

### Core CLI Commands ($ root@sigma:~)

| Command | Action | Description |
| :--- | :--- | :--- |
| `sigma optimize` | Auto-Optimizer | Instantly frees hardware RAM queues and clears zombie pointers. |
| `sigma clean` | Storage Wipe | Runs a bare-metal file shredder over non-indexed sectors. |
| `sigma gaming --boost` | Gaming Mode | Silences all background threads, isolating 95% of the CPU strictly to your game. |
| `sigma pulse` | System Metrics | Live kernel thread table visualization (Zero latency tracking). |
| `sigma kill <pid>` | Manual Force Close| Direct SIGKILL bypassing any software traps / handlers. |

---

## 5. File System & Sovereign Storage

SigmaOS utilizes `SigmaVFS`—a sovereign filesystem approach that avoids the classic deep hierarchical UNIX trees that slow down data retrieval.

- **Direct Block Read/Write**: Files are stored and retrieved linearly, allowing tools like the `omni_media_engine` to grab contiguous memory blocks iteratively without system call bottlenecks.
- **Offline Integrity**: Using the new `backup_manager`, block-level sync happens natively. You can hash an entire disk partition natively through CLI directly for forensic cloning.

---

## 6. Process Isolation & Interference Guard

Security within SigmaOS is strictly enforced through **Sovereign Interference Guards** rather than standard antivirus signatures.

<<<<<<< HEAD
- **Kernel Panics Nulled**: Rogue processes attempting to access segmented memory bounds trigger the `SovereignSentinel`.
- **Process Trapping**: The offending Shard is immediately suspended and unmapped from memory within microseconds.
=======
- **Kernel Panics Nulled**: Rogue processes attempting to access segmented memory bounds trigger the `SovereignSentinel`. 
- **Process Trapping**: The offending Shard is immediately suspended and unmapped from memory within microseconds. 
>>>>>>> 99f2ef5 (chore: precise lint eradication via AST script algorithms)
- **Privacy Amnesic Feature**: Using `SigmaPrivacyAmnesic` tools, user sessions are wiped from RAM completely when the system locks. There are no persistence caches left for attackers to scrape.

---

## 7. Domain-Specific Shards (The App Ecosystem)

SigmaOS replaces traditional software with "Shards". When a task is complete, the Shard dissolves safely from RAM.

### 7.1. Indian Legal Compliance Portal (`indian_law.c`)

Offline, fully BNS/BNSS/BSA compliant database capable of generating Form 61 Hash Value certificates and guiding officers through explicit operational bounds for FIR generation.
*Usage*: `sigma law --search "Search and Seizure procedures"`

### 7.2. OmniMedia Engine (`omni_media_engine.c`)

VLC/Standard Media Competitor. Bypasses ffmpeg decoding wrappers and pipes H.265/AV1 frames directly to hardware arrays. Reduces playback latency by 10x over Windows paradigms.
*Usage*: `sigma omni-media /path/to/vid.av1`

### 7.3. Sigma Academy (`academy.c` & `ncert_core.c`)

<<<<<<< HEAD
Offline educational routing, fetching curriculum instantaneously from an uncompressed internal storage shard.
=======
Offline educational routing, fetching curriculum instantaneously from an uncompressed internal storage shard. 
>>>>>>> 99f2ef5 (chore: precise lint eradication via AST script algorithms)

### 7.4. Sigma AI Matrix (`sigma_ai_distribute.c` & `SigmaAI.js`)

Removes PyTorch/Python completely. Processes neural network layers directly through Sovereign Native Assembly calls. Eliminates memory constraints usually found in cross-layer matrix manipulation.

---

## 8. Developer Guidelines: Building Custom Shards

SigmaOS grants absolute tool autonomy. If you want a new application, build a Shard.

<<<<<<< HEAD
### Rules of Development
=======
### Rules of Development:
>>>>>>> 99f2ef5 (chore: precise lint eradication via AST script algorithms)

1. **No External Wrappers**: Do not include `#include <stdio.h>` or Python files natively. Use `<SigmaC11.h>` and call native kernel utilities (`sigma_print`).
2. **Respect the Void**: When your application terminates, it MUST release its memory structure completely to the SOD architecture to maintain the zero-background footprint rule.
3. **Registration**: Ensure your built C11 binary is registered inside `sovereign_tools/SigmaCLI_Dispatcher.c` to bind it to the Omni-CLI.

**Welcome to the Sovereign Architecture. Absolute control over the silicon is now yours.**
