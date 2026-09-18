# Phase 2 Gap Closure Implementation Plan

This document outlines the Phase 2 implementation plan for closing core feature gaps between SigmaOS and mature Linux/BSD distributions. Phase 2 focuses on dynamic kernel capabilities and advanced memory management.

## Overview

Phase 2 (Months 4-6) addresses core kernel features that enable dynamic system configuration, advanced memory management, and modern I/O capabilities. These features build on the foundation established in Phase 1.

## Timeline

- **Duration**: 3 months
- **Priority**: High (core features)
- **Dependencies**: Phase 1 completion
- **Testing**: Each feature requires standalone tests

## 1. Dynamic Kernel Module Loading

### Current State
Dynamic ELF module linking and relocation at kernel level is simulated rather than true kernel-space ELF dynamic symbol resolution.

### Implementation Plan

#### 1.1 ELF Loader
```rust
// src/kernel/module/elf_loader.rs
pub struct ElfLoader {
    file_data: Vec<u8>,
    symbols: HashMap<String, usize>,
}

impl ElfLoader {
    pub fn load_module(&mut self, data: &[u8]) -> Result<LoadedModule, LoadError> {
        let elf = Elf::parse(data)?;
        
        // Verify ELF header
        if elf.header.e_type != ET_REL {
            return Err(LoadError::NotRelocatable);
        }
        
        // Allocate kernel memory for module
        let base_addr = self.allocate_kernel_memory(elf.header.e_size as usize)?;
        
        // Load sections
        for section in &elf.section_headers {
            if section.sh_type == SHT_PROGBITS && section.sh_flags & SHF_ALLOC != 0 {
                self.load_section(&elf, section, base_addr)?;
            }
        }
        
        // Apply relocations
        self.apply_relocations(&elf, base_addr)?;
        
        // Export symbols
        self.export_symbols(&elf, base_addr)?;
        
        Ok(LoadedModule { base_addr, size: elf.header.e_size as usize })
    }
}
```

#### 1.2 Symbol Resolution
```rust
// src/kernel/module/symbol.rs
pub struct SymbolTable {
    kernel_symbols: HashMap<String, KernelSymbol>,
    module_symbols: HashMap<String, ModuleSymbol>,
}

impl SymbolTable {
    pub fn resolve(&self, name: &str) -> Option<usize> {
        // Check kernel symbols first
        if let Some(sym) = self.kernel_symbols.get(name) {
            return Some(sym.address);
        }
        
        // Check module symbols
        if let Some(sym) = self.module_symbols.get(name) {
            return Some(sym.address);
        }
        
        None
    }
    
    pub fn export(&mut self, name: String, address: usize, is_gpl: bool) {
        self.kernel_symbols.insert(name, KernelSymbol { address, is_gpl });
    }
}
```

#### 1.3 Module Management
```rust
// src/kernel/module/mod.rs
pub struct ModuleManager {
    loaded_modules: Vec<LoadedModule>,
    symbol_table: SymbolTable,
}

impl ModuleManager {
    pub fn insmod(&mut self, path: &str) -> Result<(), ModuleError> {
        let data = read_file(path)?;
        let mut loader = ElfLoader::new(data);
        let module = loader.load_module()?;
        
        // Initialize module
        self.init_module(&module)?;
        
        self.loaded_modules.push(module);
        Ok(())
    }
    
    pub fn rmmod(&mut self, name: &str) -> Result<(), ModuleError> {
        let module = self.find_module(name)?;
        
        // Check if module is in use
        if module.ref_count > 0 {
            return Err(ModuleError::InUse);
        }
        
        // Cleanup module
        self.cleanup_module(&module)?;
        
        // Remove from list
        self.loaded_modules.retain(|m| m.name != name);
        
        Ok(())
    }
}
```

### Testing Strategy
- ELF loading unit tests
- Symbol resolution tests
- Module dependency resolution tests
- Unload safety tests

### Documentation
- [Kernel Module Development](Kernel-Module-Development) (to be created)
- [ELF Format](ELF-Format) (to be created)
- [DKMS Integration](DKMS-Integration) (to be created)

---

## 2. Interrupt Balancing and MSI-X Support

### Current State
Lacks production IRQ balancing across multi-socket NUMA topologies and real MSI-X hardware vector steering.

### Implementation Plan

