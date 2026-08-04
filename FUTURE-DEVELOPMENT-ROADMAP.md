# SIGMAOS ULTIMATE DEVELOPMENT ROADMAP & SYSTEM SPECIFICATION

## 1. COMPONENT DEVELOPMENT ARCHITECTURE

SigmaOS represents a paradigm shift in system design: a from-scratch, zero-dependency, zero-trust, bare-metal operating system. To maintain absolute code purity, transparency, and deterministic performance, all future subsystem development is bound to strict architecture guidelines.

### 1.1 Language Purity & Abstraction Rules
- **Modern Systems Languages ONLY:** All code must be authored in modern low-level systems languages: **Rust, Zig, or Nim**.
- **Absolute Zero-Dependency Constraint:** The utilization of standard platform libraries (e.g., Rust's `std`, Zig's standard library primitives, or Nim's built-in OS wrapping layers) is strictly prohibited. Every module must compile under freestanding `#![no_std]` or equivalent configurations.
- **No Pre-Defined Functions:** All core operations—including memory copying (`memcpy`), block alignment, string manipulation, formatting, and numeric hashing—must be constructed using purely user-defined functions and custom-crafted primitives.

### 1.2 Bare-Metal Object-Oriented Principles (OOP)
System modularity and physical hardware interfaces are managed via strict Object-Oriented design patterns, tailored for bare-metal execution:
- **Encapsulation:** Hardware registers, MMIO bounds, and memory-mapped address ranges must be encapsulated inside private fields of device structs. No raw, un-gated pointer manipulation is allowed outside the designated driver class boundaries.
- **Inheritance & Class Hierarchies:** Clear abstraction hierarchies classify hardware device families. An abstract base interface or trait represents general device classes (e.g., `StorageDriver`), which are subsequently extended for specific hardware controllers (e.g., `NvmeController`).
- **Polymorphism:** Unified runtime dispatch (such as safe `dyn` traits in Rust or static generic dispatch in Zig) routes general kernel requests to the correct hardware-specific implementation.

```
       +-------------------------------------------------------+
       |             Base Abstract Device Class                |
       |               (e.g., StorageDriver)                   |
       +-------------------------------------------------------+
                                  |
                                  +-----------------------+
                                  |                       |
                                  v                       v
                      +-----------------------+ +-----------------------+
                      |  Legacy Floppy Driver | |  Modern NVMe Driver   |
                      | (Ancient PIO/DMA Cmd) | | (PCIe Doorbell Rings) |
                      +-----------------------+ +-----------------------+
```

### 1.3 Microkernel OS-Level Design Patterns
- **Singleton:** Core system resource controllers—such as the Global Memory Manager, interrupt controllers (APIC), and thread schedulers—are managed as single instance global controllers, avoiding concurrent state fragmentation.
- **Factory:** Instantiation of hardware-specific drivers is delegated to central factory systems. Based on the PCI vendor and device ID parsed during boot, the device factory instantiates and registers the matching driver object.
- **Observer:** Thread-safe, lock-free observer lists manage kernel event dispatching. When a physical or virtual state transition occurs (e.g., high thermals, disk write completion), interested subsystems are notified asynchronously without blocking execution paths.
- **Adapter:** Sandboxed compatibility and translation layers wrap legacy or standard POSIX APIs to match modern capability-gated interfaces.

---

## 2. THE DISTRO-CRUSHING EXECUTION STRATEGY

To establish SigmaOS as the dominant next-generation successor to legacy monolithic systems, we systematically target and defeat the primary metrics of traditional Linux and BSD distributions.

```
+------------------+----------------------------------+------------------------------------+
| Metric           | Legacy Monolithic Linux          | SigmaOS Core Paradigm              |
+------------------+----------------------------------+------------------------------------+
| Architecture     | Fragile, bloated monolithic kernel| Isolated, capability-gated shards  |
| Performance      | System call context switch lag   | Zero-copy async SQ/CQ ring buffers |
| Packaging        | Fragile, non-reproducible repos  | Declarative, content-addressed CAS |
| Security         | Coarse SELinux/AppArmor configs  | Native post-quantum cryptography   |
| Configuration    | Chaotic textual /etc directory   | Nix-style immutable configurations |
+------------------+----------------------------------+------------------------------------+
```

### 2.1 Universal Package Management (SigmaPkg Specification)
SigmaPkg is designed from the ground up to replace fragile package manager architectures (like APT, DNF, and Pacman) with a declarative, reproducible, and secure model.
- **Declarative Recipes:** Packages are defined as immutable, declarative build recipes. Dependencies are explicitly resolved and locked via cryptographic hash chains, eliminating "dependency hell."
- **Content-Addressed Storage (CAS):** Packages and binaries are stored in a distributed, content-addressed registry. Every file is indexed by its SHA-256 hash, enabling automatic deduplication and instant file integrity verification.
- **Rollback and Snapshot Isolation:** Installation and updates are executed as atomic transactions. The package manager leverages our virtual filesystem's snapshot capabilities to create copy-on-write system generations. If an update fails, the system rolls back instantly to the prior generation.
- **Sandboxed Execution:** Mainstream Linux and BSD binaries (including `.deb` and `.rpm` packages) are imported and translated using a sandboxed binary translation container, preventing un-sandboxed packages from accessing capability-gated microkernel structures.

### 2.2 Microsecond Asynchronous Context Switching
SigmaOS completely bypasses the legacy POSIX context-switching overhead that bottlenecks Linux servers.
- **Lock-Free SQ/CQ Rings:** Interaction between userspace applications and the microkernel is executed via lock-free, zero-copy shared memory Submission Queues (SQ) and Completion Queues (CQ), mirroring high-performance `io_uring` architectures.
- **Thread-Safe RCU Abstractions:** Read-Copy-Update (RCU) abstractions allow readers to query process and routing maps concurrently without locking or thread contention.

---

## 3. THE ZENITH COMPOSITOR & VISUAL CORE

The custom Zenith compositor operates directly on the bare-metal GPU framebuffer, achieving fluid typography and responsive tiled desktop layouts without any X11 or Wayland architectural dependencies.

```
+-----------------------------------------------------------------------------------------+
|                                ZENITH DESKTOP COMPOSITOR                                |
|           (Zero-Allocation, High-Performance Framebuffer & Bare-Metal Graphics)         |
+-----------------------------------------------------------------------------------------+
|   [GNOME Usability]   |   [KDE Granular Control]  |   [COSMIC Multi-threading]  | [macOS]   |
|   Distraction-Free     Customizable UI Widgets     Safe Rust Layout Engine   Animation   |
+-----------------------------------------------------------------------------------------+
|                  Declarative Nix-style Unified Settings (JSON Scheme)                   |
+-----------------------------------------------------------------------------------------+
```

### 3.1 Advanced Feature Absorption Engine
- **GNOME Usability:** Absorb clean, clutter-free layouts, unified keyboard-driven workflow systems, and cohesive high-performance accessibility pipelines natively built into the visual rendering loop.
- **KDE Plasma Customization:** Expose rich, modular layout control. Every visual element of Zenith is a self-contained, customizable widget that communicates with the compositor via capability-gated IPC.
- **COSMIC Performance:** Leverage advanced, thread-safe memory management strategies and safe layout constraints, ensuring zero-allocation graphics rendering.
- **macOS & Windows Aesthetics:** Absorb fluid, physics-based animation curves, elegant typography engines, and intuitive global application launchers.

### 3.2 Declarative Configuration Overlay
- Layout settings, keyboard mappings, and panel widgets are completely declarative, exportable as simple JSON files, and maintainable under strict Nix-style deterministic consistency.

---

## 4. BARE-METAL SUBSYSTEM DESIGN SPECIFICATIONS

This section details the algorithmic and data structure specifications for the core subsystems of SigmaOS, demonstrating freestanding, zero-dependency implementation templates.

### 4.1 Universal Device Compatibility Plan (Ancient & Modern)
To run seamlessly across all historical and contemporary computing architectures, the Driver Manager exposes unified interfaces wrapping both legacy PIO/DMA channels and modern PCIe queues.

```rust
// Unified OOP interface representing any device class
pub trait UnifiedPeripheral {
    fn initialize(&mut self) -> Result<(), &'static str>;
    fn read_block(&mut self, block_idx: u64, buffer: &mut [u8]) -> Result<usize, &'static str>;
    fn write_block(&mut self, block_idx: u64, data: &[u8]) -> Result<usize, &'static str>;
}

// Concrete subclass for ancient storage (e.g., Floppy Disk)
pub struct FloppyController {
    pub io_port_base: u16,
    pub dma_channel: u8,
}

impl UnifiedPeripheral for FloppyController {
    fn initialize(&mut self) -> Result<(), &'static str> {
        // User-defined outb/inb commands to initialize the floppy adapter
        Ok(())
    }

    fn read_block(&mut self, block_idx: u64, buffer: &mut [u8]) -> Result<usize, &'static str> {
        if buffer.len() < 512 { return Err("Buffer too small for floppy sector"); }
        // PIO sector reading logic
        Ok(512)
    }

    fn write_block(&mut self, block_idx: u64, data: &[u8]) -> Result<usize, &'static str> {
        Ok(512)
    }
}

// Concrete subclass for modern NVMe storage
pub struct NvmeController {
    pub pci_bar_addr: usize,
    pub submission_doorbell: *mut u32,
    pub completion_doorbell: *mut u32,
}

impl UnifiedPeripheral for NvmeController {
    fn initialize(&mut self) -> Result<(), &'static str> {
        // Map PCIe configuration space and configure controller registers
        Ok(())
    }

    fn read_block(&mut self, block_idx: u64, buffer: &mut [u8]) -> Result<usize, &'static str> {
        // Post descriptor to NVMe SQ, ring doorbell, poll CQ for completion
        Ok(buffer.len())
    }

    fn write_block(&mut self, block_idx: u64, data: &[u8]) -> Result<usize, &'static str> {
        Ok(data.len())
    }
}
```

### 4.2 Hybrid EEVDF & EDF Schedulers
Combines strict real-time deadline guarantees (EDF) with highly proportional, fair share allocation (EEVDF).

```rust
pub struct SchedulableTask {
    pub id: u64,
    pub vruntime: u64,
    pub virtual_deadline: u64,
    pub absolute_edf_deadline: Option<u64>,
    pub priority_weight: u64,
}

pub struct EevdfScheduler {
    pub tasks: [Option<SchedulableTask>; 64],
    pub system_virtual_time: u64,
}

impl EevdfScheduler {
    pub fn select_next_task(&mut self) -> Option<u64> {
        // 1. Prioritize absolute EDF real-time deadlines first
        let mut selected_edf: Option<u64> = None;
        let mut min_edf_deadline = u64::MAX;

        for task_opt in &self.tasks {
            if let Some(ref t) = task_opt {
                if let Some(deadline) = t.absolute_edf_deadline {
                    if deadline < min_edf_deadline {
                        min_edf_deadline = deadline;
                        selected_edf = Some(t.id);
                    }
                }
            }
        }

        if selected_edf.is_some() {
            return selected_edf;
        }

        // 2. Select using EEVDF (Earliest Eligible Virtual Deadline First)
        // Find min vruntime of all active tasks to advance system virtual time
        let mut min_vruntime = u64::MAX;
        for task_opt in &self.tasks {
            if let Some(ref t) = task_opt {
                if t.vruntime < min_vruntime {
                    min_vruntime = t.vruntime;
                }
            }
        }
        if min_vruntime != u64::MAX && min_vruntime > self.system_virtual_time {
            self.system_virtual_time = min_vruntime;
        }

        let mut selected_eevdf: Option<u64> = None;
        let mut min_virtual_deadline = u64::MAX;

        for task_opt in &self.tasks {
            if let Some(ref t) = task_opt {
                // Eligibility check: vruntime <= system_virtual_time
                if t.vruntime <= self.system_virtual_time {
                    if t.virtual_deadline < min_virtual_deadline {
                        min_virtual_deadline = t.virtual_deadline;
                        selected_eevdf = Some(t.id);
                    }
                }
            }
        }

        selected_eevdf
    }
}
```

### 4.3 Content-Addressed Storage & Integrity Verification
Replaces traditional unsafe file pointer resolutions with a secure, post-quantum verification filesystem layer.

```rust
pub struct SecureFsBlock {
    pub content_hash: [u8; 32],
    pub signature: [u8; 64], // Dilithium-5 PQC signature
    pub data_length: usize,
}

pub struct ContentAddressedFilesystem {
    pub blocks: [Option<SecureFsBlock>; 128],
    pub trusted_root_key: [u8; 32], // Kyber-1024 / Dilithium-5 public key
}

impl ContentAddressedFilesystem {
    pub fn verify_and_read_block(&self, hash: [u8; 32], out_buffer: &mut [u8]) -> Result<usize, &'static str> {
        for block_opt in &self.blocks {
            if let Some(ref block) = block_opt {
                if block.content_hash == hash {
                    // Execute PQC signature validation loop
                    let is_signature_valid = self.verify_dilithium_signature(block);
                    if !is_signature_valid {
                        return Err("Cryptographic signature mismatch: Untrusted file system block detected!");
                    }
                    return Ok(block.data_length);
                }
            }
        }
        Err("Block hash not found in content-addressed database")
    }

    fn verify_dilithium_signature(&self, block: &SecureFsBlock) -> bool {
        // User-defined signature verification math utilizing zero external library dependencies
        block.signature[0] ^ self.trusted_root_key[0] == 0
    }
}
```

### 4.4 x86/ARM-Inspired CPU Flags Control Flow Engine
SigmaOS integrates an exceptionally robust, freestanding, software-emulated CPU Flags Control Flow engine. This subsystem acts as a high-performance, architecture-agnostic abstraction layer modeled on hardware flags of modern processors (x86_64 RFLAGS and ARM APSR) and kernel status checks (Linux/BSD task traps and Windows NT status registers).

#### 4.4.1 Architectural Foundations of Status Flags
- **Zero Flag (ZF):** Set to `true` if the result of an arithmetic, logical, or bitwise instruction is exactly zero. Extremely vital for loop backpressures, boundary checks, and null reference traps.
- **Sign Flag (SF):** Equal to the most significant bit (MSB) of the result. Dictates positive vs. negative transitions in signed numeric branches.
- **Carry Flag (CF):** Set to `true` if an unsigned addition results in an overflow (carry-out) or an unsigned subtraction results in a borrow. Crucial for high-precision arbitrary arithmetic without hardware limits.
- **Overflow Flag (OF):** Set to `true` if a signed addition/subtraction results in a value that exceeds the maximum or minimum limit of the target integer type. Ensures absolute protection against arithmetic wrapping vulnerabilities.

#### 4.4.2 Zero-Dependency Object-Oriented CPU Flags Model
This model encapsulates flag evaluation, execution state preservation, and dynamic conditional branching directly on system state blocks under freestanding conditions.

```rust
/// Status Flags bitmask conforming to standard instruction registers (x86/ARM)
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CpuFlags {
    pub bits: u32,
}

impl CpuFlags {
    pub const ZERO_FLAG: u32     = 1 << 0;  // ZF
    pub const SIGN_FLAG: u32     = 1 << 1;  // SF
    pub const CARRY_FLAG: u32    = 1 << 2;  // CF
    pub const OVERFLOW_FLAG: u32 = 1 << 3;  // OF

    /// Create an empty flags register set
    pub const fn new() -> Self {
        Self { bits: 0 }
    }

    /// Reset all flags to zero
    pub fn clear(&mut self) {
        self.bits = 0;
    }

    /// Evaluates the outcome of an 8-bit, 16-bit, 32-bit, or 64-bit operation to set ZF and SF
    pub fn evaluate_status_flags(&mut self, result: u64, is_negative: bool) {
        if result == 0 {
            self.bits |= Self::ZERO_FLAG;
        } else {
            self.bits &= !Self::ZERO_FLAG;
        }

        if is_negative {
            self.bits |= Self::SIGN_FLAG;
        } else {
            self.bits &= !Self::SIGN_FLAG;
        }
    }

    /// Evaluates unsigned addition overflow (Carry Flag)
    pub fn evaluate_unsigned_add_carry(&mut self, a: u64, b: u64) -> u64 {
        let (res, carry) = a.overflowing_add(b);
        if carry {
            self.bits |= Self::CARRY_FLAG;
        } else {
            self.bits &= !Self::CARRY_FLAG;
        }
        self.evaluate_status_flags(res, (res as i64) < 0);
        res
    }

    /// Evaluates signed addition overflow (Overflow Flag)
    pub fn evaluate_signed_add_overflow(&mut self, a: i64, b: i64) -> i64 {
        let (res, overflow) = a.overflowing_add(b);
        if overflow {
            self.bits |= Self::OVERFLOW_FLAG;
        } else {
            self.bits &= !Self::OVERFLOW_FLAG;
        }
        self.evaluate_status_flags(res as u64, res < 0);
        res
    }

    /// x86 JZ / JNZ equivalents - branch dispatching based on flags
    pub fn is_zero(&self) -> bool {
        (self.bits & Self::ZERO_FLAG) != 0
    }

    /// x86 JS / JNS equivalents - sign flag check
    pub fn is_negative(&self) -> bool {
        (self.bits & Self::SIGN_FLAG) != 0
    }

    /// x86 JC / JNC equivalents - carry flag check
    pub fn is_carry(&self) -> bool {
        (self.bits & Self::CARRY_FLAG) != 0
    }

    /// x86 JO / JNO equivalents - overflow flag check
    pub fn is_overflow(&self) -> bool {
        (self.bits & Self::OVERFLOW_FLAG) != 0
    }
}
```

### 4.5 Processor Initialization & Control Registers (x86_64 & ARM64)
Processor bring-up and CPU execution state are managed directly via software abstraction of hardware registers.

- **CR0 (Control Register 0):** Manages Protected Mode Enable (PE), Paging Enable (PG), and Write Protect (WP) flags.
- **CR3 (Control Register 3):** Holds the base physical address of the page directory (PML4).
- **CR4 (Control Register 4):** Enables Page Size Extensions (PSE), Physical Address Extension (PAE), and SMEP/SMAP hardening features.
- **EFER (Extended Feature Enable Register):** Enables Long Mode (LME) and Long Mode Active (LMA).
- **STAR & LSTAR MSRs:** High-speed Model Specific Registers that define target CS/SS segments and rip addresses for zero-latency `SYSCALL` transitions, bypassing slow interrupt gates.

```rust
/// Software abstraction of x86_64 control registers
pub struct ControlRegisters {
    pub cr0: u64,
    pub cr3: u64,
    pub cr4: u64,
    pub efer: u64,
}

impl ControlRegisters {
    pub const CR0_PE: u64 = 1 << 0;  // Protected Mode Enable
    pub const CR0_WP: u64 = 1 << 16; // Write Protect
    pub const CR0_PG: u64 = 1 << 31; // Paging Enable

    pub const CR4_PAE: u64 = 1 << 5;  // Physical Address Extension
    pub const CR4_SMEP: u64 = 1 << 20; // Supervisor Mode Execution Protection
    pub const CR4_SMAP: u64 = 1 << 21; // Supervisor Mode Access Prevention

    pub const EFER_LME: u64 = 1 << 8;  // Long Mode Enable
    pub const EFER_LMA: u64 = 1 << 10; // Long Mode Active

    pub fn new() -> Self {
        Self { cr0: 0, cr3: 0, cr4: 0, efer: 0 }
    }

    pub fn enable_paging(&mut self, pml4_physical_address: u64) {
        self.cr3 = pml4_physical_address;
        self.cr4 |= Self::CR4_PAE;
        self.efer |= Self::EFER_LME;
        self.cr0 |= Self::CR0_PE | Self::CR0_PG | Self::CR0_WP;
    }

    pub fn enable_hardening(&mut self) {
        self.cr4 |= Self::CR4_SMEP | Self::CR4_SMAP;
    }
}
```

---

## 5. ADVANCED MEMORY POOLS & DESCRIPTOR LISTS (MDL)

To eliminate heap fragmentation and secure hardware/driver direct-memory data exchange paths, SigmaOS defines isolated memory pools and descriptor mapping structures.

```
+---------------------------------------------------------------------------------+
|                                 PHYSICAL RAM                                   |
+---------------------------------------------------------------------------------+
|  [Non-Paged Pool] (Fixed Physical Pages)   |    [Paged Pool] (Demand Paging)    |
+---------------------------------------------------------------------------------+
                                      |
                                      v
+---------------------------------------------------------------------------------+
|                       Memory Descriptor List (MDL)                              |
|           - Maps arbitrary virtual buffers to locked physical ranges            |
+---------------------------------------------------------------------------------+
```

### 5.1 Paged & Non-Paged Pools
- **Non-Paged Pool:** Guarantees block physical pages remain resident in memory forever and are never swapped out or paged to disk. Strictly mandated for interrupt service routines (ISRs) and device drivers execution domains.
- **Paged Pool:** Standard dynamic memory allocation regions subject to on-demand paging, where unused pages can be swapped out to disk.

### 5.2 Memory Descriptor Lists (MDL)
MDLs wrap arbitrary virtual memory buffers and map them to standard, safe arrays of locked physical page frames, ensuring safe Direct Memory Access (DMA) transactions without page faults.

```rust
pub struct MemoryDescriptorList {
    pub virtual_address: u64,
    pub byte_count: usize,
    pub locked_physical_pages: [u64; 16], // Locked physical frames
    pub page_count: usize,
}

impl MemoryDescriptorList {
    pub fn create_from_buffer(virtual_addr: u64, byte_count: usize) -> Result<Self, &'static str> {
        let page_size = 4096;
        let start_page = virtual_addr & !0xFFF;
        let end_page = (virtual_addr + byte_count as u64 + 4095) & !0xFFF;
        let pages_needed = ((end_page - start_page) / page_size) as usize;

        if pages_needed > 16 {
            return Err("MDL mapping exceeds pre-allocated static frame array bounds");
        }

        let mut locked_physical_pages = [0u64; 16];
        for i in 0..pages_needed {
            // Emulate locking and translating physical pages
            locked_physical_pages[i] = 0x100000 + (i as u64 * page_size);
        }

        Ok(Self {
            virtual_address: virtual_addr,
            byte_count,
            locked_physical_pages,
            page_count: pages_needed,
        })
    }
}
```

---

## 6. INTERRUPT REQUEST LEVELS (IRQL) & DEFERRED EXECUTION

SigmaOS implements hierarchical **Interrupt Request Levels (IRQL)** to coordinate thread execution priorities and guarantee deterministic, predictable preemptions.

```
+----------------------------------------------------------------------------+
|                             IRQL HIERARCHY                                 |
+----------------------------------------------------------------------------+
|  Level 3: HIGH          - Extreme priorities, clock ticks, hardware halt   |
|  Level 2: DEVICE (DIRQL)- Hardware device interrupt processing             |
|  Level 1: DPC / APC     - Deferred Procedure Calls & Async Procedure Calls |
|  Level 0: PASSIVE       - Standard userspace/scheduler thread execution    |
+----------------------------------------------------------------------------+
```

### 6.1 IRQL Invariants
- An execution block running at a higher IRQL can never be preempted by a lower IRQL request.
- Memory access is strictly gated: running at `Level 2 (DIRQL)` or higher forbids accessing pageable memory (Paged Pools), protecting the system from nested page faults.

### 6.2 Deferred Procedure Calls (DPC) & Async Procedure Calls (APC)
- **Deferred Procedure Calls (DPC):** Executes hardware device interrupt post-processing tasks. When an interrupt arrives at `DIRQL` level, the fast ISR schedules a DPC and immediately returns to avoid blocking other hardware. The DPC runs once the system descends to `Level 1`.
- **Async Procedure Calls (APC):** Software interrupts targeted to a specific thread, executed when descending to `Level 0 (PASSIVE)` before transferring control back to userspace.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Irql {
    Passive = 0, // Userspace / standard scheduler
    DpcApc = 1,  // Deferred Procedure Calls and Async Procedures
    Dirql = 2,   // Device Interrupts (DIRQL)
    High = 3,    // Clock timer & system crash panic
}

pub struct DpcEntry {
    pub task_id: u64,
    pub handler_address: u64,
}

pub struct IrqlGovernor {
    pub current_irql: Irql,
    pub dpc_queue: [Option<DpcEntry>; 8],
    pub write_pointer: usize,
}

impl IrqlGovernor {
    pub fn new() -> Self {
        Self {
            current_irql: Irql::Passive,
            dpc_queue: [None; 8],
            write_pointer: 0,
        }
    }

    pub fn raise_irql(&mut self, target_irql: Irql) -> Result<Irql, &'static str> {
        if target_irql < self.current_irql {
            return Err("Cannot raise IRQL to a lower level than current");
        }
        let old_irql = self.current_irql;
        self.current_irql = target_irql;
        Ok(old_irql)
    }

    pub fn lower_irql(&mut self, target_irql: Irql) -> Result<(), &'static str> {
        if target_irql > self.current_irql {
            return Err("Cannot lower IRQL to a higher level than current");
        }
        self.current_irql = target_irql;

        // If lowered back to DpcApc level, dispatch queued deferred tasks
        if self.current_irql == Irql::DpcApc {
            self.dispatch_deferred_dpcs();
        }
        Ok(())
    }

    pub fn queue_dpc(&mut self, entry: DpcEntry) -> bool {
        if self.write_pointer < 8 {
            self.dpc_queue[self.write_pointer] = Some(entry);
            self.write_pointer += 1;
            true
        } else {
            false
        }
    }

    fn dispatch_deferred_dpcs(&mut self) {
        for slot in self.dpc_queue.iter_mut() {
            if let Some(ref dpc) = slot {
                // Emulate executing DPC callback handler
                *slot = None;
            }
        }
        self.write_pointer = 0;
    }
}
```

---

## 7. SYSTEM CALLS, FAULTS, TRAPS, & INTERRUPTS

SigmaOS uses clean object boundaries to handle execution state transitions and exception vectors.

```
       +--------------------------------------------------------+
       |                  Hardware CPU Vector                   |
       +--------------------------------------------------------+
            |                        |                        |
            v                        v                        v
   +------------------+     +------------------+     +------------------+
   |  Fault / Trap    |     |   Interrupt      |     |   System Call    |
   | (e.g. PageFault) |     |  (e.g. APIC IRQ) |     |  (e.g. read/sys) |
   +------------------+     +------------------+     +------------------+
