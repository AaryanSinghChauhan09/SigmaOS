# Low-Level Independence Architecture Spec

## Motivation
Modern operating systems often suffer from extreme bloat, pulling in massive hierarchies of libraries, interpreters, and high-level programming language runtimes. SigmaOS rejects this bloat. To maintain extreme performance, auditable security, and maximum bare-metal optimization, SigmaOS strictly limits its dependency on external pre-defined libraries and high-level abstractions in the core system.

## Principles

1. **Custom Implementations Over Black-Box Libraries**
   Whenever feasible, SigmaOS relies on in-house implementations of core utilities rather than pulling in massive pre-existing generic libraries. E.g., custom string manipulation, lightweight allocators, and bespoke crypto primitives tailored to modern instruction sets.

2. **Language Strictness**
   - **Kernel & Drivers**: Written exclusively in `no_std` Rust, bare-metal C, and Assembly.
   - **Userland Core (Init, sigpkg, IPC)**: Written with minimal abstractions, avoiding heavy POSIX compatibility layers where not needed.
   - **Avoid Interpreted/JIT Languages for Core Services**: Python, Node.js, and Java are restricted to user application domains (e.g., AI models or UI layers) and are completely banned from system initialization, package management, and kernel modules.

3. **Eliminating the Pre-Defined Function Crutch**
   Developers working on SigmaOS core components are encouraged to fully understand the underlying mechanics of their code. We discourage blindly using pre-defined helper functions from generic frameworks. If an algorithm requires a specific data structure, we implement a memory-efficient, highly-targeted version of it rather than pulling in a generalized hashmap or vector implementation with massive overhead.

4. **Hardware Specific Optimization**
   By shedding the burden of generic legacy compatibility wrappers, SigmaOS code can target the specific features of modern silicon (e.g., AVX-512, ARM SVE, hardware crypto extensions) directly using intrinsic functions or inline assembly.

5. **Toolchain and Base Environment**
   - **Base Environment**: SigmaOS utilizes lightweight alternatives to heavy predefined libraries. We use `musl libc` instead of glibc where C is strictly required, and rely on `BusyBox` or a custom Rust equivalent for coreutils, rejecting bloated GNU equivalents for the base system.
   - **Toolchains**: We enforce minimal and strictly auditable toolchains (LLVM/Clang and GCC) without heavy wrappers or fragmented build systems.

## Roadmap Integration
- Refactor `sigpkg` to ensure it only depends on the most critical dependencies (e.g., standard `core` Rust library, zero overhead).
- Ensure the MicroVM isolation runtime relies strictly on raw KVM IOCTLs and memory mapping without intermediary libvirt/QEMU bloat.
- Zenith Wayland desktop will implement its own rendering pipeline focused on pure Vulkan/DRM interaction, discarding massive legacy toolkits where possible.