#### 2.1 IRQ Balancing
```rust
// src/kernel/irq/balance.rs
pub struct IrqBalancer {
    cpu_irqs: Vec<Vec<Irq>>,
    affinity_hint: HashMap<usize, usize>,
}

impl IrqBalancer {
    pub fn balance_irqs(&mut self) {
        for irq in &self.affinity_hint {
            let target_cpu = self.calculate_optimal_cpu(irq.0);
            self.set_irq_affinity(irq.0, target_cpu);
        }
    }
    
    fn calculate_optimal_cpu(&self, irq: usize) -> usize {
        // Calculate based on:
        // - Current CPU load
        // - NUMA locality
        // - Cache affinity
        // - IRQ characteristics
        
        let least_loaded = self.find_least_loaded_cpu();
        let numa_local = self.find_numa_local_cpu(irq);
        
        // Prefer NUMA local, then least loaded
        if numa_local.is_some() {
            numa_local.unwrap()
        } else {
            least_loaded
        }
    }
}
```

#### 2.2 MSI-X Vector Allocation
```rust
// src/kernel/irq/msix.rs
pub struct MsixController {
    vectors: Vec<MsixVector>,
    free_vectors: Vec<u16>,
}

impl MsixController {
    pub fn allocate_vector(&mut self, cpu: usize) -> Result<u16, MsixError> {
        if let Some(vector) = self.free_vectors.pop() {
            self.vectors[vector as usize].cpu = cpu;
            self.vectors[vector as usize].enabled = true;
            
            // Configure vector in hardware
            self.configure_msix_vector(vector, cpu)?;
            
            Ok(vector)
        } else {
            Err(MsixError::NoVectorsAvailable)
        }
    }
    
    pub fn configure_msix_vector(&self, vector: u16, cpu: usize) -> Result<(), MsixError> {
        // Write to MSI-X table
        let table_entry = self.msix_table_entry(vector);
        
        table_entry.address = calculate_msix_address(cpu);
        table_entry.data = calculate_msix_data(vector);
        table_entry.vector_control = 0; // Enable
        
        Ok(())
    }
}
```

#### 2.3 NUMA Topology Awareness
```rust
// src/kernel/numa/topology.rs
pub struct NumaTopology {
    nodes: Vec<NumaNode>,
    distances: Vec<Vec<u32>>,
}

impl NumaTopology {
    pub fn get_local_node(&self, cpu: usize) -> usize {
        self.nodes.iter().find(|n| n.cpus.contains(&cpu)).map(|n| n.id).unwrap_or(0)
    }
    
    pub fn calculate_distance(&self, node_a: usize, node_b: usize) -> u32 {
        self.distances[node_a][node_b]
    }
}
```

### Testing Strategy
- IRQ balancing unit tests
- MSI-X allocation tests
- NUMA topology simulation
- Performance benchmarks

### Documentation
- [Interrupt Handling](Interrupt-Handling) (to be created)
- [MSI-X Configuration](MSI-X-Configuration) (to be created)
- [NUMA Programming](NUMA-Programming) (to be created)

---

## 3. Cgroups v2 Memory Controller

### Current State
Cgroups v2 limits are simulated but lack kernel page allocation hooks that freeze processes under physical RAM exhaustion.

### Implementation Plan

#### 3.1 Cgroup Memory Controller
```rust
// src/kernel/cgroup/memory.rs
pub struct MemoryController {
    cgroups: HashMap<String, MemoryCgroup>,
}

impl MemoryController {
    pub fn set_limit(&mut self, cgroup: &str, limit: usize) {
        if let Some(cg) = self.cgroups.get_mut(cgroup) {
            cg.memory_limit = limit;
        }
    }
    
    pub fn check_limit(&self, task: &Task) -> bool {
        let cgroup = self.get_task_cgroup(task);
        
        if cgroup.memory_usage > cgroup.memory_limit {
            // Trigger OOM handler
            self.handle_oom(task, cgroup);
            return false;
        }
        
        true
    }
    
    fn handle_oom(&self, task: &Task, cgroup: &MemoryCgroup) {
        if cgroup.oom_group {
            // Kill all processes in cgroup
            self.kill_cgroup(cgroup);
        } else {
            // Kill offending process
            self.kill_process(task);
        }
    }
}
```

#### 3.2 Memory Pressure Stall Information (PSI)
```rust
// src/kernel/cgroup/psi.rs
pub struct PsiMonitor {
    stats: HashMap<String, PsiStats>,
}

impl PsiMonitor {
    pub fn record_stall(&mut self, cgroup: &str, stall_type: StallType, duration: Duration) {
        let stats = self.stats.entry(cgroup.to_string()).or_default();
        
        match stall_type {
            StallType::Memory => stats.memory_stall_duration += duration,
            StallType::Io => stats.io_stall_duration += duration,
            StallType::Cpu => stats.cpu_stall_duration += duration,
        }
    }
    
    pub fn get_pressure(&self, cgroup: &str) -> Option<PressureValue> {
        self.stats.get(cgroup).map(|stats| {
            let total = stats.memory_stall_duration + stats.io_stall_duration + stats.cpu_stall_duration;
            PressureValue::from_duration(total)
        })
    }
}
```

