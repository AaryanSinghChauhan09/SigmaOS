# The Purge: Features Lost to Sovereignty

The path to absolute system sovereignty required deep sacrifices. To achieve our zero-dependency, true bare-metal C11/Assembly vision with sub-nanosecond latency, we deliberately excised several standard features that are commonplace in bloat-heavy modern operating systems. 

This page catalogs the "Lost Features," explaining why they no longer exist in the SigmaOS ecosystem.

## 1. The Standard C Library (libc / glibc)
- **Status:** Purged
- **Reasoning:** Relying on standard `stdio.h`, `stdlib.h`, or generic Windows/Linux headers abstracts away the machine layer and injects unneeded cross-platform boilerplate. 
- **Replacement:** The completely native custom `SovereignLibC.h`, built fully on granular ring-0 syscalls without legacy overhead.

## 2. Bloated Desktop Environments (DEs)
- **Status:** Purged
- **Reasoning:** Environments like KDE Plasma or GNOME require massive dependency cascades, rendering pipelines, and background daemons that siphon CPU cycles from HFT, Bio-Informatics, and AI shards.
- **Replacement:** A highly optimized, pure JS/DOM UI Orchestrator using `SigmaSystem` to maintain fractional memory footprints while delivering a Zenith-Gold aesthetic.

## 3. High-Level Language Interpreters (Python, Perl, Node.js)
- **Status:** Purged from Kernel/Native Base
- **Reasoning:** Interpreted languages inherently introduce overhead, garbage collection pauses, and execution unpredictability. We cannot calculate Bio-Informatics sequences or execute high-frequency trades securely if the OS relies on an interpreter matrix.
- **Replacement:** Absolute HLL-Reduction policy. Shards are written entirely in C11 and Assembly modules. 

## 4. Monolithic Pre-loaded Drivers
- **Status:** Purged
- **Reasoning:** Traditional operating systems boot with thousands of pre-loaded drivers for hardware you don't even own, eating up RAM.
- **Replacement:** Shard-On-Demand (SOD). Drivers and functionalities are loaded dynamically only when explicitly invoked by the user, providing true hardware democratization.

## 5. POSIX Compliance Overhead
- **Status:** Partially Purged / Refactored
- **Reasoning:** Full POSIX strictly regulates internal function behavior and timing, often forcing inefficient locking and signal handling.
- **Replacement:** Proprietary sovereign toolchains that mimic command names (maintaining user parity) but operate under native lock-free concurrency and custom IPC mechanisms beneath the surface.

## 6. Monolithic Package Managers
- **Status:** Purged
- **Reasoning:** Centralized package managers (like `apt` or `pacman`) rely on a cascade of PGP keys, third-party mirrors, and cloud dependencies, which violates our first Principle of Sovereignty: *No cloud dependencies.*
- **Replacement:** Everything required for a sovereign workflow is compiled locally via local shard loading or native C11 assembly linkage.

While standard operating systems mourn the loss of these abstracted layers, SigmaOS users embrace the direct, unfiltered power that comes with their excision.
