# Phase 1 Gap Closure Implementation Plan

This document outlines the Phase 1 implementation plan for closing critical gaps between SigmaOS and mature Linux/BSD distributions. Phase 1 focuses on foundational hardware and kernel features.

## Overview

Phase 1 (Months 1-3) addresses the most critical gaps that prevent SigmaOS from having real hardware interaction and basic kernel security enforcement. These are foundational features required for any production-grade operating system.

## Timeline

- **Duration**: 3 months
- **Priority**: Critical foundation
- **Dependencies**: None (can start immediately)
- **Testing**: Each feature requires standalone tests

## 1. Real PCI/PCIe Enumeration and Device Discovery

### Current State
SigmaOS relies on simulated bus structures (`PciBusManager`, `SimulatedPciHardwareAccess`) without real hardware device discovery.

### Implementation Plan

#### 1.1 PCI Configuration Space Access
```rust
// src/hardware/pci_config.rs
pub struct PciConfigSpace {
    pub vendor_id: u16,
    pub device_id: u16,
    pub class_code: u8,
    pub subclass: u8,
    pub prog_if: u8,
    pub revision_id: u8,
    pub bars: [PciBar; 6],
}

pub fn read_pci_config(bus: u8, device: u8, function: u8, offset: u8) -> u32 {
    // I/O port 0xCF8 for address, 0xCFC for data
    let address = 0x8000_0000 | ((bus as u32) << 16) | ((device as u32) << 11) | ((function as u32) << 8) | (offset as u32);
    unsafe {
        x86_64::io::outl(0xCF8, address);
        x86_64::io::inl(0xCFC)
    }
}
```

#### 1.2 Device Enumeration Algorithm
```rust
pub fn enumerate_pci_devices() -> Vec<PciDevice> {
    let mut devices = Vec::new();
    
    for bus in 0..256 {
        for device in 0..32 {
            for function in 0..8 {
                let vendor_device = read_pci_config(bus, device, function, 0);
                let vendor_id = (vendor_device & 0xFFFF) as u16;
                
                if vendor_id != 0xFFFF {
                    let pci_device = PciDevice::new(bus, device, function);
                    devices.push(pci_device);
                }
            }
        }
    }
    
    devices
}
```

#### 1.3 MSI/MSI-X Capability Detection
```rust
pub fn detect_msi_capability(pci_device: &PciDevice) -> Option<MsiCapability> {
    // Scan capability list starting at offset 0x34
    let mut cap_ptr = pci_device.read_config_byte(0x34);
    
    while cap_ptr != 0 {
        let cap_id = pci_device.read_config_byte(cap_ptr);
        
        if cap_id == 0x05 {
            return Some(MsiCapability::from_pci(pci_device, cap_ptr));
        }
        
        cap_ptr = pci_device.read_config_byte(cap_ptr + 1);
    }
    
    None
}
```

### Testing Strategy
- Unit tests for PCI configuration space reading
- Integration tests with QEMU virtio devices
- Mock hardware simulator for testing without real hardware

### Documentation
- [PCI Bus Architecture](PCI-Bus-Architecture) (to be created)
- [Device Driver Development](Device-Drivers)

---

## 2. Basic GPU Driver Support (Intel/AMD)

### Current State
SigmaOS has no real GPU acceleration. All graphics are simulated.

### Implementation Plan

#### 2.1 Intel Graphics Driver
```rust
// src/drivers/gpu/intel.rs
pub struct IntelGpuDevice {
    pci_device: PciDevice,
    mmio_base: usize,
    gt_sysmmio: usize,
}

impl IntelGpuDevice {
    pub fn init(&mut self) -> Result<(), &'static str> {
        // Map MMIO registers
        self.mmio_base = self.pci_device.map_bar(0)?;
        
        // Initialize GPU
        self.init_display();
        self.init_pipelines();
        
        Ok(())
    }
    
    pub fn init_display(&self) {
        // Configure display pipeline
        self.write_reg(0x69840, 0x80000000); // PIPECONF enable
    }
}
```

#### 2.2 AMDGPU Driver (Basic)
```rust
// src/drivers/gpu/amd.rs
pub struct AmdGpuDevice {
    pci_device: PciDevice,
    mmio_base: usize,
    fb_base: usize,
}

impl AmdGpuDevice {
    pub fn init(&mut self) -> Result<(), &'static str> {
        self.mmio_base = self.pci_device.map_bar(0)?;
        self.fb_base = self.pci_device.map_bar(2)?;
        
        self.init_display();
        Ok(())
    }
}
```

#### 2.3 Mode Setting Infrastructure
```rust
// src/drivers/gpu/mod.rs
pub struct DisplayMode {
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u32,
    pub pixel_format: PixelFormat,
}

pub trait GpuDriver {
    fn set_mode(&self, mode: &DisplayMode) -> Result<(), &'static str>;
    fn get_modes(&self) -> Vec<DisplayMode>;
    fn enable_output(&self, output_id: u32) -> Result<(), &'static str>;
}
```

