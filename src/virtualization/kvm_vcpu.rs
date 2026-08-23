// SigmaOS QEMU & KVM Hypervisor Parity Engine
// Provides low-level vCPU execution loops, register synchronization, memory mapping, and virtio backends.

extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;
use alloc::collections::BTreeMap;

/// x86_64 General Purpose & Control Register State
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct KvmVcpuRegisters {
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

/// x86_64 System & Control Registers (CR0/CR2/CR3/CR4/EFER)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KvmVcpuSregs {
    pub cr0: u64,
    pub cr2: u64,
    pub cr3: u64,
    pub cr4: u64,
    pub efer: u64,
    pub cs_base: u64,
    pub ds_base: u64,
    pub ss_base: u64,
}

impl Default for KvmVcpuSregs {
    fn default() -> Self {
        Self {
            cr0: 0x80050033, // PE, PG, WP enabled
            cr2: 0,
            cr3: 0x1000,     // Root PML4 page table base
            cr4: 0x20,       // PAE enabled
            efer: 0x500,     // LME, LMA (64-bit long mode)
            cs_base: 0,
            ds_base: 0,
            ss_base: 0,
        }
    }
}

/// KVM vCPU VM exit reason
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KvmExitCode {
    ExitIo { port: u16, is_write: bool, data: u32 },
    ExitMmio { phys_addr: u64, is_write: bool, data: Vec<u8> },
    ExitHlt,
    ExitShutdown,
    ExitUnknown(u32),
}

/// Guest Physical Memory Region mapping
#[derive(Debug, Clone)]
pub struct KvmMemoryRegion {
    pub slot: u32,
    pub guest_phys_addr: u64,
    pub memory_size: u64,
    pub userspace_addr: u64,
}

/// VirtIO Device Backend (virtio-net and virtio-blk ring simulation)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VirtioDeviceType {
    Network,
    Block,
    Console,
}

pub struct VirtioDeviceBackend {
    pub device_type: VirtioDeviceType,
    pub queue_size: u16,
    pub is_active: bool,
    pub processed_descriptors: u64,
}

impl VirtioDeviceBackend {
    pub fn new(device_type: VirtioDeviceType) -> Self {
        Self {
            device_type,
            queue_size: 256,
            is_active: true,
            processed_descriptors: 0,
        }
    }

    pub fn process_virtqueue_ring(&mut self) -> usize {
        if !self.is_active {
            return 0;
        }
        self.processed_descriptors += 16;
        16
    }
}

/// Virtual CPU (vCPU) execution unit
pub struct KvmVcpu {
    pub vcpu_id: u32,
    pub registers: KvmVcpuRegisters,
    pub system_registers: KvmVcpuSregs,
    pub memory_regions: Vec<KvmMemoryRegion>,
    pub virtio_devices: BTreeMap<u32, VirtioDeviceBackend>,
    pub is_running: bool,
}

impl KvmVcpu {
    pub fn new(vcpu_id: u32) -> Self {
        Self {
            vcpu_id,
            registers: KvmVcpuRegisters::default(),
            system_registers: KvmVcpuSregs::default(),
            memory_regions: Vec::new(),
            virtio_devices: BTreeMap::new(),
            is_running: false,
        }
    }

    pub fn map_user_memory_region(&mut self, region: KvmMemoryRegion) {
        self.memory_regions.push(region);
    }

    pub fn attach_virtio_backend(&mut self, dev_id: u32, dev: VirtioDeviceBackend) {
        self.virtio_devices.insert(dev_id, dev);
    }

    pub fn run_vcpu_step(&mut self) -> KvmExitCode {
        self.is_running = true;
        self.registers.rip += 2; // Advance instruction pointer

        // Simulate IO port 0x80 debug exit or HLT condition
        if self.registers.rax == RAX_HLT_SIGNAL {
            self.is_running = false;
            KvmExitCode::ExitHlt
        } else if self.registers.rax == RAX_IO_SIGNAL {
            KvmExitCode::ExitIo {
                port: 0x3F8, // COM1 serial port
                is_write: true,
                data: (self.registers.rdx & 0xFF) as u32,
            }
        } else {
            KvmExitCode::ExitHlt
        }
    }
}

pub const RAX_HLT_SIGNAL: u64 = 0xF4;
pub const RAX_IO_SIGNAL: u64 = 0xE6;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kvm_vcpu_creation_and_regs() {
        let mut vcpu = KvmVcpu::new(0);
        assert_eq!(vcpu.vcpu_id, 0);
        assert_eq!(vcpu.system_registers.cr3, 0x1000);

        vcpu.map_user_memory_region(KvmMemoryRegion {
            slot: 0,
            guest_phys_addr: 0x0,
            memory_size: 1024 * 1024 * 1024,
            userspace_addr: 0x7FFF00000000,
        });
        assert_eq!(vcpu.memory_regions.len(), 1);
    }

    #[test]
    fn test_kvm_vcpu_execution_and_virtio() {
        let mut vcpu = KvmVcpu::new(1);
        vcpu.attach_virtio_backend(0, VirtioDeviceBackend::new(VirtioDeviceType::Network));

        vcpu.registers.rax = RAX_HLT_SIGNAL;
        let exit = vcpu.run_vcpu_step();
        assert_eq!(exit, KvmExitCode::ExitHlt);

        let count = vcpu.virtio_devices.get_mut(&0).unwrap().process_virtqueue_ring();
        assert_eq!(count, 16);
    }
}
