# SigmaOS File Logic & Relationships

The SigmaOS repository operates on a strict **Zero-Dependency Microkernel Lattice** architecture. Here is the logic of how every file interacts:

## 1. The Core Primitives (`/include/`)
*   `sigma_kernel_types.h`: The absolute foundation. Defines `sigma_u32`, `SIGMA_OK`, and `sigma_malloc`. Every single file in the OS includes this. No C++ standard libraries are permitted.
*   `SigmaOOP.hpp`: Overloads `new` and `delete` to force memory allocation through our hardware-direct `sigma_malloc`, ensuring C++ classes can exist without `libc`.

## 2. Kernel Initialization (`/kernel/core/system/`)
*   `SovereignBoot.cpp`: The entry point. Traverses `SIGMA_BOOT_STAGE_INIT` to `SIGMA_BOOT_STAGE_USERLAND`.
*   `SovereignFS.cpp`: Mounts the journaling filesystem.

## 3. The Shards (`/kernel/core/`)
Each shard operates in isolated Ring-3 memory:
*   `linux_compat/SovereignLinuxSubsystem.cpp` (SSL): Absorbs Linux syscalls into SigmaOS events.
*   `network/SovereignNetStack.cpp`: TCP/IP sockets and loopback, wired to `SovereignFirewall.cpp`.
*   `desktop/SovereignWindowManager.cpp`: The Zenith GUI engine, rendering pixels without X11.
*   `ai/SovereignOpenClaw.cpp`: The autonomous AI agent ensuring system stability.

## 4. Userland & Tools (`/tools/` & `/userland/`)
*   `SovereignAppStore.cpp`: The GUI frontend for package resolution.
*   `/tools/pro/`: Profession-specific calculators (GST, Dosage, Structural Load) executing directly on the kernel.

**Relationship Logic:** Tools call the App Store -> App Store triggers `SovereignPkgManager` -> PkgManager authenticates via CRYSTALS-Dilithium-5 -> Kernel allocates memory via `sigma_malloc`.
