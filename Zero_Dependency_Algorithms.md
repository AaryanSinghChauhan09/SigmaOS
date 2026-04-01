# ⚙️ Zero-Dependency Algorithm Reference

Every algorithm used in the SigmaOS kernel is a **user-defined, zero-dependency implementation** — no `strncpy`, no `qsort`, no `malloc` from the standard library. This page catalogs them all.

---

## String Algorithms (`sigma_libc.c` / `sigma_std.c`)

| Function | Equivalent | Complexity | Description |
|----------|-----------|------------|-------------|
| `sigma_strlen(s)` | `strlen` | O(N) | Null-terminated string length |
| `sigma_strcmp(a, b)` | `strcmp` | O(N) | Lexicographic comparison |
| `sigma_strcpy(dst, src)` | `strcpy` | O(N) | String copy |
| `sigma_strcat(dst, src)` | `strcat` | O(N) | String concatenation |
| `sigma_itoa(n, buf)` | `itoa` | O(log N) | Integer to ASCII string |
| `sigma_atoi(s)` | `atoi` | O(N) | ASCII string to integer |
| `sigma_kmp_search(text, pat)` | `strstr` | O(N+M) | KMP pattern matching |

---

## Memory Algorithms (`sigma_libc.c`)

| Function | Equivalent | Description |
|----------|-----------|-------------|
| `sigma_memset(dst, val, n)` | `memset` | Fill memory block with value |
| `sigma_memcpy(dst, src, n)` | `memcpy` | Copy memory block |
| `sigma_memmove(dst, src, n)` | `memmove` | Overlap-safe memory copy |
| `sigma_memcmp(a, b, n)` | `memcmp` | Memory block comparison |
| `sigma_zero_memory(dst, n)` | — | Zero-fill (amnesic wipe) |

---

## Sorting Algorithms (`SovereignSuperCalculator.c` / `sigma_std.c`)

| Algorithm | Complexity (avg) | Complexity (worst) | Stable? |
|-----------|------------------|--------------------|---------|
| Quicksort | O(N log N) | O(N²) | No |
| Merge Sort | O(N log N) | O(N log N) | Yes |
| Heap Sort  | O(N log N) | O(N log N) | No |
| Insertion Sort | O(N²) | O(N²) | Yes |
| Counting Sort | O(N + K) | O(N + K) | Yes |

All implemented as native C11 recursive / iterative functions using the stack — no `stdlib.h qsort`.

---

## Memory Allocation (`kernel/pmm.c`, `kernel/slab.c`)

### Physical Memory Manager (PMM)

```c
// Allocate n physical pages (4KB each)
void* sigma_pmm_alloc(sigma_u32 n_pages);

// Free n physical pages
void sigma_pmm_free(void* addr, sigma_u32 n_pages);
```

### Slab Allocator

```c
// Initialize a slab cache for objects of 'size' bytes
SigmaSlab_t* sigma_slab_create(sigma_u32 obj_size, sigma_u32 capacity);

// Allocate one object from slab
void* sigma_slab_alloc(SigmaSlab_t* slab);

// Return one object to slab
void sigma_slab_free(SigmaSlab_t* slab, void* obj);
```

---

## Hash Algorithms

```c
// FNV-1a 64-bit hash (used in VFS and HashMap)
sigma_u64 sigma_fnv1a(const char* key, sigma_u32 len);

// SHA-256 (custom implementation for forensic log signing)
void sigma_sha256(const sigma_u8* data, sigma_u32 len, sigma_u8* digest);
```

---

## Graph Algorithms (`SovereignSuperCalculator.c`)

```c
// BFS from source node
void sigma_bfs(sigma_u32** adj, sigma_u32 n, sigma_u32 src, sigma_u32* dist);

// DFS (recursive)
void sigma_dfs(sigma_u32** adj, sigma_u32 n, sigma_u32 node, sigma_bool* visited);

// Dijkstra's shortest path
void sigma_dijkstra(sigma_i64** graph, sigma_u32 n, sigma_u32 src, sigma_i64* dist);
```

---

## I/O Algorithms (`SovereignLibC.asm`)

All I/O bypasses the C runtime and calls the kernel directly via inline Assembly:

```c
// Write string to stdout (wraps SYS_WRITE = 1)
void sigma_printf(const char* str);

// Read from stdin (wraps SYS_READ = 0)
sigma_i64 sigma_read(char* buf, sigma_u32 len);
```

---

## Complexity Cheat Sheet (DSA Shard Reference)

| Operation | Array | Linked List | Hash Map | BST | Heap |
|-----------|-------|-------------|----------|-----|------|
| Access | O(1) | O(N) | O(1) avg | O(log N) | O(N) |
| Search | O(N) | O(N) | O(1) avg | O(log N) | O(N) |
| Insert | O(N) | O(1) | O(1) avg | O(log N) | O(log N) |
| Delete | O(N) | O(1) | O(1) avg | O(log N) | O(log N) |
