# SigmaOS Implementation Roadmap

> This roadmap tracks the development of core features for the **Zenith v15.1**and**Horizon** microkernel releases, fully aligned with the academic syllabus implementation map.

📚 **See the full Syllabus→SigmaOS mapping:** [Syllabus-Implementation-Map](Syllabus-Implementation-Map)

---

## 🎓 Syllabus-Driven Roadmap (Zenith v15.1)

All 14 academic subjects have been mapped to SigmaOS architecture layers. See individual pages:

| Subject | SigmaOS Layer | Wiki | 
| --- | --- | --- | 
| Fundamentals of CS & IT | Kernel + HAL + CLI + Office Suite | [FCIT](Syllabus-FCIT) | 
| Discrete Mathematics | Math/Logic Engine | [DiscreteMath](Syllabus-DiscreteMath) | 
| C Programming | Developer C API Layer | [C-Prog](Syllabus-C-Programming) | 
| C++ Programming | Kernel Core (C++17) | [Cpp-Prog](Syllabus-Cpp-Programming) | 
| RDBMS & Oracle SQL | SigmaDB Engine | [RDBMS](Syllabus-RDBMS) | 
| Statistics | SigmaStats Toolkit | [Statistics](Syllabus-Statistics) | 
| Web Programming | SigmaWeb Runtime | [WebProg](Syllabus-WebProgramming) | 
| OS Concepts | Kernel Implementation | [OS-Concepts](Syllabus-OS-Concepts) | 
| Python Programming | SigmaPy Runtime | [Python](Syllabus-Python) | 
| Data Warehousing & Mining | SigmaWarehouse + Analytics | [DWDM](Syllabus-DWDM) | 
| R Programming | SigmaR Runtime | [R-Prog](Syllabus-R-Programming) | 
| Advanced Python / Data Science | SigmaAI + NumPy Pipeline | [AdvPython](Syllabus-AdvPython) | 
| AI & Machine Learning | SigmaAI Intelligence Layer | [AIML](Syllabus-AIML) | 
| Data Modeling & Visualization | SigmaModeler + SigmaViz | [DataModeling](Syllabus-DataModeling) | 

---

## 🔑 Core Kernel Features

## 1. Bootloader (Bare-Metal Start)

**Status:**Partial — assembly boot stub exists.**Plan:** Write NASM/C bootloader → GDT/IDT setup → load microkernel into memory.

- **Docs:** `Bootloader-Design.md`

## 2. Kernel Modules Architecture

**Status:**Modular shard architecture defined.**Plan:** Hot-swappable kernel modules — scheduler, MMU, PQC attestation.

- **Docs:** `Kernel-Architecture.md`

## 3. Memory Management (S-MM)

**Status:**Paging partially implemented; slab allocator skeleton exists.**Plan:** Full 4-level paging (PML4), CoW fork, slab allocator, buddy system.

- **Docs:** `Sovereign-Memory-Management.md`

## 4. Process Scheduling (S-SCHED)

**Status:**Round-robin stub present.**Plan:** CFS-like scheduler, multi-core dispatch, thread isolation per shard.

- **Docs:** `Scheduling-Algorithms.md`

## 5. Device Drivers (HAL)

**Status:**Stubs for NVMe, USB, VGA exist.**Plan:** Full HAL registry — keyboard, mouse, NVMe, SATA, USB, VGA/VESA, audio.

- **Docs:** `Driver-Development.md`

## 6. Security Model (S-ARMOR)

**Status:**PQC key types defined; enforcement incomplete.**Plan:** Ring 0/3 separation, MAC namespaces, Kyber/Dilithium syscall signing.

- **Docs:** `Security-Model.md`

## 7. Networking Stack (S-NET)

**Status:**TCP/IP stubs only.**Plan:** Full TCP/IP stack, WebSocket support, encrypted IPC via SovereignIPC.

- **Docs:** `Networking.md`

## 8. User-Space Tooling

**Status:**sigma-cli partially implemented.**Plan:** Full shell + package manager + SigmaStore + sigma-doctor CLI.

- **Docs:** `User-Tools.md`

---

## 📊 Feature Status Summary

| Feature | Status | Priority | 
| --- | --- | --- | 
| SovereignCodec (number systems) | 🟡 Planned | High | 
| sigma-cli shell | 🟢 Partial | High | 
| HAL I/O Drivers | 🟢 Partial | High | 
| S-ZFS Storage | 🟢 Implemented | High | 
| SigmaDB SQL Engine | 🟡 Planned | High | 
| SigmaStats Toolkit | 🟡 Planned | Medium | 
| SigmaWeb Runtime | 🟡 Planned | Medium | 
| SigmaAI Layer | 🟡 Planned | Medium | 
| SigmaModeler (ERD) | 🟡 Planned | Low | 
| SigmaViz Dashboards | 🟡 Planned | Low | 
| SigmaDocs/Sheets/Slides | 🟡 Planned | Low | 
| SigmaPy / SigmaR Runtimes | 🟡 Planned | Medium | 
| SentinelNeural (antivirus) | 🟡 Planned | High | 
| SovereignCloudFS | 🟢 Partial | Medium | 
| SovereignNetStack | 🟡 Planned | High | 

---

*Last updated: 2026-05-18 | SigmaOS Zenith v15.1*
