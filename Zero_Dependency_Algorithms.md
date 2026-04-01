# ⚙️ Zero-Dependency Algorithm Reference

Every algorithm used in the SigmaOS kernel is a **user-defined, zero-dependency implementation** — no `strncpy`, no `qsort`, no `malloc` from the standard library. This page catalogs the entire mathematical subset engineered natively for the Shard framework in table format.

---

## 🧵 String Algorithms (`sigma_libc.c` / `sigma_std.c`)

| Function Call | Standard Lib Equivalent | Time Complexity | Native Implementation Description |
|---|---|---|---|
| `sigma_strlen(s)` | `strlen` | O(N) | Measures null-terminated array lengths without referencing `string.h`. |
| `sigma_strcmp(a, b)` | `strcmp` | O(N) | Deep lexicographic byte-by-byte comparison logic. |
| `sigma_strcpy(dst, src)` | `strcpy` | O(N) | Memory-safe iterative string cloning. |
| `sigma_strcat(dst, src)` | `strcat` | O(N) | Direct native memory pointer concatenation. |
| `sigma_itoa(n, buf)` | `itoa` | O(log N) | Converts numeric intrinsic values to ASCII representations manually. |
| `sigma_atoi(s)` | `atoi` | O(N) | Parses raw ASCII byte structures to reconstruct integers. |
| `sigma_kmp_search(text, pat)` | `strstr` | O(N+M) | Knuth-Morris-Pratt pattern matching utilized heavily by the Amnesic Shard. |

---

## 🧠 Memory Algorithms (`sigma_libc.c`)

| Function Call | Standard Lib Equivalent | Action Documented |
|---|---|---|
| `sigma_memset(dst, val, n)` | `memset` | Fills rapid contiguous memory blocks utilizing low-level x86 instructions. |
| `sigma_memcpy(dst, src, n)` | `memcpy` | Native loop logic structured for unrolled AVX register copying. |
| `sigma_memmove(dst, src, n)` | `memmove` | Detects overlaps to prevent data obliteration during block shifting. |
| `sigma_memcmp(a, b, n)` | `memcmp` | Raw binary block assertions. |
| `sigma_zero_memory(dst, n)` | — (DOD 5220.22-M) | Volatile memory scrubber explicitly engineered for strict Amensic security protocols. |

---

## 📊 Sorting Algorithms (`SovereignSuperCalculator.c` / `sigma_std.c`)

All implementations run as absolute native `C11` recursive or iterative structures. No `qsort` wrapper layers.

| Algorithm | Av. Complexity | Worst-Case | In-Place Native Logic |
|---|---|---|---|
| **Quicksort Iterative** | O(N log N) | O(N²) | Heavily optimized for the ML shard matrices using a pivot-bound loop. |
| **Merge Sort** | O(N log N) | O(N log N) | Used strictly when topological stability is required by the File System chunks. |
| **Heap Sort** | O(N log N) | O(N log N) | Used extensively by the `SovereignProcessManager` slab allocation system. |
| **Insertion Sort** | O(N²) | O(N²) | Triggered parametrically when sub-arrays drop below 16 elements in sorting shunts. |
| **Counting Sort** | O(N + K) | O(N + K) | Specialized logic used by HFT to rapid-bucket micro-second latency arrays. |

---

## 📂 Custom Kernel Architecture Algorithms

| Fundamental Capability | Reference Call | Execution Purpose |
|---|---|---|
| **Physical Memory Pagination** | `sigma_pmm_alloc(n_pages)` | Bypasses `malloc` to directly map 4KB hardware page frames. |
| **Native Slab Structuring** | `sigma_slab_create(size)` | Dedicated continuous object array structures mapping memory deterministically. |
| **VFS Object Indexing** | `sigma_fnv1a(key, len)` | Computes rapid FNV-1a 64-bit metadata hashes for file directory snapshots. |
| **Forensic Audit Signing** | `sigma_sha256(data, len)` | Zero-dependency implementation of SHA-256 for signing amnesic scrub logs securely. |

---

## 🕸️ Sovereign Graph Topologies

These are utilized natively by the Data Science (`SigmaDS.js`) and Routing Shards.

| Algorithm | Source Implementation | Purpose |
|---|---|---|
| **Breadth-First Native** | `sigma_bfs(adj, src)` | Network protocol traversing and UI node targeting algorithms. |
| **Recursive Deep-Trace** | `sigma_dfs(adj, node)` | Process dependency graph hunting (killing orphaned process trees). |
| **Absolute Priority Route** | `sigma_dijkstra(graph, src)` | Minimum latency routing protocol constructed specifically for HFT networks. |

---

## 💾 I/O Algorithms (`SovereignLibC.asm`)

All standard user-space buffers are avoided. I/O invokes absolute ring-0 `syscall` assembly interrupts.

| Function | Descriptor Interrupt | Mechanism |
|---|---|---|
| `sigma_printf(str)` | `SYS_WRITE = 1` | Translates directly into internal register `RAX` pushing string arrays to the framebuffer DMA limits. |
| `sigma_read(buf, len)` | `SYS_READ = 0` | Intercepts keyboard hardware limits translating keycodes down the interrupt matrix. |

---

## 📘 Complexity Cheat Sheet (DSA Shard Reference Matrix)

Used by the Education/DSA Orchestrator internally.

| Internal Structure | Access Latency | Rapid Search | Native Insert | Secure Delete |
|---|---|---|---|---|
| Contiguous Arrays | O(1) | O(N) | O(N) | O(N) |
| Linked Pointer Lists | O(N) | O(N) | O(1) | O(1) |
| Hashed Memory Maps | O(1) avg | O(1) avg | O(1) avg | O(1) avg |
| Binary Search Trees | O(log N) | O(log N) | O(log N) | O(log N) |
| Binary Priority Heaps | O(N) | O(N) | O(log N) | O(log N) |
