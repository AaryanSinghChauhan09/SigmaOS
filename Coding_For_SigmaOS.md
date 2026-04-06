# Developing for SigmaOS: The Mindset Shift

If you are a traditional software engineer, reading standard tutorials, expecting POSIX frameworks, or relying on high-level languages—stop.

Developing shards or scripts for SigmaOS is a fundamentally different discipline. This OS is governed by **Absolute Sovereignty and Zero Dependencies**. If you attempt to port standard Linux/Windows mentalities here, your code will fail compilation instantly via `SovereignBuildMaster.c`.

Here is exactly what you need to know before writing your first line of code for SigmaOS.

---

## 🛑 1. The Standard Library (`libc`) is Dead

Do **not** use `#include <stdio.h>`, `#include <stdlib.h>`, or `#include <string.h>`.
The compiler will reject them.

**The Default SigmaOS Substitution:**
All tools are routed through `SovereignLibC.h`.

* Need to print? You must use `sigma_printf()` which hooks `SYS_WRITE`.
* Need a string length? Use `sigma_strlen()`.
* Need a memory wipe? Use `sigma_memset()` or `sigma_zero_memory()`.

## 🧠 2. Forget `malloc` and Garbage Collection

There is no Python-esque garbage collection. There is no standard heap allocator like `malloc` or `free`. SigmaOS relies on a highly specialized **Slab Allocator** and **Physical Memory Manager (PMM)** structured to bypass general-purpose bottlenecks.

**How to Allocate Memory Natively:**

```c
// DO NOT DO THIS:
// char* buffer = (char*)malloc(1024);

// INSTEAD, use the Slab Allocator for objects:
void* object = sigma_slab_alloc(my_active_slab);

// Or map hardware pages directly:
void* large_buffer = sigma_pmm_alloc(5); // Allocates 5 physical pages (20KB)
```

## 🧩 3. Stop Building Monolithic Binaries

SigmaOS does not execute massive `ELF` binaries loaded with thousands of background daemons. You don't build "Applications"—you build **Shards**.

* A Shard is a lightweight, discrete `.c` file containing an entry function that maps directly to the `CLASS_DECLARE` macro logic (`SigmaOOP.h`).
* Shards are compiled and injected directly into Ring-0 memory at runtime using the CLI command:
  `sigma_invoke my_shard_name`
* When the Shard’s logic concludes, it is instantly unmapped from memory by the `SovereignProcessManager`.

## 🌐 4. Networking Bypasses the Kernel Socket Layer

If you are building an HFT or high-throughput Shard, do not rely on standard BSD Sockets (`socket()`, `bind()`, `listen()`).
SigmaOS utilizes **Zero-Copy DMA (Direct Memory Access)** ring buffers.

To read from the network, your Shard must be permitted (via Persona bounds) to poll the hardware NIC ring array directly. The kernel will format the contiguous byte block in `SovereignNetMesh.c`.

## ⚡ 5. No Background Runtimes (Strict HLL-Reduction)

SigmaOS is an omni-tool platform, but it will never natively interpret `.py`, `.js` (back-end logic), or Java Bytecode within the kernel boundary.
If you need automated scripts or macro sequences, they must either be:

1. Compiled dynamically via `sigma_invoke`.
2. Passed via the `omni_shell` native bash-replacement arrays.
3. Hooked via custom `.asm` logic mapped directly to an interrupt trigger by `keyboard_master.c`.

---

## 🛠️ The SigmaOS Development Golden Rule

**"If you do not absolutely explicitly invoke it, it does not execute."**

Every byte of logic you write must justify its own existence. Rely on intrinsic loops, utilize the `SovereignSuperCalculator.c` matrix algorithms, and trust the absolute unabstracted speed of native silicon execution.
