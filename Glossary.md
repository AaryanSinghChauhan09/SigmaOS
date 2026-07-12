# SigmaOS Glossary

| Term | Definition |
|------|-----------|
| **AVC** | Access Vector Cache — O(1) SELinux-inspired MAC policy cache |
| **Buddy allocator** | Physical memory allocator using 2^n page-frame blocks |
| **CFS** | Completely Fair Scheduler — red-black tree vruntime scheduler |
| **CRDT** | Conflict-free Replicated Data Type — merge without server |
| **Cgroup** | Control group — CPU, memory, I/O quotas per process group |
| **CryptFS** | SigmaOS encrypted filesystem layer using AES-256-GCM + TPM2 |
| **DID** | Decentralised Identifier — self-sovereign identity without central authority |
| **Dilithium-5** | NIST FIPS 204 post-quantum digital signature algorithm |
| **dm-verity** | Block-level integrity verification (hash tree over disk blocks) |
| **EDF** | Earliest-Deadline-First — hard real-time scheduling algorithm |
| **HAL** | Hardware Abstraction Layer — `SovereignHAL`, hides arch differences |
| **HPET** | High Precision Event Timer — hardware timer for jiffies |
| **IPC** | Inter-Process Communication — sigma-bus message passing |
| **Kyber-1024** | NIST FIPS 203 post-quantum key encapsulation mechanism |
| **KEM** | Key Encapsulation Mechanism — asymmetric key exchange |
| **LSTAR MSR** | x86 Model-Specific Register that holds the syscall entry point |
| **MLFQ** | Multi-Level Feedback Queue — interactive task scheduler |
| **MSI-X** | Message Signalled Interrupts Extended — PCI interrupt mechanism |
| **NTT** | Number Theoretic Transform — polynomial multiplication in Kyber |
| **OSTree** | Content-addressed OS update system — used for A/B atomic updates |
| **PGO** | Profile-Guided Optimisation — compiler uses runtime profiles |
| **PLT** | Procedure Linkage Table — dynamic linker trampoline (avoided in kernel) |
| **PQC** | Post-Quantum Cryptography — resistant to quantum computer attacks |
| **Ring 0** | Kernel privilege level (full hardware access) |
| **Ring 3** | User-space privilege level (no direct hardware access) |
| **S-BUSE** | Branch Uniformity & Synchronisation Engine — keeps all branches in sync |
| **SDF** | Sovereign Driver Framework — SigmaOS driver lifecycle API |
| **Shard** | Atomic capability module — 600+ numbered `S001–S500+` |
| **sigma_pledge** | OpenBSD-inspired syscall capability restriction per process |
| **sigma_unveil** | OpenBSD-inspired filesystem path restriction per process |
| **sigma-bus** | SigmaOS IPC mechanism — capability-gated message passing |
| **sigma-pkg** | Sovereign Package Manager — `.spkg` format, Dilithium-5 signed |
| **SIMD** | Single Instruction, Multiple Data — AVX-512, ARM NEON |
| **Slab allocator** | Kernel allocator using per-type object caches (kmalloc) |
| **SPIFFE** | Secure Production Identity Framework for Everyone — workload identity |
| **SPKG** | Sovereign Package — SigmaOS native package format |
| **SVID** | SPIFFE Verifiable Identity Document — cryptographic workload ID |
| **TLB** | Translation Lookaside Buffer — hardware cache for page table entries |
| **TPM2** | Trusted Platform Module v2 — hardware root of trust for key sealing |
| **UEFI** | Unified Extensible Firmware Interface — modern PC boot standard |
| **UBC** | Unified Buffer Cache — page cache shared between filesystem and VM |
| **VFS** | Virtual Filesystem Switch — unified inode/file interface over all FSes |
| **VMA** | Virtual Memory Area — region in a process's address space |
| **VMM** | Virtual Memory Manager — manages page tables and address spaces |
| **W^X** | Write XOR Execute — no memory page is both writable and executable |
| **WPA3/SAE** | Wi-Fi Protected Access 3 with Simultaneous Authentication of Equals |
| **Zenith** | SigmaOS desktop environment and v15.0 release codename |
| **Zero-trust** | Security model: verify every request, grant minimum necessary access |

---

### See also: [Architecture-Overview](Architecture-Overview) · [FAQ](FAQ)
