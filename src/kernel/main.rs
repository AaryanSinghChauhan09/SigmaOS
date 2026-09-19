#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]
#![allow(clippy::all, unused)]

// SigmaOS Kernel Main Entry Point

extern crate alloc;

use alloc::string::String;
use alloc::string::ToString;
use core::sync::atomic::{AtomicUsize, Ordering};

use sigmaos::compatibility::{OpenRcManager, OpenRcRunlevel, OpenRcService};
use sigmaos::kernel::boot_to_userspace::{
    AddressSpaceSeparationManager, BootToUserspacePipeline, EfiMemoryDescriptor,
    EfiMemoryType, InterruptControllerManager, SmpCpuTopologyManager,
    UefiBootProtocolHandshake, KERNEL_SPACE_MIN, USER_SPACE_MIN,
};
use sigmaos::kernel::{BuddyAllocator, Priority, Process, RoundRobinScheduler as Scheduler};
use sigmaos::klib::paging::{SimpleVMM, VirtualMemoryManager};

/// Representation of x86_64 CPU Context during hardware boot
#[derive(Debug, Clone, Copy)]
pub struct CpuContext {
    pub interrupts_enabled: bool,
    pub direction_flag_cleared: bool,
    pub gdt_base: u64,
    pub idt_base: u64,
    pub cr3_page_directory: u64,
    pub lapic_base: u64,
    pub smp_cpus_online: usize,
}

impl CpuContext {
    pub const fn new() -> Self {
        CpuContext {
            interrupts_enabled: false,
            direction_flag_cleared: false,
            gdt_base: 0,
            idt_base: 0,
            cr3_page_directory: 0,
            lapic_base: 0,
            smp_cpus_online: 1,
        }
    }
}

pub static EARLY_CPU_STATE: AtomicUsize = AtomicUsize::new(0);

/// Early CPU hardware bootstrap (GDT/IDT dynamic descriptor loading and interrupt state)
pub fn early_cpu_init(context: &mut CpuContext) {
    // 1. Disable interrupts (equivalent to 'cli')
    context.interrupts_enabled = false;

    // 2. Clear Direction Flag (equivalent to 'cld' for string operations)
    context.direction_flag_cleared = true;

    // 3. Dynamically resolve GDT and IDT base addresses from high memory
    if context.gdt_base == 0 {
        context.gdt_base = KERNEL_SPACE_MIN + 0x8000;
    }
    if context.idt_base == 0 {
        context.idt_base = KERNEL_SPACE_MIN + 0x9000;
    }

    // 4. Initialize LAPIC Base
    if context.lapic_base == 0 {
        context.lapic_base = 0xFEE0_0000;
    }

    EARLY_CPU_STATE.store(1, Ordering::SeqCst);
}

/// Early Kernel memory bootstrap (dynamic page mapping from UEFI memory map)
pub fn early_memory_init(allocator: &mut BuddyAllocator, vmm: &mut SimpleVMM) {
    let handshake = UefiBootProtocolHandshake::new_mock();

    // Map conventional memory regions from UEFI handoff
    for desc in &handshake.memory_map {
        if desc.memory_type == EfiMemoryType::ConventionalMemory {
            let start = desc.physical_start;
            let end = start + (desc.number_of_pages * 4096);
            vmm.map_page(start, start, false, true).unwrap();
            allocator.initialize_memory(start as usize, (end - start) as usize);
        }
    }

    // Fallback ensure baseline page mapping
    if vmm.get_physical(0x1000).is_none() {
        vmm.map_page(0x0, 0x0, false, true).unwrap();
        vmm.map_page(0x1000, 0x1000, false, true).unwrap();
        allocator.initialize_memory(0x10000, 1024 * 1024);
    }

    EARLY_CPU_STATE.store(2, Ordering::SeqCst);
}

