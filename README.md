1


SigmaOS is an experimental, bare-metal operating system kernel built to explore extreme modularity using C++ singleton patterns. While currently in a conceptual phase (v100.0), its goal is to provide a clean, zero-dependency alternative to legacy monolithic kernels.


1


SigmaOS is **not** a daily-driver operating system. Currently, the kernel can:


1


SigmaOS currently lacks a fully functional filesystem, robust device drivers (e.g., USB, GPU), and a mature networking stack, though stubs exist.


1



1



1



1



1


make clean
make singularity


1



1



1



1


make zenith-iso


1



1



1



1


make qemu


1



1



1


SigmaOS uses unique terminology for its architectural concepts. Here is what they mean in standard OS engineering terms: | SigmaOS Term | Standard Technical Meaning | | :------------------------------- | :----------------------------------------------------------------------------------------------- | | **Sovereign Lattice** | The operating system architecture as a whole. | | **Shard** | A distinct subsystem or driver encapsulated as a C++ Singleton class. | | **Amnesic Memory** | Stateless RAM allocation; memory buffers that are eagerly zeroed out after use to prevent leaks. | | **Zenith** | The target milestone version denoting a stable, complete foundation. | | **ZCLN (Zero-Copy Lattice Net)** | A zero-copy networking stack (bypassing redundant buffer copies between kernel and userland). | ## 🤝 Contributing

We welcome contributions to help evolve SigmaOS from an experimental kernel into a fully usable distribution.


1



1


SigmaOS has reached its architectural zenith. The kernel is now:


1


For a detailed look at our implementation history, refer to the project Wiki and GitHub Insights.

---


1