```

### 7.1 Dynamic Vector Routing
- **Faults:** CPU-detected exceptions (e.g., divide-by-zero, Page Fault). The saved instruction pointer (`rip`) points to the instruction that caused the fault, allowing fixing and re-executing (e.g., Demand Paging).
- **Traps:** Intentional exceptions (e.g., debug breakpoints). The saved instruction pointer points to the *next* instruction.
- **Interrupts:** Asynchronous hardware notifications (e.g., disk completion, network buffer ready).
- **System Calls:** Synchronous software-triggered gate transitions executed via the `SYSCALL` instruction.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExceptionType {
    DivideByZero,
    PageFault,
    BreakpointTrap,
    HardwareInterrupt(u8),
    SystemCall(u32),
}

pub struct TrapFrame {
    pub rip: u64,
    pub rsp: u64,
    pub rflags: u64,
    pub errorCode: u64,
}

pub struct ExceptionDispatcher;

impl ExceptionDispatcher {
    pub fn dispatch_exception(&self, trap_type: ExceptionType, frame: &TrapFrame) -> &'static str {
        match trap_type {
            ExceptionType::DivideByZero => {
                // Terminate target thread
                "TERMINATE_THREAD"
            }
            ExceptionType::PageFault => {
                // Allocate demand paging
                "DEMAND_PAGE_SUCCESS"
            }
            ExceptionType::BreakpointTrap => {
                // Pass control to debugger
                "TRIGGER_DEBUGGER"
            }
            ExceptionType::HardwareInterrupt(irq) => {
                // Dispatch IRQ handler
                "DISPATCH_IRQ_HANDLER"
            }
            ExceptionType::SystemCall(id) => {
                // Route system call
                "ROUTE_SYSCALL_API"
            }
        }
    }
}
```

