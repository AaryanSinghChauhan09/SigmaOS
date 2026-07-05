# Operating System Concepts → SigmaOS Kernel

> Maps the OS Concepts syllabus directly to the SigmaOS Zenith microkernel implementation, emphasizing silicon sovereignty, deterministic resource management, and failure isolation.

---

## Unit I: Introduction to Operating Systems & Sovereignty

### What is an OS?

An Operating System is foundational system software that manages hardware resources, schedules computational execution, and provides protected service abstractions to application programs.

**Unique Selling Point (USP):** Deterministic resource management, absolute kernel stability, and silicon-direct execution free from high-level runtime overhead.

**SigmaOS Position:** A sovereign bare-metal microkernel OS — zero-dependency, freestanding C++17, and AVX-512 optimized for silicon sovereignty.

### OS Functions → SigmaOS Modules

| OS Function | SigmaOS Module | File |
| :--- | :--- | :--- |
| **Process Management** | `SovereignScheduler` | `kernel/core/SovereignScheduler.cpp` |
| **Memory Management** | `SovereignAllocator` | `kernel/core/SovereignAllocator.cpp` |
| **File System** | `SovereignFS` + S-ZFS | `kernel/fs/` |
| **Device Management** | HAL Driver Registry | `kernel/core/drivers/` |
| **Security** | `SentinelNeural` | `kernel/security/` |
| **UI** | `ZenithDesktop` | `userland/desktop/` |
| **Networking** | `SovereignNetStack` | `kernel/net/` |

### Types of OS

| Type | Example | SigmaOS Parallel |
| :--- | :--- | :--- |
| **Batch** | IBM OS/360 | — |
| **Time-sharing** | Unix | SigmaOS multi-user interactive |
| **Real-time** | VxWorks | RTOS format of SigmaOS |
| **Distributed** | Plan 9 | SigmaOS cluster mode |
| **Embedded** | FreeRTOS | SigmaOS embedded build |
| **Microkernel** | Mach, L4 | **SigmaOS Zenith** |

---

## Unit II: Process & Thread Management

### Process States

```
NEW → READY → RUNNING → TERMINATED
                ↓ ↑
             WAITING/BLOCKED
```

```cpp
// kernel/core/SovereignScheduler.cpp
enum class ProcessState {
    NEW,       // Just created, not yet admitted
    READY,     // In run queue, waiting for CPU
    RUNNING,   // Currently executing on a CPU core
    WAITING,   // Blocked on I/O or event
    TERMINATED // Finished, waiting for cleanup
};

struct SovereignProcess {
    uint32_t     pid;
    uint32_t     ppid;           // Parent PID
    Process_State state;
    uint8_t      priority;       // 0-255
    uint64_t     stack_base;
    uint64_t     pc;             // Program Counter
    uint64_t     sp;             // Stack Pointer
    uint64_t     cpu_time_ns;    // Time on CPU
    char         name[64];
};
```

### Scheduling Algorithms

```cpp
class SovereignScheduler {
    // FCFS: First-Come First-Served (non-preemptive)
    SovereignProcess* fcfs_next();

    // SJF: Shortest Job First
    SovereignProcess* sjf_next();

    // Round Robin (preemptive, time quantum = 10ms)
    SovereignProcess* round_robin_next(uint32_t time_quantum_ms);

    // Priority Scheduling
    SovereignProcess* priority_next();

    // CFS: Completely Fair Scheduler (Linux-inspired)
    // vruntime = actual_runtime * (default_weight / task_weight)
    SovereignProcess* cfs_next();

    // Multi-core dispatch
    void dispatch_to_core(SovereignProcess* p, uint32_t core_id);
};
```

### Threads & Synchronization

```cpp
// Thread vs Process
// Process: separate address space, heavy context switch
// Thread: shared address space, light context switch

class SovereignThread {
    uint32_t tid;
    uint32_t pid;  // owning process
    uint64_t stack_base;
    uint64_t register_state[16];
    ThreadState state;

public:
    static SovereignThread* create(SovereignProcess* parent, void (*entry)(void*));
    void join();
    void detach();
};

// Synchronization
class SovereignMutex {
    std::atomic<bool> locked;
public:
    void lock();
    void unlock();
    bool try_lock();
};

class SovereignSemaphore {
    std::atomic<int> count;
public:
    void wait();   // P operation (down)
    void signal(); // V operation (up)
};
```

### Inter-Process Communication (IPC)

```cpp
// Pipes
SigmaPipe pipe = sigma_pipe_create();
pipe.write("hello from parent", 17);
pipe.read(buffer, 64);

// Message Queues
SigmaMQ mq = sigma_mq_open("/sigma/ipc/kernel_events");
mq.send({ .type = MSG_PROCESS_DIED, .pid = 42 });

// Shared Memory
void* shm = sigma_shm_create("/sigma/shm/framebuffer", FRAMEBUFFER_SIZE);
sigma_shm_attach(shm);

// Signals
sigma_signal(PROC_42, SIGTERM);  // Request termination
sigma_signal(PROC_42, SIGKILL);  // Force termination
```

---

## Unit III: Memory Management

### Memory Hierarchy

```
Registers (< 1ns) → L1 Cache (1ns) → L2 Cache (3ns) → L3 Cache (10ns)
→ RAM (50ns) → NVMe SSD (100μs) → HDD (10ms) → Cloud (RTT)
```

