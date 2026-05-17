# SigmaOS Glossary

Understanding SigmaOS terminology is essential for contributing. Below are the definitions of core concepts used throughout the architecture.

 **Atomic Shard**: A self-contained, highly specialized module (e.g., memory manager, file system) that operates independently within the microkernel.

 **Compositor**: The UI engine (Zenith Desktop) responsible for combining graphical windows into the final display buffer using hardware acceleration.

 **HAL (Hardware Abstraction Layer)**: The `SovereignHAL` subsystem that bridges the kernel and the physical silicon, abstracting CPU architectures (x86, ARM).

 **Lattice**: The overarching interconnected framework of shards that makes up the SigmaOS environment. It is decentralized, meaning shards communicate via messages rather than direct memory access.

 **PQC (Post-Quantum Cryptography)**: Cryptographic algorithms (like Kyber and Dilithium) implemented in the kernel designed to be secure against attacks by quantum computers.

* **Profession Matrix**: A set of pre-configured OS profiles (e.g., S-FINANCE, S-MEDICAL) that dynamically load only the shards required for specific industries.

 **Sovereign**: Indicates complete independence from legacy monolithic kernels (like Linux or NT). SigmaOS is built from the ground up, sharing no code with standard systems.

* **Syscall (Z-SYSCALL)**: The Application Binary Interface used by userland programs to request services from the kernel safely.

 **Zenith Singularity**: The code name for the v15.0 stable release of SigmaOS, representing the point where the architecture becomes fully production-ready.

* **Zero-Data Remanence**: A security principle ensuring that once memory or storage is freed, it is immediately wiped to prevent data scavenging.
