//! KVM-style Hardware Virtualization
//! Virtual machine monitor using CPU virtualization extensions
//! Reference: Linux KVM and QEMU/KVM

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

/// VM identifier
pub type VmId = u32;

/// Virtual CPU identifier
pub type VcpuId = u32;

/// VM exit reasons (from guest to host)
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VmExitReason {
    Halt = 0,
    Io = 1,
    Mmio = 2,
    Cpuid = 3,
    Msr = 4,
    Interrupt = 5,
    Exception = 6,
    Hypercall = 7,
    Shutdown = 8,
}

/// VCPU run structure (shared with guest)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VcpuRun {
    pub exit_reason: u32,
    pub request_interrupt_window: u8,
    pub immediate_exit: u8,
    pub padding: [u8; 6],

    // Exit-specific data (union in C)
    pub exit_data: VmExitData,
}

/// VM exit data (variant based on exit_reason)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VmExitData {
    pub io: IoExit,
    // Other variants would overlay this
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IoExit {
    pub direction: u8, // 0=in, 1=out
    pub size: u8,      // 1, 2, 4, 8 bytes
    pub port: u16,
    pub count: u32,
    pub data_offset: u64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MmioExit {
    pub phys_addr: u64,
    pub data: [u8; 8],
    pub len: u32,
    pub is_write: u8,
}

/// VCPU registers (x86-64)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VcpuRegs {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rsp: u64,
    pub rbp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rip: u64,
    pub rflags: u64,
}