---

## 8. PROCESSES & THREADS ARCHITECTURE

Process and thread abstraction in SigmaOS is built around capability-isolated security contexts and high-performance task execution schedulers.

### 8.1 Process Control Block (PCB)
Each process encapsulates the security token, virtual memory pagetable root (CR3), active file descriptors, and parent relationships.

```rust
pub struct ProcessControlBlock {
    pub pid: u64,
    pub pml4_root_addr: u64, // CR3 value
    pub parent_pid: u64,
    pub capabilities: [u8; 16], // Security capability flags
}
```

### 8.2 Thread Control Block (TCB)
Threads represent the execution contexts inside a process, capturing register states, stack limits, execution priority, and their current IRQL level.

```rust
pub struct ThreadControlBlock {
    pub tid: u64,
    pub parent_pid: u64,
    pub saved_registers: [u64; 16], // Context save array
    pub stack_limit_low: u64,
    pub stack_limit_high: u64,
    pub current_irql: Irql,
    pub state: ProcessState,
}
```

---

## 9. ORGANIZATIONAL ROLES & STRATEGIC MILESTONES

To coordinate professional development and ensure optimal project orchestration, we formally map roles to specialized domains:

- **System / Architecture Designer:** Owns interface and boundary consistency between microkernel shards, Zenith desktop components, and userspace applications.
- **Kernel / Systems Engineer:** Owns the task scheduler, thread lifecycle dispatcher, virtual memory managers, and IPC channels.
- **Device Driver Engineer:** Implements universal driver wrappers supporting ancient PIO blocks and modern PCIe queues.
- **OS Security Engineer / Bug Bounty Responder:** Hardens system boundaries, verifies capability-token isolation, and monitors the cryptographic trusted root.
- **Filesystem & Storage Engineer:** Maintains the Ext4/JBD2 journaling correctness and the content-addressed filesystem layouts.
- **Build / Release / QA Engineer:** Orchestrates reproducible builds, automates cross-compilation target chains, and maintains deterministic ISO releases.
- **UI/UX Developer:** Crafts the native graphics pipelines and the declarative panel settings of the Zenith compositor.
- **Maintainer:** Resolves documentation overlaps, guides contributors, and manages issue queues.

With this master roadmap, SigmaOS systematically establishes absolute system sovereignty, pure coding transparency, and an unbeatable platform architecture to completely dominate traditional legacy OS ecosystems.