#### 3.3 Page Allocation Hooks
```rust
// src/memory/cgroup_hooks.rs
pub fn on_page_alloc(task: &Task, size: usize) -> Result<(), AllocError> {
    let cgroup = get_task_cgroup(task);
    
    // Check if allocation would exceed limit
    if cgroup.memory_usage + size > cgroup.memory_limit {
        // Try to reclaim memory
        reclaim_memory(cgroup, size)?;
        
        // Still over limit?
        if cgroup.memory_usage + size > cgroup.memory_limit {
            return Err(AllocError::MemoryLimitExceeded);
        }
    }
    
    // Update usage
    cgroup.memory_usage += size;
    
    Ok(())
}
```

### Testing Strategy
- Memory limit enforcement tests
- OOM killer tests
- PSI accuracy tests
- Multi-cgroup isolation tests

### Documentation
- [Cgroups v2 Guide](Cgroups-v2-Guide) (to be created)
- [Memory Controller](Memory-Controller) (to be created)
- [PSI Monitoring](PSI-Monitoring) (to be created)

---

## 4. Linux io_uring Implementation

### Current State
Implemented as Rust struct memory but lacks true kernel-level ring-buffer shared memory mapping.

### Implementation Plan

#### 4.1 io_uring Submission Queue
```rust
// src/kernel/io_uring/sq.rs
pub struct SubmissionQueue {
    ring_buffer: SharedMemoryRegion,
    head: AtomicU32,
    tail: AtomicU32,
    entries: Vec<Sqe>,
}

impl SubmissionQueue {
    pub fn new(entries: u32) -> Result<Self, IoUringError> {
        let size = entries as usize * mem::size_of::<Sqe>();
        let ring_buffer = SharedMemoryRegion::allocate(size)?;
        
        Ok(SubmissionQueue {
            ring_buffer,
            head: AtomicU32::new(0),
            tail: AtomicU32::new(0),
            entries: vec![Sqe::default(); entries as usize],
        })
    }
    
    pub fn submit(&self, sqe: &Sqe) -> Result<(), IoUringError> {
        let tail = self.tail.load(Ordering::Acquire);
        let head = self.head.load(Ordering::Acquire);
        
        if tail.wrapping_add(1) == head {
            return Err(IoUringError::QueueFull);
        }
        
        let index = (tail % self.entries.len() as u32) as usize;
        self.entries[index] = *sqe;
        
        self.tail.store(tail.wrapping_add(1), Ordering::Release);
        
        Ok(())
    }
}
```

#### 4.2 io_uring Completion Queue
```rust
// src/kernel/io_uring/cq.rs
pub struct CompletionQueue {
    ring_buffer: SharedMemoryRegion,
    head: AtomicU32,
    tail: AtomicU32,
    entries: Vec<Cqe>,
}

impl CompletionQueue {
    pub fn complete(&self, cqe: &Cqe) {
        let tail = self.tail.load(Ordering::Acquire);
        let head = self.head.load(Ordering::Acquire);
        
        let index = (tail % self.entries.len() as u32) as usize;
        self.entries[index] = *cqe;
        
        self.tail.store(tail.wrapping_add(1), Ordering::Release);
    }
    
    pub fn peek(&self) -> Option<Cqe> {
        let head = self.head.load(Ordering::Acquire);
        let tail = self.tail.load(Ordering::Acquire);
        
        if head == tail {
            return None;
        }
        
        let index = (head % self.entries.len() as u32) as usize;
        Some(self.entries[index])
    }
}
```

#### 4.3 Shared Memory Mapping
```rust
// src/kernel/io_uring/mmap.rs
pub fn map_io_uring_rings(task: &Task, sq: &SubmissionQueue, cq: &CompletionQueue) -> Result<Mapping, IoUringError> {
    // Map submission queue into userspace
    let sq_mapping = task.mmap(
        sq.ring_buffer.physical_address(),
        sq.ring_buffer.size(),
        ProtFlags::READ | ProtFlags::WRITE,
        MapFlags::SHARED,
    )?;
    
    // Map completion queue into userspace
    let cq_mapping = task.mmap(
        cq.ring_buffer.physical_address(),
        cq.ring_buffer.size(),
        ProtFlags::READ | ProtFlags::WRITE,
        MapFlags::SHARED,
    )?;
    
    Ok(Mapping { sq: sq_mapping, cq: cq_mapping })
}
```