/// VCPU special registers
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VcpuSregs {
    pub cs: Segment,
    pub ds: Segment,
    pub es: Segment,
    pub fs: Segment,
    pub gs: Segment,
    pub ss: Segment,
    pub tr: Segment,
    pub ldt: Segment,
    pub gdt: Dtable,
    pub idt: Dtable,
    pub cr0: u64,
    pub cr2: u64,
    pub cr3: u64,
    pub cr4: u64,
    pub cr8: u64,
    pub efer: u64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Segment {
    pub base: u64,
    pub limit: u32,
    pub selector: u16,
    pub seg_type: u8,
    pub present: u8,
    pub dpl: u8,
    pub db: u8,
    pub s: u8,
    pub l: u8,
    pub g: u8,
    pub avl: u8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Dtable {
    pub base: u64,
    pub limit: u16,
}

/// Memory region for VM
#[derive(Debug, Clone)]
pub struct MemoryRegion {
    pub slot: u32,
    pub flags: u32,
    pub guest_phys_addr: u64,
    pub memory_size: u64,
    pub userspace_addr: u64, // Host virtual address
}

/// IRQ routing entry
#[derive(Debug, Clone, Copy)]
pub struct IrqRoute {
    pub gsi: u32, // Global System Interrupt
    pub irq_type: IrqType,
    pub chip: u32,
    pub pin: u32,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum IrqType {
    Irqchip = 0,
    Msi = 1,
}

/// Virtual CPU structure
pub struct Vcpu {
    pub id: VcpuId,
    pub vm_id: VmId,
    pub regs: VcpuRegs,
    pub sregs: VcpuSregs,
    pub run: VcpuRun,
    pub halted: bool,
}

impl Vcpu {
    pub fn new(id: VcpuId, vm_id: VmId) -> Self {
        Self {
            id,
            vm_id,
            regs: unsafe { core::mem::zeroed() },
            sregs: unsafe { core::mem::zeroed() },
            run: unsafe { core::mem::zeroed() },
            halted: false,
        }
    }

    /// Run VCPU (enter guest mode)
    pub fn run(&mut self) -> Result<VmExitReason, KvmError> {
        if self.halted {
            return Ok(VmExitReason::Halt);
        }

        // In real implementation:
        // 1. Load guest state (VMCS on Intel, VMCB on AMD)
        // 2. Execute VMLAUNCH/VMRUN
        // 3. Handle VM exit
        // 4. Save guest state

        let exit_reason = VmExitReason::Halt;
        Ok(exit_reason)
    }

    /// Set general purpose registers
    pub fn set_regs(&mut self, regs: VcpuRegs) {
        self.regs = regs;
    }

    /// Get general purpose registers
    pub fn get_regs(&self) -> VcpuRegs {
        self.regs
    }

    /// Set special registers
    pub fn set_sregs(&mut self, sregs: VcpuSregs) {
        self.sregs = sregs;
    }

    /// Get special registers
    pub fn get_sregs(&self) -> VcpuSregs {
        self.sregs
    }
}

/// Virtual machine structure
pub struct Vm {
    pub id: VmId,
    pub vcpus: Vec<Vcpu>,
    pub memory_regions: Vec<MemoryRegion>,
    pub irq_routes: Vec<IrqRoute>,
    pub paused: bool,
}

impl Vm {
    pub fn new(id: VmId) -> Self {
        Self {
            id,
            vcpus: Vec::new(),
            memory_regions: Vec::new(),
            irq_routes: Vec::new(),
            paused: false,
        }
    }

    /// Create VCPU
    pub fn create_vcpu(&mut self, vcpu_id: VcpuId) -> Result<(), KvmError> {
        if self.vcpus.iter().any(|v| v.id == vcpu_id) {
            return Err(KvmError::VcpuExists);
        }

        let vcpu = Vcpu::new(vcpu_id, self.id);
        self.vcpus.push(vcpu);
        Ok(())
    }

    /// Get VCPU
    pub fn get_vcpu(&mut self, vcpu_id: VcpuId) -> Option<&mut Vcpu> {
        self.vcpus.iter_mut().find(|v| v.id == vcpu_id)
    }

    /// Set user memory region
    pub fn set_user_memory_region(&mut self, region: MemoryRegion) -> Result<(), KvmError> {
        // Validate region doesn't overlap with existing regions
        for existing in &self.memory_regions {
            let existing_end = existing.guest_phys_addr + existing.memory_size;
            let new_end = region.guest_phys_addr + region.memory_size;

            if (region.guest_phys_addr < existing_end) && (new_end > existing.guest_phys_addr) {
                return Err(KvmError::MemoryOverlap);
            }
        }

        self.memory_regions.push(region);
        Ok(())
    }

    /// Inject IRQ into VM
    pub fn irq_line(&mut self, _irq: u32, _level: bool) -> Result<(), KvmError> {
        // Set IRQ line state
        // In real implementation: trigger VCPU interrupt
        Ok(())
    }

    /// Create IRQ routing
    pub fn set_gsi_routing(&mut self, routes: Vec<IrqRoute>) -> Result<(), KvmError> {
        self.irq_routes = routes;
        Ok(())
    }
}

/// KVM hypervisor
pub struct Kvm {
    pub vms: BTreeMap<VmId, Vm>,
    pub next_vm_id: VmId,
    pub api_version: u32,
}

impl Kvm {
    pub fn new() -> Self {
        Self {
            vms: BTreeMap::new(),
            next_vm_id: 1,
            api_version: 12, // KVM API version
        }
    }

    /// Check KVM extensions support
    pub fn check_extension(&self, extension: KvmExtension) -> bool {
        // Check if CPU supports virtualization
        match extension {
            KvmExtension::Irqchip => true,
            KvmExtension::User_memory => true,
            KvmExtension::IrqRouting => true,
            _ => false,
        }
    }

    /// Get API version
    pub fn get_api_version(&self) -> u32 {
        self.api_version
    }

    /// Create VM
    pub fn create_vm(&mut self) -> VmId {
        let vm_id = self.next_vm_id;
        self.next_vm_id += 1;

        let vm = Vm::new(vm_id);
        self.vms.insert(vm_id, vm);
        vm_id
    }

    /// Get VM
    pub fn get_vm(&mut self, vm_id: VmId) -> Option<&mut Vm> {
        self.vms.get_mut(&vm_id)
    }

    /// Destroy VM
    pub fn destroy_vm(&mut self, vm_id: VmId) -> Result<(), KvmError> {
        self.vms.remove(&vm_id).ok_or(KvmError::VmNotFound)?;
        Ok(())
    }
}

/// KVM extensions
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum KvmExtension {
    Irqchip = 0,
    User_memory = 3,
    IrqRouting = 7,
    Ioeventfd = 36,
    Irqfd = 32,
}

/// KVM error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KvmError {
    VmNotFound,
    VcpuExists,
    VcpuNotFound,
    MemoryOverlap,
    InvalidMemoryRegion,
    HardwareNotSupported,
}

/// CPU virtualization capabilities check
pub fn check_virtualization_support() -> bool {
    // Check CPUID for VMX (Intel) or SVM (AMD)
    // CPUID.1:ECX.VMX[bit 5] or CPUID.0x80000001:ECX.SVM[bit 2]
    true // Placeholder
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kvm_create_vm() {
        let mut kvm = Kvm::new();
        let vm_id = kvm.create_vm();
        assert!(kvm.get_vm(vm_id).is_some());
    }

    #[test]
    fn test_vcpu_create() {
        let mut vm = Vm::new(1);
        assert!(vm.create_vcpu(0).is_ok());
        assert!(vm.create_vcpu(0).is_err()); // Duplicate
    }

    #[test]
    fn test_memory_region() {
        let mut vm = Vm::new(1);
        let region = MemoryRegion {
            slot: 0,
            flags: 0,
            guest_phys_addr: 0,
            memory_size: 0x10000,
            userspace_addr: 0x100000,
        };
        assert!(vm.set_user_memory_region(region).is_ok());
    }
}