### Paging

```cpp
// 4-level paging (x86-64): PML4 → PDPT → PD → PT → Physical
// Virtual Address: [PML4 idx 9b][PDPT idx 9b][PD idx 9b][PT idx 9b][Offset 12b]

struct PageTableEntry {
    uint64_t present      : 1;   // Is page in RAM?
    uint64_t writable     : 1;   // Can write?
    uint64_t user_access  : 1;   // User or kernel only?
    uint64_t write_through: 1;
    uint64_t cache_disable: 1;
    uint64_t accessed     : 1;
    uint64_t dirty        : 1;
    uint64_t huge_page    : 1;   // 2MB or 1GB page
    uint64_t global       : 1;
    uint64_t reserved     : 3;
    uint64_t phys_addr    : 40;  // Physical frame number
    uint64_t reserved2    : 11;
    uint64_t nx           : 1;   // No-execute bit
};
```

### Virtual Memory & Page Replacement

```cpp
// Page Fault Handler
void sigma_page_fault_handler(uintptr_t faulting_addr, uint32_t error_code) {
    if (error_code & PAGE_NOT_PRESENT) {
        // Load page from swap or file
        load_page_from_swap(faulting_addr);
    } else if (error_code & WRITE_VIOLATION) {
        // Copy-on-write for fork()
        cow_copy_page(faulting_addr);
    } else {
        sigma_panic("Illegal memory access at 0x%llx", faulting_addr);
    }
}

// Page Replacement Algorithms
// FIFO: evict oldest loaded page
// LRU: evict least recently used
// Optimal: evict page used furthest in future (theoretical)
// Clock Algorithm: circular buffer with reference bits (SigmaOS uses this)
```

### Memory Allocation

```cpp
// Buddy System: split/merge powers-of-2 blocks
void* buddy_alloc(size_t size);  // 4KB, 8KB, 16KB...

// Slab Allocator: fixed-size object caches (used for kernel objects)
SigmaSlabCache* proc_cache = sigma_slab_create(sizeof(SovereignProcess));
SovereignProcess* p = (SovereignProcess*)sigma_slab_alloc(proc_cache);
```

---

## Unit IV: I/O & File Systems

### I/O Subsystem

```cpp
// I/O Request Queue (elevator algorithm for HDDs)
struct IORequest {
    uint64_t sector;
    size_t   count;
    void*    buffer;
    IODir    direction;  // READ / WRITE
    IOPriority priority;
};

class SovereignIOScheduler {
    // Algorithms: FCFS, SSTF, SCAN (Elevator), C-SCAN, LOOK
    void enqueue(IORequest* req);
    IORequest* next_request();  // Returns by SCAN order
};
```

### File System — S-ZFS

```
SigmaOS Virtual File System (VFS)
├── /sigma/          — OS root
│   ├── kernel/      — Kernel modules
│   ├── apps/        — Userland applications
│   ├── log/         — System logs
│   ├── devices/     — Device files (like /dev)
│   ├── proc/        — Process info (like /proc)
│   └── data/        — User data
│
Backed by: SovereignZFSPool (CoW, Snapshots, RAID-Z, PQC encrypted)
```

```cpp
// File Operations
SigmaFile* f = sigma_fopen("/sigma/data/test.txt", "rw");
sigma_fseek(f, 0, SEEK_END);
size_t size = sigma_ftell(f);
sigma_fread(buf, 1, size, f);
sigma_fwrite(new_data, 1, len, f);
sigma_fclose(f);

// Directory Operations
SigmaDir* dir = sigma_opendir("/sigma/apps");
SigmaDirEntry* entry;
while ((entry = sigma_readdir(dir)) != nullptr) {
    sigma_klog(LOG_INFO, "  %s\n", entry->name);
}
sigma_closedir(dir);
```

---

## Debugging & Problem-Solving in Operating Systems

### Common Issues & Fix Strategies

- **Issue - Kernel Memory Leaks:** Unreleased slab allocations or orphaned page frames exhaust physical RAM over time.
- *Fix Strategy:* Implement strict RAII smart pointer wrappers (`SigmaUniquePtr`), execute automated kernel slab leak tracking, and run Valgrind/KASAN profiling tools during boot validation.

- **Issue - Concurrency Deadlocks:** Threads acquire multiple mutex locks in conflicting orders, stalling kernel execution indefinitely.
- *Fix Strategy:* Enforce a strict lock acquisition hierarchy across all kernel subsystems, utilize `try_lock` with exponential backoff, and integrate priority inheritance protocols to prevent priority inversion.

- **Issue - File System Corruption & Incorrect Indexing:** Sudden power loss leaves VFS B+ Tree directory indices fragmented or corrupted.
- *Fix Strategy:* Utilize S-ZFS Copy-on-Write (CoW) transactional semantics, ensuring directory structures are updated atomically via merkle tree root updates without requiring fsck disk scrubbing.

- **Issue - Unhandled Interrupt Storms:** Faulty peripheral hardware floods the APIC with unhandled IRQs, starving userland CPU execution.
- *Fix Strategy:* Implement interrupt throttling and switch from pure interrupt-driven I/O to polling mode (NAPI equivalent) under high packet/event loads.

---

### Last updated: 2026-05-19 | SigmaOS Zenith v15.2