### Testing Strategy
- QEMU with virtio-gpu for basic testing
- Real hardware testing with Intel graphics (most common)
- Framebuffer verification tests

### Documentation
- [Intel Graphics Driver](Intel-Graphics-Driver) (to be created)
- [AMDGPU Driver](AMDGPU-Driver) (to be created)
- [Mode Setting](Mode-Setting) (to be created)

---

## 3. Demand Paging with Swap Support

### Current State
SigmaOS manages memory via Rust heap allocators but lacks real swap partition page-out/page-in pipelines.

### Implementation Plan

#### 3.1 Page Fault Handler
```rust
// src/memory/paging.rs
pub fn handle_page_fault(fault_address: usize, error_code: u32) {
    let current_task = current_task();
    
    if error_code & 0x1 == 0 {
        // Page not present - handle demand paging
        handle_demand_paging(fault_address, current_task);
    } else if error_code & 0x2 != 0 {
        // Write fault - handle copy-on-write
        handle_cow(fault_address, current_task);
    }
}

fn handle_demand_paging(address: usize, task: &Task) {
    let page_table = task.page_table();
    
    if let Some(physical_page) = page_table.get_entry(address) {
        // Page exists but not mapped - map it
        page_table.map_page(address, physical_page);
    } else {
        // Page doesn't exist - allocate or swap in
        allocate_or_swap_in(address, task);
    }
}
```

#### 3.2 Swap Manager
```rust
// src/memory/swap.rs
pub struct SwapManager {
    swap_device: BlockDevice,
    swap_map: Bitmap,
    swap_slots: Vec<SwapSlot>,
}

impl SwapManager {
    pub fn swap_out(&mut self, page: usize) -> Result<(), &'static str> {
        // Find free swap slot
        let slot = self.allocate_swap_slot()?;
        
        // Write page to swap
        self.swap_device.write_block(slot.sector, page_to_physical(page))?;
        
        // Update page table to mark as swapped
        mark_page_swapped(page, slot);
        
        Ok(())
    }
    
    pub fn swap_in(&mut self, page: usize) -> Result<(), &'static str> {
        let slot = get_swap_slot(page)?;
        
        // Read from swap
        let physical_page = self.swap_device.read_block(slot.sector)?;
        
        // Map back into page table
        current_task().page_table().map_page(page, physical_page);
        
        Ok(())
    }
}
```

#### 3.3 zswap Compression
```rust
// src/memory/zswap.rs
pub struct ZswapPool {
    compressed_pages: HashMap<usize, CompressedPage>,
    lru: LruCache<usize>,
}

impl ZswapPool {
    pub fn compress_and_store(&mut self, page: usize) -> Result<(), &'static str> {
        let data = read_page(page);
        let compressed = zstd_compress(data)?;
        
        self.compressed_pages.insert(page, compressed);
        self.lru.put(page);
        
        Ok(())
    }
}
```

### Testing Strategy
- Page fault injection tests
- Swap performance benchmarks
- Memory pressure simulation

### Documentation
- [Memory Management](Memory-Management) (to be created)
- [Swap Configuration](Swap-Configuration) (to be created)
- [Demand Paging](Demand-Paging) (to be created)

---

## 4. Kernel-Space Syscall Enforcement (Pledge/Unveil)

### Current State
Pledge/unveil are implemented as userland checks but lack kernel-space syscall entry trap enforcement.

### Implementation Plan

#### 4.1 Pledge Enforcement
```rust
// src/security/pledge.rs
pub struct PledgeContext {
    allowed_promises: PledgePromises,
    current_promises: PledgePromises,
}

pub fn check_pledge(syscall: SyscallNumber) -> bool {
    let task = current_task();
    let context = task.pledge_context();
    
    if !context.current_promises.allows(syscall) {
        // Violation - terminate process
        terminate_process(task);
        return false;
    }
    
    true
}

// Hook into syscall entry
#[no_mangle]
pub extern "C" syscall_entry(syscall: SyscallNumber) -> usize {
    if !check_pledge(syscall) {
        // Signal sent in check_pledge
        return -1;
    }
    
    // Proceed with syscall
    dispatch_syscall(syscall)
}
```