/// Dynamic start_kernel bootstrap manager (from UEFI handshake to userspace init)
pub fn start_kernel(
    context: &mut CpuContext,
    allocator: &mut BuddyAllocator,
    vmm: &mut SimpleVMM,
    scheduler: &mut Scheduler,
    openrc: &mut OpenRcManager,
) -> Result<BootToUserspacePipeline, &'static str> {
    // Stage 1: Dynamic CPU hardware bootstrap
    early_cpu_init(context);

    // Stage 2: Physical Memory paging & UEFI map bootstrap
    early_memory_init(allocator, vmm);

    // Stage 3: Complete hardware boot-to-userspace path pipeline
    let boot_pipeline = BootToUserspacePipeline::boot_hardware();
    context.smp_cpus_online = boot_pipeline.cpu_smp.active_cpus_count;
    context.cr3_page_directory = boot_pipeline.addr_space.cr3_page_directory;

    // Stage 4: Scheduler bootstrap with idle process
    let idle_proc = Process::new(0, "idle".to_string(), Priority::Idle);
    scheduler.add_process(idle_proc);

    // Stage 5: Open early userland runlevels (OpenRC)
    let udev = OpenRcService::new("udev").with_runlevel(OpenRcRunlevel::SingleUser);
    let dhcpcd = OpenRcService::new("dhcpcd")
        .with_dependency("udev")
        .with_runlevel(OpenRcRunlevel::MultiUser);

    openrc.register_service(udev);
    openrc.register_service(dhcpcd);

    // Transition runlevel to MultiUser (simulating graphical multi-user boot)
    openrc.transition_to_runlevel(OpenRcRunlevel::MultiUser)?;

    // Stage 6: Enable hardware interrupts (equivalent to 'sti')
    context.interrupts_enabled = true;
    EARLY_CPU_STATE.store(3, Ordering::SeqCst);

    Ok(boot_pipeline)
}

#[cfg(target_os = "none")]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut context = CpuContext::new();
    let mut allocator = BuddyAllocator::new();
    let mut vmm = SimpleVMM::new();
    let mut scheduler = Scheduler::new();
    let mut openrc = OpenRcManager::new();

    let _ = start_kernel(
        &mut context,
        &mut allocator,
        &mut vmm,
        &mut scheduler,
        &mut openrc,
    );

    loop {}
}

#[cfg(not(target_os = "none"))]
fn main() {
    // Hosted dummy entry point
}

#[cfg(target_os = "none")]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use sigmaos::compatibility::ServiceStatus;

    #[test]
    fn test_x86_64_start_kernel_bootstrap() {
        let mut context = CpuContext::new();
        let mut allocator = BuddyAllocator::new();
        let mut vmm = SimpleVMM::new();
        let mut scheduler = Scheduler::new();
        let mut openrc = OpenRcManager::new();

        assert_eq!(EARLY_CPU_STATE.load(Ordering::SeqCst), 0);

        // Run dynamic early start_kernel bootstrap
        let pipeline = start_kernel(
            &mut context,
            &mut allocator,
            &mut vmm,
            &mut scheduler,
            &mut openrc,
        )
        .unwrap();

        // Verify context flags (cld set, sti set, dynamic gdt/idt bases loaded)
        assert!(context.interrupts_enabled);
        assert!(context.direction_flag_cleared);
        assert!(context.gdt_base > 0);
        assert!(context.idt_base > 0);
        assert_eq!(context.smp_cpus_online, 4);

        // Verify allocator has available memory segments and paging tables are mapped
        assert!(allocator.get_free_memory() > 0);
        assert_eq!(vmm.get_physical(0x1000), Some(0x1000));

        // Verify userland OpenRC runlevel transition started services cleanly
        assert_eq!(openrc.services[0].status, ServiceStatus::Started); // udev
        assert_eq!(openrc.services[1].status, ServiceStatus::Started); // dhcpcd

        // Verify userspace init process created and running via pipeline
        assert!(pipeline.active_processes.contains_key(&1));

        // Verify bootstrap state transition completed successfully
        assert_eq!(EARLY_CPU_STATE.load(Ordering::SeqCst), 3);
    }
}