#### 4.4 SQPOLL Support
```rust
// src/kernel/io_uring/sqpoll.rs
pub struct SqpollThread {
    sq: Arc<SubmissionQueue>,
    running: AtomicBool,
}

impl SqpollThread {
    pub fn new(sq: Arc<SubmissionQueue>) -> Self {
        SqpollThread {
            sq,
            running: AtomicBool::new(true),
        }
    }
    
    pub fn run(&self) {
        while self.running.load(Ordering::Acquire) {
            // Check for new submissions
            let head = self.sq.head.load(Ordering::Acquire);
            let tail = self.sq.tail.load(Ordering::Acquire);
            
            if head != tail {
                // Process submissions
                for i in head..tail {
                    let sqe = self.sq.entries[i as usize];
                    self.process_sqe(&sqe);
                }
                
                self.sq.head.store(tail, Ordering::Release);
            }
            
            // Sleep or yield
            thread::yield_now();
        }
    }
}
```

### Testing Strategy
- io_uring submission/completion tests
- Shared memory mapping tests
- SQPOLL functionality tests
- Performance benchmarks vs syscalls

### Documentation
- [io_uring Programming](io_uring-Programming) (to be created)
- [Async I/O Guide](Async-I-O-Guide) (to be created)
- [Zero-Copy Networking](Zero-Copy-Networking) (to be created)

---

## 5. Capsicum Capability Mode with VFS Enforcement

### Current State
Capsicum rights lack VFS kernel-gate enforcement to block non-capability syscalls.

### Implementation Plan

#### 5.1 Capability Mode Entry
```rust
// src/security/capsicum.rs
pub fn cap_enter() -> Result<(), CapsicumError> {
    let task = current_task();
    
    // Enter capability mode
    task.set_capability_mode(true);
    
    // Hide global VFS namespace
    task.hide_global_namespace();
    
    // Restrict to capability-based access only
    task.set_vfs_gate(VfsGate::CapabilityOnly);
    
    Ok(())
}
```

#### 5.2 VFS Gate Enforcement
```rust
// src/vfs/gate.rs
pub enum VfsGate {
    CapabilityOnly,
    Standard,
}

pub fn vfs_gate_check(operation: VfsOperation, path: Option<&str>) -> Result<(), VfsError> {
    let task = current_task();
    
    match task.vfs_gate() {
        VfsGate::CapabilityOnly => {
            // Must use file descriptor rights
            if operation.requires_path() {
                return Err(VfsError::PathAccessInCapabilityMode);
            }
            
            // Check file descriptor rights
            if let Some(fd) = operation.fd() {
                let rights = task.get_fd_rights(fd);
                if !rights.allows(operation) {
                    return Err(VfsError::InsufficientRights);
                }
            }
            
            Ok(())
        }
        VfsGate::Standard => {
            // Standard path-based access control
            Ok(())
        }
    }
}
```

#### 5.3 Rights Management
```rust
// src/security/capsicum/rights.rs
pub struct CapsicumRights {
    rights: u64,
}

impl CapsicumRights {
    pub fn limit(&mut self, rights: u64) {
        self.rights &= rights;
    }
    
    pub fn allows(&self, operation: VfsOperation) -> bool {
        let required = operation.required_rights();
        (self.rights & required) == required
    }
}

pub fn cap_rights_limit(fd: FileDescriptor, rights: u64) -> Result<(), CapsicumError> {
    let task = current_task();
    task.set_fd_rights(fd, CapsicumRights { rights });
    Ok(())
}
```

### Testing Strategy
- Capability mode entry/exit tests
- VFS gate enforcement tests
- Rights management tests
- Security verification tests

### Documentation
- [Capsicum Guide](Capsicum-Guide) (to be created)
- [Capability Security](Capability-Security) (to be created)
- [VFS Gate](VFS-Gate) (to be created)

---

## Success Criteria

Phase 2 is considered complete when:

1. **Kernel Modules**: Can load/unload ELF kernel modules with symbol resolution
2. **IRQ Balancing**: IRQs are balanced across CPUs with MSI-X vector allocation
3. **Cgroups v2**: Memory limits are enforced with OOM handling and PSI monitoring
4. **io_uring**: Zero-syscall async I/O works with shared memory mapping
5. **Capsicum**: VFS operations are blocked when capability mode is active

## Testing Requirements

Each feature must have:
- Unit tests for core functionality
- Integration tests with real hardware where applicable
- Performance benchmarks
- Security verification tests

## Dependencies

Phase 2 requires:
- Phase 1 completion (PCI enumeration, memory management)
- Shared memory infrastructure
- VFS layer improvements

## Next Steps

After Phase 2 completion:
- Proceed to Phase 3 (Advanced Features)
- Update driver development guides
- Create module signing infrastructure
- Establish performance regression tests

---

**[Phase 1 Implementation Plan](Phase-1-Gap-Closure-Implementation-Plan)** | **[Phase 3 Implementation Plan](Phase-3-Gap-Closure-Implementation-Plan)** | **[Kernel Development](Category-Development)**