#### 4.2 Unveil Enforcement
```rust
// src/security/unveil.rs
pub struct UnveilContext {
    allowed_paths: Vec<UnveilPath>,
    mode: UnveilMode,
}

pub fn check_unveil(path: &str, operation: FileOperation) -> bool {
    let task = current_task();
    let context = task.unveil_context();
    
    for allowed in &context.allowed_paths {
        if path_matches(path, &allowed.path) {
            return allowed.mode.allows(operation);
        }
    }
    
    // Path not unveiled - deny
    false
}

// Hook into VFS operations
pub fn vfs_open(path: &str) -> Result<FileDescriptor, &'static str> {
    if !check_unveil(path, FileOperation::Read) {
        return Err("Path not unveiled");
    }
    
    // Proceed with open
    real_vfs_open(path)
}
```

### Testing Strategy
- Syscall violation tests
- Path restriction tests
- Process termination verification

### Documentation
- [Pledge Security](Pledge-Security) (to be created)
- [Unveil Security](Unveil-Security) (to be created)
- [Kernel Security Hooks](Kernel-Security-Hooks) (to be created)

---

## 5. Basic eBPF JIT Compilation

### Current State
XDP filtering is implemented as Rust methods but lacks eBPF JIT compilation.

### Implementation Plan

#### 5.1 eBPF Interpreter
```rust
// src/ebpf/interpreter.rs
pub struct EbpfProgram {
    instructions: Vec<EbpfInstruction>,
    maps: Vec<EbpfMap>,
}

impl EbpfProgram {
    pub fn execute(&self, ctx: &mut EbpfContext) -> Result<u64, EbpfError> {
        let mut registers = [0u64; 11];
        let mut pc = 0;
        
        while pc < self.instructions.len() {
            let insn = &self.instructions[pc];
            
            match insn.opcode {
                EbpfOpcode::Add => {
                    let dst = insn.dst as usize;
                    let src = insn.src as usize;
                    registers[dst] = registers[dst].wrapping_add(registers[src]);
                }
                EbpfOpcode::Load => {
                    registers[insn.dst as usize] = insn.imm as u64;
                }
                // ... more instructions
            }
            
            pc += 1;
        }
        
        Ok(registers[0])
    }
}
```

#### 5.2 JIT Compiler
```rust
// src/ebpf/jit.rs
pub struct EbpfJit {
    codegen: CodeGenerator,
}

impl EbpfJit {
    pub fn compile(&mut self, program: &EbpfProgram) -> Result<JitCode, JitError> {
        for insn in &program.instructions {
            self.compile_instruction(insn)?;
        }
        
        Ok(self.codegen.finalize())
    }
    
    fn compile_instruction(&mut self, insn: &EbpfInstruction) -> Result<(), JitError> {
        match insn.opcode {
            EbpfOpcode::Add => {
                self.codegen.emit_add_reg_reg(insn.dst, insn.src);
            }
            // ... more instructions
        }
        
        Ok(())
    }
}
```

#### 5.3 XDP Integration
```rust
// src/networking/xdp.rs
pub struct XdpProgram {
    program: EbpfProgram,
    jit_code: Option<JitCode>,
}

impl XdpProgram {
    pub fn attach(&mut self, interface: &NetworkInterface) -> Result<(), XdpError> {
        // Compile to native code
        self.jit_code = Some(EbpfJit::compile(&self.program)?);
        
        // Attach to driver hook
        interface.set_xdp_hook(self.jit_code.as_ref().unwrap());
        
        Ok(())
    }
    
    pub fn process_packet(&self, packet: &mut NetworkPacket) -> XdpAction {
        let mut ctx = EbpfContext::from_packet(packet);
        let result = self.program.execute(&mut ctx);
        
        XdpAction::from(result)
    }
}
```

### Testing Strategy
- eBPF program execution tests
- JIT compilation verification
- XDP packet filtering tests

### Documentation
- [eBPF Programming](eBPF-Programming) (to be created)
- [XDP Networking](XDP-Networking) (to be created)
- [JIT Compilation](JIT-Compilation) (to be created)

---

## Success Criteria

Phase 1 is considered complete when:

1. **PCI/PCIe**: Can enumerate and access real PCI devices on QEMU and hardware
2. **GPU Drivers**: Can set display mode and output video on Intel graphics
3. **Demand Paging**: Page faults trigger page allocation or swap-in correctly
4. **Pledge/Unveil**: Syscall violations terminate processes at kernel level
5. **eBPF JIT**: Can compile and execute eBPF programs with native performance

## Testing Requirements

Each feature must have:
- Unit tests for core functionality
- Integration tests with QEMU
- Documentation and examples
- Performance benchmarks where applicable

## Dependencies

Phase 1 has no external dependencies and can proceed immediately. All features build on the existing std-based architecture.

## Next Steps

After Phase 1 completion:
- Proceed to Phase 2 (Core Features)
- Update documentation
- Create driver development guides
- Establish hardware testing lab

---

**[Phase 2 Implementation Plan](Phase-2-Gap-Closure-Implementation-Plan)** | **[Hardware Support Matrix](SUPPORT_MATRIX)** | **[Kernel Development](Category-Development)**
